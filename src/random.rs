
/// generic random number generation.
/// 
/// this type is dyn-compatible, and implemented for all generators in this
/// crate, allowing for very easy composition of algorithms. this trait
/// also provides multiple common utilities for working with the generators.
/// 
/// when implementing `Random`, the only methods required to be implemented
/// are [`Random::random_u32()`] and [`Random::random_u64`]. all other
/// methods are derived from these, though some implementations may override
/// [`Random::random_f64()`], [`Random::random_f32()`],
/// [`Random::random_u128()`], [`Random::random_u16()`],
/// [`Random::random_u8()`], and [`Random::random_bool()`].
/// overriding any other method is discouraged.
pub trait Random: Iterator<Item = f64> {
	/// returns a new random value.
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

	/// returns a new f64.
	#[inline]
	fn random_f64(&mut self) -> f64 {
		crate::common::u64_normalize_f64(self.random_u64())
	}

	/// returns a new f32.
	#[inline]
	fn random_f32(&mut self) -> f32 {
		crate::common::u32_normalize_f32(self.random_u32())
	}

	/// returns a new u128.
	#[inline]
	fn random_u128(&mut self) -> u128 {
		crate::common::u64_compose_u128(self.random_u64(), self.random_u64())
	}

	/// returns a new u64.
	fn random_u64(&mut self) -> u64;

	/// returns a new u32.
	fn random_u32(&mut self) -> u32;

	/// returns a new u16.
	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.random_u32() as u16
	}

	/// returns a new u8.
	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.random_u32() as u8
	}

	/// returns a new bool.
	#[inline]
	fn random_bool(&mut self) -> bool {
		self.random_u8() & 1 == 1
	}

	/// fill a byte buffer with new values.
	fn random_bytes(&mut self, dst: &mut [u8]) {
		let (chunks, extra) = dst.as_chunks_mut::<{ core::mem::size_of::<u128>() }>();

		for chunk in chunks {
			*chunk = self.random_u128().to_le_bytes();
		}

		if extra.is_empty() {
			return;
		}

		let last = self.random_u128().to_le_bytes();

		for (o, i) in extra.iter_mut().zip(last.iter()) {
			*o = *i;
		}
	}

	/// fill a buffer with random values.
	fn random_fill<T: FromRandom>(&mut self, dst: &mut [T]) where Self: Sized {
		for i in dst {
			*i = self.random();
		}
	}

	/// fill an uninitiaized buffer with random values.
	/// by the end of this method, `dst` will be fully initialized.
	fn random_fill_uninit<T: FromRandom>(&mut self, dst: &mut [core::mem::MaybeUninit<T>]) where Self: Sized {
		for i in dst {
			*i = core::mem::MaybeUninit::new(self.random());
		}
	}

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

	#[inline]
	fn random_u64_bound(&mut self, bound: u64) -> u64 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u64();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	#[inline]
	fn random_u32_bound(&mut self, bound: u32) -> u32 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u32();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	#[inline]
	fn random_u16_bound(&mut self, bound: u16) -> u16 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u16();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	#[inline]
	fn random_u8_bound(&mut self, bound: u8) -> u8 {
		let threshold = bound.wrapping_neg() % bound;
		loop {
			let x = self.random_u8();
			if x >= threshold {
				return x % bound;
			}
		}
	}

	/// derived from a call to [`Random::random_f64()`].
	#[inline]
	fn random_range(&mut self, range: core::ops::Range<f64>) -> f64 {
		range.start + self.random_f64() * (range.end - range.start)
	}

	#[inline]
	fn random_into_iter(self) -> crate::Iter<Self> where Self: Sized {
		crate::Iter::new(self)
	}

	#[inline]
	fn random_iter(&mut self) -> crate::Iter<&mut Self> where Self: Sized {
		crate::Iter::new(self)
	}

	#[inline]
	fn random_into_buffer64<const N: usize>(self)
		-> crate::Buffer64<N, Self> where Self: Sized
	{
		crate::Buffer64::new(self)
	}

	#[inline]
	fn random_buffer64<const N: usize>(&mut self)
		-> crate::Buffer64<N, &mut Self> where Self: Sized
	{
		crate::Buffer64::new(self)
	}

	#[inline]
	fn random_into_buffer32<const N: usize>(self)
		-> crate::Buffer32<N, Self> where Self: Sized
	{
		crate::Buffer32::new(self)
	}

	#[inline]
	fn random_buffer32<const N: usize>(&mut self)
		-> crate::Buffer32<N, &mut Self> where Self: Sized
	{
		crate::Buffer32::new(self)
	}

	#[inline]
	fn random_into_buffer8<const N: usize>(self)
		-> crate::Buffer8<N, Self> where Self: Sized
	{
		crate::Buffer8::new(self)
	}

	#[inline]
	fn random_buffer8<const N: usize>(&mut self)
		-> crate::Buffer8<N, &mut Self> where Self: Sized
	{
		crate::Buffer8::new(self)
	}

	#[inline]
	fn random_into_crush<const N: usize>(self, hasher: impl core::hash::Hasher)
		-> crate::Crush<N, Self, impl core::hash::Hasher> where Self: Sized
	{
		crate::Crush::new(self, hasher)
	}

	#[inline]
	fn random_crush<const N: usize>(&mut self, hasher: impl core::hash::Hasher)
		-> crate::Crush<N, &mut Self, impl core::hash::Hasher> where Self: Sized
	{
		crate::Crush::new(self, hasher)
	}
}

impl<T: Random> Random for &mut T {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		(*self).random_f64()
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		(*self).random_f32()
	}

	#[inline]
	fn random_u128(&mut self) -> u128 {
		(*self).random_u128()
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		(*self).random_u64()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		(*self).random_u32()
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		(*self).random_u16()
	}
	
	#[inline]
	fn random_u8(&mut self) -> u8 {
		(*self).random_u8()
	}
}

impl Random for &mut dyn Random {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		(*self).random_f64()
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		(*self).random_f32()
	}

	#[inline]
	fn random_u128(&mut self) -> u128 {
		(*self).random_u128()
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		(*self).random_u64()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		(*self).random_u32()
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		(*self).random_u16()
	}
	
	#[inline]
	fn random_u8(&mut self) -> u8 {
		(*self).random_u8()
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
		random.random_f64()
	}
}

impl FromRandom for f32 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_f32()
	}
}

impl FromRandom for u128 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u128()
	}
}

impl FromRandom for i128 {
	fn from_random(random: &mut impl Random) -> Self {
		random.random_u128().cast_signed()
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

		assert_eq!(rng.random_range(0.0..2.0), 1.0);

		let _x: (i16, u64) = rng.random();
	}

	#[test]
	fn test_dyn() {
		let _object: &mut dyn crate::Random = &mut crate::Static::new(|| 0.0);
		let _object: &mut dyn crate::Random = &mut crate::Static::new(|| 0.0).random_iter();
	}

	#[test]
	fn test_iter() {
		let mut rng = crate::Static::new(|| 0.0);

		for i in rng.random_iter().take(4) {
			assert_eq!(i, 0.0);
		}
	}
}

