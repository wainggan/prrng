
// remember to skip first 48/96 states
// https://arxiv.org/abs/2312.17043
pub struct CollatzWeyl64 {
	x: u64,
	a: u64,
	weyl: u64,
	s: u64,
}

impl CollatzWeyl64 {
	#[inline]
	pub fn new_raw(state: u64, seed: u64) -> Self {
		Self {
			a: 0,
			weyl: 0,
			x: state,
			s: seed,
		}
	}

	#[inline]
	pub fn new_one(seed: u64) -> Self {
		// seed should always be odd
		Self::new_raw(0, seed | 1)
	}

	#[inline]
	pub fn new_two(state: u64, seed: u64) -> Self {
		// seed should always be odd
		Self::new_raw(state, seed | 1)
	}

	#[inline]
	pub fn get(&mut self) -> u64 {
		self.a = self.a.wrapping_add(self.x);
		self.weyl = self.weyl.wrapping_add(self.s);
		self.x = (self.x >> 1).wrapping_mul(self.a | 1) ^ self.weyl;
		// is this the correct precedence?
		(self.a >> 48) ^ self.x
	}
}

impl crate::Random for CollatzWeyl64 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl Iterator for CollatzWeyl64 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

pub struct CollatzWeyl128_64 {
	x: u128,
	a: u64,
	weyl: u64,
	s: u64,
}

impl CollatzWeyl128_64 {
	#[inline]
	pub fn new_raw(state: u128, seed: u64) -> Self {
		Self {
			a: 0,
			weyl: 0,
			x: state,
			s: seed,
		}
	}

	#[inline]
	pub fn new_one(seed: u64) -> Self {
		// seed should always be odd
		Self::new_raw(0, seed | 1)
	}

	#[inline]
	pub fn new_two(state: u128, seed: u64) -> Self {
		Self::new_raw(
			state,
			// seed should always be odd
			seed | 1
		)
	}

	#[inline]
	pub fn get(&mut self) -> u128 {
		self.a = (self.a as u128).wrapping_add(self.x) as u64;
		self.weyl = self.weyl.wrapping_add(self.s);
		self.x = (self.x | 1).wrapping_mul((self.a >> 1) as u128) ^ self.weyl as u128;
		(self.a >> 48) as u128 ^ self.x
	}
}

impl crate::Random for CollatzWeyl128_64 {
	#[inline]
	fn random_u128(&mut self) -> u128 {
		self.get()
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get() as u64
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.get() as u16
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

impl Iterator for CollatzWeyl128_64 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

pub struct CollatzWeyl128 {
	x: u128,
	a: u128,
	weyl: u128,
	s: u128,
}

impl CollatzWeyl128 {
	#[inline]
	pub fn new_raw(state: u128, seed: u128) -> Self {
		Self {
			a: 0,
			weyl: 0,
			x: state,
			s: seed,
		}
	}

	#[inline]
	pub fn new_one(seed: u128) -> Self {
		// seed should always be odd
		Self::new_raw(0, seed | 1)
	}

	#[inline]
	pub fn new_two(state: u128, seed: u128) -> Self {
		Self::new_raw(
			state,
			// seed should always be odd
			seed | 1
		)
	}

	#[inline]
	pub fn get(&mut self) -> u128 {
		self.a = self.a.wrapping_add(self.x);
		self.weyl = self.weyl.wrapping_add(self.s);
		self.x = (self.x >> 1).wrapping_mul(self.a | 1) ^ self.weyl;
		(self.a >> 96) ^ self.x
	}
}

impl crate::Random for CollatzWeyl128 {
	#[inline]
	fn random_u128(&mut self) -> u128 {
		self.get()
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get() as u64
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.get() as u16
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

impl Iterator for CollatzWeyl128 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

