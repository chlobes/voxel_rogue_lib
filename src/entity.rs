use crate::prelude::*;
use crate::item::Item;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Entity {
	pub id: u64,
	pub typ: EntityType,
	pub pos: Vec3<f64>,
	pub hp: Option<(f64, f64)>,
	pub properties: HashMap<String, String>,
}

impl Entity {
	pub fn detailed_info(&self) -> String {
		let mut r = format!("type: {}",self.typ);
		self.hp.map(|(hp, max)| r.push_str(&format!("\nhp: {}/{}({})",hp.nice_fmt(6, false),max.nice_fmt(6, false),(hp / max).nice_fmt(6, false))));
		r.push_str(&format!("\nposition: {}",self.pos.nice_fmt(6, false)));
		r.push_str(&format!("\nid: {}",self.id));
		for (name, prop) in self.properties.iter() {
			r.push_str(&format!("\n{}: {}",name,prop));
		}
		if let EntityType::Item(i) = &self.typ {
			for i in i.iter() {
				r.push_str(&format!("\n{}",i));
			}
		}
		r
	}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum EntityType {
	Player,
	Portal,
	Mob(String, Model),
	Item(Vec<Item>),
}

use std::fmt;
impl fmt::Display for Entity {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut r = format!("{} {}",self.typ,self.pos.nice_fmt(6, false));
		self.hp.map(|(hp, max)| if hp != max { r.push_str(&format!(" {}",(hp / max).nice_fmt(6, false))) });
		self.properties.get(&"channel".to_string()).map(|c| r.push_str(&format!(" {}",c)));
		if let EntityType::Item(i) = &self.typ {
			for i in i.iter() {
				r.push_str(&format!("\n{}",i));
			}
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
			Mob(name, _model) => name.as_str(),
			Item(_items) => "item",
		})
	}
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Model {
	pub size: Vec3<f64>,
	pub verts: Vec<Vertex>,
}
