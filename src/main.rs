#![allow(unused)]
#![no_std]
#![no_main]

use rp_pico::entry;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
use defmt::println;
use defmt_rtt as _;
use rp_pico::hal;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
	defmt::error!("[PANIC OCCURED]");
	let location = panic_info.location();
	let payload = panic_info.payload().downcast_ref::<&str>();
	if location.and(payload).is_none() {
		defmt::println!("'no further information could be recovered'");
	} else {
		if let Some(location) = panic_info.location() {
			println!(
				"'panic occurred in file '{}' at line {}'",
				location.file(),
				location.line(),
			);
		} else {
			println!("'can't get panic location information…'");
		}
		if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
			println!("'panic payload: '{}''", s);
		}
	}
	loop {}
}

#[entry]
fn main() -> ! {
	let mut pac = rp_pico::hal::pac::Peripherals::take().unwrap();
	let core = rp_pico::hal::pac::CorePeripherals::take().unwrap();
	let mut watchdog = rp_pico::hal::Watchdog::new(pac.WATCHDOG);
	let clocks = hal::clocks::init_clocks_and_plls(
		rp_pico::XOSC_CRYSTAL_FREQ,
		pac.XOSC,
		pac.CLOCKS,
		pac.PLL_SYS,
		pac.PLL_USB,
		&mut pac.RESETS,
		&mut watchdog,
	)
	.ok()
	.unwrap();
	let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
	let sio = hal::Sio::new(pac.SIO);
	let pins = rp_pico::Pins::new(
		pac.IO_BANK0,
		pac.PADS_BANK0,
		sio.gpio_bank0,
		&mut pac.RESETS,
	);
	loop {
	}
}
