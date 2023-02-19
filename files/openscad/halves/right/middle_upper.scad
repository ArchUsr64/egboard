// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on
module right_middle_upper(upper = false) {
	difference() {
		union() {
			difference() {
				offset(outer_offset) right_top_plate(holes = false);
				offset(inner_offset) right_top_plate(holes = false);
			}
			//Standoff supports
			for (i = [0:len(hole_pos) - 1]) {
				translate([hole_pos[i][0], hole_pos[i][1], 0])
					circle(d = 2 * standoff_size, $fn = 6);
			}
			if (!upper) {
				//Barrel Jack supports
				translate([barrel_jack_pos[0], barrel_jack_pos[1], 0])
					barrel_connector_support();
			}
		}
		//Magnet slit
		for (i = [0:1]) {
			translate([-length + magnet_boundary_offset, magnet_hole_posY[i], 0])
				magnet(magnet_upper_middle_plate_channel_width);
		}
		//Standoff holes
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0])
				circle(d = standoff_size, $fn = 6);
		}
		if (!upper) {
			//Barrel Jack
			translate([barrel_jack_pos[0], barrel_jack_pos[1], 0]) barrel_connector();
		}
	}
}
right_middle_upper();
