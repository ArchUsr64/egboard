// clang-format off
include<../config.scad>;
include<config.scad>;
// clang-format on
module right_slider_cover() {
	difference() {
		translate([-length - outer_offset, sliding_channel_posY])
			slider_cover(sliding_channel_extension);
		for (i = [0:len(slider_wall_hole) - 1]) {
			translate([slider_wall_hole[i][0], slider_wall_hole[i][1], 0])
				circle(d = screw_hole_m3);
		}
	}
}
right_slider_cover();
