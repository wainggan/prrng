
/// [Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
/// 
/// ```
/// # use prrng::WichHill;
/// let mut rng = WichHill::new([10, 20, 30]);
/// 
/// assert_eq!(rng.get(), 0.33818773630473775);
/// assert_eq!(rng.get(), 0.7754188755966642);
/// assert_eq!(rng.get(), 0.5273524613909046);
/// assert_eq!(rng.get(), 0.44624074405335046);
/// ```
#[derive(Clone)]
pub struct WichHill {
	seed: (u32, u32, u32),
}

impl WichHill {
	/// construct a new `WichHill` instance from raw seeds.
	/// 
	/// all three seeds should be between `1..=30000`. values outside of
	/// this may produce unexpected values.
	/// 
	/// ## examples
	/// 
	/// 
	/// beware setting any of the three seeds to `0`:
	/// 
	/// ```
	/// # use prrng::WichHill;
	/// let mut rng = WichHill::new_raw([0, 0, 0]);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0); // not random at all!
	/// ```
	#[inline]
	pub const fn new_raw(seed: [u32; 3]) -> Self {
		Self {
			seed: (seed[0], seed[1], seed[2]),
		}
	}

	#[inline]
	pub const fn new(mut seed: [u32; 3]) -> Self {
		seed[0] = crate::common::u32_or_1(seed[0]);
		seed[1] = crate::common::u32_or_1(seed[1]);
		seed[2] = crate::common::u32_or_1(seed[2]);
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn get(&mut self) -> f64 {
		self.seed.0 = (self.seed.0 * 171) % 30269;
		self.seed.1 = (self.seed.1 * 172) % 30307;
		self.seed.2 = (self.seed.2 * 170) % 30323;
		let x = self.seed.0 as f64 / 30269.0 + self.seed.1 as f64 / 30307.0 + self.seed.2 as f64 / 30323.0;
		x % 1.0
	}
}

impl crate::Random for WichHill {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		self.get()
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		self.get() as f32
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::f64_to_u32(self.get())
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		crate::common::f64_to_u16(self.get())
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		crate::common::f64_to_u8(self.get())
	}
}

impl core::fmt::Debug for WichHill {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "WichHill")
	}
}

