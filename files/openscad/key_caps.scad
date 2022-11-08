use<key_holes.scad> module keycaps() {}
$fn = 100;
module keycap(size2, center2) {
	translate([center2[0], center2[1], 0]) {
		multmatrix(m = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]])
			linear_extrude(1.5, scale = 0.9) offset(4)
				square_centered([size2[0] - 8, size2[0] - 8], [0, 0]);
		linear_extrude(4) projection() import("keycap-stud.stl");
	}
}
diff_key_count = 7;
keycap_size = [
	[2, 1.95, 3.95],
	[3, 3.95, 3.95],
	[3, 5.95, 3.95],
	[3, 4.45, 3.95],
	[3, 2.45, 3.95],
	[3, 2.45, 1.95],
	[4, 3.95, 5.35]
];
module keycaps() {
	for (i = [0:diff_key_count - 1]) {
		for (j = [0:keycap_size[i][0] - 1]) {
			keycap(
				[14.05 + (keycap_size[i][1] / 2) - .5, keycap_size[i][2] - .5],
				[20 * i, 20 * j]);
		}
	}
}
