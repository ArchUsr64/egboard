# Egboard

A column staggered ortholinear keyboard powered by raspberry pi pico running rust.
![top](https://github.com/ArchUsr64/egboard/assets/83179501/86e5122f-7d14-4ca2-91a6-49251adbade7)

### Steps to build the firmware:
1. Install [rust](https://rust-lang.org)
2. Download the toolchain for the `thumbv6-none-eabi`(ARM Cortex-M0+) target\
   `rustup target add thumbv6-none-eabi`
3. Build the firmware elf\
   `cargo build`
4. Converting elf to uf2 using [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs)\
   `cargo install elf2uf2-rs`\
   `elf2uf2-rs target/thumbv6-none-eabi/release/egboard out.uf2`
5. A new `out.uf2` file should be created

The hardware files required to generate the layout can be found [here](https://github.com/ArchUsr64/egboard/tree/main/files/hardware)

### Typing Test:


https://github.com/ArchUsr64/egboard/assets/83179501/e7029c53-4c0d-4cbe-ae44-c0f4a8198730

