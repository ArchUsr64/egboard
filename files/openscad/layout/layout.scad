include<config.scad> $fn = 20;
module top_plate(holes = true, right = false) {
	echo(-(key_size / 2 + offset));
	translate([key_size / 2 + offset, -(key_size / 2 + offset), 0]) difference() {
		offset(offset) union() {
			holes(true, right);
			translate([push_button_pos[0], push_button_pos[1], 0])
				circle(d = push_button_head - offset * 2);
		};
		if (holes) {
			holes(right = right);
			translate([push_button_pos[0], push_button_pos[1], 0])
				circle(d = push_button_hole);
		}
	}
}
top_plate(holes = true, right = true);
module holes(holes = false, right = false) {
	module square_centered(center2 = [0, 0]) {
		size = key_size;
		if (holes)
			translate([center2[0] - size / 2, center2[1] - size / 2, 0])
				square([size, size]);
		else
			translate(
				[center2[0] - key_hole_size / 2, center2[1] - key_hole_size / 2, 0])
				square([key_hole_size, key_hole_size]);
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
		rotation_offset = rotation_offset_till(thumb_r, key_size, i);
		translate([
			thumb_axis2[0] + thumb_r * cos(rotation_offset + key_rotation_init),
			thumb_axis2[1] + thumb_r * sin(rotation_offset + key_rotation_init),
			0
		]) rotate(key_rotation_init + rotation_offset) square_centered();
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
	if (right) {
		linear_offset_sum = (key_size * linear_key_countR);
		if (linear_key_countR > 0)
			for (i = [0:linear_key_countR - 1]) {
				index_actual = linear_key_countR - i - 1;
				linear_offset = linear_offset_sum - key_size * index_actual;
				square_centered(
					[linear_key_pos_init2[0] - linear_offset, linear_key_pos_init2[1]]);
			}
	} else {
		linear_offset_sum = (key_size * linear_key_count);
		if (linear_key_count > 0)
			for (i = [0:linear_key_count - 1]) {
				index_actual = linear_key_count - i - 1;
				linear_offset = linear_offset_sum - key_size * index_actual;
				square_centered(
					[linear_key_pos_init2[0] - linear_offset, linear_key_pos_init2[1]]);
			}
	}
}
