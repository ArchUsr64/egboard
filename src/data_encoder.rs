trait AddCRC {
	fn add_crc(self) -> u32;
}

impl AddCRC for u32 {
	fn add_crc(self) -> u32 {
		(self << 3) + ((self << 3) % 7)
	}
}

type Bits = bool;
pub fn encode(&data: &[Bits; 21]) -> [u8; 3] {
	let mut data = data_from_bit_stream(&data);
	data = data.add_crc();
	(data << 8).to_be_bytes()[..3].try_into().unwrap()
}

fn data_from_bit_stream(&stream: &[Bits; 21]) -> u32 {
	let mut data: u32 = 0;
	for (index, bit) in stream.iter().enumerate() {
		if *bit {
			data += 2u32.pow(index.try_into().unwrap_or(0));
		}
	}
	data
}
