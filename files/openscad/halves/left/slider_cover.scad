// clang-format off
include<../config.scad>;
include<config.scad>;
// clang-format on
module left_slider_cover() {
	difference() {
		translate([length + outer_offset, sliding_channel_pos[1], 0])
			mirror([1, 0, 0]) slider_cover(sliding_channel_full_length, left = true);
		//Magnet holes
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
		//Slider nut channel
		nut_d = 3.6;
		translate([
			sliding_channel_pos[0] + hooker_screw_hole_offsetY / 2,
			sliding_channel_pos[1],
			0
		]) square([sliding_channel_extension + nut_d, nut_d], center = true);
	}
}
left_slider_cover();
