
/// random number generation.
/// 
/// see [`Random`] for more information.
pub trait RandomImpl {
	/// returns a new `u64`.
	/// 
	/// consider using [`crate::common::u64_from_bytes()`] or
	/// [`crate::common::u32_compose_u64()`] when implementing this.
	fn random_u64(&mut self) -> u64;
	
	/// returns a new `u32`.
	/// 
	/// consider using [`crate::common::u32_from_bytes()`]
	/// when implementing this.
	fn random_u32(&mut self) -> u32;
	
	/// fills a buffer with new values.
	/// 
	/// consider using [`crate::common::bytes_from_u32()`] or
	/// [`crate::common::bytes_from_u64()`] when implementing this.
	fn random_bytes(&mut self, dst: &mut [u8]);
}

mod private {
	pub struct Seal;
}

/// generic random number generation.
/// 
/// this type is dyn-compatible, and implemented for all generators in this
/// crate, allowing for very easy composition of algorithms. this trait
/// also provides multiple common utilities for working with the generators.
/// 
/// implementing `Random` is done via [`RandomImpl`], where you'd implement
/// [`RandomImpl::random_u64()`], [`RandomImpl::random_u32()`], and
/// [`RandomImpl::random_bytes()`]. `Random` is free, as `Random` is blanket
/// implemented for `RandomImpl`.
/// see the [`crate::common`] module for helper methods.
pub trait Random: RandomImpl {
	/// reserving the right to implement `Random`. just in case.
	#[doc(hidden)]
	fn __random_sealed(_: private::Seal) where Self: Sized;

	/// returns a new random value `T`.
	/// 
	/// implement [`FromRandom`] to have this method work on your own types.  
	/// 
	/// ## examples
	/// 
	/// ```
	/// use prrng::Random;
	/// use prrng::XorShift64;
	/// 
	/// let mut rng = XorShift64::new(1);
	/// 
	/// let a: u64 = rng.random();
	/// let a = rng.random::<i8>();
	/// let (a, b): (u32, u16) = rng.random();
	/// let a = rng.random::<[u8; 64]>();
	/// ```
	/// 
	/// this method can be particularly convenient when initializing
	/// an rng with another rng:
	/// 
	/// ```
	/// use prrng::Random;
	/// use prrng::SplitMix64;
	/// use prrng::XorShift256ss;
	/// 
	/// let rng = XorShift256ss::new({
	///     let mut temp = SplitMix64::new(24935945);
	///     temp.random()
	/// });
	/// ``` 
	#[inline]
	fn random<T: FromRandom>(&mut self) -> T where Self: Sized {
		T::from_random(self)
	}

	/// returns a new `f64`.
	fn random_f64(&mut self) -> f64 {
		crate::common::u64_normalize_f64(self.random_u64())
	}

	/// returns a new `f32`.
	fn random_f32(&mut self) -> f32 {
		crate::common::u32_normalize_f32(self.random_u32())
	}

	/// returns a new `u128`.
	fn random_u128(&mut self) -> u128 {
		crate::common::u64_compose_u128(self.random_u64(), self.random_u64())
	}

	/// returns a new `u16`.
	fn random_u16(&mut self) -> u16 {
		self.random_u32() as u16
	}

	/// returns a new `u8`.
	fn random_u8(&mut self) -> u8 {
		self.random_u32() as u8
	}

	/// returns a new `bool`.
	fn random_bool(&mut self) -> bool {
		self.random_u32() & 1 == 1
	}

	/// fill a buffer with random values `T`.
	#[inline]
	fn random_fill<T: FromRandom>(&mut self, dst: &mut [T]) where Self: Sized {
		for i in dst {
			*i = self.random();
		}
	}

	/// fill an uninitiaized buffer with random values `T`.
	/// by the end of this method, `dst` will be fully initialized.
	fn random_fill_uninit<T: FromRandom>(&mut self, dst: &mut [core::mem::MaybeUninit<T>]) where Self: Sized {
		for i in dst {
			*i = core::mem::MaybeUninit::new(self.random());
		}
	}

	/// returns a new `u128`, uniformly distributed within `0 .. bound`.
	#[inline]
	fn random_u128_bound(&mut self, bound: u128) -> u128 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u128();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// returns a new `u128`, uniformly distributed within `0 .. bound`.
	fn random_u64_bound(&mut self, bound: u64) -> u64 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u64();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// returns a new `u128`, uniformly distributed within `0 .. bound`.
	fn random_u32_bound(&mut self, bound: u32) -> u32 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u32();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// returns a new `u128`, uniformly distributed within `0 .. bound`.
	fn random_u16_bound(&mut self, bound: u16) -> u16 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u16();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// returns a new `u128`, uniformly distributed within `0 .. bound`.
	fn random_u8_bound(&mut self, bound: u8) -> u8 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u8();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// returns a new `f64`, uniformly distributed within `range`.
	#[inline]
	fn random_range(&mut self, range: core::ops::Range<f64>) -> f64 {
		range.start + self.random_f64() * (range.end - range.start)
	}

	/// consume `self`, wrapping it in an iterator [`crate::Iter`]. its [`Iterator::next()`] returns `T`.
	#[inline]
	fn random_into_iter<T: crate::FromRandom>(self) -> crate::Iter<T, Self> where Self: Sized {
		crate::Iter::new(self)
	}

	/// wrap `&mut self` in an iterator [`crate::Iter`]. its [`Iterator::next()`] returns `T`.
	#[inline]
	fn random_iter<T: crate::FromRandom>(&mut self) -> crate::Iter<T, &mut Self> where Self: Sized {
		crate::Iter::new(self)
	}

	/// consume `self`, wrapping it in a [`crate::buffer::Buffer`] with size `N`.
	#[inline]
	fn random_into_buffer<T: FromRandom, const N: usize>(self)
		-> crate::Buffer<T, N, Self> where Self: Sized
	{
		crate::Buffer::new(self)
	}

	/// wrap `&mut self` in a [`crate::buffer::Buffer`] with size `N`.
	#[inline]
	fn random_buffer<T: FromRandom, const N: usize>(&mut self)
		-> crate::Buffer<T, N, &mut Self> where Self: Sized
	{
		crate::Buffer::new(self)
	}

	/// consume `self`, wrapping it in a [`crate::buffer::Buffer8`] with size `N`.
	#[inline]
	fn random_into_buffer8<const N: usize>(self)
		-> crate::Buffer8<N, Self> where Self: Sized
	{
		crate::Buffer8::new(self)
	}

	/// wrap `&mut self` in a [`crate::buffer::Buffer8`] with size `N`.
	#[inline]
	fn random_buffer8<const N: usize>(&mut self)
		-> crate::Buffer8<N, &mut Self> where Self: Sized
	{
		crate::Buffer8::new(self)
	}

	/// consume `self`, wrapping it in a [`crate::Crush`], where `N` is how many
	/// hashes are run per value.
	#[inline]
	fn random_into_crush<const N: usize>(self, hasher: impl core::hash::Hasher)
		-> crate::Crush<N, Self, impl core::hash::Hasher> where Self: Sized
	{
		crate::Crush::new(self, hasher)
	}

	/// wrap `&mut self` in a [`crate::Crush`], where `N` is how many
	/// hashes are run per value.
	#[inline]
	fn random_crush<const N: usize>(&mut self, hasher: impl core::hash::Hasher)
		-> crate::Crush<N, &mut Self, impl core::hash::Hasher> where Self: Sized
	{
		crate::Crush::new(self, hasher)
	}
}

impl<T: RandomImpl> Random for T {
	fn __random_sealed(_: private::Seal) where Self: Sized {}
}

impl<T: RandomImpl> RandomImpl for &mut T {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		(*self).random_u64()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		(*self).random_u32()
	}

	#[inline]
	fn random_bytes(&mut self, dst: &mut [u8]) {
		(*self).random_bytes(dst);
	}
}

impl RandomImpl for &mut dyn RandomImpl {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		(*self).random_u64()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		(*self).random_u32()
	}

	fn random_bytes(&mut self, dst: &mut [u8]) {
		(*self).random_bytes(dst);
	}
}

/// randomized constructor.
/// 
/// this trait defines a constructor [`FromRandom::from_random()`], that
/// takes a [`Random`] and returns a fully initialized `Self`.
/// this is used with `Random`'s generic [`Random::random()`] method.
/// 
/// `FromRandom` is already implemented for most of Rust's primitives, arrays,
/// and tuples up to a length of 8.
pub trait FromRandom {
	fn from_random(random: &mut impl Random) -> Self;
}

impl FromRandom for f64 {
	fn from_random(random: &mut impl Random) -> Self {
		crate::common::u64_normalize_f64(random.random_u64())
	}
}

impl FromRandom for f32 {
	fn from_random(random: &mut impl Random) -> Self {
		crate::common::u32_normalize_f32(random.random_u32())
	}
}

impl FromRandom for u128 {
	fn from_random(random: &mut impl Random) -> Self {
		crate::common::u64_compose_u128(random.random_u64(), random.random_u64())
	}
}

impl FromRandom for i128 {
	fn from_random(random: &mut impl Random) -> Self {
		crate::common::u64_compose_u128(random.random_u64(), random.random_u64()).cast_signed()
	}
}

impl FromRandom for u64 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u64()
	}
}

impl FromRandom for i64 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u64().cast_signed()
	}
}

impl FromRandom for u32 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u32()
	}
}

impl FromRandom for i32 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u32().cast_signed()
	}
}

impl FromRandom for u16 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u16()
	}
}

impl FromRandom for i16 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u16().cast_signed()
	}
}

impl FromRandom for u8 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u8()
	}
}

impl FromRandom for i8 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u8().cast_signed()
	}
}

impl FromRandom for bool {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_bool()
	}
}

impl<const N: usize, T: FromRandom> FromRandom for [T; N] {
	fn from_random(random: &mut impl Random) -> Self {
		core::array::from_fn(|_| random.random())
	}
}

impl FromRandom for () {
	fn from_random(_: &mut impl Random) -> Self {}
}

impl<A: FromRandom> FromRandom for (A,) {
	fn from_random(random: &mut impl Random) -> Self {
		(random.random(),)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	> FromRandom for (A, B) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	> FromRandom for (A, B, C) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	D: FromRandom,
	> FromRandom for (A, B, C, D) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	D: FromRandom,
	E: FromRandom,
	> FromRandom for (A, B, C, D, E) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	D: FromRandom,
	E: FromRandom,
	F: FromRandom,
	> FromRandom for (A, B, C, D, E, F) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	D: FromRandom,
	E: FromRandom,
	F: FromRandom,
	G: FromRandom,
	> FromRandom for (A, B, C, D, E, F, G) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
		)
	}
}

impl<
	A: FromRandom,
	B: FromRandom,
	C: FromRandom,
	D: FromRandom,
	E: FromRandom,
	F: FromRandom,
	G: FromRandom,
	H: FromRandom,
	> FromRandom for (A, B, C, D, E, F, G, H) {
	fn from_random(random: &mut impl Random) -> Self {
		(
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
			random.random(),
		)
	}
}


#[cfg(test)]
mod test {
    use crate::Random;

	#[test]
	fn test_main() {
		let mut rng = crate::Static::new(|| 0.5);

		let _x: (i16, u64) = rng.random();
	}

	#[test]
	fn test_dyn() {
		let object: &mut dyn crate::Random = &mut crate::Static::new(|| 0.0);

		object.random_f64();
		object.random_f32();
		object.random_u64();
		object.random_u32();

		let _object: &mut dyn crate::Random = &mut crate::Static::new(|| 0.0).random_iter::<()>();
	}

	#[test]
	fn test_iter() {
		let mut rng = crate::Static::new(|| 0.0);

		for i in rng.random_iter::<f64>().take(4) {
			assert_eq!(i, 0.0);
		}
	}
}

