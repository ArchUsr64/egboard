#![allow(unused)]
#![no_std]
#![no_main]

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::entry;
use rp_pico::hal;
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
	let mut value = 100;
	let mut led = pins.led.into_push_pull_output();
	loop {
		delay.delay_ms(value);
		led.set_high();
		delay.delay_ms(value);
		led.set_low();
	}
}
