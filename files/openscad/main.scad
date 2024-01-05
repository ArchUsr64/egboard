// clang-format off
use<layout/layout.scad>;
// clang-format on
$fn = 20;
screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;
screw_offset = 10;

module layout_with_holes(holes = false) {
	angle = -18;
	offset = 20;
	module holes(size) {
		holes = [
			[5, -5],
			[35, -5],
			[65, -5],
			[95, -5],
			[5, -37.5],
			[5, -70],
			[43, -80],
			[83, -92],
			[127, -104],
		];
		for (i = [0:len(holes) - 1]) {
			translate([holes[i][0], holes[i][1], 0]) circle(d = size);
		}
	}
	module plate() {
		difference() {
			union() {
				translate([108, -50, 0]) rotate(16) square([22, 85], center = true);
				layout(false);
				rounded = 4;
				size = [106.1 - 2 * rounded, 76 - 2 * rounded];
				offset(rounded) translate([rounded, -size[1] - rounded, 0]) square(size);
				holes(screw_offset);
			}
			holes(screw_hole_m3);
			if (holes) {
				layout(true);
			}
		}
	}
	size = [40, 104];
	translate([0, size[1] / 2 - 10, 0]) square(size, center = true);
	mirror([1, 0, 0]) translate([-offset, 0, 0]) rotate(angle) translate([-127, 90, 0])
		plate();
	translate([-offset, 0, 0]) rotate(angle) translate([-127, 90, 0])
		plate();
}
bottom();
// rotate(90) translate([0, -50, 0]) top();
offset = 10;
channel_height = 103;
module top() {
	module type_a_screw_holes() {
		x_offset = 11.82;
		y_offset = -2.755;
		translate([x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
		translate([-x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
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
	difference() {
		union(){
			layout_with_holes(holes = true);
			holes(screw_offset);
		}
		translate([0, 40, 0]) pi_pico();
		translate([0, 75, 0])type_a_screw_holes();
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
	}
}

module holes(size) {
	holes = [
		[24, 35],
	];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = size);
		translate([-holes[i][0], holes[i][1], 0]) circle(d = size);
	}
}
