use std::thread::{self, JoinHandle};
use evdev::{Device, InputEventKind, Key};
use chrono::Utc;
use std::sync::mpsc::{Sender};
use std::fs;

const EVDEV_INPUT_PATH: &str = "/dev/input/";

pub struct Entry {
    pub key : Key,
    pub time_stamp : String, //TODO: use a struct and format it in log_writer_thread
}

fn is_keyboard(dev: &Device) -> bool {
   if let Some(keys) = dev.supported_keys() { 
      let required_keys = [
            Key::KEY_ENTER,
            Key::KEY_SPACE,
        ];
        return required_keys.iter().all(|k| keys.contains(*k));
    }
    false
}

fn get_keyboards(inputpath: &str) -> std::io::Result<Vec<Device>> {
    let mut keyboards = Vec::new();

    for entry in fs::read_dir(inputpath)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if !filename.starts_with("event") {
                continue;
            }
        }

        if let Ok(dev) = Device::open(&path) {
            if is_keyboard(&dev) {
                keyboards.push(dev);
            }
        }
    }

    Ok(keyboards)
}

pub fn event_listener(
    tx: Sender<Entry>,
) -> std::io::Result<Vec<JoinHandle<std::io::Result<()>>>> {

    let keyboards = get_keyboards(EVDEV_INPUT_PATH)?;
    let mut handles = Vec::new();

    for mut dev in keyboards {

        let tx = tx.clone();
        let handle = thread::spawn(move || -> std::io::Result<()> {
            loop {
                let events = match dev.fetch_events() {
                    Ok(ev) => ev,
                    Err(e) => {
                        eprintln!("fetch_events failed: {}", e);
                        return Err(e);
                    }
                };

                for ev in events {
                    if let InputEventKind::Key(key) = ev.kind() {
                        if ev.value() == 0 {
                            let utc_now = Utc::now();
                            let entry = Entry {
                                key,
                                time_stamp: utc_now.format("%Y-%m-%d %H:%M:%S.%6f").to_string(),
                            };

                            if let Err(e) = tx.send(entry) {
                                eprintln!("failed to send: {}", e);
                            }
                        }
                    }
                }
            }
        });

        handles.push(handle);
    }

    Ok(handles)
}
