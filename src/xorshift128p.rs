
/// [128bit non-linear xorshift](https://en.wikipedia.org/wiki/Xorshift#xorshift+) psuedo-rng. yields u64 values.
/// 
/// ```
/// # use prrng::XorShift128p;
/// let mut rng = XorShift128p::new(0);
/// 
/// assert_eq!(rng.get(), 2350952794504575203);
/// assert_eq!(rng.get(), 10647469811762407304);
/// assert_eq!(rng.get(), 2296785643744461824);
/// assert_eq!(rng.get(), 6600060142384134327);
/// ```
#[derive(Debug, Clone)]
pub struct XorShift128p {
	seed0: u64,
	seed1: u64,
}

impl XorShift128p {
	#[inline]
	pub const fn new_raw(seed0: u64, seed1: u64) -> Self {
		Self {
			seed0,
			seed1,
		}
	}

	#[inline]
	pub const fn new(seed: u64) -> Self {
		let mut rng = crate::XorShift64::new(seed);
		Self::new_raw(rng.get(), rng.get())
	}

	#[inline]
	pub const fn seed0(&mut self) -> &mut u64 {
		&mut self.seed0
	}

	#[inline]
	pub const fn seed1(&mut self) -> &mut u64 {
		&mut self.seed1
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		let mut t: u64 = self.seed0;
		let s: u64 = self.seed1;
		self.seed0 = s;
		t ^= t << 23;
		t ^= t >> 18;
		t ^= s ^ (s >> 5);
		self.seed1 = t;
		t.wrapping_add(s)
	}
}

impl crate::Random for XorShift128p {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		crate::common::u64_to_f64(self.get())
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}
}

impl Iterator for XorShift128p {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

