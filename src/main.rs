#![no_std]
#![no_main]

mod keys_macro;
mod panic_handler;

use defmt::*;
use defmt_rtt as _;
use rp_pico::{entry, hal};

#[entry]
fn main() -> ! {
	use cortex_m::delay::Delay;
	use rp_pico::hal::{
		pac::{CorePeripherals, Peripherals},
		Clock, Watchdog,
	};
	let mut pac = Peripherals::take().unwrap();
	let core = CorePeripherals::take().unwrap();
	let mut watchdog = Watchdog::new(pac.WATCHDOG);
	let clocks = rp_pico::hal::clocks::init_clocks_and_plls(
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
	let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
	delay.delay_ms(100);
	let sio = hal::Sio::new(pac.SIO);
	let pins = rp_pico::Pins::new(
		pac.IO_BANK0,
		pac.PADS_BANK0,
		sio.gpio_bank0,
		&mut pac.RESETS,
	);

	let mut led = pins.led.into_push_pull_output();
	led.set_high().unwrap();

	use core::convert::Infallible;
	use embedded_hal::digital::v2::{InputPin, OutputPin};

	let mut col: [&mut dyn OutputPin<Error = Infallible>; 10] =
		output_keys!(pins, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
	col.iter_mut().for_each(|pin| pin.set_low().unwrap());
	let row: [&dyn InputPin<Error = Infallible>; 4] = input_keys!(pins, 11, 12, 13, 14);

	let mut previous_state: u64;
	let mut state = 0;
	loop {
		previous_state = state;
		state = 0;
		for (i, col_pin) in col.iter_mut().enumerate() {
			for (j, row_pin) in row.iter().enumerate() {
				let _ = col_pin.set_high();
				if row_pin.is_high().unwrap() {
					state |= 1 << ((j * 10 + i) as u64);
				}
				let _ = col_pin.set_low();
			}
		}
		if state != previous_state {
			println!("{:040b}", state);
		}
		delay.delay_ms(50);
	}
}
