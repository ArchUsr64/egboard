use<screw_stud.scad> include<key_plate.scad> use<key_holes.scad> magnet_d = 12;
module magnet() {
	linear_extrude(3) circle(d = magnet_d);
}
module magnet_pair() {
	translate([130.045, -76, 4]) rotate(20.4) {
		translate([6.01, 33, 0]) mirror([1, 0, 1]) magnet();
		translate([6.01, 75, 0]) mirror([1, 0, 1]) magnet();
	}
}
nut_plate_d = [16.6, 14, 14, 18, 12.4, 18, 14, 18, 18];
nut_plate_rotation = [7, 16, -7, -9.6, 0, -9.6, 0, 4, -15];
nut_hole_d = 3.05;
module outside_world() {
	difference() {
		offset(10) plate();
		plate();
	}
}
module basic_plate() {
	difference() {
		plate();
		union() {
			offset(-6) plate();
			for (i = [0:screw_count - 1]) {
				translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
					circle(d = screw_hole_d);
			}
		}
	}
}
module support() {
	translate([7, -46, 0]) square([4, 55]);
	translate([45.27, -46, 0]) square([5.5, 55]);
	translate([50.2, -50.75, 0]) square([34.5, 5.1]);
	translate([88.4, -54.4, 0]) square([35, 12.4]);
	translate([120.4, -41.5, 0]) rotate(20.4) difference() {
		circle(20);
		translate([0, -20, 0]) square(40);
	}
}
module top_plate(include_support = true) {
	difference() {
		union() {
			basic_plate();
			for (i = [0:screw_count - 1]) {
				translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
					rotate(nut_plate_rotation[i]) nut(7, nut_hole_d);
			}
			if (include_support)
				support();
		};
		union() {
			left_top(true);
			for (i = [0:screw_count - 1]) {
				translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
					circle(d = nut_hole_d);
			}
		}
	}
}
module magnet_edge() {
	linear_extrude(4) {
		difference() {
			translate([130.045, -76, 0]) rotate(20.4) square([7, 93]);
			for (i = [0:screw_count - 1]) {
				translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
					rotate(nut_plate_rotation[i]) difference() {
					nut(5.6, 0);
				}
			}
			outside_world();
		}
	}
}
module nuts() {
	for (i = [0:screw_count - 1]) {
		difference() {
			translate([screw_hole_pos[i][0], screw_hole_pos[i][1], 0])
				rotate(nut_plate_rotation[i]) difference() {
				nut(nut_plate_d[i], nut_hole_d);
				nut(5.6, 0);
			}
			left_top(true);
			outside_world();
		}
	}
}
module right_top_support() {
	union() {
		linear_extrude(4) nuts();
		difference() {
			union() {
				linear_extrude(1) top_plate();
				magnet_edge();
			}
			magnet_pair();
			charging_port();
		}
	}
}
module cube_centered(l, b, h) {
	translate([-l / 2, -b / 2, -h / 2]) resize([l, b, h]) cube(5);
}
module charging_port(s = 1) {
	translate([70, 5, 4]) color([0, 1, 0]) scale(s) cube_centered(11, 19, 8);
}
