//Middle layers
outer_offset = 2;
inner_offset = -7;

//Roundedness
offset = 8;
channel_height = 107;

standoff_size = 9;
screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;

//Vertical 20x2 magnet holes
magnet_hole_posY = [-20, -87];
magnet_boundary_offset = 0;
magnet_hole_thicknes = 2.1;
magnet_top_plate_channel_width = 16.5;
magnet_upper_middle_plate_channel_width = 20.5;
magnet_lower_middle_plate_channel_width = 20.5;
module magnet(channel_width) {
	translate([magnet_hole_thicknes / 2, 0, 0])
		square([magnet_hole_thicknes, channel_width], center = true);
}
