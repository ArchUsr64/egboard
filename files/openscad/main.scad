// clang-format off
use<layout/layout.scad>;
// clang-format on
$fn = 20;
screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;
module plate(holes = true) {
	mirror([1, 0, 0]) layout(holes);
	layout(holes);
}
offset = 10;
channel_height = 103;
module top() {
	difference() {
		union() {
			plate();
			translate([0, -channel_height / 2, 0]) offset(offset)
				square([280 - 2 * offset, channel_height - 2 * offset], center = true);
		}
		difference() {
			plate(holes = false);
			plate(holes = true);
		}
		translate([0, -28, 0]) color([1, 0, 0, 0.5]) oled();
		holes();
	}
}

module type_a_screw_holes() {
	x_offset = 11.82;
	y_offset = -2.755;
	translate([x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
	translate([-x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
}
module type_c() {
	type_c_size = [40, 25.5];
	x_offset = 32.74;
	y_offset = 4.6;
	translate([-x_offset / 2, type_c_size[1] / 2 - y_offset, 0])
		circle(d = screw_hole_m2);
	translate([x_offset / 2, type_c_size[1] / 2 - y_offset, 0])
		circle(d = screw_hole_m2);
}
module pi_pico() {
	hole_offset_x = 11.4;
	hole_offset_y = 46;
	holes =
		[[hole_offset_x / 2, hole_offset_y / 2],
		 [hole_offset_x / 2, -hole_offset_y / 2],
		 [-hole_offset_x / 2, hole_offset_y / 2],
		 [-hole_offset_x / 2, -hole_offset_y / 2]];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m2);
	}
}
module bottom() {
	difference() {
		union() {
			plate(holes = false);
			translate([0, -channel_height / 2, 0]) offset(offset)
				square([280 - 2 * offset, channel_height - 2 * offset], center = true);
		}
		translate([75, -50, 0]) pi_pico();
		translate([-100, -14, 0]) type_c();
		translate([-100, -74, 0]) type_c();
		translate([100, -15, 0]) type_a_screw_holes();
		holes();
	}
}
module holes() {
	holes = [
		[22, -5],
		[75, -5],
		[22, -50],
		[125, -5],
		[135, -50],
		[125, -98],
		[22, -115],
		[0, -98],
		[75, -98]
	];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m3);
		translate([-holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m3);
	}
}
