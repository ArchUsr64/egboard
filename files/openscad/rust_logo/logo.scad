screw_offset = 1.15;
screw_hole_m2 = 2.5;
module logo(scale = 1, holes_only = false) {
	r = scale * 13.3;
	if (!holes_only) {
		scale([0.1 * scale, 0.1 * scale, 1]) difference() {
			square([340, 340], center = true);
			projection() import("rust_logo.stl");
		}
	}
	for (i = [0:4]) {
		scale(screw_offset) rotate(i * 360 / 5) translate([0, r, 0])
			circle(d = screw_hole_m2);
	}
}
module logo_screen(scale = 1) {
	r = scale * 13.3;
	difference() {
		circle(d = 2.8 * r);
		for (i = [0:4]) {
			scale(screw_offset) rotate(i * 360 / 5) translate([0, r, 0])
				circle(d = screw_hole_m2);
		}
	}
}
logo();
