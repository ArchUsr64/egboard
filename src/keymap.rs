const FINGER_CLUSTER_SIZE: usize = 30;
const THUMB_CLUSTER_SIZE: usize = 8;

use crate::USB_POLLING_DELAY_MS;
use usbd_human_interface_device::device::mouse::WheelMouseReport;
use usbd_human_interface_device::page::Keyboard;

#[derive(Clone, Copy)]
enum ThumbKey {
	OneShotModifier(Modifier),
	LayerModifier(u8),
	UpDown(Keyboard),
}
#[derive(Clone, Copy)]
struct ScrollKey {
	previous_state: bool,
	pressed_since: u32,
}
impl ScrollKey {
	const fn new() -> Self {
		Self {
			previous_state: false,
			pressed_since: 0,
		}
	}
	fn fire(&mut self, pressed: bool) -> bool {
		let result;
		const HOLD_TIMER: u32 = 300 / USB_POLLING_DELAY_MS;
		match (pressed, self.previous_state) {
			(true, true) => self.pressed_since += 1,
			_ => self.pressed_since = 0,
		}
		result = if self.pressed_since > HOLD_TIMER || (pressed & !self.previous_state) {
			true
		} else {
			false
		};
		self.previous_state = pressed;
		result
	}
}
#[derive(Clone, Copy)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}
#[derive(Clone, Copy)]
enum MouseEvent {
	LeftClick,
	RightClick,
	MiddleClick,
	Cursor(Direction),
	Scroll(Direction),
	/// (cursor, scroll) speeds
	SetSpeed((i8, i8)),
}
struct MouseReportBuilder {
	cursor: (i8, i8),
	buttons: [bool; 3],
	scroll_keys: [ScrollKey; 4],
	scroll_key_state: [bool; 4],
	speed: (i8, i8),
}
impl MouseReportBuilder {
	const DEFAULT_SPEED: (i8, i8) = (10, 1);

	const fn new() -> Self {
		Self {
			cursor: (0, 0),
			buttons: [false; 3],
			scroll_keys: [ScrollKey::new(); 4],
			scroll_key_state: [false; 4],
			speed: Self::DEFAULT_SPEED,
		}
	}

	fn add_event(&mut self, mouse_event: MouseEvent) {
		use Direction::*;
		match mouse_event {
			MouseEvent::LeftClick => self.buttons[0] = true,
			MouseEvent::RightClick => self.buttons[1] = true,
			MouseEvent::MiddleClick => self.buttons[2] = true,
			MouseEvent::Cursor(direction) => match direction {
				Up => self.cursor.1 -= 1,
				Down => self.cursor.1 += 1,
				Left => self.cursor.0 -= 1,
				Right => self.cursor.0 += 1,
			},
			MouseEvent::Scroll(direction) => match direction {
				Up => self.scroll_key_state[0] = true,
				Down => self.scroll_key_state[1] = true,
				Left => self.scroll_key_state[2] = true,
				Right => self.scroll_key_state[3] = true,
			},
			MouseEvent::SetSpeed(val) => self.speed = val,
		}
	}

	fn build(&mut self) -> WheelMouseReport {
		self.scroll_key_state
			.iter_mut()
			.enumerate()
			.for_each(|(i, pressed)| *pressed = self.scroll_keys[i].fire(*pressed));
		let buttons = self
			.buttons
			.iter()
			.enumerate()
			.map(|(i, pressed)| *pressed as u8 * (1 << i))
			.sum::<u8>();
		//Divide by sqrt(2) if the cursor speed is two dimensional
		let (mut cursor_speed, scroll_speed) = self.speed;
		if self.cursor.0 != 0 && self.cursor.1 != 0 {
			cursor_speed = cursor_speed.saturating_mul(10);
			cursor_speed /= 14;
			if cursor_speed == 0 {
				cursor_speed = 1;
			}
		}
		let vertical_wheel =
			(self.scroll_key_state[0] as i8 - self.scroll_key_state[1] as i8) * scroll_speed;
		let horizontal_wheel =
			(self.scroll_key_state[3] as i8 - self.scroll_key_state[2] as i8) * scroll_speed;
		let report = WheelMouseReport {
			buttons,
			x: self.cursor.0 * cursor_speed,
			y: self.cursor.1 * cursor_speed,
			vertical_wheel,
			horizontal_wheel,
		};
		*self = Self {
			scroll_keys: self.scroll_keys,
			..Self::new()
		};
		report
	}
}

#[derive(Clone, Copy)]
enum FingerKey {
	Keyboard(Keyboard),
	Mouse(MouseEvent),
}

#[derive(Clone, Copy)]
pub struct Layer {
	finger_cluster: FingerLayer,
	thumb_cluster: ThumbLayer,
}
type FingerLayer = [Option<FingerKey>; FINGER_CLUSTER_SIZE];
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
	const fn new() -> Self {
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
	) -> (
		[Keyboard; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 4],
		WheelMouseReport,
	) {
		static mut MOUSE_REPORT_BUILDER: MouseReportBuilder = MouseReportBuilder::new();
		let layer = self.get_buffered_layer(key_state);
		let mut key_events = [Keyboard::default(); FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 4];

		key_events[0..FINGER_CLUSTER_SIZE]
			.iter_mut()
			.enumerate()
			.filter(|(i, _)| key_state.finger_cluster[*i])
			.for_each(|(index, event)| {
				let key = layer.finger_cluster[index]
					.unwrap_or(self.default_layer().finger_cluster[index].unwrap());
				match key {
					FingerKey::Keyboard(key) => *event = key,
					FingerKey::Mouse(mouse_event) => unsafe {
						MOUSE_REPORT_BUILDER.add_event(mouse_event)
					},
				}
			});
		key_events[FINGER_CLUSTER_SIZE..FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE]
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
		let any_key_pressed = key_events
			.iter()
			.filter(|event| **event != Keyboard::default())
			.count() > 0;
		let thumb_events = self.generate_thumb_events(key_state.thumb_cluster, any_key_pressed);
		key_events[FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE..]
			.iter_mut()
			.enumerate()
			.for_each(|(i, key)| *key = thumb_events[i]);
		unsafe { (key_events, MOUSE_REPORT_BUILDER.build()) }
	}

	fn generate_thumb_events(&self, state: ThumbState, other_key_pressed: bool) -> [Keyboard; 4] {
		static mut MOD_KEYS: [ModKey; 4] = [
			ModKey::new(Modifier::Control),
			ModKey::new(Modifier::Shift),
			ModKey::new(Modifier::Alt),
			ModKey::new(Modifier::Gui),
		];
		let mut key_state = [false; 4];
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
				MOD_KEYS[3].fire(other_key_pressed),
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
	pressed_since: u32,
	released_since: u32,
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
		const HOLD_TIMER: u32 = 300 / USB_POLLING_DELAY_MS;
		self.released_since > HOLD_TIMER && state
	}
	fn tick(&mut self, pressed: bool) {
		let key_held = self.key_held(pressed);
		use OneShotModifierState::*;
		let new_state = match (self.state, pressed, key_held) {
			(Released, true, false) => Ready,
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
			(Released, _) | (Ready, false) => Keyboard::default(),
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
	Gui,
}
impl Modifier {
	fn to_event(&self) -> Keyboard {
		match self {
			Self::Control => Keyboard::LeftControl,
			Self::Shift => Keyboard::LeftShift,
			Self::Alt => Keyboard::LeftAlt,
			Self::Gui => Keyboard::LeftGUI,
		}
	}
	fn index(&self) -> usize {
		*self as usize
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
		#[path = "default_keymap.rs"]
		mod default_keymap;
		default_keymap::keymap()
	}
}
