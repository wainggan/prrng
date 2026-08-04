
// https://github.com/imneme/pcg-c-basic/blob/master/pcg_basic.c
#[derive(Clone)]
pub struct Pcg32 {
	state: u64,
	index: u64,
}

impl Pcg32 {
	#[inline]
	pub const fn new_raw(seed: u64, id: u64) -> Self {
		Self {
			state: seed,
			index: id,
		}
	}

	#[inline]
	pub const fn new(seed: u64, id: u64) -> Self {
		let mut ret = Self::new_raw(0, (id << 1) | 1);
		ret.get();
		ret.state = ret.state.wrapping_add(seed);
		ret.get();
		ret
	}

	#[inline]
	pub const fn get(&mut self) -> u32 {
		let prev: u64 = self.state;

		self.state = prev
			.wrapping_mul(6364136223846793005)
			.wrapping_add(self.index);
		let x: u32 = (((prev >> 18) ^ prev) >> 27) as u32;
		let rot: u32 = (prev >> 59) as u32;

		(x >> rot) | (x << (rot.wrapping_neg() & 31))
	}
}

impl crate::RandomImpl for Pcg32 {
	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u32(|| self.get(), dst);
	}
}

impl core::fmt::Debug for Pcg32 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Pcg32")
	}
}

