#![feature(decl_macro)]

pub mod prelude;
use prelude::*;
pub mod item;
pub mod entity;
pub mod stats;
pub use item::Item;

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct AuthInfo {
	pub name: ArrayString<[u8; 32]>,
	pub data: [u64; 4],
}

pub type ServerPacket = Vec<SubPacket>;

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct SubPacket {
	pub stats: Stats<Item>,
	pub entities: HashMap<u64, Entity>,
	pub events: Vec<Event>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Event {
	Text(String),
	#[serde(serialize_with = "serialize_structymap")]
	#[serde(deserialize_with = "deserialize_structymap")]
	Blocks(HashMap<Vec3<i32>, u32>),
	ClearBlocks,
}

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ClientPacket {
	pub ready: bool,
	pub actions: Vec<(Action, f64)>, //time should be specified from 0 to 1, actions should be sorted from earliest to latest, actions will never be taken before the specified time, but may happen after, or not at all, interrupted actions will not be repeated, putting a time of 0 ensures an action is run as soon as the previous one finishes
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Action {
	Move(Vec2<f64>, f64, bool), //(direction, max_acc, jumping?)
	Attack(bool, u64), //(left or right hand, target)
}

impl From<HashMap<Vec3<i32>, u32>> for Event {
	fn from(x: HashMap<Vec3<i32>, u32>) -> Self {
		Event::Blocks(x)
	}
}

impl From<String> for Event {
	fn from(t: String) -> Self {
		Event::Text(t)
	}
}

use std::fmt;
impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use Action::*;
		match self {
			Move(dir, max, jump) => write!(f,"move {}[{}]{}",dir.nice_fmt(5, false),max.nice_fmt(4, false),if *jump { " jumping" } else { "" }),
			Attack(hand, target) => write!(f,"attack {} with {} hand",target,if *hand { "right" } else { "left" }),
		}
	}
}
