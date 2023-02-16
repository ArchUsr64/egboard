// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on
module left_bottom() {
	difference() {
		left_top_plate(holes = false);
		//Standoff holes
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0])
				circle(d = screw_hole_m3);
		}
		//Sliding channel holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
	}
}
left_bottom();
