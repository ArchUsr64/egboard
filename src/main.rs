#![no_std]
#![no_main]

mod get_key_state_macro;
mod keys_macro;
mod panic_handler;

type KeyDown = bool;

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
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
	let sio = hal::Sio::new(pac.SIO);
	let pins = rp_pico::Pins::new(
		pac.IO_BANK0,
		pac.PADS_BANK0,
		sio.gpio_bank0,
		&mut pac.RESETS,
	);

	info!("Started!");

	let uart_tx = pins.gpio0.into_mode::<hal::gpio::FunctionUart>();
	let uart_rx = pins.gpio1.into_mode::<hal::gpio::FunctionUart>();

	use hal::uart::{common_configs::_9600_8_N_1 as _9600, UartPeripheral};
	let uart = UartPeripheral::new(pac.UART0, (uart_tx, uart_rx), &mut pac.RESETS)
		.enable(_9600, clocks.peripheral_clock.freq())
		.unwrap();

	let keys =
		keys!(pins, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22);

	let key_status: [KeyDown; 21] = get_key_state!(keys);

	loop {
		delay.delay_ms(1);
		uart.write_full_blocking(b"broThisisNUTS!...");
	}
}
