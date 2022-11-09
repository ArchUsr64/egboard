// clang-format off
use <../openscad/key_caps.scad>;
// clang-format on
gap = 19;
keycaps(gap);
translate([6 * gap, 5 * gap, 0]) rotate(180) keycaps(gap);
