// clang-format off
include <config.scad>;
use<../../rust_logo/logo.scad>;
// clang-format on
module logo_screen() {
	difference() {
		circle(d = logo_scale * 32);
		logo(logo_scale, holes_only = true);
		circle(d = logo_scale * 22);
		translate([-16, 0, 0]) square([9, 30], center = true);
	}
}
logo_screen();
