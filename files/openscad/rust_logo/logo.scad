// clang-format off
include <../halves/config.scad>
	// clang-format on
	module logo(scale = 1, holes_only = false) {
	if (holes_only) {
		r = scale * 13.3;
		for (i = [0:4]) {
			rotate(i * 360 / 5) translate([0, r, 0]) circle(d = screw_hole_m2);
		}
	} else {
		scale([0.1 * scale, 0.1 * scale, 1]) difference() {
			square([340, 340], center = true);
			translate([-176.5, -176.5, 0]) import("rust.svg");
		}
	}
}
logo();
