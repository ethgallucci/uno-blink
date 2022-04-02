# Flashing to board 
```sh
avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:rust-arduino-blink.elf

```
'ttyACM0' should be replaced with the port that your board occupies.