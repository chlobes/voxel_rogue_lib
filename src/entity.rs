use crate::prelude::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Entity {
	pub id: u64,
	pub typ: EntityType,
	pub pos: Vec3<f64>,
	pub properties: HashMap<String, String>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum EntityType {
	Player,
	Portal,
}
