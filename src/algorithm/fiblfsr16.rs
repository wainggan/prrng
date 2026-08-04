
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
	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u16(|| self.get(), dst);
	}
}

impl core::fmt::Debug for FibLFSR16 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "FibLFSR16")
	}
}

