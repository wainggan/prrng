
/// [128bit non-linear xorshift+](https://en.wikipedia.org/wiki/Xorshift#xorshift+) psuedo-rng. yields u64 values.
/// 
/// ```
/// # use prrng::XorShift128p;
/// let mut rng = XorShift128p::new([10, 20]);
/// 
/// assert_eq!(rng.get(), 83886450);
/// assert_eq!(rng.get(), 338167070);
/// assert_eq!(rng.get(), 703687785278400);
/// assert_eq!(rng.get(), 2111062671688522);
/// ```
#[derive(Clone)]
pub struct XorShift128p {
	seed: (u64, u64),
}

impl XorShift128p {
	#[inline]
	pub const fn new_raw(seed: [u64; 2]) -> Self {
		Self {
			seed: (seed[0], seed[1])
		}
	}

	#[inline]
	pub const fn new(mut seed: [u64; 2]) -> Self {
		seed[0] = crate::common::u64_or_1(seed[0]);
		seed[1] = crate::common::u64_or_1(seed[1]);
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		let mut t: u64 = self.seed.0;
		let s: u64 = self.seed.1;
		self.seed.0 = s;
		t ^= t << 23;
		t ^= t >> 18;
		t ^= s ^ (s >> 5);
		self.seed.1 = t;
		t.wrapping_add(s)
	}
}

impl crate::RandomImpl for XorShift128p {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u64(self, dst);
	}
}

impl core::fmt::Debug for XorShift128p {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "XorShift128p")
	}
}

