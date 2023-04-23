const FINGER_CLUSTER_SIZE: usize = 30;
const THUMB_CLUSTER_SIZE: usize = 7;

use usbd_human_interface_device::page::Keyboard;

#[derive(Clone, Copy, Debug)]
enum ThumbKey {
	LayerModifier(u8),
	UpDown(Keyboard),
}

#[derive(Clone, Copy, Debug)]
pub struct Layer {
	finger_cluster: FingerLayer,
	thumb_cluster: ThumbLayer,
}
type FingerLayer = [Option<Keyboard>; FINGER_CLUSTER_SIZE];
type ThumbLayer = [Option<ThumbKey>; THUMB_CLUSTER_SIZE];
impl Layer {
	pub const fn new() -> Self {
		Self {
			finger_cluster: [None; FINGER_CLUSTER_SIZE],
			thumb_cluster: [None; THUMB_CLUSTER_SIZE],
		}
	}
}

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
	fn get_active_layer(&self, state: ThumbState) -> Layer {
		let new_layer_index = self
			.default_layer()
			.thumb_cluster
			.iter()
			.enumerate()
			.filter_map(|(index, key)| match (key, state[index]) {
				(Some(ThumbKey::LayerModifier(layer_index)), true) => Some(*layer_index),
				_ => None,
			})
			.last()
			.unwrap_or(0);
		self.layers[new_layer_index as usize].unwrap()
	}

	fn get_buffered_layer(&self, key_state: KeyState) -> Layer {
		static mut PREVIOUS_LAYER: Layer = Layer::new();
		let mut layer = self.get_active_layer(key_state.thumb_cluster);
		unsafe {
			layer
				.finger_cluster
				.iter_mut()
				.enumerate()
				.for_each(|(index, keymap)| {
					if key_state.finger_cluster[index] {
						*keymap = PREVIOUS_LAYER.finger_cluster[index]
					}
				});
			layer
				.thumb_cluster
				.iter_mut()
				.enumerate()
				.for_each(|(index, keymap)| {
					if key_state.thumb_cluster[index] {
						*keymap = PREVIOUS_LAYER.thumb_cluster[index]
					}
				});
		}
		unsafe { PREVIOUS_LAYER = layer }
		layer
	}

	pub fn generate_events(
		&self,
		key_state: KeyState,
	) -> [Keyboard; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 4] {
		let layer = self.get_buffered_layer(key_state);
		let mut result = [Keyboard::NoEventIndicated; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 4];

		result[0..FINGER_CLUSTER_SIZE]
			.iter_mut()
			.enumerate()
			.filter(|(i, _)| key_state.finger_cluster[*i])
			.for_each(|(index, event)| {
				let key = layer.finger_cluster[index]
					.unwrap_or(self.default_layer().finger_cluster[index].unwrap());
				*event = key;
			});
		result[FINGER_CLUSTER_SIZE..FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE]
			.iter_mut()
			.enumerate()
			.filter(|(i, _)| key_state.thumb_cluster[*i])
			.for_each(|(index, event)| {
				if let ThumbKey::UpDown(key) = layer.thumb_cluster[index]
					.unwrap_or(self.default_layer().thumb_cluster[index].unwrap())
				{
					*event = key
				}
			});
		result
	}
}

#[derive(Clone, Copy, Debug)]
pub struct KeyState {
	finger_cluster: FingerState,
	thumb_cluster: ThumbState,
}
type FingerState = [bool; FINGER_CLUSTER_SIZE];
type ThumbState = [bool; THUMB_CLUSTER_SIZE];

impl KeyState {
	pub const fn new() -> Self {
		Self {
			finger_cluster: [false; FINGER_CLUSTER_SIZE],
			thumb_cluster: [false; THUMB_CLUSTER_SIZE],
		}
	}
}

pub fn key_state(state: u64) -> KeyState {
	let mut result = KeyState::new();
	result
		.finger_cluster
		.iter_mut()
		.enumerate()
		.for_each(|(index, key_state)| *key_state = state & 1 << index != 0);
	result
		.thumb_cluster
		.iter_mut()
		.enumerate()
		.for_each(|(index, key_state)| {
			*key_state = state & 1 << (FINGER_CLUSTER_SIZE + index) != 0
		});
	result
}

impl Default for Keymap {
	fn default() -> Self {
		let keymap = Self::new()
			.add_layer(Layer {
				finger_cluster: [
					//Row 1
					Some(Keyboard::Q),
					Some(Keyboard::W),
					Some(Keyboard::E),
					Some(Keyboard::R),
					Some(Keyboard::T),
					Some(Keyboard::Y),
					Some(Keyboard::U),
					Some(Keyboard::I),
					Some(Keyboard::O),
					Some(Keyboard::P),
					//Row 2
					Some(Keyboard::A),
					Some(Keyboard::S),
					Some(Keyboard::D),
					Some(Keyboard::F),
					Some(Keyboard::G),
					Some(Keyboard::H),
					Some(Keyboard::J),
					Some(Keyboard::K),
					Some(Keyboard::L),
					Some(Keyboard::Semicolon),
					//Row 3
					Some(Keyboard::Z),
					Some(Keyboard::X),
					Some(Keyboard::C),
					Some(Keyboard::V),
					Some(Keyboard::B),
					Some(Keyboard::N),
					Some(Keyboard::M),
					Some(Keyboard::Comma),
					Some(Keyboard::Dot),
					Some(Keyboard::ForwardSlash),
				],
				thumb_cluster: [
					Some(ThumbKey::UpDown(Keyboard::LeftAlt)),
					Some(ThumbKey::LayerModifier(2)),
					Some(ThumbKey::UpDown(Keyboard::LeftShift)),
					Some(ThumbKey::UpDown(Keyboard::Space)),
					Some(ThumbKey::LayerModifier(1)),
					Some(ThumbKey::UpDown(Keyboard::LeftControl)),
					None,
				],
			})
			.add_layer(Layer {
				finger_cluster: [
					Some(Keyboard::Grave),
					Some(Keyboard::LeftBrace),
					Some(Keyboard::RightBrace),
					Some(Keyboard::Minus),
					Some(Keyboard::Equal),
					Some(Keyboard::LeftArrow),
					Some(Keyboard::DownArrow),
					Some(Keyboard::UpArrow),
					Some(Keyboard::RightArrow),
					Some(Keyboard::Backslash),
					//Row 2
					Some(Keyboard::Keyboard1),
					Some(Keyboard::Keyboard2),
					Some(Keyboard::Keyboard3),
					Some(Keyboard::Keyboard4),
					Some(Keyboard::Keyboard5),
					Some(Keyboard::Keyboard6),
					Some(Keyboard::Keyboard7),
					Some(Keyboard::Keyboard8),
					Some(Keyboard::Keyboard9),
					Some(Keyboard::Keyboard0),
					//Row 3
					None,
					None,
					None,
					Some(Keyboard::Escape),
					None,
					None,
					Some(Keyboard::Apostrophe),
					None,
					None,
					None,
				],
				thumb_cluster: [
					None,
					Some(ThumbKey::UpDown(Keyboard::DeleteBackspace)),
					None,
					None,
					None,
					None,
					None,
				],
			})
			.add_layer(Layer {
				finger_cluster: [
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
					Some(Keyboard::Tab),
					None,
					None,
					None,
					None,
					None,
					None,
					None,
					Some(Keyboard::ReturnEnter),
					None,
					//Row 3
					None,
					None,
					None,
					Some(Keyboard::Escape),
					None,
					None,
					None,
					None,
					None,
					None,
				],
				thumb_cluster: [
					None,
					None,
					None,
					None,
					Some(ThumbKey::UpDown(Keyboard::DeleteBackspace)),
					None,
					None,
				],
			});
		keymap
	}
}
