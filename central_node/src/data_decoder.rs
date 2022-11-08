use defmt::*;
pub trait VerifyCRC {
	fn crc_valid(self) -> bool;
}

impl VerifyCRC for u32 {
	fn crc_valid(self) -> bool {
		self % 8 == (self >> 3) % 7
	}
}

pub enum DecodeError {
	InvalidPacket,
	UndetectedEndByte,
}

type Bits = bool;
use DecodeError::*;
pub fn decode(&data: &[u8; 4]) -> Result<[Bits; 21], DecodeError> {
	let end_byte_index = get_end_byte_index(&data)?;
	let data_num = (1..=3)
		.map(|o| data[(end_byte_index + o).rem_euclid(4)])
		.rev()
		.enumerate()
		.fold(0u32, |acc, (i, data)| acc + ((data as u32) << (8 * i)));
	decode_packet(data_num)
}

fn get_end_byte_index(&data: &[u8; 4]) -> Result<usize, DecodeError> {
	for (index, byte) in data.iter().enumerate() {
		if (*byte ^ 0b10101010).count_zeros() >= 7 {
			return Ok(index);
		}
	}
	Err(UndetectedEndByte)
}

fn decode_packet(data: u32) -> Result<[Bits; 21], DecodeError> {
	if !data.crc_valid() {
		return Err(InvalidPacket);
	}
	let mut packet= [false; 21];
	let mut data = data >> 3;
	for bit in packet.iter_mut(){
		*bit = data.rem_euclid(2) == 1;
		data >>= 1;
	}
	Ok(packet)
}
