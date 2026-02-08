## Ructivity


add current user to input group
```
sudo usermod -aG input $USER
```

log out and log in again

check groups
```
groups
```

Usage: cargo run <keyboard_output_log_file_path_to_create> [mouse_output_log_file_path_to_create]

run the program (example)

```
cargo run "keyboard.log" "mouse.log"
```
