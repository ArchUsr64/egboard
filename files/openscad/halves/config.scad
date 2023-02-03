offset = 5;

channel_height = 105;

screw_hole_m3 = 3.5;
screw_hole_m2 = 2.5;

//Vertical 20x2 magnet holes
magnet_hole_posY = [-20, -85];
magnet_boundary_offset = 1;
magnet_hole_thicknes = 2.1;
module magnet() {
	translate([magnet_hole_thicknes / 2, 0, 0])
		square([magnet_hole_thicknes, 16.5], center = true);
}
