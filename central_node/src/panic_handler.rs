use defmt::{error, println};

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
	error!("[PANIC OCCURED]");
	let location = panic_info.location();
	let payload = panic_info.payload().downcast_ref::<&str>();
	if location.and(payload).is_none() {
		println!("'no further information could be recovered'");
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
