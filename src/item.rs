use crate::prelude::*;

#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Item {
	pub id: u32,
	pub name: String,
	pub mass: f64,
	pub agi: f64,
	pub atk: f64,
	pub str: f64,
	pub vit: f64,
	pub con: f64,
	pub slot: Option<ItemSlot>,
	pub properties: HashMap<String, String>,
}

impl Item {
	pub fn equipped_mass(&self) -> f64 {
		if let Some(slot) = self.slot {
			self.mass * EQUIP_MASS_MUL[slot.num()]
		} else {
			self.mass
		}
	}
}

use std::fmt;
impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = if self.id != 0 { format!("{}({})",self.name,self.id) } else { format!("name: {}",self.name) };
		let mut r = format!("{}\nmass: {}\n",name,self.mass.nice_fmt(5, false));
		if self.agi != 0.0 || self.atk != 0.0 || self.str != 0.0 || self.vit != 0.0 {
			r.push_str(&format!("\nagi: {}({})",self.agi.nice_fmt(5, false),(self.agi-self.equipped_mass()).nice_fmt(5, false)));
			r.push_str(&format!("\natk: {}\nstr: {}\nvit: {}\ncon: {}",self.atk.nice_fmt(5, false),self.str.nice_fmt(5, false),self.con.nice_fmt(5, false),self.vit.nice_fmt(5, false)));
		}
		if let Some(slot) = self.slot {
			r.push_str(&format!("\nslot: {}",slot.display(true)));
		}
		let mut names = self.properties.keys().collect::<Vec<_>>();
		names.sort();
		for name in names.iter() {
			r.push_str(&format!("\n{}: {}",name,self.properties.get(*name).unwrap()));
		}
		write!(f,"{}",r)
	}
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Serialize,Deserialize)]
pub enum ItemSlot {
	Hand1,
	Hand2,
	Head,
	Chest,
	Feet,
}

use ItemSlot::*;

impl ItemSlot {
	pub fn display(&self, target: bool) -> &str {
		match self {
			Hand1 => if target { "one handed" } else { "left hand" },
			Hand2 => if target { "two handed" } else { "right hand" },
			Head => "head",
			Chest => "chest",
			Feet => "feet",
		}
	}
	
	pub fn num(&self) -> usize {
		match self {
			Hand1 => 0,
			Hand2 => 1,
			Head => 2,
			Chest => 3,
			Feet => 4,
		}
	}
	
	pub fn try_from_num(i: usize) -> Option<Self> {
		match i {
			0 => Some(Hand1),
			1 => Some(Hand2),
			2 => Some(Head),
			3 => Some(Chest),
			4 => Some(Feet),
			_ => None,
		}
	}
	
	pub fn from_num(i: usize) -> Self {
		ItemSlot::try_from_num(i).expect(&format!("tried to get item slot from num: {}",i))
	}
	
	pub fn base_mass(&self, tier: f64) -> f64 {
		tier * match self {
			Hand1 => 5.0 / EQUIP_MASS_MUL[0],
			Hand2 => 10.0 / EQUIP_MASS_MUL[0],
			Head => 2.0 / EQUIP_MASS_MUL[2],
			Chest => 6.0 / EQUIP_MASS_MUL[3],
			Feet => 2.0 / EQUIP_MASS_MUL[4],
		}
	}
	
	pub fn value(&self, tier: f64) -> f64 {
		5f64.powf(tier - 1.0) * match self {
			Hand1 => 4.0,
			Hand2 => 8.0,
			Head => 1.0,
			Chest => 2.0,
			Feet => 1.0,
		}
	}
}
