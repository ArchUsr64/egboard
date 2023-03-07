// clang-format off
include <config.scad>;
use<../../rust_logo/logo.scad>;
// clang-format on
module logo_screen() {
	difference() {
		circle(d = logo_scale * 38);
		logo(logo_scale, holes_only = true);
		translate([-16, 0, 0]) square([7.4, 30], center = true);
	}
}
logo_screen();
