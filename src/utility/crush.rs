
/// utility type for accumulating [`crate::Random`] calls.
/// 
/// many prng algorithms are not ["cryptographically secure"](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator).
/// that is, given outputs of the algorithm, future or past outputs can be mathematically predicted.
/// 
/// `Crush` doesn't help with this. this type wraps a [`crate::Random`] type along with a
/// [`core::hash::Hasher`] type. all random getters are set to insert a random value
/// into the hasher `N` times, afterwards returning the hasher's `finish()` value.
/// this may [improve the percieved randomness of an algorithm](https://en.wikipedia.org/wiki/Randomness_extractor).
/// 
/// that being said, this is not a replacement for proper security. do not use this in
/// lieu of a properly, provably cryptographically secure rng like, say, [`crate::ChaCha`].
#[derive(Clone)]
pub struct Crush<const N: usize, R, H>
where R: crate::Random, H: core::hash::Hasher {
	inner: R,
	hash: H,
}

impl<const N: usize, R, H> Crush<N, R, H>
where R: crate::Random, H: core::hash::Hasher {
	/// construct a new `Crush`.
	/// 
	/// ## examples
	/// 
	/// ```
	/// # use prrng::MTwister;
	/// # use prrng::Crush;
	/// # extern crate std;
	/// use prrng::Random;
	/// 
	/// let rng = MTwister::new(0);
	/// let hasher = std::hash::DefaultHasher::new();
	/// 
	/// let crush = rng.random_into_crush::<4>(hasher);
	/// ```
	#[inline]
	pub const fn new(inner: R, hasher: H) -> Self {
		Self {
			inner,
			hash: hasher,
		}
	}

	/// consume `self` and return the inner rng and hasher.
	#[inline]
	pub fn unwrap(self) -> (R, H) {
		(self.inner, self.hash)
	}

	/// write into the hasher `N` times and return the value.
	pub fn get(&mut self) -> u64 {
		for _ in 0..N {
			self.hash.write_u64(self.inner.random_u64());
		}
		self.hash.finish()
	}
}

impl<const N: usize, R, H> crate::RandomImpl for Crush<N, R, H>
where R: crate::Random, H: core::hash::Hasher {
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

impl<const N: usize, R, H> core::fmt::Debug for Crush<N, R, H>
where
	R: crate::Random + core::fmt::Debug,
	H: core::hash::Hasher + core::fmt::Debug,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Crush{}({:?}, {:?})", N, self.inner, self.hash)
	}
}

