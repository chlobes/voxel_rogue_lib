use crate::prelude::*;

#[repr(C)]
#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct Vertex {
	pub pos: Vec3<f32>,
	pub col: [f32; 4],
	pub normal: Vec3<f32>,
}

pub const VERTICES: [[f32; 3]; 36] = [
	[1.0, 1.0, 1.0], //north 1
	[1.0, 0.0, 0.0],
	[1.0, 1.0, 0.0],
	[1.0, 0.0, 0.0], //north 2
	[1.0, 1.0, 1.0],
	[1.0, 0.0, 1.0],
	[0.0, 0.0, 0.0], //south 1
	[0.0, 0.0, 1.0],
	[0.0, 1.0, 1.0],
	[0.0, 0.0, 0.0], //south 2
	[0.0, 1.0, 1.0],
	[0.0, 1.0, 0.0],
	[1.0, 1.0, 1.0], //west 1
	[1.0, 1.0, 0.0],
	[0.0, 1.0, 0.0],
	[1.0, 1.0, 1.0], //west 2
	[0.0, 1.0, 0.0],
	[0.0, 1.0, 1.0],
	[1.0, 0.0, 1.0], //east 1
	[0.0, 0.0, 0.0],
	[1.0, 0.0, 0.0],
	[1.0, 0.0, 1.0], //east 2
	[0.0, 0.0, 1.0],
	[0.0, 0.0, 0.0],
	[0.0, 1.0, 1.0], //up 1
	[0.0, 0.0, 1.0],
	[1.0, 0.0, 1.0],
	[1.0, 1.0, 1.0], //up 2
	[0.0, 1.0, 1.0],
	[1.0, 0.0, 1.0],
	[1.0, 1.0, 0.0], //down 1
	[0.0, 0.0, 0.0],
	[0.0, 1.0, 0.0],
	[1.0, 1.0, 0.0], //down 2
	[1.0, 0.0, 0.0],
	[0.0, 0.0, 0.0],
];
/*
pub const _UVS: [[f32; 2]; 36] = [
	[1.0, 1.0], //north 1
	[0.0, 0.0],
	[1.0, 0.0],
	[0.0, 0.0], //north 2
	[1.0, 1.0],
	[0.0, 1.0],
	[0.0, 0.0], //south 1
	[0.0, 1.0],
	[1.0, 1.0],
	[0.0, 0.0], //south 2
	[1.0, 1.0],
	[1.0, 0.0],
	[1.0, 1.0], //west 1
	[1.0, 0.0],
	[0.0, 0.0],
	[1.0, 1.0], //west 2
	[0.0, 0.0],
	[0.0, 1.0],
	[1.0, 1.0], //east 1
	[0.0, 0.0],
	[1.0, 0.0],
	[1.0, 1.0], //east 2
	[0.0, 1.0],
	[0.0, 0.0],
	[0.0, 1.0], //up 1
	[0.0, 0.0],
	[1.0, 0.0],
	[1.0, 1.0], //up 2
	[0.0, 1.0],
	[1.0, 0.0],
	[1.0, 1.0], //down 1
	[0.0, 0.0],
	[0.0, 1.0],
	[1.0, 1.0], //down 2
	[1.0, 0.0],
	[0.0, 0.0],
];*/

pub fn quad(direction: usize, pos: Vec3<f32>, size: Vec3<f32>, col: [f32; 4]) -> [Vertex; 6] {
	let normal = dir_to_normal(direction);
	let mut result = [Vertex {
		pos: Vec3::zero(),
		col,
		normal,
	}; 6];
	for i in 0..6 {
		let translate: Vec3<_> = VERTICES[direction * 6 + i].into();
		result[i].pos = pos + size * (translate - 0.5);
		//result[i].uv = [UVS[i+offset][0], UVS[i+offset][1], texture]
	}
	result
}

pub fn rotated_quad(direction: usize, pos: Vec3<f32>, size: Vec3<f32>, col: [f32; 4], rot: Mat3<f32>) -> [Vertex; 6] {
	let normal = dir_to_normal(direction);
	let mut result = [Vertex {
		pos: Vec3::zero(),
		col,
		normal,
	}; 6];
	for i in 0..6 {
		let translate: Vec3<_> = VERTICES[direction * 6 + i].into();
		result[i].pos = pos + rot * (size * (translate - 0.5));
		result[i].normal = rot * result[i].normal;
		//result[i].uv = [UVS[i+offset][0], UVS[i+offset][1], texture]
	}
	result
}

pub fn dir_to_normal(dir: usize) -> Vec3<f32> {
	match dir {
		0 => vec3( 1.,0.,0.), //North
		1 => vec3(-1.,0.,0.), //South
		2 => vec3(0., 1.,0.), //West
		3 => vec3(0.,-1.,0.), //East
		4 => vec3(0.,0., 1.), //Up
		5 => vec3(0.,0.,-1.), //Down
		x => panic!("{} is not a direction",x),
	}
}

pub fn adjacent(c: Vec3<i32>) -> [Vec3<i32>; 6] {
	let mut result = [
		vec3(1, 0, 0),
		vec3(-1, 0, 0),
		vec3(0, 1, 0),
		vec3(0, -1, 0),
		vec3(0, 0, 1),
		vec3(0, 0, -1),
	];
	for i in 0..6 {
		result[i] += c;
	}
	result
}

pub fn block_colors() -> HashMap<u32, [f32; 4]> {
	let mut r = HashMap::new();
	r.insert(1, [1.0, 0.0, 0.0, 1.0]);
	r.insert(2, [0.0, 1.0, 0.0, 1.0]);
	r
}

pub fn gen_box(r: &mut Vec<Vertex>, pos: Vec3<f32>, size: Vec3<f32>, color: [f32; 4]) {
	for i in 0..6 {
		r.extend_from_slice(&quad(i, pos, size, color));
	}
}
