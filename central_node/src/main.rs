#![no_std]
#![no_main]

mod data_decoder;
mod panic_handler;

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::serial::Read;
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
	let mut uart_0 = UartPeripheral::new(pac.UART0, (uart_tx, uart_rx), &mut pac.RESETS)
		.enable(_9600, clocks.peripheral_clock.freq())
		.unwrap();

	let uart_tx = pins.gpio4.into_mode::<hal::gpio::FunctionUart>();
	let uart_rx = pins.gpio5.into_mode::<hal::gpio::FunctionUart>();

	let mut uart_1 = UartPeripheral::new(pac.UART1, (uart_tx, uart_rx), &mut pac.RESETS)
		.enable(_9600, clocks.peripheral_clock.freq())
		.unwrap();
	
	let mut led = pins.led.into_push_pull_output();

	let mut incoming_data_0 = [0u8; 4];
	let mut incoming_data_1 = [0u8; 4];
	loop {
		led.set_low().unwrap();
		while uart_0.uart_is_readable(){
			uart_0.read();
		}
		let err_0 = uart_0.read_full_blocking(&mut incoming_data_0);
		if err_0.is_err() {
			info!("Found nothing on UART 0");
			led.set_high().unwrap();
		// delay.delay_ms(1);
		} else {
			use data_decoder::{decode, DecodeError};
			match decode(&incoming_data_0) {
				Ok(val) => println!("Received on UART 0: 0x{:06x}", data_from_bit_stream(&val)),
				Err(DecodeError::InvalidPacket) => info!("Invalid packet on UART 0"),
				Err(DecodeError::UndetectedEndByte) => info!("End byte not detected on UART 0"),
			};
		}
		while uart_1.uart_is_readable(){
			uart_1.read();
		}
		let err_1 = uart_1.read_full_blocking(&mut incoming_data_1);
		if err_1.is_err() {
			led.set_high().unwrap();
			info!("Found nothing on UART 1");
		} else {
			use data_decoder::{decode, DecodeError};
			match decode(&incoming_data_1) {
				Ok(val) => println!("Received on UART 1: 0x{:06x}", data_from_bit_stream(&val)),
				Err(DecodeError::InvalidPacket) => info!("Invalid packet on UART 1"),
				Err(DecodeError::UndetectedEndByte) => info!("End byte not detected on UART 1"),
			};
		}
		// println!("0x{:06x} 0x{:06x}", incoming_data_0, incoming_data_1);
	}
}
fn data_from_bit_stream(&stream: &[bool; 21]) -> u32 {
	let mut data: u32 = 0;
	for (index, bit) in stream.iter().enumerate() {
		if *bit {
			data += 2u32.pow(index.try_into().unwrap_or(0));
		}
	}
	data
}
