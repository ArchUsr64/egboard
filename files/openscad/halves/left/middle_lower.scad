// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on

module sliding_channel(holes = true) {
	translate([length + outer_offset, sliding_channel_posY, 0]) mirror([1, 0, 0])
		sliding_channel_add(sliding_channel_full_length, holes);
}

module type_c(upper) {
	if (!upper) {
		square(type_c_size, center = true);
	}
	translate([0, type_c_size[1] / 2, 0]) square([18, 12], center = true);
}
module left_middle_lower(upper = false) {
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
			//Standoff supports
			for (i = [0:len(hole_pos) - 1]) {
				translate([hole_pos[i][0], hole_pos[i][1], 0])
					circle(d = 2 * standoff_size, $fn = 6);
			}
			//Slidng channel supports
			translate([53, -90, 0]) square([5, 35], center = true);
			translate([53, -17, 0]) square([5, 35], center = true);
			translate([94, -17, 0]) square([5, 32], center = true);
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
		//Type C hole
		translate([type_c_pos[0], type_c_pos[1], 0]) type_c(upper);
		//Type A hole
		translate([type_a_pos[0], type_a_pos[1], 0]) type_a(upper);
	}
}
left_middle_lower();
