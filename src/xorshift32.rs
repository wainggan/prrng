
/// [32bit xorshift](https://en.wikipedia.org/wiki/Xorshift) psuedo-rng.
/// 
/// ```
/// # use prrng::XorShift32;
/// let mut rng = XorShift32::new(0);
/// 
/// assert_eq!(rng.get(), 270369);
/// assert_eq!(rng.get(), 67634689);
/// assert_eq!(rng.get(), 2647435461);
/// assert_eq!(rng.get(), 307599695);
/// ```
#[derive(Debug, Clone)]
pub struct XorShift32 {
	seed: u32
}

impl XorShift32 {
	#[inline]
	pub const fn new_raw(seed: u32) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn new(seed: u32) -> Self {
		let seed = if seed == 0 {
			1
		} else {
			seed
		};
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn seed(&mut self) -> &mut u32 {
		&mut self.seed
	}

	#[inline]
	pub const fn get(&mut self) -> u32 {
		let mut x = self.seed;
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		self.seed = x;
		x
	}
}

impl crate::Random for XorShift32 {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		crate::common::u32_to_f64(self.get())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}
}

impl Iterator for XorShift32 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

