
pub struct SplitMix64 {
	seed: u64,
}

impl SplitMix64 {
	#[inline]
	pub const fn new(seed: u64) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		let mut x = self.seed.wrapping_add(0x9e3779b97f4a7c15);
		self.seed = x;
		x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
		x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
		x ^ (x >> 31)
	}
}

impl crate::Random for SplitMix64 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl Iterator for SplitMix64 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

