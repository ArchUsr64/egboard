const FINGER_CLUSTER_SIZE: usize = 30;
const THUMB_CLUSTER_SIZE: usize = 7;

use usbd_human_interface_device::page::Keyboard;

#[derive(Clone, Copy)]
enum ThumbKey {
	OneShotModifier(Modifier),
	LayerModifier(u8),
	UpDown(Keyboard),
}

#[derive(Clone, Copy)]
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
	) -> [Keyboard; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 3] {
		let layer = self.get_buffered_layer(key_state);
		let mut result = [Keyboard::NoEventIndicated; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 3];

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
		let any_key_pressed = result
			.iter()
			.filter(|event| **event != Keyboard::NoEventIndicated)
			.count() > 0;
		let thumb_events = self.generate_thumb_events(key_state.thumb_cluster, any_key_pressed);
		result[FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE..]
			.iter_mut()
			.enumerate()
			.for_each(|(i, key)| *key = thumb_events[i]);
		result
	}

	fn generate_thumb_events(&self, state: ThumbState, other_key_pressed: bool) -> [Keyboard; 3] {
		static mut MOD_KEYS: [ModKey; 3] = [
			ModKey::new(Modifier::Control),
			ModKey::new(Modifier::Shift),
			ModKey::new(Modifier::Alt),
		];
		let mut key_state = [false; 3];
		self.default_layer()
			.thumb_cluster
			.iter()
			.enumerate()
			.filter_map(|(index, key)| match (*key, state[index]) {
				(Some(ThumbKey::OneShotModifier(modifier)), true) => Some(modifier),
				_ => None,
			})
			.for_each(|modifier| key_state[modifier.index()] = true);
		unsafe {
			MOD_KEYS
				.iter_mut()
				.enumerate()
				.for_each(|(i, key)| key.tick(key_state[i]));
			[
				MOD_KEYS[0].fire(other_key_pressed),
				MOD_KEYS[1].fire(other_key_pressed),
				MOD_KEYS[2].fire(other_key_pressed),
			]
		}
	}
}
#[derive(Clone, Copy)]
struct ModKey {
	state: OneShotModifierState,
	previous_state: OneShotModifierState,
	previous_key_state: bool,
	modifier: Modifier,
	pressed_since: usize,
	released_since: usize,
}
impl ModKey {
	const fn new(modifier: Modifier) -> Self {
		Self {
			state: OneShotModifierState::Released,
			previous_state: OneShotModifierState::Released,
			previous_key_state: false,
			modifier,
			pressed_since: 0,
			released_since: 0,
		}
	}
	fn key_held(&self, state: bool) -> bool {
		const HOLD_TIMER: usize = 30;
		self.released_since > HOLD_TIMER && state
	}
	fn tick(&mut self, pressed: bool) {
		let key_held = self.key_held(pressed);
		use OneShotModifierState::*;
		let new_state = match (self.state, pressed, key_held) {
			(Released, true, false) => {
				if self.previous_key_state {
					Released
				} else {
					Ready
				}
			}
			(Released, true, true) => Held,
			(Ready, true, true) => Held,
			(Held, false, _) => Released,
			(x, _, _) => x,
		};
		if pressed {
			self.released_since += 1;
			self.pressed_since = 0;
		} else {
			self.pressed_since += 1;
			self.released_since = 0;
		}
		if self.state != new_state {
			self.previous_state = self.state;
			self.state = new_state
		}
		if self.previous_key_state != pressed {
			self.previous_key_state = pressed;
		}
	}
	fn fire(&mut self, other_pressed: bool) -> Keyboard {
		use OneShotModifierState::*;
		let event = self.modifier.to_event();
		match (self.state, other_pressed) {
			(Ready, true) => {
				self.state = if self.previous_key_state {
					Held
				} else {
					Released
				};
				event
			}
			(Released, _) | (Ready, false) => Keyboard::NoEventIndicated,
			_ => event,
		}
	}
}

#[derive(Clone, Copy, PartialEq)]
enum OneShotModifierState {
	Released,
	Ready,
	Held,
}

#[derive(Clone, Copy)]
enum Modifier {
	Control,
	Shift,
	Alt,
}
impl Modifier {
	fn to_event(&self) -> Keyboard {
		match self {
			Self::Control => Keyboard::LeftControl,
			Self::Shift => Keyboard::LeftShift,
			Self::Alt => Keyboard::LeftAlt,
		}
	}
	fn index(&self) -> usize {
		match self {
			Self::Control => 0,
			Self::Shift => 1,
			Self::Alt => 2,
		}
	}
}

#[derive(Clone, Copy)]
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
					Some(ThumbKey::OneShotModifier(Modifier::Alt)),
					Some(ThumbKey::LayerModifier(2)),
					Some(ThumbKey::OneShotModifier(Modifier::Shift)),
					Some(ThumbKey::UpDown(Keyboard::Space)),
					Some(ThumbKey::LayerModifier(1)),
					Some(ThumbKey::OneShotModifier(Modifier::Control)),
					Some(ThumbKey::UpDown(Keyboard::LockingCapsLock)),
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
