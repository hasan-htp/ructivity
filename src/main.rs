use chrono::Utc;
use evdev::{Device, InputEventKind, Key};

use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

struct Entry {
    key : Key,
    time_stamp : String, //TODO: use a struct and format it in log_writer_thread
}

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <input_device_path> <output_log_file_path_to_create>");
        return Ok(());
    }

    let inputpath = &args[1];
    let mut device = Device::open(inputpath)?;

    let outputpath = &args[2];
    let mut log_file = File::create(outputpath)?;
    
    let (tx,rx): (Sender<Entry>, Receiver<Entry>) = mpsc::channel();

    let key_event_thread = thread::spawn(move ||  -> std::io::Result<()> {
        loop {
            let events = match device.fetch_events() {
                Ok(ev) => ev,
                Err(e) => {
                    eprintln!("fetch_events failed: {}", e);
                    break Err(e) ;
                }
            };

            for ev in events {
                if let InputEventKind::Key(key) = ev.kind() {
                    // let's print only when a key released (1 on pressed, 0 on released)
                    if ev.value() == 0 {
                        let utc_now: chrono::DateTime<Utc>= Utc::now();
                        let entry = Entry{
                            key: key,
                            time_stamp: format!("{}",utc_now.format("%Y-%m-%d %H:%M:%S.%6f")),
                        };
                        match tx.send(entry) {
                            Ok(()) => {}
                            Err(e) => {
                                eprintln!("failed to send: {}",e);
                            }
                        }                    
                    }
                }
            }
        }
    });

    let log_writer_thread = thread::spawn(move || -> std::io::Result<()> {
        loop {
            match  rx.recv() {
                Ok(entry) => {
                    let line = format!("{}, {:?}", entry.time_stamp, entry.key);
                    writeln!(log_file, "{}", line)?;
                    println!("{}", line);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    });

    match key_event_thread.join() {
        Ok(_) => println!("key_event_thread ok"),
        Err(e) => eprintln!("key_event_thread panicked: {:?}", e),
    }

    match log_writer_thread.join() {
        Ok(_) => println!("log_writer_thread ok"),
        Err(e) => eprintln!("log_writer_thread panicked: {:?}", e),
    }
    
    Ok(())

}
