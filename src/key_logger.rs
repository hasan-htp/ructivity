use crate::event_listener::Event;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};

pub fn key_logger(rx: Receiver<Event>, mut log_file: File) -> JoinHandle<std::io::Result<()>> {
    thread::spawn(move || -> std::io::Result<()> {
        loop {
            match rx.recv() {
                Ok(entry) => match entry {
                    Event::Key(key_event) => {
                        let line = format!("{}, {:?}", key_event.time_stamp, key_event.key);
                        writeln!(log_file, "{}", line)?;
                    }
                    Event::Mouse(mouse_event) => {
                        let line = format!("{}", mouse_event.time_stamp);
                        writeln!(log_file, "{}", line)?;
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    })
}
