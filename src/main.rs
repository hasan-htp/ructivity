use std::env;
use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod event_listener;
mod key_logger;

use crate::event_listener::event_listener;
use crate::event_listener::Event;

use crate::key_logger::key_logger;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <output_log_file_path_to_create>");
        return Ok(());
    }

    let outputpath = &args[1];

    let log_file = File::create(outputpath)?;

    let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let key_event_threads = event_listener(tx);

    let log_writer_thread = key_logger(rx, log_file);

    for handle in key_event_threads.into_iter() {
        let id = handle.thread().id();
        match handle.join() {
            Ok(_) => println!("key_event_thread {:?} ok", id),
            Err(e) => eprintln!("key_event_thread {:?} panicked: {:?}", id, e),
        }
    }

    match log_writer_thread.join() {
        Ok(_) => println!("log_writer_thread ok"),
        Err(e) => eprintln!("log_writer_thread panicked: {:?}", e),
    }

    Ok(())
}
