use crate::prelude::*;

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Stats<I> {
	pub pos: Vec3<f64>,
	pub vel: Vec3<f64>,
	pub mass: f64,
	pub hp: f64,
	pub vit: f64,
	pub atk: f64,
	pub str: f64,
	pub agi: f64,
	pub attack_cooldown: f64, //when attacking, this is increased by (1 + attack_cooldown) / attack_speed , cannot attack if attack_cooldown > PLAYER_MAX_ATTACK_COOLDOWN
	pub equips: [Option<I>; 5],
	pub inventory: Vec<I>,
}

impl<I> Stats<I> {
	pub fn convert<T: From<I>>(mut self) -> Stats<T> { //because the From impl is illegal 
		let mut r = Stats {
			pos: self.pos,
			vel: self.vel,
			mass: self.mass,
			hp: self.hp,
			vit: self.vit,
			atk: self.atk,
			str: self.str,
			agi: self.agi,
			attack_cooldown: self.attack_cooldown,
			equips: Default::default(),
			inventory: Vec::new(),
		};
		for i in 0..self.equips.len() {
			r.equips[i] = self.equips[i].take().map(|i| i.into());
		}
		r.inventory = self.inventory.into_iter().rev().map(|i| i.into()).collect();
		r
	}
	
	pub fn max_hp(&self) -> f64 {
		50.0 * 1.01f64.powf(self.vit / 2.0)
	}
	
	pub fn agi_mul(&self) -> f64 {
		1.01f64.powf(self.agi - self.mass)
	}
	
	pub fn acceleration(&self, air: bool) -> f64 {
		self.agi_mul() * PLAYER_MOVESPEED * if air { PLAYER_AIR_CONTROL } else { 1.0 }
	}
	
	pub fn jump_vel(&self) -> f64 {
		self.agi_mul().sqrt() * PLAYER_JUMP_VEL
	}
	
	pub fn jump_vel_hor(&self) -> f64 {
		self.agi_mul() * PLAYER_JUMP_VEL_HOR
	}
	
	pub fn jump_height(&self) -> f64 { //approximate
		let u = self.jump_vel();
		let a = -GRAVITY;
		let t = u/a;
		t * (u + a * t / 2.0)
	}
}

/*default impl<A, B: From<A>> From<Stats<A>> for Stats<B> { //how to specialization?
	fn from(s: Stats<A>) -> Stats<B> {
		let mut r = Self {
			pos: s.pos,
			vel: s.vel,
			mass: s.mass,
			hp: s.hp,
			vit: s.vit,
			atk: s.atk,
			str: s.str,
			agi: s.agi,
			attack_cooldown: s.attack_cooldown,
			equips: unsafe { std::mem::uninitialized() },
			inventory: Vec::new(),
		};
		for i in 0..s.equips.len() {
			unsafe { std::ptr::write(&mut r.equips[i] as *mut _, s.equips[i].take().into()); }
		}
		r.inventory = s.inventory.into_iter().rev().map(|i| i.into()).collect();
		r
	}
}*/

impl<I> Default for Stats<I> { //because deriving it creates the unnecessary bound of I: Default
	fn default() -> Self {
		let mut r = Self {
			pos: Default::default(),
			vel: Default::default(),
			mass: Default::default(),
			hp: Default::default(),
			vit: Default::default(),
			atk: Default::default(),
			str: Default::default(),
			agi: Default::default(),
			attack_cooldown: Default::default(),
			equips: Default::default(),
			inventory: Vec::new(),
		};
		r.hp = r.max_hp();
		r
	}
}
