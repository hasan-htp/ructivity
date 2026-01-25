use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;

mod event_listener;

use crate::event_listener::event_listener;
use crate::event_listener::Entry;

fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <output_log_file_path_to_create>");
        return Ok(());
    }

    let outputpath = &args[1];
    
    let mut log_file = File::create(outputpath)?;
    
    let (tx,rx): (Sender<Entry>, Receiver<Entry>) = mpsc::channel();

    let key_event_threads = event_listener(tx)?;

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

    for (i, handle) in key_event_threads.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => println!("key_event_thread {} ok", i),
            Err(e) => eprintln!("key_event_thread {} panicked: {:?}", i, e),
        }
    }

    match log_writer_thread.join() {
        Ok(_) => println!("log_writer_thread ok"),
        Err(e) => eprintln!("log_writer_thread panicked: {:?}", e),
    }
    
    Ok(())

}
