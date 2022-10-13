#![allow(unused)]
#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use fugit::HertzU32;
use hal::gpio::pin::bank0::{Gpio0, Gpio1};
use hal::uart::{DataBits, StopBits, UartConfig};
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal::pac;
use rp_pico::hal::prelude::*;
type UartPins = (
	hal::gpio::Pin<Gpio0, hal::gpio::Function<hal::gpio::Uart>>,
	hal::gpio::Pin<Gpio1, hal::gpio::Function<hal::gpio::Uart>>,
);
type Uart = hal::uart::UartPeripheral<hal::uart::Enabled, pac::UART0, UartPins>;
use rp_pico::hal;

#[entry]
fn main() -> ! {
	pub const CUSTOM_CONFIG: UartConfig = UartConfig {
		baudrate: HertzU32::from_raw(115_200),
		data_bits: DataBits::Eight,
		stop_bits: StopBits::One,
		parity: None,
	};
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
	let uart_pins = (
		pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
		pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
	);
	let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
		.enable(
			CUSTOM_CONFIG,
			clocks.peripheral_clock.freq(),
		)
		.unwrap();
	loop {
		uart.write_full_blocking(b"Hello World\n");
	}
}
