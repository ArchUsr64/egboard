$fn = 100;
module nut(d, hole_d, Offset = 0) {
	difference() {
		offset(Offset)
		circle(r = d / sqrt(3), $fn = 6);
		circle(d = hole_d);
	}
}
