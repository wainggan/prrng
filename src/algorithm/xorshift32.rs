
/// [32bit xorshift](https://en.wikipedia.org/wiki/Xorshift) psuedo-rng.
/// 
/// this algorithm is *extremely* fast, and emits a generally 'good enough'
/// uniform distrution, though it is not of particularly high quality.
/// additionally, knowing one value allows one to very trivially predict
/// all future and previous values.
/// 
/// this implementation has a period of `2^32-1`.
/// 
/// ```
/// # use prrng::XorShift32;
/// let mut rng = XorShift32::new(1);
/// 
/// assert_eq!(rng.get(), 270369);
/// assert_eq!(rng.get(), 67634689);
/// assert_eq!(rng.get(), 2647435461);
/// assert_eq!(rng.get(), 307599695);
/// ```
#[derive(Clone)]
pub struct XorShift32 {
	seed: u32,
}

impl XorShift32 {
	/// construct a new [`XorShift32`].
	/// 
	/// `seed` should be any number except `0`, as a `0` seed will cause this
	/// rng to only emit `0`s. see [`Self::new()`] for a constructor that
	/// accounts for this.
	/// 
	/// ## examples
	/// 
	/// ```
	/// # use prrng::XorShift32;
	/// let rng = XorShift32::new_raw(1);
	/// 
	/// // beware of accidentally setting the seed to `0`:
	/// let mut rng = XorShift32::new_raw(0);
	/// 
	/// assert_eq!(rng.get(), 0);
	/// assert_eq!(rng.get(), 0);
	/// assert_eq!(rng.get(), 0);
	/// assert_eq!(rng.get(), 0);
	/// ```
	#[inline]
	pub const fn new_raw(seed: u32) -> Self {
		Self {
			seed,
		}
	}

	/// construct a new [`XorShift32`].
	/// 
	/// this rng's seed should not be `0`. if `seed` is `0`, then this
	/// method will set the seed to `1`. see [`Self::new_raw()`] for a
	/// constructor that does not do this.
	/// 
	/// ## examples
	/// 
	/// ```
	/// # use prrng::XorShift32;
	/// let rng = XorShift32::new(1);
	/// 
	/// // beware of accidentally setting the seed to `0`:
	/// let mut rng_0 = XorShift32::new(0);
	/// let mut rng_1 = XorShift32::new(1);
	/// 
	/// assert_eq!(rng_0.get(), rng_1.get()); // these streams are identical
	/// assert_eq!(rng_0.get(), rng_1.get());
	/// assert_eq!(rng_0.get(), rng_1.get());
	/// ```
	#[inline]
	pub const fn new(seed: u32) -> Self {
		let seed = crate::common::u32_or_1(seed);
		Self::new_raw(seed)
	}

	#[inline]
	pub const fn get(&mut self) -> u32 {
		let mut x = self.seed;
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		self.seed = x;
		x
	}
}

impl crate::RandomImpl for XorShift32 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.get(), self.get())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		crate::common::bytes_from_u32(self, dst);
	}
}

impl core::fmt::Debug for XorShift32 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "XorShift32")
	}
}

