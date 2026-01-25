// clang-format off
use<layout/layout.scad>;
// clang-format on
$fn = 20;
screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;
screw_offset = 10;

module layout_with_holes(holes = false) {
	angle = -18;
	offset = 30;
	module holes(size) {
		holes = [
			[5, 0],
			[35, 0],
			[65, 0],
			[95, 0],
			[5, -37.5],
			[5, -70],
			[43, -80],
			[64.5, -93],
			[83, -93],
			[99, -99],
			[127, -104],
		];
		for (i = [0:len(holes) - 1]) {
			translate([holes[i][0], holes[i][1], 0]) circle(d = size);
		}
	}
	module plate() {
		difference() {
			val = 59;
			x_val = 50.5;
			union() {
				color([0, 0, 1]) offset(rounded) rotate(-angle) translate([x_val,  - val / 2, 0]) square([x_val * 2 - 2 * rounded, 10 + val - 2 * rounded], center = true);
				color([1, 0, 0]) translate([108.75, -52.5, 0]) rotate(-angle) square([28, 92], center = true);
				translate([0, -10, 0]) square([10, 10]);
				layout(false);
				rounded = 4;
				size = [106.1 - 2 * rounded, 76 - 2 * rounded];
				color([0, 1, 0]) offset(rounded) translate([rounded, -size[1] - rounded, 0]) square(size);
				holes(screw_offset);
			}
			holes(screw_hole_m3);
			if (holes) {
				layout(true);
			}
		}
	}
	size = [2 * offset, 141.85];
	translate([0, size[1] / 2 - 12, 0]) square(size, center = true);
	// translate([0, 106.29996, 0]) square([220.809, 35], center = true);
	mirror([1, 0, 0]) translate([-offset, 0, 0]) rotate(angle) translate([-127, 90, 0])
		plate();
	translate([-offset, 0, 0]) rotate(angle) translate([-127, 90, 0])
		plate();
}

// translate([60, 0, 0]) rotate(90) bottom();
top();
offset = 10;
channel_height = 103;

module top() {
	module type_a_screw_holes() {
		x_offset = 11.82;
		y_offset = -2.755;
		translate([x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
		translate([-x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
	}
	module type_c_screw_holes() {
		hole_offset_x = 14.5;
		hole_offset_y = 18.5;
		holes =
			[[hole_offset_x / 2, hole_offset_y / 2],
			 [hole_offset_x / 2, -hole_offset_y / 2],
			 [-hole_offset_x / 2, hole_offset_y / 2],
			 [-hole_offset_x / 2, -hole_offset_y / 2]];
		for (i = [0:len(holes) - 1]) {
			translate([holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m2);
		}
	}
	module pi_pico() {
		hole_offset_x = 15.5;
		hole_offset_y = 76;
		holes =
			[[hole_offset_x / 2, hole_offset_y / 2],
			 [hole_offset_x / 2, -hole_offset_y / 2],
			 [-hole_offset_x / 2, hole_offset_y / 2],
			 [-hole_offset_x / 2, -hole_offset_y / 2]];
		for (i = [0:len(holes) - 1]) {
			translate([holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m2);
		}
		square([22, 65], center = true);
	}
	difference() {
		union(){
			layout_with_holes(holes = true);
			holes(screw_offset);
		}
		translate([0, 35, 0]) pi_pico();
		translate([0, 117, 0])type_c_screw_holes();
		holes(screw_hole_m3);
	}
}

module bottom() {
	difference() {
		union(){
			layout_with_holes(holes = false);
			holes(screw_offset);
		}
		holes(screw_hole_m3);
		attachment_holes(screw_hole_m2);
	}
}

module attachment_holes(size) {
	holes = [
		[0, -8],
		[0, 124.5],
		[110, 124.5],
		[132, 54],
	];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = size);
		translate([-holes[i][0], holes[i][1], 0]) circle(d = size);
	}
}

module holes(size) {
	holes = [
		[43, 25],
		[20, 35],
		[33, 55],
		[33, 124.5],
		[73, 124.5],
		[15, -10]
	];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = size);
		translate([-holes[i][0], holes[i][1], 0]) circle(d = size);
	}
}
