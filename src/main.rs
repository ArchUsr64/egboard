#![no_std]
#![no_main]

mod keys_macro;
mod panic_handler;

use defmt::*;
use defmt_rtt as _;
use rp_pico::{entry, hal};

use embedded_hal::digital::v2::*;
use embedded_hal::prelude::*;
use fugit::ExtU32;
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::page::Keyboard;
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
	delay.delay_ms(1);
	info!("Pins initialised");
	let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
		pac.USBCTRL_REGS,
		pac.USBCTRL_DPRAM,
		clocks.usb_clock,
		true,
		&mut pac.RESETS,
	));

	let mut keyboard = UsbHidClassBuilder::new()
		.add_interface(
			usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig::default(),
		)
		.build(&usb_bus);

	//https://pid.codes
	let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
		.manufacturer("ArchUsr64")
		.product("egboard")
		.serial_number("1234")
		.build();

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
	let mut input_count_down = timer.count_down();
	input_count_down.start(10.millis());

	let mut tick_count_down = timer.count_down();
	tick_count_down.start(1.millis());

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
		//Poll the keys every 10ms
		if input_count_down.wait().is_ok() {
			let keys = get_keys(state);

			match keyboard.interface().write_report(keys) {
				Err(UsbHidError::WouldBlock) => {}
				Err(UsbHidError::Duplicate) => {}
				Ok(_) => {}
				Err(e) => {
					core::panic!("Failed to write keyboard report: {:?}", e)
				}
			};
		}

		//Tick once per ms
		if tick_count_down.wait().is_ok() {
			match keyboard.interface().tick() {
				Err(UsbHidError::WouldBlock) => {}
				Ok(_) => {}
				Err(e) => {
					core::panic!("Failed to process keyboard tick: {:?}", e)
				}
			};
		}

		if usb_dev.poll(&mut [&mut keyboard]) {
			match keyboard.interface().read_report() {
				Err(UsbError::WouldBlock) => {
					//do nothing
				}
				Err(e) => {
					core::panic!("Failed to read keyboard report: {:?}", e)
				}
				Ok(leds) => {
					led.set_state(PinState::from(leds.num_lock)).ok();
				}
			}
		}
	}
}

fn get_keys(state: u64) -> [Keyboard; 32] {
	let key_state_i = |index: u64| state & (1 << index) != 0;
	let bind_key = |index, key_event| {
		if key_state_i(index) {
			key_event
		} else {
			Keyboard::NoEventIndicated
		}
	};
	use Keyboard::*;
	[
		bind_key(0, P),
		bind_key(1, O),
		bind_key(2, I),
		bind_key(3, U),
		bind_key(4, Y),
		bind_key(5, T),
		bind_key(6, R),
		bind_key(7, E),
		bind_key(8, W),
		bind_key(9, Q),
		bind_key(10, Semicolon),
		bind_key(11, L),
		bind_key(12, K),
		bind_key(13, J),
		bind_key(14, H),
		bind_key(15, G),
		bind_key(16, F),
		bind_key(17, D),
		bind_key(18, S),
		bind_key(19, A),
		bind_key(20, ForwardSlash),
		bind_key(21, Dot),
		bind_key(22, Comma),
		bind_key(23, M),
		bind_key(24, N),
		bind_key(25, B),
		bind_key(26, V),
		bind_key(27, C),
		bind_key(28, X),
		bind_key(29, Z),
		bind_key(34, Space),
		if key_state_i(33) && key_state_i(36) {
			DeleteBackspace
		} else {
			NoEventIndicated
		},
	]
}
