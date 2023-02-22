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

module pcb_holes() {
	x_offset = 37.9;
	y_offset = [5.35, -17.9];
	pos = [y_offset[0], y_offset[1], -y_offset[0], -y_offset[1]];
	for (i = [0:1]) {
		translate([-x_offset / 2, pos[i], 0]) circle(d = screw_hole_m3);
		translate([x_offset / 2, pos[i], 0]) circle(d = screw_hole_m3);
	}
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
		//Battery zip tie holes
		translate([battery_position, sliding_channel_pos[1], 0]) battery_holes();
		//PCB holes
		translate([pcb_position, sliding_channel_pos[1], 0]) pcb_holes();
		//Type A holes
		translate([type_a_pos[0], type_a_pos[1], 0]) type_a_screw_holes();
		//Barrel Jack
		translate([barrel_jack_pos[0], barrel_jack_pos[1], 0]) barrel_jack_pcb_screw_holes();
	}
}
right_bottom();
