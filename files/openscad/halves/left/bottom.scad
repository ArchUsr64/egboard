// clang-format off
use<top_plate.scad>
use<hooker.scad>;
include<../config.scad>
include<config.scad>;
// clang-format on
module pi_pico() {
	hole_offset_x = 11.4;
	hole_offset_y = 46;
	holes =
		[[hole_offset_x / 2, hole_offset_y / 2],
		 [hole_offset_x / 2, -hole_offset_y / 2],
		 [-hole_offset_x / 2, hole_offset_y / 2],
		 [-hole_offset_x / 2, -hole_offset_y / 2]];
	for (i = [0:len(holes) - 1]) {
		translate([holes[i][0], holes[i][1], 0]) circle(d = screw_hole_m2);
	}
}
module type_c() {
	x_offset = 32.74;
	y_offset = 4.6;
	translate([-x_offset / 2, type_c_size[1] / 2 - y_offset, 0])
		circle(d = screw_hole_m2);
	translate([x_offset / 2, type_c_size[1] / 2 - y_offset, 0])
		circle(d = screw_hole_m2);
}
module left_bottom() {
	difference() {
		left_top_plate(holes = false);
		//Standoff holes
		for (i = [0:len(hole_pos) - 1]) {
			translate([hole_pos[i][0], hole_pos[i][1], 0]) circle(d = screw_hole_m3);
		}
		//Sliding channel holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
		//Sliding hooker channel
		translate([110, sliding_channel_posY, 0]) rotate(-90)
			hooker_channel(sliding_channel_extension);
		//Pico holes
		translate([27, sliding_channel_posY, 0]) pi_pico();
		translate([type_c_pos[0], type_c_pos[1], 0]) type_c();
	}
}
left_bottom();
