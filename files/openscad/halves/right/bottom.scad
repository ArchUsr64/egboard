// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on
module right_bottom() {
	difference() {
		right_top_plate(holes = false);
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0])
				circle(d = screw_hole_m3, $fn = 6);
		}
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
	}
}
right_bottom();
