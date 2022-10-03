include<case_plate.scad>;
module right_bottom_plate() {
	difference() {
		union() {
			linear_extrude(1) plate(false);
			linear_extrude(7) top_plate(false);
			charging_port(1.7);
			translate([65, -10.9, 0.1]) mirror([0, 0, 1]) rotate(180)
				mirror([1, 0, 1]) linear_extrude(14.4)
					polygon([[7, 0], [0, 7], [0, 0]]);
			translate([79, -10.9, 0.1]) mirror([0, 0, 1]) rotate(270)
				mirror([1, 0, 1]) linear_extrude(14.4)
					polygon([[7, 0], [0, 7], [0, 0]]);
		}
		translate([0, 0, 8])
			multmatrix(m = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, -1, 0], [0, 0, 0, 1]])
				right_top_support();
		magnet_pair();
		charging_port();
		translate([0, 0, 7]) linear_extrude(13) plate(false);
		translate([0, 0, -10]) linear_extrude(24) outside_world();
		translate([0, 0, -10]) linear_extrude(10) plate(false);
		color([1, 1, 0]) translate([60, -11.2, 1]) resize([5, 19.2, 8]) cube(1);
		translate([0, 0, 4]) for (i = [0:screw_count - 1]) {
			linear_extrude(3) square_centered([10, 10], screw_hole_pos[i]);
		}
	}
}
