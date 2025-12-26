
/// [64bit xorshift](https://en.wikipedia.org/wiki/Xorshift) psuedo-rng.
/// 
/// ```
/// # use prrng::XorShift64;
/// let mut rng = XorShift64::new(0);
/// 
/// assert_eq!(rng.get(), 1082269761);
/// assert_eq!(rng.get(), 1152992998833853505);
/// assert_eq!(rng.get(), 11177516664432764457);
/// assert_eq!(rng.get(), 17678023832001937445);
/// ```
#[derive(Clone)]
pub struct XorShift64 {
	seed: u64,
}

impl XorShift64 {
	#[inline]
	pub const fn new_raw(seed: u64) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn new(seed: u64) -> Self {
		let seed = crate::common::u64_or_1(seed);
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		let mut x = self.seed;
		x ^= x << 13;
		x ^= x >> 7;
		x ^= x << 17;
		self.seed = x;
		x
	}
}

impl crate::Random for XorShift64 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl core::fmt::Debug for XorShift64 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "XorShift64")
	}
}

