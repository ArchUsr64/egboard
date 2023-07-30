# Egboard

A column staggered ortholinear keyboard powered by raspberry pi pico running rust.
![top](https://github.com/ArchUsr64/egboard/assets/83179501/86e5122f-7d14-4ca2-91a6-49251adbade7)

### Bulding the firmware:
1. Install [rust](https://rust-lang.org)
2. Download the toolchain for the `thumbv6-none-eabi`(ARM Cortex-M0+) target\
   `rustup target add thumbv6-none-eabi`
3. Build the firmware elf\
   `cargo build`
4. Converting elf to uf2 using [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs)\
   `cargo install elf2uf2-rs`\
   `elf2uf2-rs target/thumbv6-none-eabi/release/egboard out.uf2`
5. A new `out.uf2` file should be created
   
### Hardware:

The hardware files required to generate the top and bottom plates can be found [here](https://github.com/ArchUsr64/egboard/tree/main/files/hardware)

#### Parts required:
| Part | Quantity | Description |
|  -   |    -     |  -  |
| Raspberry Pi Pico | 1 | Microcontroller that powers the whole thing |
| M3 screw | 36 | Used to mount the top and bottom plates together |
| M2 screw | 4 | Used to mount the raspberry pi pico to the top plate |
| M3 standoff | 18 | Provide spacing between the top and bottom plate to house the electronics |
| Switch | 38 | Key switches mounted to the top plate |
| Keycaps | 38 | Keycaps for the switches |
| Diodes | 38 | Required to achieve n-key rollover |
| USB Cable | 1 | Used to connect the pico to the computer |
| Bump Switch (optional) | 1 | Used to get the board to bootloader mode |

### Project report:
View on [google docs](https://docs.google.com/document/d/e/2PACX-1vQndY82YCXaxrvmDw9xcZhzOaJTsP58XWRm1BeVet43TQnjNqOJDFl5XpR4vhXsUciPnCYtNsxyYR8w/pub)

### Typing Test:


https://github.com/ArchUsr64/egboard/assets/83179501/e7029c53-4c0d-4cbe-ae44-c0f4a8198730

<iframe src="https://docs.google.com/document/d/e/2PACX-1vQndY82YCXaxrvmDw9xcZhzOaJTsP58XWRm1BeVet43TQnjNqOJDFl5XpR4vhXsUciPnCYtNsxyYR8w/pub?embedded=true"></iframe>
