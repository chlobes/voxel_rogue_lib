pub use serde::{Serialize,Deserialize};
pub use std::collections::{HashMap,HashSet};
pub use std::mem;
pub use arrayvec::ArrayString;
pub use array_tuple::ArrayTuple;
pub use math_lib::{vec3::*,vec2::*,traits::NiceFmt};
pub use serde_json::to_vec_pretty as serialize;
pub use serde_json::from_slice as deserialize;
pub use serde_json::to_writer_pretty as serialize_into;
pub use serde_json::from_reader as deserialize_from;

pub use crate::{AuthInfo,ServerPacket,ClientPacket,Action};
pub use crate::entity::{Entity,EntityType};
pub use crate::stats::Stats;

pub macro l() {
	&concat!(file!(), " ", line!())
}

pub fn serialize_structymap<S: serde::Serializer, K: Serialize, V: Serialize> (map: &HashMap<K, V>, s: S) -> std::result::Result<S::Ok, S::Error> {
	map.iter().map(|(k,v)| (k.clone(),v.clone())).collect::<Vec<(_,_)>>().serialize(s)
}

pub fn deserialize_structymap<'de, D: serde::Deserializer<'de>, K: Deserialize<'de> + Eq + std::hash::Hash, V: Deserialize<'de>>(d: D)
	-> std::result::Result<HashMap<K, V>, D::Error> {
	let vec = <Vec<(K, V)>>::deserialize(d)?;
	let mut map = HashMap::with_capacity(vec.len());
	for (k, v) in vec {
		map.insert(k, v);
	}
	Ok(map)
}

pub const TPS: usize = 100;
pub const PLAYER_HITBOX_SIZE: Vec3<f64> = Vec3{ x: 0.3, y: 0.3, z: 0.6, };
pub const PLAYER_BASE_MASS: f64 = 100.0;
pub const PLAYER_BASE_HP: f64 = 50.0;
pub const PLAYER_HP_REGEN: f64 = 0.0005; //as a fraction of max hp
//pub const PLAYER_MOVESPEED: f64 = 4.0 * 1.01f64.powf(PLAYER_BASE_MASS);
pub const PLAYER_MOVESPEED: f64 = 4.0 * 2.70481382942; //no const pow function for no reason thx rust
pub const PLAYER_JUMP_VEL: f64 = 7.0;
pub const PLAYER_JUMP_VEL_HOR: f64 = 4.0;
pub const PLAYER_AIR_CONTROL: f64 = 0.1;
pub const PLAYER_MAX_ATTACK_COOLDOWN: f64 = 2.0;
pub const PLAYER_ATTACK_OUT_OF_RANGE_PENALTY: f64 = 0.1;
pub const CONST_DRAG: f64 = 0.02;
pub const LINEAR_DRAG: f64 = 0.1;
pub const AIR_DRAG_MOD: f64 = 0.1;
pub const GRAVITY: f64 = 3.0;
