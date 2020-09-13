pub use serde::{Serialize,Deserialize};
pub use std::collections::{HashMap,HashSet};
pub use std::mem;
pub use arrayvec::ArrayString;
pub use array_tuple::ArrayTuple;
pub use math_lib::{vec2::*,vec3::*,vec4::*,mat2::*,mat3::*,mat4::*,traits::NiceFmt};
pub use serde_json::to_vec_pretty as serialize;
pub use serde_json::from_slice as deserialize;
pub use serde_json::to_writer_pretty as serialize_into;
pub use serde_json::from_reader as deserialize_from;

pub use crate::{AuthInfo,ServerPacket,ClientPacket};
pub use crate::entity::{EntityType,Model};
pub use crate::stats::Stats;
pub use crate::Event;
pub use crate::item::ItemSlot;
pub use crate::vertex::*;

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
pub const BASE_HP: f64 = 20.0;
pub const MAX_ATTACK_COOLDOWN: f64 = 2.0;
pub const ATTACK_OUT_OF_RANGE_PENALTY: f64 = 0.1;
pub const QUADRATIC_DRAG: f64 = 0.45;
pub const GRAVITY: f64 = 9.8;
pub const FRICTION: f64 = 35.0;
pub const BASE_FRICTION: f64 = 0.8; //friction applied is proportional to velocity + BASE_FRICTION
pub const KNOCKBACK_STRENGTH: f64 = 12.0;
pub const EQUIP_MASS_MUL: [f64; 5] = [10.0, 10.0, 6.0, 3.0, 6.0]; //equipped weapon/offhand is heavier, equipped chest is lighter, which means base chests can be heavier and weapons can be lighter so carrying many weapons is viable but carrying many chests is not, overall equipped stuff is heavier which allows carrying extra gear being not tooooo much of a downside

pub const PLAYER_HITBOX_SIZE: Vec3<f64> = Vec3{ x: 0.3, y: 0.3, z: 0.6, };
pub const PLAYER_HP_REGEN: f64 = 0.004; //as a fraction of max hp
pub const PLAYER_MOVESPEED: f64 = 7.5;
pub const PLAYER_JUMP_VEL: f64 = GRAVITY * 1.08;
pub const PLAYER_JUMP_VEL_HOR: f64 = 4.5;
pub const PLAYER_AIR_CONTROL: f64 = 0.3;
pub const PICKUP_TIME: f64 = 0.2;
pub const PICKUP_RANGE: f64 = 3.0;

pub const PORTAL_SIZE: Vec3<f64> = Vec3{ x: 0.45, y: 0.45, z: 0.9, };
pub const ITEM_SIZE: Vec3<f64> = Vec3{ x: 0.15, y: 0.15, z: 0.15 };
