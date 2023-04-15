#[macro_export]
macro_rules! output_keys {
	(@num $pin:expr, 0) => {$pin.gpio0.into_push_pull_output()};
	(@num $pin:expr, 1) => {$pin.gpio1.into_push_pull_output()};
	(@num $pin:expr, 2) => {$pin.gpio2.into_push_pull_output()};
	(@num $pin:expr, 3) => {$pin.gpio3.into_push_pull_output()};
	(@num $pin:expr, 4) => {$pin.gpio4.into_push_pull_output()};
	(@num $pin:expr, 5) => {$pin.gpio5.into_push_pull_output()};
	(@num $pin:expr, 6) => {$pin.gpio6.into_push_pull_output()};
	(@num $pin:expr, 7) => {$pin.gpio7.into_push_pull_output()};
	(@num $pin:expr, 8) => {$pin.gpio8.into_push_pull_output()};
	(@num $pin:expr, 9) => {$pin.gpio9.into_push_pull_output()};
	(@num $pin:expr, 10) => {$pin.gpio10.into_push_pull_output()};
	(@num $pin:expr, 11) => {$pin.gpio11.into_push_pull_output()};
	(@num $pin:expr, 12) => {$pin.gpio12.into_push_pull_output()};
	(@num $pin:expr, 13) => {$pin.gpio13.into_push_pull_output()};
	(@num $pin:expr, 14) => {$pin.gpio14.into_push_pull_output()};
	(@num $pin:expr, 15) => {$pin.gpio15.into_push_pull_output()};
	(@num $pin:expr, 16) => {$pin.gpio16.into_push_pull_output()};
	(@num $pin:expr, 17) => {$pin.gpio17.into_push_pull_output()};
	(@num $pin:expr, 18) => {$pin.gpio18.into_push_pull_output()};
	(@num $pin:expr, 19) => {$pin.gpio19.into_push_pull_output()};
	(@num $pin:expr, 20) => {$pin.gpio20.into_push_pull_output()};
	(@num $pin:expr, 21) => {$pin.gpio21.into_push_pull_output()};
	(@num $pin:expr, 22) => {$pin.gpio22.into_push_pull_output()};
	(@num $pin:expr, 23) => {$pin.gpio23.into_push_pull_output()};
	(@num $pin:expr, 24) => {$pin.gpio24.into_push_pull_output()};
	(@num $pin:expr, 25) => {$pin.gpio25.into_push_pull_output()};
	(@num $pin:expr, 26) => {$pin.gpio26.into_push_pull_output()};
	(@num $pin:expr, 27) => {$pin.gpio27.into_push_pull_output()};
	(@num $pin:expr, 28) => {$pin.gpio28.into_push_pull_output()};
	($pin:expr, $($N:tt), +) => {
		[$(&mut output_keys!(@num $pin, $N)), *]
	}
}
#[macro_export]
macro_rules! input_keys {
	(@num $pin:expr, 0) => {$pin.gpio0.into_pull_down_input()};
	(@num $pin:expr, 1) => {$pin.gpio1.into_pull_down_input()};
	(@num $pin:expr, 2) => {$pin.gpio2.into_pull_down_input()};
	(@num $pin:expr, 3) => {$pin.gpio3.into_pull_down_input()};
	(@num $pin:expr, 4) => {$pin.gpio4.into_pull_down_input()};
	(@num $pin:expr, 5) => {$pin.gpio5.into_pull_down_input()};
	(@num $pin:expr, 6) => {$pin.gpio6.into_pull_down_input()};
	(@num $pin:expr, 7) => {$pin.gpio7.into_pull_down_input()};
	(@num $pin:expr, 8) => {$pin.gpio8.into_pull_down_input()};
	(@num $pin:expr, 9) => {$pin.gpio9.into_pull_down_input()};
	(@num $pin:expr, 10) => {$pin.gpio10.into_pull_down_input()};
	(@num $pin:expr, 11) => {$pin.gpio11.into_pull_down_input()};
	(@num $pin:expr, 12) => {$pin.gpio12.into_pull_down_input()};
	(@num $pin:expr, 13) => {$pin.gpio13.into_pull_down_input()};
	(@num $pin:expr, 14) => {$pin.gpio14.into_pull_down_input()};
	(@num $pin:expr, 15) => {$pin.gpio15.into_pull_down_input()};
	(@num $pin:expr, 16) => {$pin.gpio16.into_pull_down_input()};
	(@num $pin:expr, 17) => {$pin.gpio17.into_pull_down_input()};
	(@num $pin:expr, 18) => {$pin.gpio18.into_pull_down_input()};
	(@num $pin:expr, 19) => {$pin.gpio19.into_pull_down_input()};
	(@num $pin:expr, 20) => {$pin.gpio20.into_pull_down_input()};
	(@num $pin:expr, 21) => {$pin.gpio21.into_pull_down_input()};
	(@num $pin:expr, 22) => {$pin.gpio22.into_pull_down_input()};
	(@num $pin:expr, 23) => {$pin.gpio23.into_pull_down_input()};
	(@num $pin:expr, 24) => {$pin.gpio24.into_pull_down_input()};
	(@num $pin:expr, 25) => {$pin.gpio25.into_pull_down_input()};
	(@num $pin:expr, 26) => {$pin.gpio26.into_pull_down_input()};
	(@num $pin:expr, 27) => {$pin.gpio27.into_pull_down_input()};
	(@num $pin:expr, 28) => {$pin.gpio28.into_pull_down_input()};
	($pin:expr, $($N:tt), +) => {
		[$(&input_keys!(@num $pin, $N)), *]
	}
}
