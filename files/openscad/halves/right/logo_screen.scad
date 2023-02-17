// clang-format off
include <config.scad>;
use<../../rust_logo/logo.scad>;
// clang-format on
module logo_screen() {
	difference() {
		circle(d = logo_scale * 27);
		logo(logo_scale, holes_only = true);
	}
}
logo_screen();
