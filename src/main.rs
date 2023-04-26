#![no_std]
#![no_main]

pub const POLLING_DELAY_MS: u32 = 10;

mod keymap;
mod keys_macro;
mod panic_handler;
use core::panic;
use keymap::Keymap;

use defmt_rtt as _;
use rp_pico::{entry, hal};

use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use fugit::ExtU32;
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::prelude::*;

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
	let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
	let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
	let sio = hal::Sio::new(pac.SIO);
	let pins = rp_pico::Pins::new(
		pac.IO_BANK0,
		pac.PADS_BANK0,
		sio.gpio_bank0,
		&mut pac.RESETS,
	);
	let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
		pac.USBCTRL_REGS,
		pac.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut pac.RESETS,
	));

	use usbd_human_interface_device::device;
	let mut egboard = UsbHidClassBuilder::new()
		.add_interface(device::keyboard::NKROBootKeyboardConfig::default())
		.add_interface(device::mouse::BootMouseConfig::default())
		.build(&usb_bus);

	//https://pid.codes
	let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x6969, 0x1234))
		.manufacturer("Keyboard")
		.product("Keyboard")
		.build();

	let mut led = pins.led.into_push_pull_output();
	led.set_high().unwrap();

	use core::convert::Infallible;
	use embedded_hal::digital::v2::{InputPin, OutputPin};

	let mut col: [&mut dyn OutputPin<Error = Infallible>; 10] =
		output_keys!(pins, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1);
	col.iter_mut().for_each(|pin| pin.set_low().unwrap());
	let row: [&dyn InputPin<Error = Infallible>; 4] = input_keys!(pins, 11, 12, 13, 14);

	let mut input_count_down = timer.count_down();
	input_count_down.start(POLLING_DELAY_MS.millis());

	let mut tick_count_down = timer.count_down();
	tick_count_down.start(1.millis());

	let mut previous_state = !0;

	let mut mouse_report = device::mouse::BootMouseReport::default();
	mouse_report.y = 1;

	let keymap = Keymap::default();
	loop {
		let mut state = 0;
		for (i, col_pin) in col.iter_mut().enumerate() {
			for (j, row_pin) in row.iter().enumerate() {
				let _ = col_pin.set_high();
				delay.delay_us(1);
				if row_pin.is_high().unwrap() {
					state |= 1 << ((j * 10 + i) as u64);
				}
				let _ = col_pin.set_low();
			}
		}
		let debounced_state = state & previous_state;
		previous_state = state;
		//Poll the keys every 10ms
		if input_count_down.wait().is_ok() {
			// Remove the always unset bit 30 as no key is connected to it
			let debounced_state_normalised =
				(debounced_state & 0x3fffffff) | ((debounced_state >> 32) << 30);
			let key_events = keymap.generate_events(keymap::key_state(debounced_state_normalised));

			match egboard
				.interface::<device::keyboard::NKROBootKeyboardInterface<'_, _>, _>()
				.write_report(key_events)
			{
				Err(UsbHidError::WouldBlock) => {}
				Err(UsbHidError::Duplicate) => {}
				Ok(_) => {}
				Err(e) => {
					panic!("Failed to write keyboard report: {:?}", e)
				}
			};
		}

		//Tick once per ms
		if tick_count_down.wait().is_ok() {
			match egboard
				.interface::<device::keyboard::NKROBootKeyboardInterface<'_, _>, _>()
				.tick()
			{
				Err(UsbHidError::WouldBlock) => {}
				Ok(_) => {}
				Err(e) => {
					panic!("Failed to process keyboard tick: {:?}", e)
				}
			};
		}

		let mouse = egboard.interface::<device::mouse::BootMouseInterface<'_, _>, _>();
		mouse_report.y *= -1;

		match mouse.write_report(&mouse_report) {
			Err(UsbHidError::WouldBlock) => {}
			Ok(_) => {}
			Err(e) => {
				core::panic!("Failed to write mouse report: {:?}", e)
			}
		};

		if usb_dev.poll(&mut [&mut egboard]) {
			match egboard
				.interface::<device::keyboard::NKROBootKeyboardInterface<'_, _>, _>()
				.read_report()
			{
				Err(UsbError::WouldBlock) => {
					//do nothing
				}
				Err(e) => {
					panic!("Failed to read keyboard report: {:?}", e)
				}
				Ok(leds) => {
					led.set_state(PinState::from(leds.caps_lock)).ok();
				}
			}
		}
	}
}
