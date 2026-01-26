#![no_std]
#![no_main]

pub const USB_POLLING_DELAY_MS: u32 = 1;
pub const DEBOUNCE_BUFFER_SIZE: usize = 64;

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
use fugit::RateExtU32;
use heapless::Vec;
use usb_device::class_prelude::*;
use usb_device::prelude::*;
use usbd_human_interface_device::device::mouse::WheelMouseReport;
use usbd_human_interface_device::prelude::*;

#[entry]
fn main() -> ! {
	use rp_pico::hal::{pac::Peripherals, Watchdog};
	let mut pac = Peripherals::take().unwrap();
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

	// Configure two pins as being I²C, not GPIO
	let sda_pin = pins.gpio18.into_mode::<hal::gpio::FunctionI2C>();
	let scl_pin = pins.gpio19.into_mode::<hal::gpio::FunctionI2C>();

	// Create the I²C drive, using the two pre-configured pins. This will fail
	// at compile time if the pins are in the wrong mode, or if this I²C
	// peripheral isn't available on these pins!
	let mut i2c = hal::I2C::i2c1(
		pac.I2C1,
		sda_pin,
		scl_pin, // Try `not_an_scl_pin` here
		400_u32.kHz(),
		&mut pac.RESETS,
		&clocks.system_clock,
	);

	use usbd_human_interface_device::device;
	let mut egboard = UsbHidClassBuilder::new()
		.add_interface(device::keyboard::NKROBootKeyboardConfig::default())
		.add_interface(device::mouse::WheelMouseConfig::default())
		.build(&usb_bus);

	let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x6969, 0x1234))
		.manufacturer("ArchUsr64")
		.product("Rusty Egboard")
		.build();

	let mut led = pins.led.into_push_pull_output();
	led.set_high().unwrap();

	let mut usb_polling_count_down = timer.count_down();
	usb_polling_count_down.start(USB_POLLING_DELAY_MS.millis());

	let mut tick_count_down = timer.count_down();
	tick_count_down.start(1.millis());

	let mut state_buffer = Vec::<_, DEBOUNCE_BUFFER_SIZE>::new();
	(0..DEBOUNCE_BUFFER_SIZE).for_each(|_| {
		let _ = state_buffer.push(0);
	});
	let mut prev_mouse_button = 0;

	let keymap = Keymap::default();
	let empty_mouse_report = WheelMouseReport::default();

	loop {
		state_buffer.remove(0);

		let mut data = [0; 5];
		i2c.read(0x64, &mut data).unwrap();
		let state = data[0] as u64
			| (data[1] as u64) << 8
			| (data[2] as u64) << 16
			| (data[3] as u64) << 24
			| (data[4] as u64) << 32;
		let _ = state_buffer.push(state);

		if usb_polling_count_down.wait().is_ok() {
			let debounced_state = state_buffer.iter().fold(!0, |acc, x| acc & x);
			let (key_events, mouse_report) =
				keymap.generate_events(keymap::key_state(debounced_state));

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

			if mouse_report != empty_mouse_report || mouse_report.buttons != prev_mouse_button {
				match egboard
					.interface::<device::mouse::WheelMouseInterface<'_, _>, _>()
					.write_report(&mouse_report)
				{
					Err(UsbHidError::WouldBlock) => {}
					Ok(_) => {
						prev_mouse_button = mouse_report.buttons;
					}
					Err(e) => {
						core::panic!("Failed to write mouse report: {:?}", e)
					}
				};
			}
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
