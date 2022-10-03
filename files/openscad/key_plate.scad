use<key_holes.scad>;
$fn = 100;
plate_offset = 5;

screw_count = 9;
screw_hole_pos = [
	[7, 3],
	[47.1, 8],
	[85.5, 8],
	[112.1, -25],
	[86.5, -48.2],
	[124.7, -59],
	[117.5, -84.5],
	[48, -49],
	[7, -35]
];
screw_hole_d = 3.5;

module plate(with_holes) {
	difference() {
		screw_hole_material_d = 0.1;
		offset(plate_offset) union() {
			left_top(false);
			for (i = [0:screw_count - 1]) {
				translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
					circle(d = screw_hole_material_d);
			}
			translate([114, -73, 0]) rotate(20.4) square([15, 76]);
		};
		if (with_holes)
			union() {
				left_top(true);
				for (i = [0:screw_count - 1]) {
					translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
						circle(d = screw_hole_d);
				}
			}
		translate([127.045, -76, 0]) rotate(20.4) {
			translate([6.01, 33 - 5.53, 0]) square(9);
			translate([6.01, 75 - 5.53, 0]) square(9);
		}
	}
}
