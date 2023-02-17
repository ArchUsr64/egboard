// clang-format off
include<../config.scad>
use<../../layout/layout.scad>
use<../../rust_logo/logo.scad>
include<config.scad>;
// clang-format on
module switch () {
	d = 19;
	translate([d / 2, 0, 0]) circle(d = screw_hole_m2);
	translate([-d / 2, 0, 0]) circle(d = screw_hole_m2);
	square([16, 7.5], center = true);
}
module right_top_plate(holes = true) {
	difference() {
		union() {
			mirror([1, 0, 0]) top_plate(holes = false, right = true);
			translate([-length, 0, 0]) offset(offset)
				translate([offset, -channel_height + offset, 0]) square(
					[extension + length - 2 * offset, channel_height - 2 * offset]);
		}
		if (holes) {
			//Get holes for the top_plate
			mirror([1, 0, 0]) difference() {
				top_plate(holes = false, right = true);
				top_plate(holes = true, right = true);
			}
			translate([-50, -63 - 19.525, 0]) switch ();
			translate([-122, -51, 0]) logo(logo_scale);
			//Magnet holes
			for (i = [0:1]) {
				translate([-length + magnet_boundary_offset, magnet_hole_posY[i], 0])
					magnet(magnet_top_plate_channel_width);
			}
			//Screw holes
			for (i = [0:len(hole_pos) - 1]) {
				translate([hole_pos[i][0], hole_pos[i][1], 0])
					circle(d = screw_hole_m3);
			}
		}
	}
}
right_top_plate();
