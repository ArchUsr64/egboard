// clang-format off
include<../config.scad>
use<../../layout/layout.scad>
include<config.scad>;
// clang-format on
module led_bar() {
	difference() {
		square([26, 12.5], center = true);
		square([26, 3], center = true);
	}
}
module oled() {
	screw_hole_pos = [[15.2, 13.7], [15.2, -15], [-15.2, -15], [-15.2, 13.7]];
	square([36, 20], center = true);
	for (i = [0:len(screw_hole_pos) - 1]) {
		translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
			circle(d = screw_hole_m3, $fn = 10);
	}
}
module left_top_plate(holes = true) {
	difference() {
		union() {
			top_plate(holes = false);
			offset(offset) translate([offset, -channel_height + offset, 0])
				square([length - 2 * offset, channel_height - 2 * offset]);
		}
		if (holes) {
			//Get holes for the top_plate
			difference() {
				top_plate(holes = false);
				top_plate(holes = true);
			}
			translate([126, -40, 0]) oled();
			translate([126, -73, 0]) led_bar();
			for (i = [0:1]) {
				translate([
					length - magnet_boundary_offset - magnet_hole_thicknes,
					magnet_hole_posY[i],
					0
				]) magnet(magnet_top_plate_channel_width);
			}
			//Screw holes
			for (i = [0:len(hole_pos) - 1]) {
				translate([hole_pos[i][0], hole_pos[i][1], 0])
					circle(d = screw_hole_m3);
			}
		}
	}
}
left_top_plate();
