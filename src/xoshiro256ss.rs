
#[derive(Clone)]
pub struct XorShift256ss {
	seed: (u64, u64, u64, u64),
}

impl XorShift256ss {
	#[inline]
	pub const fn new_raw(seed: [u64; 4]) -> Self {
		Self {
			seed: (seed[0], seed[1], seed[2], seed[3]),
		}
	}

	#[inline]
	pub const fn new(mut seed: [u64; 4]) -> Self {
		seed[0] = crate::common::u64_or_1(seed[0]);
		seed[1] = crate::common::u64_or_1(seed[1]);
		seed[2] = crate::common::u64_or_1(seed[2]);
		seed[3] = crate::common::u64_or_1(seed[3]);
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		let result = self.seed.1
			.wrapping_mul(5)
			.rotate_left(7)
			.wrapping_mul(9);
		let t = self.seed.1 << 17;

		self.seed.2 ^= self.seed.0;
		self.seed.3 ^= self.seed.1;
		self.seed.1 ^= self.seed.2;
		self.seed.0 ^= self.seed.3;

		self.seed.2 ^= t;
		self.seed.3 = self.seed.3.rotate_left(45);

		result
	}
}

impl crate::Random for XorShift256ss {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl core::fmt::Debug for XorShift256ss {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "XorShift256ss")
	}
}

