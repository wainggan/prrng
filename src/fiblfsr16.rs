
/// [16bit fibonacci linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Fibonacci_LFSRs) psuedo-rng.
#[derive(Clone)]
pub struct FibLFSR16 {
	bit: u16,
	lfsr: u16,
}

impl FibLFSR16 {
	#[inline]
	pub const fn new_raw(seed: u16) -> Self {
		Self {
			bit: 0,
			lfsr: seed,
		}
	}

	#[inline]
	pub const fn new(seed: u16) -> Self {
		let seed = crate::common::u16_or_1(seed);
		Self::new_raw(seed)
	}
	
	#[inline]
	pub const fn get(&mut self) -> u16 {
		self.bit = ((self.lfsr) ^ (self.lfsr >> 2) ^ (self.lfsr >> 3) ^ (self.lfsr >> 5)) & 1;
		self.lfsr = (self.lfsr >> 1) | (self.bit << 15);
		self.lfsr
	}
}

impl crate::RandomImpl for FibLFSR16 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}
	
	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::u16_compose_u32(self.get(), self.get())
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u32(self, dst);
	}
}

impl core::fmt::Debug for FibLFSR16 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "FibLFSR16")
	}
}

