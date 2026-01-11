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

run the program (example)

```
cargo run "/dev/input/by-id/usb-SteelSeries_SteelSeries_Apex_5-event-kbd" "test_output.log"
```
