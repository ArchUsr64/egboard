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
	SetSpeed(u8),
}
struct MouseReportBuilder {
	cursor: (i8, i8),
	buttons: [bool; 3],
	scroll_keys: [ScrollKey; 4],
	scroll_key_state: [bool; 4],
	speed: i8,
}
impl MouseReportBuilder {
	const DEFAULT_SPEED: i8 = 10;
	//To match(QMK)
	//Cursor speed benchmarks from https://cps-check.com/mouse-acceleration
	//Default => 1530px/s
	//Speed 0 => 20px/s
	//Speed 1 => 240px/s
	//Speed 2 => 3840px/s
	//
	//Scroll benchmarks from https://cpstest.org/scroll-test.php
	//Default => 2880px/s
	//Speed 0 => 360px/s
	//Speed 1 => 960px/s
	//Speed 2 => 5640px/s

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
			MouseEvent::SetSpeed(val) => self.speed = val as i8,
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
		let mut cursor_speed = self.speed;
		if self.cursor.0 != 0 && self.cursor.1 != 0 {
			cursor_speed = cursor_speed.saturating_mul(10);
			cursor_speed /= 14;
			if cursor_speed == 0 {
				cursor_speed = 1;
			}
		}
		self.speed = 1;
		let vertical_wheel =
			(self.scroll_key_state[0] as i8 - self.scroll_key_state[1] as i8) * self.speed;
		let horizontal_wheel =
			(self.scroll_key_state[2] as i8 - self.scroll_key_state[3] as i8) * self.speed;
		defmt::println!("{}, {}", vertical_wheel, horizontal_wheel);
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
	) -> (
		[Keyboard; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 3],
		WheelMouseReport,
	) {
		static mut MOUSE_REPORT_BUILDER: MouseReportBuilder = MouseReportBuilder::new();
		let layer = self.get_buffered_layer(key_state);
		let mut key_events =
			[Keyboard::NoEventIndicated; FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE + 3];

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
			.filter(|event| **event != Keyboard::NoEventIndicated)
			.count() > 0;
		let thumb_events = self.generate_thumb_events(key_state.thumb_cluster, any_key_pressed);
		key_events[FINGER_CLUSTER_SIZE + THUMB_CLUSTER_SIZE..]
			.iter_mut()
			.enumerate()
			.for_each(|(i, key)| *key = thumb_events[i]);
		unsafe { (key_events, MOUSE_REPORT_BUILDER.build()) }
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
					Some(FingerKey::Keyboard(Keyboard::Q)),
					Some(FingerKey::Keyboard(Keyboard::W)),
					Some(FingerKey::Keyboard(Keyboard::E)),
					Some(FingerKey::Keyboard(Keyboard::R)),
					Some(FingerKey::Keyboard(Keyboard::T)),
					Some(FingerKey::Keyboard(Keyboard::Y)),
					Some(FingerKey::Keyboard(Keyboard::U)),
					Some(FingerKey::Keyboard(Keyboard::I)),
					Some(FingerKey::Keyboard(Keyboard::O)),
					Some(FingerKey::Keyboard(Keyboard::P)),
					//Row 2
					Some(FingerKey::Keyboard(Keyboard::A)),
					Some(FingerKey::Keyboard(Keyboard::S)),
					Some(FingerKey::Keyboard(Keyboard::D)),
					Some(FingerKey::Keyboard(Keyboard::F)),
					Some(FingerKey::Keyboard(Keyboard::G)),
					Some(FingerKey::Keyboard(Keyboard::H)),
					Some(FingerKey::Keyboard(Keyboard::J)),
					Some(FingerKey::Keyboard(Keyboard::K)),
					Some(FingerKey::Keyboard(Keyboard::L)),
					Some(FingerKey::Keyboard(Keyboard::Semicolon)),
					//Row 3
					Some(FingerKey::Keyboard(Keyboard::Z)),
					Some(FingerKey::Keyboard(Keyboard::X)),
					Some(FingerKey::Keyboard(Keyboard::C)),
					Some(FingerKey::Keyboard(Keyboard::V)),
					Some(FingerKey::Keyboard(Keyboard::B)),
					Some(FingerKey::Keyboard(Keyboard::N)),
					Some(FingerKey::Keyboard(Keyboard::M)),
					Some(FingerKey::Keyboard(Keyboard::Comma)),
					Some(FingerKey::Keyboard(Keyboard::Dot)),
					Some(FingerKey::Keyboard(Keyboard::ForwardSlash)),
				],
				thumb_cluster: [
					Some(ThumbKey::UpDown(Keyboard::Escape)),
					Some(ThumbKey::OneShotModifier(Modifier::Alt)),
					Some(ThumbKey::LayerModifier(2)),
					Some(ThumbKey::OneShotModifier(Modifier::Shift)),
					Some(ThumbKey::UpDown(Keyboard::Space)),
					Some(ThumbKey::LayerModifier(1)),
					Some(ThumbKey::OneShotModifier(Modifier::Control)),
					Some(ThumbKey::LayerModifier(3)),
				],
			})
			.add_layer(Layer {
				finger_cluster: [
					Some(FingerKey::Keyboard(Keyboard::Grave)),
					Some(FingerKey::Keyboard(Keyboard::LeftBrace)),
					Some(FingerKey::Keyboard(Keyboard::RightBrace)),
					Some(FingerKey::Keyboard(Keyboard::Minus)),
					Some(FingerKey::Keyboard(Keyboard::Equal)),
					Some(FingerKey::Keyboard(Keyboard::LeftArrow)),
					Some(FingerKey::Keyboard(Keyboard::DownArrow)),
					Some(FingerKey::Keyboard(Keyboard::UpArrow)),
					Some(FingerKey::Keyboard(Keyboard::RightArrow)),
					Some(FingerKey::Keyboard(Keyboard::Backslash)),
					//Row 2
					Some(FingerKey::Keyboard(Keyboard::Keyboard1)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard2)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard3)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard4)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard5)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard6)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard7)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard8)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard9)),
					Some(FingerKey::Keyboard(Keyboard::Keyboard0)),
					//Row 3
					None,
					None,
					None,
					Some(FingerKey::Keyboard(Keyboard::Escape)),
					None,
					None,
					Some(FingerKey::Keyboard(Keyboard::Apostrophe)),
					None,
					None,
					None,
				],
				thumb_cluster: [
					None,
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
					Some(FingerKey::Mouse(MouseEvent::Cursor(Direction::Up))),
					None,
					None,
					Some(FingerKey::Mouse(MouseEvent::Scroll(Direction::Left))),
					Some(FingerKey::Mouse(MouseEvent::Scroll(Direction::Down))),
					Some(FingerKey::Mouse(MouseEvent::Scroll(Direction::Up))),
					Some(FingerKey::Mouse(MouseEvent::Scroll(Direction::Right))),
					Some(FingerKey::Mouse(MouseEvent::SetSpeed(16))),
					//Row 2
					Some(FingerKey::Keyboard(Keyboard::Tab)),
					Some(FingerKey::Mouse(MouseEvent::Cursor(Direction::Left))),
					Some(FingerKey::Mouse(MouseEvent::Cursor(Direction::Down))),
					Some(FingerKey::Mouse(MouseEvent::Cursor(Direction::Right))),
					Some(FingerKey::Keyboard(Keyboard::LeftGUI)),
					Some(FingerKey::Mouse(MouseEvent::MiddleClick)),
					Some(FingerKey::Mouse(MouseEvent::LeftClick)),
					Some(FingerKey::Mouse(MouseEvent::RightClick)),
					Some(FingerKey::Keyboard(Keyboard::ReturnEnter)),
					Some(FingerKey::Mouse(MouseEvent::SetSpeed(2))),
					//Row 3
					None,
					None,
					None,
					Some(FingerKey::Keyboard(Keyboard::Escape)),
					None,
					None,
					None,
					None,
					None,
					Some(FingerKey::Mouse(MouseEvent::SetSpeed(1))),
				],
				thumb_cluster: [
					None,
					None,
					None,
					None,
					None,
					Some(ThumbKey::UpDown(Keyboard::DeleteBackspace)),
					None,
					None,
				],
			})
			.add_layer(Layer {
				finger_cluster: [
					Some(FingerKey::Keyboard(Keyboard::Mute)),
					Some(FingerKey::Keyboard(Keyboard::VolumeDown)),
					Some(FingerKey::Keyboard(Keyboard::VolumeUp)),
					Some(FingerKey::Keyboard(Keyboard::Insert)),
					Some(FingerKey::Keyboard(Keyboard::PrintScreen)),
					Some(FingerKey::Keyboard(Keyboard::Home)),
					Some(FingerKey::Keyboard(Keyboard::PageDown)),
					Some(FingerKey::Keyboard(Keyboard::PageUp)),
					Some(FingerKey::Keyboard(Keyboard::End)),
					Some(FingerKey::Keyboard(Keyboard::CapsLock)),
					//Row 2
					Some(FingerKey::Keyboard(Keyboard::F1)),
					Some(FingerKey::Keyboard(Keyboard::F2)),
					Some(FingerKey::Keyboard(Keyboard::F3)),
					Some(FingerKey::Keyboard(Keyboard::F4)),
					Some(FingerKey::Keyboard(Keyboard::F5)),
					Some(FingerKey::Keyboard(Keyboard::F6)),
					Some(FingerKey::Keyboard(Keyboard::F7)),
					Some(FingerKey::Keyboard(Keyboard::F8)),
					Some(FingerKey::Keyboard(Keyboard::F9)),
					Some(FingerKey::Keyboard(Keyboard::F10)),
					//Row 3
					None,
					None,
					None,
					Some(FingerKey::Keyboard(Keyboard::F11)),
					None,
					None,
					Some(FingerKey::Keyboard(Keyboard::F12)),
					None,
					None,
					None,
				],
				thumb_cluster: [
					None,
					None,
					Some(ThumbKey::UpDown(Keyboard::DeleteForward)),
					None,
					None,
					Some(ThumbKey::UpDown(Keyboard::DeleteForward)),
					None,
					None,
				],
			});
		keymap
	}
}
