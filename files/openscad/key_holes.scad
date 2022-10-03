hole_size2 = [14.05, 14.05];
function vec_sum(v, i) = i == 0 ? v[0] : v[i] + vec_sum(v, i - 1);
function rotation_offset_till(radius, sizeXN, sizeYN, i) = i == 0
	? 0
	: 2 * atan(sizeXN[i] / (2 * radius - sizeYN[i]))
		+ rotation_offset_till(radius, sizeXN, sizeYN, i - 1);
module square_centered(size2, center2) {
	translate([center2[0] - size2[0] / 2, center2[1] - size2[1] / 2, 0])
		square([size2[0], size2[1]]);
}
module vert_square_arrayX(length, y_offset, square_size2, center2) {
	for (i = [0:length - 1]) {
		translate([0, (square_size2[1] + y_offset) * -i, 0])
			square_centered(square_size2, center2);
	}
}

module finger_key_set(holes_only) {
	key_sizeXN = [16, 20, 20, 20, 17, 16];
	key_sizeYN = [16, 18, 18, 18, 18, 16];
	key_countYN = [2, 3, 3, 3, 3, 3];
	key_count2 = [6, 3];
	key_offsetYN = [-8, -4, -2.5, 0, -2.5, -3];
	for (i = [0:key_count2[0] - 1]) {
		centerX = i == 0
			? 0
			: vec_sum(key_sizeXN, i) - key_sizeXN[0] / 2 - key_sizeXN[i] / 2;
		if (!holes_only)
			vert_square_arrayX(
				key_countYN[i],
				0,
				[key_sizeXN[i], key_sizeYN[i]],
				[centerX, key_offsetYN[i]]);
		else
			vert_square_arrayX(
				key_countYN[i],
				key_sizeYN[i] - hole_size2[1],
				hole_size2,
				[centerX, key_offsetYN[i]]);
	}
}

module thumb_set(holes_only) {
	thumb_axis2 = [80, -120];
	thumb_r = 61;
	linear_key_count = 2;
	total_key_count = 4;
	rotating_key_count = total_key_count - linear_key_count;
	key_sizeXN = [18, 18, 18, 18, 18];
	key_sizeYN = [18, 18, 18, 18, 18];
	linear_univ_offset = [0, 1.2];
	key_rotation_init = 54;
	for (i = [0:rotating_key_count - 1]) {
		rotation_offset = rotation_offset_till(thumb_r, key_sizeXN, key_sizeYN, i);
		translate([
			thumb_axis2[0] + thumb_r * cos(key_rotation_init + rotation_offset),
			thumb_axis2[1] + thumb_r * sin(key_rotation_init + rotation_offset),
			0
		]) rotate(key_rotation_init + rotation_offset) if (!holes_only)
			square_centered(
				[
					key_sizeXN[total_key_count - i - 1],
					key_sizeYN[total_key_count - i - 1]
				],
				[0, 0]);
		else square_centered(hole_size2, [0, 0]);
	}
	linear_key_pos_init2 = [
		thumb_axis2[0]
			+ thumb_r
				* cos(
					key_rotation_init
					+ rotation_offset_till(
						thumb_r,
						key_sizeXN,
						key_sizeYN,
						rotating_key_count)),
		thumb_axis2[1]
			+ thumb_r
				* sin(
					key_rotation_init
					+ rotation_offset_till(
						thumb_r,
						key_sizeXN,
						key_sizeYN,
						rotating_key_count)),
	] + linear_univ_offset;
	linear_offset_sum = vec_sum(key_sizeXN, linear_key_count);
	for (i = [0:linear_key_count - 1]) {
		index_actual = linear_key_count - i - 1;
		linear_offset = linear_offset_sum - vec_sum(key_sizeXN, index_actual)
			- key_sizeXN[index_actual];
		if (!holes_only)
			square_centered(
				[key_sizeXN[index_actual], key_sizeXN[index_actual]],
				[linear_key_pos_init2[0] - linear_offset, linear_key_pos_init2[1]]);
		else
			square_centered(
				hole_size2,
				[linear_key_pos_init2[0] - linear_offset, linear_key_pos_init2[1]]);
	}
}

module left_top(holes_only) {
	thumb_set(holes_only);
	finger_key_set(holes_only);
}
