//Zip tie dimensions
zip_tie_thickness = 2;
zip_tie_width = 3.5;

//Middle layers
outer_offset = 2;
inner_offset = -7;

//Roundedness
offset = 8;
channel_height = 107;

//Screw size
standoff_size = 6;
screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;

//Vertical 20x2 magnet holes
magnet_hole_posY = [-20, -87];
magnet_boundary_offset = 0;
magnet_hole_thicknes = 2.05;
magnet_top_plate_channel_width = 0;
magnet_upper_middle_plate_channel_width = 20.1;
magnet_lower_middle_plate_channel_width = 20.1;
magnet_bottom_plate_channel_width = 0;
module magnet(channel_width) {
	translate([magnet_hole_thicknes / 2, 0, 0])
		square([magnet_hole_thicknes, channel_width], center = true);
}

//Type A
module type_a(upper) {
	if (!upper) {
		square([16.81, 17.11], center = true);
	}
	translate([0, 11.28, 0]) square([14.5, 5.45], center = true);
}
module type_a_screw_holes() {
	x_offset = 11.82;
	y_offset = -2.755;
	translate([x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
	translate([-x_offset / 2, y_offset, 0]) circle(d = screw_hole_m3);
}

//Sliding channel
sliding_channel_magnet_offset = 25;
sliding_channel_magnet_d = 12;
sliding_channel_cover_magnet_d = 10;
sliding_channel_roundedness = 5;
sliding_channel_wall_thickness = 10;
sliding_channel_pos = [90, (magnet_hole_posY[0] + magnet_hole_posY[1]) / 2];
sliding_channel_width = 40;
sliding_channel_full_length = 100;
sliding_channel_extension = 50;

module sliding_channel_add(channel_length, holes = true) {
	module insertion() {
		offset(sliding_channel_roundedness) {
			square(
				[
					channel_length - 2 * sliding_channel_roundedness,
					sliding_channel_width - 2 *
					sliding_channel_roundedness
				],
				center = true);
		}
		translate([-sliding_channel_roundedness / 2, 0, 0]) square(
			[channel_length - sliding_channel_roundedness, sliding_channel_width],
			center = true);
	}
	translate([channel_length / 2, 0, 0]) difference() {
		offset(sliding_channel_wall_thickness) insertion();
		if (holes) {
			insertion();
		}
		translate([-(channel_length + sliding_channel_wall_thickness) / 2, 0, 0])
			square(
				[
					sliding_channel_wall_thickness,
					sliding_channel_width + 2 *
					sliding_channel_wall_thickness
				],
				center = true);
	}
}

module slider() {
	difference() {
		offset(sliding_channel_roundedness) square(
			[
				sliding_channel_full_length - 2 * sliding_channel_roundedness,
				sliding_channel_width - 2 * sliding_channel_roundedness,
			],
			center = true);
		mirror([1, 0, 0]) translate(
			[-sliding_channel_full_length / 2 + sliding_channel_magnet_offset, 0, 0])
			circle(d = sliding_channel_magnet_d);
	}
}

module slider_cover(length, left = false) {
	difference() {
		sliding_channel_add(length, holes = false);
		translate([(outer_offset - inner_offset) / 2, 0, 0]) square(
			[
				outer_offset - inner_offset,
				sliding_channel_width + 2 *
				sliding_channel_wall_thickness
			],
			center = true);
		if (left) {
			translate([sliding_channel_magnet_offset, 0, 0])
				circle(d = sliding_channel_cover_magnet_d);
		} else {
			translate([length - sliding_channel_magnet_offset, 0, 0])
				circle(d = sliding_channel_cover_magnet_d);
		}
	}
}
