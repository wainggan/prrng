
/// [16bit fibonacci linear-feedback shift register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Fibonacci_LFSRs) psuedo-rng.
#[derive(Debug, Clone)]
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

impl crate::Random for FibLFSR16 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}
	
	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::u16_compose_u32(self.get(), self.get())
	}
	
	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.get()
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

impl Iterator for FibLFSR16 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

