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
    if args.len() != 2 && args.len() != 3 {
        println!("Usage: cargo run <keyboard_output_log_file_path_to_create> [mouse_output_log_file_path_to_create]");
        return Ok(());
    }

    let keyborad_outputpath = &args[1];

    let keyboard_log_file = File::create(keyborad_outputpath)?;

    let mouse_log_file: Option<File>;
    if args.len() > 2 {
        let mouse_outputpath = &args[2];
        mouse_log_file = Some(File::create(mouse_outputpath)?);
    } else {
        mouse_log_file = None;
    }

    let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let key_event_threads = event_listener(tx);

    let log_writer_thread = key_logger(rx, keyboard_log_file, mouse_log_file);

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
