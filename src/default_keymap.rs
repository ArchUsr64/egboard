use crate::keymap::*;

pub fn keymap() -> Keymap {
	Keymap::new()
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
				Some(ThumbKey::OneShotModifier(Modifier::Gui)),
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
				Some(FingerKey::Mouse(MouseEvent::SetSpeed((16, 5)))),
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
				Some(FingerKey::Mouse(MouseEvent::SetSpeed((2, 1)))),
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
				Some(FingerKey::Mouse(MouseEvent::SetSpeed((1, 1)))),
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
		})
}
