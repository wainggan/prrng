
/// utility type for accumulating [`crate::Random`] calls.
/// 
/// many prng algorithms are not ["cryptographically secure"](https://en.wikipedia.org/wiki/Cryptographically_secure_pseudorandom_number_generator).
/// that is, given outputs of the algorithm, future outputs can be mathematically predicted.
/// 
/// `Crush` doesn't help with this. this type wraps a [`crate::Random`] type along with a
/// [`core::hash::Hasher`] type. all random getters are set to insert a random value
/// into the hasher `N` times, afterwards returning the hasher's `finish()` value.
/// this may [improve the percieved randomness of an algorithm](https://en.wikipedia.org/wiki/Randomness_extractor).
/// 
/// that being said, this is not a replacement for proper security. do not use this in
/// lieu of a properly, provably cryptographically secure rng like [`crate::ChaCha`].
pub struct Crush<const N: usize, R: crate::Random, H: core::hash::Hasher> {
	inner: R,
	hash: H,
}

impl<const N: usize, R: crate::Random, H: core::hash::Hasher> Crush<N, R, H> {
	#[inline]
	pub const fn new(inner: R, hasher: H) -> Self {
		Self {
			inner,
			hash: hasher,
		}
	}

	#[inline]
	pub fn unwrap(self) -> (R, H) {
		(self.inner, self.hash)
	}

	pub fn get(&mut self) -> u64 {
		for _ in 0..N {
			self.hash.write_u64(self.inner.random_u64());
		}
		self.hash.finish()
	}
}

impl<const N: usize, R: crate::Random, H: core::hash::Hasher> crate::Random for Crush<N, R, H> {
	fn random_u64(&mut self) -> u64 {
		self.get()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl<const N: usize, R: crate::Random, H: core::hash::Hasher> Iterator for Crush<N, R, H> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

