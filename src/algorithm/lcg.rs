//! a [linear congruential generator](https://en.wikipedia.org/wiki/Linear_congruential_generator).
//! 
//! multiple of them, actually.
//! 
//! the output of a LCG follows this formula: `(seed * A + C) % M`.
//! performance-wise, this is excellent if multiplication and modular
//! division is fast. alternatively, if `M` is one less than a power of
//! 2, then the modulus can be optimized into a simple bitwise `&`.
//! given well selected parameters, an LCG can also generate relatively
//! high quality values.
//! 
//! security-wise, you should never use an LCG for unpredictable numbers.
//! 
//! this module packages up LCGs of different bit sizes, with associated
//! constants representing these different parameters.

/// 8 bit linear congruential generator. see [module level documenation](self).
pub struct Lcg8<const A: u8, const C: u8, const M: u8> {
	seed: u8,
}

impl<const A: u8, const C: u8, const M: u8,> Lcg8<A, C, M> {
	#[inline]
	pub const fn new(seed: u8) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u8 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u8, const C: u8, const M: u8> crate::RandomImpl for Lcg8<A, C, M> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u64_from_bytes(self)
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::u32_from_bytes(self)
	}

	#[inline]
	fn random_bytes(&mut self, dst: &mut [u8]) {
		for i in dst {
			*i = self.get();
		}
	}
}

impl<const A: u8, const C: u8, const M: u8> core::fmt::Debug for Lcg8<A, C, M> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Lcg8({}, {}, {}", A, C, M)
	}
}


/// 16 bit linear congruential generator. see [module level documenation](self).
pub struct Lcg16<const A: u16, const C: u16, const M: u16> {
	seed: u16,
}

impl<const A: u16, const C: u16, const M: u16> Lcg16<A, C, M> {
	#[inline]
	pub const fn new(seed: u16) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u16 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u16, const C: u16, const M: u16> crate::RandomImpl for Lcg16<A, C, M> {
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

impl<const A: u16, const C: u16, const M: u16> core::fmt::Debug for Lcg16<A, C, M> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Lcg16({}, {}, {}", A, C, M)
	}
}

/// 32 bit linear congruential generator. see [module level documenation](self).
pub struct Lcg32<const A: u32, const C: u32, const M: u32> {
	seed: u32,
}

impl<const A: u32, const C: u32, const M: u32> Lcg32<A, C, M> {
	#[inline]
	pub const fn new(seed: u32) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u32 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u32, const C: u32, const M: u32> crate::RandomImpl for Lcg32<A, C, M> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u32(self, dst);
	}
}

impl<const A: u32, const C: u32, const M: u32> core::fmt::Debug for Lcg32<A, C, M> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Lcg32({}, {}, {}", A, C, M)
	}
}

/// 64 bit linear congruential generator. see [module level documenation](self).
pub struct Lcg64<const A: u64, const C: u64, const M: u64> {
	seed: u64,
}

impl<const A: u64, const C: u64, const M: u64> Lcg64<A, C, M> {
	#[inline]
	pub const fn new(seed: u64) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u64 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u64, const C: u64, const M: u64> crate::RandomImpl for Lcg64<A, C, M> {
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

impl<const A: u64, const C: u64, const M: u64> core::fmt::Debug for Lcg64<A, C, M> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Lcg64({}, {}, {}", A, C, M)
	}
}

/// 64 bit linear congruential generator. see [module level documenation](self).
pub struct Lcg128<const A: u128, const C: u128, const M: u128> {
	seed: u128,
}

impl<const A: u128, const C: u128, const M: u128> Lcg128<A, C, M> {
	#[inline]
	pub const fn new(seed: u128) -> Self {
		Self {
			seed,
		}
	}

	#[inline]
	pub const fn get(&mut self) -> u128 {
		self.seed = self.seed.wrapping_mul(A).wrapping_add(C) % M;
		self.seed
	}
}

impl<const A: u128, const C: u128, const M: u128> crate::RandomImpl for Lcg128<A, C, M> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get() as u64
	}

	
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u64(self, dst);
	}
}

impl<const A: u128, const C: u128, const M: u128> core::fmt::Debug for Lcg128<A, C, M> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Lcg128({}, {}, {}", A, C, M)
	}
}

// https://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf
pub type Lecuyer8 = Lcg8<55, 0, 251>;
pub type Lecuyer16 = Lcg16<17364, 0, 65521>;

/// ```
/// # use prrng::lcg::MINSTD88;
/// let mut rng = MINSTD88::new(1);
/// assert_eq!(rng.get(), 16807);
/// assert_eq!(rng.get(), 282475249);
/// assert_eq!(rng.get(), 1622650073);
/// ```
pub type MINSTD88 = Lcg64<16807, 0, 2147483647>;

/// ```
/// # use prrng::lcg::MINSTD;
/// let mut rng = MINSTD::new(1);
/// assert_eq!(rng.get(), 48271);
/// assert_eq!(rng.get(), 182605794);
/// assert_eq!(rng.get(), 1291394886);
/// ```
pub type MINSTD = Lcg64<48271, 0, 2147483647>;

// https://www.jstor.org/stable/2008698
// https://oeis.org/A384546
pub type Fishman = Lcg32<950706376, 0, 2147483647>;

/// based on the RANDF [`LCG`](`Lcg32`) constants.
/// 
/// ```
/// # use prrng::lcg::RANF;
/// let mut rng = RANF::new(1);
/// // https://oeis.org/A384696
/// assert_eq!(rng.get(), 44485709377909);
/// assert_eq!(rng.get(), 232253848878969);
/// assert_eq!(rng.get(), 94800993741645);
/// assert_eq!(rng.get(), 243522309605169);
/// assert_eq!(rng.get(), 20783065360997);
/// ```
pub type RANF = Lcg64<44485709377909, 0, 0x1000000000000>;

/// based on the [RANDU](https://en.wikipedia.org/wiki/RANDU) [`LCG`](`Lcg32`) constants.
/// these constants are notoriously terrible; it is not recommended to use this generator.
/// 
/// ```
/// # use prrng::lcg::RANDU;
/// let mut rng = RANDU::new(1);
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
pub type RANDU = Lcg32<65539, 0, 0x80000000>;

pub type VisualBasic6 = Lcg32<0x43fd43fd, 0xc39ec3, 0xffffff>;



