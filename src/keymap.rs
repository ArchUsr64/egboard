const KEY_COUNT: usize = 37;

use usbd_human_interface_device::page::Keyboard;
#[derive(Clone, Copy, Debug)]
enum Key {
	LayerModifier(u8),
	UpDown(Keyboard),
}

type Layer = [Option<Key>; KEY_COUNT];

pub struct Keymap {
	layers: [Option<Layer>; 256],
}
impl Keymap {
	fn new() -> Self {
		Self {
			layers: [None; 256],
		}
	}
	fn default_layer(&self) -> Layer {
		self.layers.first().unwrap().unwrap()
	}
	/// Adds a new layer to the keymap
	fn add_layer(mut self, layer: Layer) -> Self {
		let empty_index = self.layers.iter().position(|x| x.is_none()).unwrap();
		self.layers[empty_index] = Some(layer);
		self
	}
	fn get_active_layer(&self, state: u64) -> Layer {
		fn key_state(index: u64, state: u64) -> bool {
			state & (1 << index) != 0
		}
		let mut result = self.default_layer();
		loop {
			let mut new_index = Option::<u8>::None;
			result.iter().enumerate().for_each(|(index, key)| {
				if let Some(Key::LayerModifier(layer_index)) = key {
					if key_state(index as u64, state) {
						new_index = Some(*layer_index);
					}
				}
			});
			if let Some(index) = new_index {
				result = self.layers[index as usize].unwrap();
				continue;
			}
			break;
		}
		result
	}
	pub fn generate_events(&self, state: u64) -> [Keyboard; KEY_COUNT] {
		let layer = self.get_active_layer(state);
		let mut result = [Keyboard::NoEventIndicated; KEY_COUNT];
		result
			.iter_mut()
			.enumerate()
			.filter(|(i, _)| state & 1 << i != 0)
			.for_each(|(index, event)| {
				let key = layer[index].unwrap_or(self.default_layer()[index].unwrap());
				*event = {
					match key {
						Key::UpDown(keycode) => keycode,
						_ => Keyboard::NoEventIndicated,
					}
				}
			});
		result
	}
}
impl Default for Keymap {
	fn default() -> Self {
		let keymap = Self::new()
			.add_layer([
				//Row 1
				Some(Key::UpDown(Keyboard::P)),
				Some(Key::UpDown(Keyboard::O)),
				Some(Key::UpDown(Keyboard::I)),
				Some(Key::UpDown(Keyboard::U)),
				Some(Key::UpDown(Keyboard::Y)),
				Some(Key::UpDown(Keyboard::T)),
				Some(Key::UpDown(Keyboard::R)),
				Some(Key::UpDown(Keyboard::E)),
				Some(Key::UpDown(Keyboard::W)),
				Some(Key::UpDown(Keyboard::Q)),
				//Row 2
				Some(Key::UpDown(Keyboard::Semicolon)),
				Some(Key::UpDown(Keyboard::L)),
				Some(Key::UpDown(Keyboard::K)),
				Some(Key::UpDown(Keyboard::J)),
				Some(Key::UpDown(Keyboard::H)),
				Some(Key::UpDown(Keyboard::G)),
				Some(Key::UpDown(Keyboard::F)),
				Some(Key::UpDown(Keyboard::D)),
				Some(Key::UpDown(Keyboard::S)),
				Some(Key::UpDown(Keyboard::A)),
				//Row 3
				Some(Key::UpDown(Keyboard::ForwardSlash)),
				Some(Key::UpDown(Keyboard::Dot)),
				Some(Key::UpDown(Keyboard::Comma)),
				Some(Key::UpDown(Keyboard::M)),
				Some(Key::UpDown(Keyboard::N)),
				Some(Key::UpDown(Keyboard::B)),
				Some(Key::UpDown(Keyboard::V)),
				Some(Key::UpDown(Keyboard::C)),
				Some(Key::UpDown(Keyboard::X)),
				Some(Key::UpDown(Keyboard::Z)),
				//Row 4
				None,
				Some(Key::UpDown(Keyboard::LeftControl)),
				Some(Key::LayerModifier(1)),
				Some(Key::UpDown(Keyboard::Space)),
				Some(Key::UpDown(Keyboard::LeftShift)),
				Some(Key::LayerModifier(2)),
				Some(Key::UpDown(Keyboard::LeftAlt)),
			])
			.add_layer([
				Some(Key::UpDown(Keyboard::Backslash)),
				Some(Key::UpDown(Keyboard::RightArrow)),
				Some(Key::UpDown(Keyboard::UpArrow)),
				Some(Key::UpDown(Keyboard::DownArrow)),
				Some(Key::UpDown(Keyboard::LeftArrow)),
				Some(Key::UpDown(Keyboard::Equal)),
				Some(Key::UpDown(Keyboard::Minus)),
				Some(Key::UpDown(Keyboard::RightBrace)),
				Some(Key::UpDown(Keyboard::LeftBrace)),
				Some(Key::UpDown(Keyboard::Grave)),
				//Row 2
				Some(Key::UpDown(Keyboard::Keyboard0)),
				Some(Key::UpDown(Keyboard::Keyboard9)),
				Some(Key::UpDown(Keyboard::Keyboard8)),
				Some(Key::UpDown(Keyboard::Keyboard7)),
				Some(Key::UpDown(Keyboard::Keyboard6)),
				Some(Key::UpDown(Keyboard::Keyboard5)),
				Some(Key::UpDown(Keyboard::Keyboard4)),
				Some(Key::UpDown(Keyboard::Keyboard3)),
				Some(Key::UpDown(Keyboard::Keyboard2)),
				Some(Key::UpDown(Keyboard::Keyboard1)),
				//Row 3
				None,
				None,
				None,
				Some(Key::UpDown(Keyboard::Apostrophe)),
				None,
				None,
				Some(Key::UpDown(Keyboard::Tab)),
				None,
				None,
				None,
				//Thumb cluster
				None,
				None,
				None,
				None,
				None,
				Some(Key::UpDown(Keyboard::DeleteBackspace)),
				None,
			])
			.add_layer([
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				//Row 2
				Some(Key::UpDown(Keyboard::ReturnEnter)),
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				Some(Key::UpDown(Keyboard::Escape)),
				//Row 3
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				None,
				//Thumb cluster
				None,
				None,
				Some(Key::UpDown(Keyboard::DeleteBackspace)),
				None,
				None,
				None,
				None,
			]);
		keymap
	}
}
