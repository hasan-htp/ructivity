use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;


mod event_listner;

use crate::event_listner::event_listner;
use crate::event_listner::Entry;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <input_device_path> <output_log_file_path_to_create>");
        return Ok(());
    }

    let inputpath = &args[1];
    let outputpath = &args[2];
    
    let mut log_file = File::create(outputpath)?;
    
    let (tx,rx): (Sender<Entry>, Receiver<Entry>) = mpsc::channel();

    let key_event_thread = event_listner(inputpath,tx);

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

    match key_event_thread?.join() {
        Ok(_) => println!("key_event_thread ok"),
        Err(e) => eprintln!("key_event_thread panicked: {:?}", e),
    }

    match log_writer_thread.join() {
        Ok(_) => println!("log_writer_thread ok"),
        Err(e) => eprintln!("log_writer_thread panicked: {:?}", e),
    }
    
    Ok(())

}
