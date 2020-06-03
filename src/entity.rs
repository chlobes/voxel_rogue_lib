use crate::prelude::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Entity {
	pub id: u64,
	pub typ: EntityType,
	pub pos: Vec3<f64>,
	pub hp: Option<(f64, f64)>,
	pub properties: HashMap<String, String>,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum EntityType {
	Player,
	Portal,
}

use std::fmt;
impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut r = format!("{}) {} {}",self.id,self.typ,self.pos.nice_fmt(6, false));
		self.hp.map(|(hp, max)| r.push_str(&format!(" {}/{}",hp.nice_fmt(6, false),max.nice_fmt(6, false))));
		for (name, prop) in self.properties.iter() {
			r.push_str(&format!("\n{}: {}",name,prop));
		}
		write!(f,"{}",r)
	}
}

impl fmt::Display for EntityType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use EntityType::*;
		write!(f,"{}",match self {
			Player => "player",
			Portal => "portal",
		})
	}
}
