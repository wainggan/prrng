
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
		let seed = if seed == 0 {
			1
		} else {
			seed
		};

		Self::new_raw(seed)
	}

	#[inline]
	pub const fn bit(&mut self) -> &mut u16 {
		&mut self.bit
	}

	#[inline]
	pub const fn seed(&mut self) -> &mut u16 {
		&mut self.lfsr
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
	fn random_f64(&mut self) -> f64 {
		crate::common::u16_to_f64(self.get())
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.get()
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

