module rounded_square(size) {
	square(size, center = true);
	translate([0, size[1] / 2, 0]) circle(d = size[0]);
	translate([0, -size[1] / 2, 0]) circle(d = size[0]);
}
module hooker(holes = true) {
	translate([0, 2, 0]) difference() {
		union() {
			//Torso
			rounded_square([9, 3.5]);
			//Helmet
			translate([1.6, 1.6, 0]) rotate(90) rounded_square([5, 2]);
			o = 1.2;
			//Bag
			translate([-4.5, -2.3, 0]) offset(o)
				square([3 - 2 * o, 8 - 2 * o], center = true);
			//Left Leg
			translate([-2.7, -5, 0]) offset(o)
				square([3.4 - 2 * o, 8 - 2 * o], center = true);
			//Right Leg
			translate([2.8, -4, 0]) offset(o)
				square([3.4 - 2 * o, 8 - 2 * o], center = true);
		}
		if (holes) {
			scale = 0.7;
			//Eyes
			translate([1.0, 1.7, 0]) rotate(90) scale([scale, scale, 1])
				rounded_square([5, 2]);
		}
	}
}
module hooker_channel(length) {
	module hooker_upper() {
		difference() {
			hooker(false);
			translate([0, -5, 0]) square([20, 10], center = true);
		}
	}
	module hooker_lower() {
		difference() {
			hooker();
			translate([0, 5, 0]) square([20, 10], center = true);
		}
	}
	translate([0, length / 2, 0]) hooker_upper();
	translate([0, -length / 2, 0]) hooker_upper();
	translate([0, -length / 2, 0]) hooker_lower();
	translate([-0.75, 0, 0]) square([10.5, length], center = true);
	translate([4.75, +3.6, 0]) square([0.7, length], center = true);
	translate([-0.75, 0, 0]) square([10.5, length], center = true);
}
hooker();
