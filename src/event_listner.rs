use std::thread::{self, JoinHandle};
use evdev::{Device, InputEventKind, Key};
use chrono::Utc;
use std::sync::mpsc::{Sender};


pub struct Entry {
    pub key : Key,
    pub time_stamp : String, //TODO: use a struct and format it in log_writer_thread
}

pub fn event_listner(inputpath: &str , tx: Sender<Entry>) -> std::io::Result<JoinHandle<std::io::Result<()>>>{

    let mut _device = Device::open(inputpath)?;

    let handler = thread::spawn(move ||  -> std::io::Result<()> {
        loop {
            let events = match _device.fetch_events() {
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
    Ok(handler)
}