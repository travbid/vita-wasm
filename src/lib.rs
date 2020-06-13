#![allow(clippy::identity_op)]

use core::convert::TryInto;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::HashSet;

use wasm_bindgen::prelude::*;

#[allow(non_camel_case_types)]
// type int = isize;
#[allow(non_camel_case_types)]
type unt = usize;

#[derive(Clone, PartialOrd)]
struct Vertex {
	x: f32,
	y: f32,
	z: f32,
}

// Must be derived manually because Hash is manually derived
impl PartialEq for Vertex {
	fn eq(&self, other: &Self) -> bool {
		self.x.to_bits() == other.x.to_bits()
			&& self.y.to_bits() == other.y.to_bits()
			&& self.z.to_bits() == other.z.to_bits()
	}
}

impl Eq for Vertex {} // Required for Ord

impl Ord for Vertex {
	fn cmp(&self, other: &Self) -> core::cmp::Ordering {
		let diff = (self.x + self.y + self.z) - (other.x + other.y + other.z);
		if diff > 0.0 {
			core::cmp::Ordering::Greater
		} else if diff < 0.0 {
			core::cmp::Ordering::Less
		} else {
			core::cmp::Ordering::Equal
		}
	}
}

impl core::hash::Hash for Vertex {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		self.x.to_bits().hash(state);
		self.y.to_bits().hash(state);
		self.z.to_bits().hash(state);
	}
}

#[derive(Eq, Ord, PartialOrd)]
struct Edge {
	a: u32,
	b: u32,
}

// Equal even if a and b are swapped
impl PartialEq for Edge {
	fn eq(&self, other: &Self) -> bool {
		(self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
	}
}

impl core::hash::Hash for Edge {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		let (x, y) = if self.a > self.b {
			(self.a, self.b)
		} else {
			(self.b, self.a)
		};
		x.hash(state);
		y.hash(state);
	}
}

#[wasm_bindgen(js_name = "parseSTL")]
pub fn parse_stl(
	buf: Vec<u8>,
	vertices: &mut [f32],
	normals: &mut [f32],
	v_indices: &mut [u32],
	e_indices: &mut [u32],
) -> Option<String> {
	if buf.len() < 80 {
		return Some(String::from(
			"File is too small to be an STL. File header should be 80 bytes.",
		));
	}
	if buf.len() < 84 {
		return Some(String::from(
			"File is too small to be an STL. There should be a UINT32 at position 80.",
		));
	}

	let mut b: [u8; 4] = [0, 0, 0, 0];
	b.copy_from_slice(&buf[80..84]);
	let num_triangles = u32::from_le_bytes(b);
	println!("num_triangles = {}", num_triangles);

	// FRAME_SIZE = (4 * vertex) + Uint16 (1 Normal, 3 Vertices, 1 Attribute byte count)
	//            = (4 * 3 * 4) + 2 = 50
	const FRAME_SIZE: unt = 4 * 3 * std::mem::size_of::<f32>() + std::mem::size_of::<u16>();

	if buf.len() < 84 + (num_triangles as unt) * FRAME_SIZE {
		let s: String = format!(
			"Invalid STL. {} triangles declared but only {} bytes in file",
			num_triangles,
			buf.len()
		);
		return Some(s);
	}

	if let Some(s) = check_sufficient_memory(num_triangles, vertices, normals, v_indices, e_indices) {
		return Some(s);
	};

	let mut vset = HashMap::<Vertex, u32>::new();
	vset.reserve((num_triangles as unt / 2) + 2);
	let mut eset = HashSet::<Edge>::new();
	eset.reserve((num_triangles as f32 * 1.5) as unt);
	let mut norm_count = Vec::<u32>::new();
	norm_count.resize((num_triangles as unt / 2) + 2, 0);

	let mut vpos = 0;
	let mut epos = 0;
	for i in 0..num_triangles as unt {
		let fpos = 84 + i * FRAME_SIZE;

		let normal = read_vertex(&buf[fpos..]).unwrap();

		let mut indexes: [u32; 3] = [0, 0, 0];

		for j in 0..3 {
			let v = read_vertex(&buf[fpos + (12 * (j + 1))..]).unwrap();
			let index: u32 = match vset.get(&v) {
				Some(idx) => {
					if (idx * 3) as unt + 2 >= normals.len() {
						return Some(format!(
							"normals bound exceeded: idx*3+2 {}, len: {}, i: {}",
							idx * 3 + 2,
							normals.len(),
							i
						));
					}
					normals[(idx * 3) as unt + 0] += normal.x;
					normals[(idx * 3) as unt + 1] += normal.y;
					normals[(idx * 3) as unt + 2] += normal.z;
					*idx
				}
				None => {
					let idx = (vpos / 3) as u32;
					if vpos + 2 >= vertices.len() {
						return Some(format!(
							"vertices bound exceeded: {}, len: {}, i: {}",
							vpos,
							vertices.len(),
							i
						));
					}
					vertices[vpos + 0] = v.x;
					vertices[vpos + 1] = v.y;
					vertices[vpos + 2] = v.z;
					if vpos + 2 >= normals.len() {
						return Some(format!(
							"normals bound exceeded: vpos {}, len: {}, i: {}",
							vpos,
							normals.len(),
							i
						));
					}
					normals[vpos + 0] = normal.x;
					normals[vpos + 1] = normal.y;
					normals[vpos + 2] = normal.z;
					vpos += 3;
					vset.insert(v, idx);
					idx
				}
			};
			if (i * 3) + j >= v_indices.len() {
				return Some(format!(
					"v_indices bound exceeded: {}, len: {}, i: {}",
					(i * 3) + j,
					v_indices.len(),
					i
				));
			}
			v_indices[(i * 3) + j] = index;
			if index as unt >= norm_count.len() {
				return Some(format!(
					"norm_count bound exceeded: {}, len: {}, i: {}",
					index,
					norm_count.len(),
					i
				));
			}
			norm_count[index as unt] += 1;
			indexes[j] = index;
			// return Some(format!("first vertex"));
		}

		// return Some(format!("first vertices"));

		let edge1 = Edge {
			a: indexes[0],
			b: indexes[1],
		};
		if !eset.contains(&edge1) {
			e_indices[epos] = indexes[0];
			e_indices[epos + 1] = indexes[1];
			epos += 2;
			eset.insert(edge1);
		}

		let edge2 = Edge {
			a: indexes[1],
			b: indexes[2],
		};
		if !eset.contains(&edge2) {
			e_indices[epos] = indexes[1];
			e_indices[epos + 1] = indexes[2];
			epos += 2;
			eset.insert(edge2);
		}

		let edge3 = Edge {
			a: indexes[2],
			b: indexes[0],
		};
		if !eset.contains(&edge3) {
			e_indices[epos] = indexes[2];
			e_indices[epos + 1] = indexes[0];
			epos += 2;
			eset.insert(edge3);
		}

		// return Some(format!("First iteration"));
	}

	// return Some(format!("loop finished"));

	for i in 0..vpos {
		normals[i] /= norm_count[i / 3] as f32;
	}

	None
}

fn check_sufficient_memory(
	num_triangles: u32,
	vertices: &[f32],
	normals: &[f32],
	v_indices: &[u32],
	e_indices: &[u32],
) -> Option<String> {
	let len_req = 3 * ((num_triangles as unt / 2) + 2);
	if vertices.len() < len_req {
		let s: String = format!(
			"Insufficient memory allocated for vertices. {} float64 elements allocated, but {} required for {} triangles",
			vertices.len(),
			len_req,
			num_triangles,
		);
		return Some(s);
	}

	if normals.len() < len_req {
		let s: String = format!(
			"Insufficient memory allocated for normals. {} float64 elements allocated, but {} required for {} triangles",
			normals.len(),
			len_req,
			num_triangles,
		);
		return Some(s);
	}

	let len_req = num_triangles as unt * 3;
	if v_indices.len() < len_req {
		let s: String = format!(
			"Insufficient memory allocated for vertex indices. {} float64 elements allocated, but {} required for {} triangles",
			v_indices.len(),
			len_req,
			num_triangles,
		);
		return Some(s);
	}

	if e_indices.len() < len_req {
		let s: String = format!(
			"Insufficient memory allocated for edge indices. {} float64 elements allocated, but {} required for {} triangles",
			e_indices.len(),
			len_req,
			num_triangles,
		);
		return Some(s);
	}

	None
}

fn read_vertex(buf: &[u8]) -> Result<Vertex, std::array::TryFromSliceError> {
	let f0 = f32_from_le_bytes(buf[0..4].try_into()?);
	let f1 = f32_from_le_bytes(buf[4..8].try_into()?);
	let f2 = f32_from_le_bytes(buf[8..12].try_into()?);
	Ok(Vertex { x: f0, y: f1, z: f2 })
}

pub fn f32_from_le_bytes(bytes: [u8; 4]) -> f32 {
	f32::from_bits(u32::from_le_bytes(bytes))
}

// #[wasm_bindgen]
// extern "C" {
// 	pub fn log(s: &str);
// }

#[wasm_bindgen(js_name = "invertMat4x4")]
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

#[wasm_bindgen(js_name = "invertedMat4x4")]
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

#[wasm_bindgen(js_name = "rotateMat4x4")]
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

	let sina = angle.sin();
	let cosa = angle.cos();
	let t = 1.0 - cosa;

	// Construct the elements of the rotation matrix
	let b00 = x * x * t + cosa;     let b01 = x * y * t - z * sina; let b02 = x * z * t + y * sina;
	let b10 = y * x * t + z * sina; let b11 = y * y * t + cosa;     let b12 = y * z * t - x * sina;
	let b20 = x * z * t - y * sina; let b21 = y * z * t + x * sina; let b22 = z * z * t + cosa;

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

	mat[8]  = b20 * a00 + b21 * a10 + b22 * a20;
	mat[9]  = b20 * a01 + b21 * a11 + b22 * a21;
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
		const PI: f64 = core::f64::consts::PI;
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
