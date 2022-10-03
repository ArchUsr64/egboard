// clang-format off
use <../openscad/case_plate.scad>
// clang-format on
right_top_support();
translate([280, 0, 0]) mirror([1, 0, 0]) right_top_support();;
