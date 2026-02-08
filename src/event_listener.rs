use chrono::Utc;
use evdev::{Device, EventSummary, KeyCode, RelativeAxisCode};
use std::fs;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

const EVDEV_INPUT_PATH: &str = "/dev/input/";

pub struct KeyEvent {
    pub key: KeyCode,
    pub time_stamp: String,
}

// for mouse events, i don't care about the nature of the movment, time_stamp is enough
pub struct MouseEvent {
    pub time_stamp: String,
}
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

fn is_keyboard(dev: &Device) -> bool {
    if let Some(keys) = dev.supported_keys() {
        let required_keys = [KeyCode::KEY_ENTER, KeyCode::KEY_SPACE];
        return required_keys.iter().all(|k| keys.contains(*k));
    }
    false
}

fn is_mouse(dev: &Device) -> bool {
    if let Some(axes) = dev.supported_relative_axes() {
        let required_axes = [RelativeAxisCode::REL_X, RelativeAxisCode::REL_Y];
        if required_axes.iter().all(|k| axes.contains(*k)) {
            if let Some(keys) = dev.supported_keys() {
                let required_keys = [KeyCode::BTN_LEFT, KeyCode::BTN_RIGHT];
                return required_keys.iter().all(|k| keys.contains(*k));
            }
        }
    }
    false
}

fn get_devices<DeviceCheck>(is_device: DeviceCheck, inputpath: &str) -> std::io::Result<Vec<Device>>
where
    DeviceCheck: Fn(&Device) -> bool,
{
    let mut devices = Vec::new();

    for entry in fs::read_dir(inputpath)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if !filename.starts_with("event") {
                continue;
            }
        }

        if let Ok(dev) = Device::open(&path) {
            if is_device(&dev) {
                devices.push(dev);
            }
        }
    }

    Ok(devices)
}

pub fn event_listener(tx: Sender<Event>) -> Vec<JoinHandle<std::io::Result<()>>> {
    let keyboards = get_devices(is_keyboard, EVDEV_INPUT_PATH).unwrap_or_default();
    println!("{} keyboard evdev found", keyboards.len());

    let mice = get_devices(is_mouse, EVDEV_INPUT_PATH).unwrap_or_default();
    println!("{} mouse evdev found", mice.len());

    let mut devices = keyboards;
    devices.extend(mice);

    let mut handles = Vec::new();

    for mut dev in devices {
        let dev_name = dev.name().unwrap_or("unknown").to_string();

        println!("device name:{}", dev_name);

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
                    match ev.destructure() {
                        EventSummary::Key(_, key, value) => {
                            if value == 0 {
                                let utc_now = Utc::now();
                                let entry = KeyEvent {
                                    key: key,
                                    time_stamp: utc_now.format("%Y-%m-%d %H:%M:%S.%6f").to_string(),
                                };
                                if let Err(e) = tx.send(Event::Key(entry)) {
                                    eprintln!("failed to send: {}", e);
                                }
                            }
                        }
                        //Note: touchpad is not RelativeAxis event, consider adding AbsoluteAxis events too
                        EventSummary::RelativeAxis(_, _, _) => {
                            let utc_now = Utc::now();
                            let entry = MouseEvent {
                                time_stamp: utc_now.format("%Y-%m-%d %H:%M:%S.%6f").to_string(),
                            };
                            if let Err(e) = tx.send(Event::Mouse(entry)) {
                                eprintln!("failed to send: {}", e);
                            }
                        }
                        _ => {}
                    }
                }
            }
        });

        handles.push(handle);
    }

    handles
}
