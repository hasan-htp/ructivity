use std::thread;
//use evdev::{Device, InputEventKind, Key};
use evdev::{Device, InputEventKind};
use std::env;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <input_device_path>");
        return Ok(());
    }

    let path = &args[1];
    let mut device = Device::open(path)?;

    let worker_thread = thread::spawn(move || {
        println!("worker_thread:");
        loop {
            let events = match device.fetch_events() {
                Ok(ev) => ev,
                Err(e) => {
                    eprintln!("fetch_events failed: {}", e);
                    break;
                }
            };

            for ev in events {
                if let InputEventKind::Key(key) = ev.kind() {
                    // let's print only when a key released
                    if ev.value() == 0{
                        println!("{:?}", key);
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
