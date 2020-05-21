use crate::prelude::*;

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Item {
	pub id: u32,
	pub volume: f64,
	pub density: f64,
	pub name: String,
	pub properties: HashMap<String, String>,
}

impl Item {
	pub fn mass(&self) -> f64 {
		self.volume * self.density
	}
}
