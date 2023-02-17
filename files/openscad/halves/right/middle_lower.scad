// clang-format off
use<top_plate.scad>
include<../config.scad>
include<config.scad>;
// clang-format on

module sliding_channel(holes = true) {
	translate([-length - outer_offset, sliding_channel_posY])
		sliding_channel_add(sliding_channel_extension, holes);
}

module right_middle_lower() {
	difference() {
		union() {
			sliding_channel();
			difference() {
				difference() {
					offset(outer_offset) right_top_plate(holes = false);
					offset(inner_offset) right_top_plate(holes = false);
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
		}
		//Magnet slits
		for (i = [0:1]) {
			translate([-length + magnet_boundary_offset, magnet_hole_posY[i], 0])
				magnet(magnet_lower_middle_plate_channel_width);
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
		//Type A holes
		translate([type_a_pos[0], type_a_pos[1], 0]) type_a();
		//Barrel Jack
		translate([barrel_jack_pos[0], barrel_jack_pos[1], 0]) barrel_jack_pcb();
	}
}
right_middle_lower();
