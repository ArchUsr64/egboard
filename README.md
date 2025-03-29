# Egboard

A column staggered ortholinear keyboard powered by the Raspberry Pi Pico running Rust!
![top_compressed](https://github.com/ArchUsr64/egboard/assets/83179501/a38c7ab2-6e2d-4140-8c67-bd22cb1abe27)

## Features
- Mouse Input
- Multiple layers
- 1 kHz polling rate
- One-shot modifiers

## Typing Test:
https://github.com/ArchUsr64/egboard/assets/83179501/ff6934fe-465e-4626-92a0-021e62bb25d1

## Bulding the firmware
Pre-built `uf2` is available at [releases](https://github.com/ArchUsr64/egboard/releases)    
Otherwise to build from source:
1. Install [rust](https://rust-lang.org)
2. Download the toolchain for the `thumbv6m-none-eabi`(ARM Cortex-M0+) target 
   `rustup target add thumbv6m-none-eabi`
3. Build the firmware elf  
   `cargo build --release`
4. Converting elf to uf2 using [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs)  
   `cargo install elf2uf2-rs`  
   `elf2uf2-rs target/thumbv6m-none-eabi/release/egboard egboard.uf2`
5. A new `egboard.uf2` file should be created

### Uploading the uf2 to pico:
1. Enter the pico into bootloader mode by holding the `Bootsel` button
2. Mount the pico's mass storage to the PC's file system (should be done automatically in most Operating Systems)
3. Copy the `firmware.uf2` file to the mounted location

## Hardware

The hardware files required to generate the top and bottom plates can be found [here](https://github.com/ArchUsr64/egboard/tree/main/files/hardware)

### Parts required:
| Part | Quantity | Description |
|  -   |    -     |  -  |
| Raspberry Pi Pico | 1 | Microcontroller that powers the whole thing |
| M3 screw | 40 | Used to mount the top and bottom plates together |
| M2 screw | 4 | Used to mount the raspberry pi pico to the top plate |
| M3 standoff | 20 | Provide spacing between the top and bottom plate to house the electronics |
| Switch | 38 | Key switches mounted to the top plate (both 3-pin and 5-pin work)|
| Keycaps | 38 | Keycaps for the switches |
| Diodes | 38 | Required to achieve n-key rollover |
| USB Cable | 1 | Used to connect the pico to the computer |
| Bump Switch (optional) | 1 | Used to get the board to bootloader mode |


### Matrix Layout:
Connect the switches in a `10x4` matrix with the following layout:  

![wiring](https://github.com/ArchUsr64/egboard/assets/83179501/21ba049a-b216-4cbb-9427-33e24838ed3b)

**NOTE:** The above picture shows the layout from top side, flip it horizontally when doing the wiring with the board upside down.

### Wiring:
The diodes should be connected in `Column to Row` ordering i.e the `cathode` of the diode (the side with the line) should be facing away from the keys and the `cathode` of the diodes in same row should all be connected.  
The `anode` of the diodes should each connect to one pin of the switch and the other pin should be connected straight with all the pins of switches in same column. 
For wiring the Raspberry Pi Pico, the indexing used in wiring diagram above correspond to the pico's pinout as follows:


| Row | 0 | 1 | 2 | 3 |
|  -  | - | - | - | - |
| Pin | 0 | 1 | 2 | 3 |


| Column | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|    -   | - | - | - | - | - | - | - | - | - | - |
| Pin | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 

## Keymap configuration
The keymap for the board is specified in [`default_keymap.rs`](https://github.com/ArchUsr64/egboard/blob/main/src/default_keymap.rs) file.  
Each `Keymap` can include upto 256 `Layers` which are added using the `add_layer` method.  
Each `Layer` comprises of a 30 element array for the finger cluster and 8 element array for the thumb clusters.  

### Provided default keymap
#### Layer 0
![image](https://github.com/ArchUsr64/egboard/assets/83179501/0fab39cf-b962-45c4-89ce-3a13087ea864)
#### Layer 1
![image](https://github.com/ArchUsr64/egboard/assets/83179501/c27d3230-f99c-4007-8cd1-f5dbe76c6152)
#### Layer 2
![image](https://github.com/ArchUsr64/egboard/assets/83179501/333706c0-db0f-48df-8c7d-0c67c6ebdffa)
#### Layer 3
![image](https://github.com/ArchUsr64/egboard/assets/83179501/1be92aee-8e91-41f6-9cb2-fe361d7eb696)

