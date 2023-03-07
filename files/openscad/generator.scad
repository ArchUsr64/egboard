// clang-format off
use<halves/right/top_plate.scad>
use<halves/right/middle_lower.scad>
use<halves/right/middle_upper.scad>;
use<halves/right/bottom.scad>;
use<halves/right/slider_cover.scad>;
use<halves/right/logo_screen.scad>;
use<halves/left/top_plate.scad>
use<halves/left/middle_lower.scad>
use<halves/left/middle_upper.scad>;
use<halves/left/bottom.scad>;
use<halves/left/slider.scad>;
use<halves/left/slider_cover.scad>;
use<halves/left/hooker.scad>;
use<halves/../rust_logo/logo.scad>;
use<halves/config.scad>;
// clang-format on
$fn = 20;
module mm_5_lower() {
	translate([142, 60, 0]) right_middle_upper();
	translate([-160, 60, 0]) left_middle_upper();
	translate([32, -180, 0]) rotate(180) right_middle_lower();
	translate([0, -180, 0]) rotate(180) left_middle_lower();
}
module mm_10() {
	slider();
}
module mm_5_upper() {
	translate([142, 60, 0]) right_middle_upper(true);
	translate([-160, 60, 0]) left_middle_upper();
	translate([32, -180, 0]) rotate(180) right_middle_lower(true);
	translate([0, -180, 0]) rotate(180) left_middle_lower(true);
}
module mm_1() {
	translate([140, 60, 0]) right_top_plate();
	translate([-160, 60, 0]) left_top_plate();
}
module mm_2() {
	translate([140, 60, 0]) right_bottom();
	translate([-160, 60, 0]) left_bottom();
}
module mm_2_transparent() {
	translate([185, 53.5, 0]) right_slider_cover();
	translate([-100, 53.5, 0]) left_slider_cover();
	translate([-60, 0, 0]) hooker();
	translate([120, 0, 0]) logo_screen();
}
mm_5_lower();
