// clang-format off
use <../openscad/magholder.scad>;
// clang-format on
magnet_plate();
translate([280, 0, 0]) mirror([1, 0, 0]) magnet_plate();
;
