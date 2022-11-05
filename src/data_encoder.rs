trait AddCRC {
	fn add_crc(self) -> u32;
}

impl AddCRC for u32 {
	fn add_crc(self) -> u32 {
		(self << 11) + ((self << 11) % 1483)
	}
}

type bits = bool;
pub fn encode(&data: &[bits; 21]) -> [u8; 4] {
	let mut data = data_from_bit_stream(&data);
	data = data.add_crc();
	data.to_ne_bytes()
}

fn data_from_bit_stream(&stream: &[bits; 21]) -> u32 {
	let mut data: u32 = 0;
	for (index, bit) in stream.iter().enumerate() {
		if *bit {
			data += 2u32.pow(index.try_into().unwrap_or(0));
		}
	}
	data
}
