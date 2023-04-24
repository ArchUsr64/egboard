include<config.scad>;
module layout(holes = true) {
	translate([-140, 0, 0])
		translate([key_size / 2 + offset, -(key_size / 2 + offset), 0])
			difference() {
		offset(offset) union() {
			holes(true);
		};
		if (holes) {
			holes();
		}
	}
}
layout(holes = true);
module holes(holes = false) {
	module square_centered(center2 = [0, 0]) {
		size = key_hole_size;
		if (holes) {
			size = key_size;
		}
		translate([center2[0], center2[1], 0])
		square([size, size], center = true);
	}

	for (i = [0:len(key_column_count) - 1]) {
		center_x = i * key_size;
		for (j = [0:key_column_count[i] - 1]) {
			center_y = j * -key_size + key_y_offset[i];
			square_centered([center_x, center_y]);
		}
	}

	function vec_sum(v, i) = i == 0 ? v[0] : v[i] + vec_sum(v, i - 1);
	function rotation_offset_till(radius, key_size, i) =
		i * 2 * atan(.5 * key_size / (radius - key_size * 0.5));
	for (i = [0:rotating_key_count - 1]) {
		rotation_offset = rotation_offset_till(thumb_r, key_size, i) + key_rotation_init;
		translate([
			thumb_axis2[0] + thumb_r * cos(rotation_offset),
			thumb_axis2[1] + thumb_r * sin(rotation_offset),
			0
		]) rotate(rotation_offset) square_centered();
	}
	linear_key_pos_init2 = [
		thumb_axis2[0]
			+ thumb_r
				* cos(
					key_rotation_init
					+ rotation_offset_till(thumb_r, key_size, rotating_key_count - 1)),
		thumb_axis2[1]
			+ thumb_r
				* sin(
					key_rotation_init
					+ rotation_offset_till(thumb_r, key_size, rotating_key_count - 1)),
	] + linear_univ_offset;
	linear_offset_sum = (key_size * linear_key_count);
	if (linear_key_count > 0)
		for (i = [0:linear_key_count - 1]) {
			index_actual = linear_key_count - i - 1;
			linear_offset = linear_offset_sum - key_size * index_actual;
			square_centered(
				[linear_key_pos_init2[0] - linear_offset, linear_key_pos_init2[1]]);
		}
}
