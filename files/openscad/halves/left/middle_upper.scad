// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on
module left_middle_upper() {
	difference() {
		difference() {
			offset(outer_offset) left_top_plate(holes = false);
			offset(inner_offset) left_top_plate(holes = false);
		}
		for (i = [0:1]) {
			translate([
				length - magnet_boundary_offset - magnet_hole_thicknes,
				magnet_hole_posY[i],
				0
			]) magnet(magnet_upper_middle_plate_channel_width);
		}
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0])
				circle(d = standoff_size, $fn = 6);
		}
	}
}
left_middle_upper();
