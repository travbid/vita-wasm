use wasm_bindgen::prelude::*;

use std::boxed::Box;

// #[wasm_bindgen]
// extern "C" {
// 	pub fn log(s: &str);
// }

#[wasm_bindgen(js_name="invertMat4x4")]
pub fn invert_mat4x4(mat: &mut [f64]) {
	if mat.len() != 16 {
		return;
	}

	let a00 = mat[0];
	let a01 = mat[1];
	let a02 = mat[2];
	let a03 = mat[3];
	let a10 = mat[4];
	let a11 = mat[5];
	let a12 = mat[6];
	let a13 = mat[7];
	let a20 = mat[8];
	let a21 = mat[9];
	let a22 = mat[10];
	let a23 = mat[11];
	let a30 = mat[12];
	let a31 = mat[13];
	let a32 = mat[14];
	let a33 = mat[15];

	let b00 = a00 * a11 - a01 * a10;
	let b01 = a00 * a12 - a02 * a10;
	let b02 = a00 * a13 - a03 * a10;
	let b03 = a01 * a12 - a02 * a11;
	let b04 = a01 * a13 - a03 * a11;
	let b05 = a02 * a13 - a03 * a12;
	let b06 = a20 * a31 - a21 * a30;
	let b07 = a20 * a32 - a22 * a30;
	let b08 = a20 * a33 - a23 * a30;
	let b09 = a21 * a32 - a22 * a31;
	let b10 = a21 * a33 - a23 * a31;
	let b11 = a22 * a33 - a23 * a32;

	// Calculate the determinant
	let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

	if det == 0.0 {
		return;
	}
	det = 1.0 / det;

	mat[0] = (a11 * b11 - a12 * b10 + a13 * b09) * det;
	mat[1] = (a02 * b10 - a01 * b11 - a03 * b09) * det;
	mat[2] = (a31 * b05 - a32 * b04 + a33 * b03) * det;
	mat[3] = (a22 * b04 - a21 * b05 - a23 * b03) * det;
	mat[4] = (a12 * b08 - a10 * b11 - a13 * b07) * det;
	mat[5] = (a00 * b11 - a02 * b08 + a03 * b07) * det;
	mat[6] = (a32 * b02 - a30 * b05 - a33 * b01) * det;
	mat[7] = (a20 * b05 - a22 * b02 + a23 * b01) * det;
	mat[8] = (a10 * b10 - a11 * b08 + a13 * b06) * det;
	mat[9] = (a01 * b08 - a00 * b10 - a03 * b06) * det;
	mat[10] = (a30 * b04 - a31 * b02 + a33 * b00) * det;
	mat[11] = (a21 * b02 - a20 * b04 - a23 * b00) * det;
	mat[12] = (a11 * b07 - a10 * b09 - a12 * b06) * det;
	mat[13] = (a00 * b09 - a01 * b07 + a02 * b06) * det;
	mat[14] = (a31 * b01 - a30 * b03 - a32 * b00) * det;
	mat[15] = (a20 * b03 - a21 * b01 + a22 * b00) * det;
}

#[wasm_bindgen(js_name="invertedMat4x4")]
pub fn inverted_mat4x4(mat: &[f64]) -> Box<[f64]> {
	// log("invert_mat4x4");

	if mat.len() != 16 {
		return Box::new([]);
	}

	let a00 = mat[0];
	let a01 = mat[1];
	let a02 = mat[2];
	let a03 = mat[3];
	let a10 = mat[4];
	let a11 = mat[5];
	let a12 = mat[6];
	let a13 = mat[7];
	let a20 = mat[8];
	let a21 = mat[9];
	let a22 = mat[10];
	let a23 = mat[11];
	let a30 = mat[12];
	let a31 = mat[13];
	let a32 = mat[14];
	let a33 = mat[15];

	let b00 = a00 * a11 - a01 * a10;
	let b01 = a00 * a12 - a02 * a10;
	let b02 = a00 * a13 - a03 * a10;
	let b03 = a01 * a12 - a02 * a11;
	let b04 = a01 * a13 - a03 * a11;
	let b05 = a02 * a13 - a03 * a12;
	let b06 = a20 * a31 - a21 * a30;
	let b07 = a20 * a32 - a22 * a30;
	let b08 = a20 * a33 - a23 * a30;
	let b09 = a21 * a32 - a22 * a31;
	let b10 = a21 * a33 - a23 * a31;
	let b11 = a22 * a33 - a23 * a32;

	// Calculate the determinant
	let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

	if det == 0.0 {
		return Box::new([]);
	}
	det = 1.0 / det;

	let res: [f64; 16] = [
		(a11 * b11 - a12 * b10 + a13 * b09) * det,
		(a02 * b10 - a01 * b11 - a03 * b09) * det,
		(a31 * b05 - a32 * b04 + a33 * b03) * det,
		(a22 * b04 - a21 * b05 - a23 * b03) * det,
		(a12 * b08 - a10 * b11 - a13 * b07) * det,
		(a00 * b11 - a02 * b08 + a03 * b07) * det,
		(a32 * b02 - a30 * b05 - a33 * b01) * det,
		(a20 * b05 - a22 * b02 + a23 * b01) * det,
		(a10 * b10 - a11 * b08 + a13 * b06) * det,
		(a01 * b08 - a00 * b10 - a03 * b06) * det,
		(a30 * b04 - a31 * b02 + a33 * b00) * det,
		(a21 * b02 - a20 * b04 - a23 * b00) * det,
		(a11 * b07 - a10 * b09 - a12 * b06) * det,
		(a00 * b09 - a01 * b07 + a02 * b06) * det,
		(a31 * b01 - a30 * b03 - a32 * b00) * det,
		(a20 * b03 - a21 * b01 + a22 * b00) * det,
	];

	Box::new(res)
}

#[wasm_bindgen(js_name="rotateMat4x4")]
pub fn rotate_mat4x4(mat: &mut [f64], angle: f64, axis: &[f64]) {
	if mat.len() != 16 || axis.len() != 3 {
		return;
	}

	const EPSILON: f64 = 0.00001;
	let mut x = axis[0];
	let mut y = axis[1];
	let mut z = axis[2];
	let mut len = ((x * x) + (y * y) + (z * z)).sqrt();

	if len < EPSILON {
		return;
	}

	len = 1.0 / len;
	x *= len;
	y *= len;
	z *= len;

	let s = angle.sin();
	let c = angle.cos();
	let t = 1.0 - c;

	// Construct the elements of the rotation matrix
	let b00 = x * x * t + c;     let b01 = x * y * t - z * s; let b02 = x * z * t + y * s;
	let b10 = y * x * t + z * s; let b11 = y * y * t + c;     let b12 = y * z * t - x * s;
	let b20 = x * z * t - y * s; let b21 = y * z * t + x * s; let b22 = z * z * t + c;

	let a00 = mat[0]; let a01 = mat[1]; let a02 = mat[2];  let a03 = mat[3];
	let a10 = mat[4]; let a11 = mat[5]; let a12 = mat[6];  let a13 = mat[7];
	let a20 = mat[8]; let a21 = mat[9]; let a22 = mat[10]; let a23 = mat[11];

	mat[0] = b00 * a00 + b01 * a10 + b02 * a20;
	mat[1] = b00 * a01 + b01 * a11 + b02 * a21;
	mat[2] = b00 * a02 + b01 * a12 + b02 * a22;
	mat[3] = b00 * a03 + b01 * a13 + b02 * a23;

	mat[4] = b10 * a00 + b11 * a10 + b12 * a20;
	mat[5] = b10 * a01 + b11 * a11 + b12 * a21;
	mat[6] = b10 * a02 + b11 * a12 + b12 * a22;
	mat[7] = b10 * a03 + b11 * a13 + b12 * a23;

	mat[8] = b20 * a00 + b21 * a10 + b22 * a20;
	mat[9] = b20 * a01 + b21 * a11 + b22 * a21;
	mat[10] = b20 * a02 + b21 * a12 + b22 * a22;
	mat[11] = b20 * a03 + b21 * a13 + b22 * a23;
}

#[cfg(test)]
mod tests {
	#[test]
	fn inversion() {
		#[rustfmt::skip]
		let mut mat1: [f64; 16] = [1.0, 0.0, 1.0, 2.0,
		                          -1.0, 1.0, 2.0, 0.0,
		                          -2.0, 0.0, 1.0, 2.0,
		                           0.0, 0.0, 0.0, 1.0];
		let res: Box<[f64]> = super::inverted_mat4x4(&mat1);

		#[rustfmt::skip]
		let expected1: [f64; 16] = [1.0/3.0, 0.0, -1.0/3.0,  0.0,
		                               -1.0, 1.0,     -1.0,  4.0,
		                            2.0/3.0, 0.0,  1.0/3.0, -2.0,
		                                0.0, 0.0,      0.0,  1.0];

		for i in 0..16 {
			assert!((res[i] - expected1[i]).abs() <= core::f64::EPSILON);
		}

		super::invert_mat4x4(&mut mat1);
		for i in 0..16 {
			assert!((mat1[i] - expected1[i]).abs() <= core::f64::EPSILON);
		}

		#[rustfmt::skip]
		let mut mat2 = [4.0, 0.0, 0.0, 0.0,
		                0.0, 0.0, 2.0, 0.0,
		                0.0, 1.0, 2.0, 0.0,
		                1.0, 0.0, 0.0, 1.0];

		#[rustfmt::skip]
		let expected2: [f64; 16] = [0.25, 0.0, 0.0, 0.0,
		                            0.0, -1.0, 1.0, 0.0,
		                            0.0,  0.5, 0.0, 0.0,
		                           -0.25, 0.0, 0.0, 1.0];

		let res2 = super::inverted_mat4x4(&mat2);
		for i in 0..16 {
			assert!((res2[i] - expected2[i]).abs() <= core::f64::EPSILON);
		}

		super::invert_mat4x4(&mut mat2);
		for i in 0..16 {
			assert!((mat2[i] - expected2[i]).abs() <= core::f64::EPSILON);
		}
	}

	#[test]
	fn rotation() {
		const PI: f64 = 3.141592653589793238462643383279502884;
		#[rustfmt::skip]
		let mut mat1: [f64; 16] = [1.0, 0.0, 0.0, 1.0,
		                           0.0, 1.0, 0.0, 0.0,
		                           0.0, 0.0, 1.0, 0.0,
		                           0.0, 0.0, 0.0, 0.0];
		#[rustfmt::skip]
		let expected1: [f64; 16] = [0.0, 0.0, 1.0,  0.0,
		                            0.0, 1.0, 0.0,  0.0,
		                           -1.0, 0.0, 0.0, -1.0,
		                            0.0, 0.0, 0.0,  0.0];
		super::rotate_mat4x4(&mut mat1, PI / 2.0, &[0.0, 1.0, 0.0]);
		for i in 0..16 {
			assert!((mat1[i] - expected1[i]).abs() <= core::f64::EPSILON);
		}

		#[rustfmt::skip]
		let mut mat2: [f64; 16] = [1.0, 0.0, 0.0, 0.0,
		                           0.0, 1.0, 0.0, 1.0,
		                           0.0, 0.0, 1.0, 0.0,
		                           0.0, 0.0, 0.0, 0.0];

		#[rustfmt::skip]
		let expected2: [f64; 16] = [0.0, -1.0, 0.0, -1.0,
		                            1.0,  0.0, 0.0,  0.0,
		                            0.0,  0.0, 1.0,  0.0,
		                            0.0,  0.0, 0.0,  0.0];
		super::rotate_mat4x4(&mut mat2, PI / 2.0, &[0.0, 0.0, 1.0]);
		for i in 0..16 {
			assert!((mat2[i] - expected2[i]).abs() <= core::f64::EPSILON);
		}
	}
}
