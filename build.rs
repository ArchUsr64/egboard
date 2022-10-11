// use std::process::Command;
//
fn main() {
	// println!("cargo:rerun-if-changed=target/thumbv6m-none-eabi/release/egboard");
	// Command::new("elf2uf2-rs")
    println!("cargo:rustc-link=-Tdefmt.x");
}
