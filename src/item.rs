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

use std::fmt;
impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = if self.id != 0 { format!("{}({})",self.name,self.id) } else { format!("{}",self.name) };
		let mut r = format!("{} {} {}",name,self.volume.nice_fmt(6, false),self.density.nice_fmt(6, false));
		for (name, prop) in self.properties.iter() {
			r.push_str(&format!("\n{}: {}",name,prop));
		}
		write!(f,"{}",r)
	}
}
