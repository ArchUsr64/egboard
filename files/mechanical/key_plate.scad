// clang-format off
use <../openscad/key_plate.scad>;
// clang-format on
plate(true);
translate([280, 0, 0]) mirror([1, 0, 0]) plate(true);
