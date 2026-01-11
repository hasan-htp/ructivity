use std::thread;
//use evdev::{Device, InputEventKind, Key};
use evdev::{Device, InputEventKind};
use std::env;
use chrono::Utc;
use std::fs::File;
use std::io::Write;

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
    
    let worker_thread = thread::spawn(move ||  -> std::io::Result<()> {
        println!("worker_thread:");
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
                        let now= Utc::now();
                        let line = format!("{} {:?}", now.format("%Y-%m-%d %H:%M:%S.%6f: "), key);
                        writeln!(log_file, "{}", line)?;
                        println!("{}", line);
                    }
                }
            }
        }
    });

    match worker_thread.join() {
        Ok(_) => println!("thread ok"),
        Err(e) => eprintln!("thread panicked: {:?}", e),
    }

    Ok(())
}
