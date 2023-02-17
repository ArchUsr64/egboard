// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on

module battery_holes() {
	battery_width = 21;
	holes_offset = 60;
	module hole() {
		difference() {
			square(
				[battery_width + 2 * zip_tie_thickness, zip_tie_width],
				center = true);
			square([battery_width, zip_tie_width], center = true);
		}
	}
	hole();
	translate([0, -holes_offset / 2, 0]) hole();
	translate([0, holes_offset / 2, 0]) hole();
}

module right_bottom() {
	difference() {
		right_top_plate(holes = false);
		//Standoff holes
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0]) circle(d = screw_hole_m3);
		}
		//Sliding channel holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
		translate([battery_position, sliding_channel_posY, 0]) battery_holes();
	}
}
right_bottom();
