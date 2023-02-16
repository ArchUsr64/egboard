// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on

module sliding_channel(holes = true) {
	translate([length + outer_offset, sliding_channel_posY, 0]) mirror([1, 0, 0])
		sliding_channel_add(sliding_channel_full_length, holes);
}

module left_middle_lower() {
	difference() {
		union() {
			sliding_channel();
			difference() {
				difference() {
					offset(outer_offset) left_top_plate(holes = false);
					offset(inner_offset) left_top_plate(holes = false);
				}
				difference() {
					sliding_channel(holes = false);
					sliding_channel();
				}
			}
		}
		//Magnet slits
		for (i = [0:1]) {
			translate([
				length - magnet_boundary_offset - magnet_hole_thicknes,
				magnet_hole_posY[i],
				0
			]) magnet(magnet_lower_middle_plate_channel_width);
		}
		//Standoff holes
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0])
				circle(d = standoff_size, $fn = 6);
		}
		//Sliding channel screw holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
	}
}
left_middle_lower();
