// clang-format off
include<../config.scad>;
include<config.scad>;
// clang-format on
module left_slider_cover() {
	difference() {
		translate([length + outer_offset, sliding_channel_posY, 0])
			mirror([1, 0, 0]) slider_cover(sliding_channel_full_length, left = true);
		//Magnet holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
	}
}
left_slider_cover();
