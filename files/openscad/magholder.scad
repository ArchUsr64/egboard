use<key_plate.scad> use<case_plate.scad>;
module magnet_plate()difference() {
	plate(false);
	translate([96, -9, 0]) circle(d = 20.05);
	translate([60, -55, 0]) circle(d = 20.05);
	translate([127.045, -76, 0]) rotate(20.4) {
		translate([6.01, 33-5.53, 0]) square(9);
		translate([6.01, 75-5.53, 0]) square(9);
	}
}
