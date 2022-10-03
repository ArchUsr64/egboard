// clang-format off
use <../openscad/base_plate.scad>;
// clang-format on
right_bottom_plate();
translate([280, 0, 0]) mirror([1, 0, 0]) right_bottom_plate();;
