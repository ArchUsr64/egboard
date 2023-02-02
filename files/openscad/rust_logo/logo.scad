module logo(scale = 1) {
	scale([0.1 * scale, 0.1 * scale, 1]) difference() {
		square([340, 340], center = true);
		translate([-176.5, -176.5, 0]) import("rust.svg");
	}
}
