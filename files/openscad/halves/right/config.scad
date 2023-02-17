// clang-format off
include <../config.scad>;
// clang-format on
//Entire halve length excluding offsets
length = 140;

//Extension length for accommodating the battery
extension = 25;

//Sliding channel holes
slider_wall_hole = [[-122, -28.5], [-122, -78.5], [-87, -53.5]];

//Standoff holes
hole_pos = [[-20, -7], [-20, -100], [-87, -100], [-122, -7]];

battery_position = 3;

pcb_position = -47;

type_a_pos = [-100, -12];

logo_scale = 0.8;

//Barrel Jack
barrel_jack_pos = [-70, -10.5];
module barrel_jack_pcb() {
	square([38.6, 25.3], center = true);
}
module barrel_connector() {
	translate([-4.2 - 9.1 / 2, 25.3 / 2 - 14.4 / 2, 0])
		square([9.1, 14.4], center = true);
}
module barrel_jack_pcb_screw_holes() {
	pos = [[-11.3, -6.85], [2.12, -2.23], [13.33, 5.1]];
	for (i = [0:2]) {
		translate([pos[i][0], pos[i][1], 0]) circle(d = screw_hole_m3);
	}
}
