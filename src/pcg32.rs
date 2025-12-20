
// https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.c
#[derive(Clone)]
pub struct Pcg32 {
	state: u64,
	index: u64,
}

impl Pcg32 {
	#[inline]
	pub fn new_raw(seed: u64, id: u64) -> Self {
		Self {
			state: seed,
			index: id,
		}
	}

	#[inline]
	pub fn new(seed: u64, id: u64) -> Self {
		let mut ret = Self::new_raw(0, (id << 1) | 1);
		ret.get();
		ret.state = ret.state.wrapping_add(seed);
		ret.get();
		ret
	}

	#[inline]
	pub fn get(&mut self) -> u32 {
		let prev: u64 = self.state;

		self.state = prev
			.wrapping_mul(6364136223846793005)
			.wrapping_add(self.index);
		let x: u32 = (((prev >> 18) ^ prev) >> 27) as u32;
		let rot: u32 = (prev >> 59) as u32;

		(x >> rot) | (x << (rot.wrapping_neg() & 31))
	}
}

impl crate::Random for Pcg32 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.get(), self.get())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}
}

impl Iterator for Pcg32 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl core::fmt::Debug for Pcg32 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Pcg32")
	}
}

