

const STATE_N: usize = 624;
const STATE_M: usize = 397;

const MATRIX_A: u32 = 0x9908b0df;
const MASK_B: u32 = 0x9d2c5680;
const MASK_C: u32 = 0xefc60000;

const UPPER_MASK: u32 = 0x80000000;
const LOWER_MASK: u32 = 0x7fffffff;

// https://www.math.sci.hiroshima-u.ac.jp/m-mat/MT/MT2002/emt19937ar.html
// https://github.com/ESultanik/mtwister
#[derive(Clone)]
pub struct MTwister {
	buf: [u32; STATE_N],
	index: usize,
}

impl MTwister {
	pub fn new(seed: u32) -> Self {
		let mut buf = [0u32; STATE_N];

		buf[0] = seed;
		
		for i in 1..buf.len() {
			buf[i] = 1812433253u32
				.wrapping_mul(buf[i - 1] ^ (buf[i - 1] >> 30))
				.wrapping_add(i as u32);
		}

		Self {
			buf,
			index: STATE_N,
		}
	}
	
	pub fn run(&mut self) {
		let mut kk = 0;

		while kk < STATE_N - STATE_M {
			let x = (self.buf[kk] & UPPER_MASK) | (self.buf[kk + 1] & LOWER_MASK);
			self.buf[kk] = self.buf[kk + STATE_M] ^ (x >> 1) ^ ((x & 1) * MATRIX_A);
			kk += 1;
		}

		while kk < STATE_N - 1 {
			let x = (self.buf[kk] & UPPER_MASK) | (self.buf[kk + 1] & LOWER_MASK);
			self.buf[kk] = self.buf[kk + STATE_M - STATE_N] ^ (x >> 1) ^ ((x & 1) * MATRIX_A);
			kk += 1;
		}
		
		let x = (self.buf[STATE_N - 1] & UPPER_MASK) | (self.buf[0] & LOWER_MASK);
		self.buf[STATE_N - 1] = self.buf[STATE_M - 1] ^ (x >> 1) ^ ((x & 1) * MATRIX_A);

		self.index = 0;
	}

	fn temper(mut value: u32) -> u32 {
		value ^= value >> 11;
		value ^= (value << 7) & MASK_B;
		value ^= (value << 15) & MASK_C;
		value ^= value >> 18;
		value
	}

	pub fn get_checked(&mut self) -> Option<u32> {
		if self.index >= STATE_N {
			None
		} else {
			let ret = self.buf[self.index];
			self.index += 1;
			Some(Self::temper(ret))
		}
	}

	pub fn get(&mut self) -> u32 {
		if self.index >= STATE_N {
			self.run();
		}

		let ret = self.buf[self.index];
		self.index += 1;
		Self::temper(ret)
	}
}

impl crate::Random for MTwister {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.get(), self.get())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}
}

impl Iterator for MTwister {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl core::fmt::Debug for MTwister {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "MTwister")
	}
}

