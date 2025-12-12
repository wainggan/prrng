
/// [Wichmann-Hill](https://en.wikipedia.org/wiki/Wichmann%E2%80%93Hill) psuedo-rng.
/// 
/// ```
/// # use prrng::WichHill;
/// let mut rng = WichHill::new(0);
/// 
/// assert_eq!(rng.get(), 0.1905942791341093);
/// assert_eq!(rng.get(), 0.21332214064505495);
/// assert_eq!(rng.get(), 0.8948422044484658);
/// assert_eq!(rng.get(), 0.028670929064924966);
/// ```
#[derive(Debug, Clone)]
pub struct WichHill {
	seed0: u32,
	seed1: u32,
	seed2: u32,
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
	/// let mut rng = WichHill::new_raw(0, 0, 0);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0);
	/// assert_eq!(rng.get(), 0.0); // not random at all!
	/// ```
	#[inline]
	pub const fn new_raw(seed0: u32, seed1: u32, seed2: u32) -> Self {
		Self {
			seed0,
			seed1,
			seed2,
		}
	}

	#[inline]
	pub const fn new(seed: u32) -> Self {
		let mut rng = crate::XorShift32::new(seed);
		
		let seed0 = rng.get() % 30000;
		let seed0 = if seed0 == 0 {
			1
		} else {
			seed0
		};

		let seed1 = rng.get() % 30000;
		let seed1 = if seed1 == 0 {
			1
		} else {
			seed1
		};

		let seed2 = rng.get() % 30000;
		let seed2 = if seed2 == 0 {
			1
		} else {
			seed2
		};

		Self::new_raw(seed0, seed1, seed2)
	}

	#[inline]
	pub const fn seed0(&mut self) -> &mut u32 {
		&mut self.seed0
	}

	#[inline]
	pub const fn seed1(&mut self) -> &mut u32 {
		&mut self.seed1
	}

	#[inline]
	pub const fn seed2(&mut self) -> &mut u32 {
		&mut self.seed2
	}

	#[inline]
	pub const fn get(&mut self) -> f64 {
		self.seed0 = (self.seed0 * 171) % 30269;
		self.seed1 = (self.seed1 * 172) % 30307;
		self.seed2 = (self.seed2 * 170) % 30323;
		let x = self.seed0 as f64 / 30269.0 + self.seed1 as f64 / 30307.0 + self.seed2 as f64 / 30323.0;
		x % 1.0
	}
}

impl crate::Random for WichHill {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		self.get()
	}
}

impl Iterator for WichHill {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(self.get())
	}
}

