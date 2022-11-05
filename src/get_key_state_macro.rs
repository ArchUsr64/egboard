#[macro_export]
macro_rules! get_key_state {
	($keys: expr) => {
		[
			$keys.0.is_low().unwrap(),
			$keys.1.is_low().unwrap(),
			$keys.2.is_low().unwrap(),
			$keys.3.is_low().unwrap(),
			$keys.4.is_low().unwrap(),
			$keys.5.is_low().unwrap(),
			$keys.6.is_low().unwrap(),
			$keys.7.is_low().unwrap(),
			$keys.8.is_low().unwrap(),
			$keys.9.is_low().unwrap(),
			$keys.10.is_low().unwrap(),
			$keys.11.is_low().unwrap(),
			$keys.12.is_low().unwrap(),
			$keys.13.is_low().unwrap(),
			$keys.14.is_low().unwrap(),
			$keys.15.is_low().unwrap(),
			$keys.16.is_low().unwrap(),
			$keys.17.is_low().unwrap(),
			$keys.18.is_low().unwrap(),
			$keys.19.is_low().unwrap(),
			$keys.20.is_low().unwrap(),
		]
	};
}
