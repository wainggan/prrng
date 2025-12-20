
pub struct Lcg8<const A: u8, const C: u8, const M: u8> {
	seed: u8,
}

impl<const A: u8, const C: u8, const M: u8,> Lcg8<A, C, M> {
	pub const fn new(seed: u8) -> Self {
		Self {
			seed,
		}
	}

	pub const fn seed(&mut self) -> &mut u8 {
		&mut self.seed
	}

	pub const fn get(&mut self) -> u8 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u8, const C: u8, const M: u8> Iterator for Lcg8<A, C, M> {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl<const A: u8, const C: u8, const M: u8> crate::Random for Lcg8<A, C, M> {
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	fn random_u32(&mut self) -> u32 {
		crate::common::u16_compose_u32(self.random_u16(), self.random_u16())
	}

	fn random_u16(&mut self) -> u16 {
		crate::common::u8_compose_u16(self.get(), self.get())
	}

	fn random_u8(&mut self) -> u8 {
		self.get()
	}
}

pub struct Lcg16<const A: u16, const C: u16, const M: u16> {
	seed: u16,
}

impl<const A: u16, const C: u16, const M: u16> Lcg16<A, C, M> {
	pub const fn new(seed: u16) -> Self {
		Self {
			seed,
		}
	}

	pub const fn seed(&mut self) -> &mut u16 {
		&mut self.seed
	}

	pub const fn get(&mut self) -> u16 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u16, const C: u16, const M: u16> Iterator for Lcg16<A, C, M> {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl<const A: u16, const C: u16, const M: u16> crate::Random for Lcg16<A, C, M> {
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	fn random_u32(&mut self) -> u32 {
		crate::common::u16_compose_u32(self.random_u16(), self.random_u16())
	}

	fn random_u16(&mut self) -> u16 {
		self.get()
	}
	
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

pub struct Lcg32<const A: u32, const C: u32, const M: u32> {
	seed: u32,
}

impl<const A: u32, const C: u32, const M: u32> Lcg32<A, C, M> {
	pub const fn new(seed: u32) -> Self {
		Self {
			seed,
		}
	}

	pub const fn seed(&mut self) -> &mut u32 {
		&mut self.seed
	}

	pub const fn get(&mut self) -> u32 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u32, const C: u32, const M: u32> Iterator for Lcg32<A, C, M> {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl<const A: u32, const C: u32, const M: u32> crate::Random for Lcg32<A, C, M> {
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	fn random_u32(&mut self) -> u32 {
		self.get()
	}

	fn random_u16(&mut self) -> u16 {
		self.get() as u16
	}
	
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

pub struct Lcg64<const A: u64, const C: u64, const M: u64> {
	seed: u64,
}

impl<const A: u64, const C: u64, const M: u64> Lcg64<A, C, M> {
	pub const fn new(seed: u64) -> Self {
		Self {
			seed,
		}
	}

	pub const fn seed(&mut self) -> &mut u64 {
		&mut self.seed
	}

	pub const fn get(&mut self) -> u64 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u64, const C: u64, const M: u64> Iterator for Lcg64<A, C, M> {
	type Item = f64;

	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl<const A: u64, const C: u64, const M: u64> crate::Random for Lcg64<A, C, M> {
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}

	fn random_u16(&mut self) -> u16 {
		self.get() as u16
	}
	
	fn random_u8(&mut self) -> u8 {
		self.get() as u8
	}
}

// https://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf
pub const LECUYER8: Lcg8<55, 0, 251> = Lcg8::new(1);
pub const LECUYER16: Lcg16<17364, 0, 65521> = Lcg16::new(1);

// old version
pub const MINSTD88: Lcg32<16807, 0, 2147483647> = Lcg32::new(1);

pub const MINSTD: Lcg32<48271, 0, 2147483647> = Lcg32::new(1);

// https://www.jstor.org/stable/2008698
// https://oeis.org/A384546
pub const FISHMAN: Lcg32<950706376, 0, 2147483647> = Lcg32::new(1);

/// based on the RANDF [`LCG`](`Lcg32`) constants.
/// 
/// this const defaults to a seed of `0x1`.
/// 
/// ```
/// # use prrng::lcg::RANF;
/// let mut rng = RANF;
/// // https://oeis.org/A384696
/// assert_eq!(rng.get(), 44485709377909);
/// assert_eq!(rng.get(), 232253848878969);
/// assert_eq!(rng.get(), 94800993741645);
/// assert_eq!(rng.get(), 243522309605169);
/// assert_eq!(rng.get(), 20783065360997);
/// ```
pub const RANF: Lcg64<44485709377909, 0, 0x1000000000000> = Lcg64::new(1);

/// based on the [RANDU](https://en.wikipedia.org/wiki/RANDU) [`LCG`](`Lcg32`) constants.
/// these constants are notoriously terrible; it is not recommended to use this generator.
/// 
/// this const defaults to a seed of `0x1`.
/// 
/// ```
/// # use prrng::lcg::RANDU;
/// let mut rng = RANDU;
/// // https://oeis.org/A096555
/// assert_eq!(rng.get(), 65539);
/// assert_eq!(rng.get(), 393225);
/// assert_eq!(rng.get(), 1769499);
/// assert_eq!(rng.get(), 7077969);
/// assert_eq!(rng.get(), 26542323);
/// assert_eq!(rng.get(), 95552217);
/// assert_eq!(rng.get(), 334432395);
/// assert_eq!(rng.get(), 1146624417);
/// assert_eq!(rng.get(), 1722371299);
/// assert_eq!(rng.get(), 14608041);
/// ```
pub const RANDU: Lcg32<65539, 0, 0x80000000> = Lcg32::new(1);

pub const VISUAL_BASIC_6: Lcg32<0x43fd43fd, 0xc39ec3, 0xffffff> = Lcg32::new(0x50000);



