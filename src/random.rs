
/// generic random number generation.
/// 
/// this type is dyn-compatible, and implemented for all generators in this
/// crate, allowing for very easy composition of algorithms. this trait
/// also provides multiple common utilities for working with the generators.
/// 
/// the only methods required to be implemented
pub trait Random: Iterator<Item = f64> {
	/// returns a new f64.
	#[inline]
	fn random_f64(&mut self) -> f64 {
		crate::common::u64_normalize_f64(self.random_u64())
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		crate::common::u32_normalize_f32(self.random_u32())
	}

	#[inline]
	fn random_u128(&mut self) -> u128 {
		crate::common::u64_compose_u128(self.random_u64(), self.random_u64())
	}

	fn random_u64(&mut self) -> u64;

	fn random_u32(&mut self) -> u32;

	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.random_u32() as u16
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.random_u32() as u8
	}

	#[inline]
	fn random_bool(&mut self) -> bool {
		self.random_u8() & 1 == 1
	}

	fn random_fill(&mut self, dst: &mut [u8]) {
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
	fn random_into_iter(self) -> Iter<Self> where Self: Sized {
		Iter::new(self)
	}

	#[inline]
	fn random_iter(&mut self) -> Iter<&mut Self> where Self: Sized {
		Iter::new(self)
	}

	#[inline]
	fn ranodm_into_buffer64<const N: usize>(self)
		-> crate::Buffer64<N, Self> where Self: Sized
	{
		crate::Buffer64::new(self)
	}

	#[inline]
	fn ranodm_buffer64<const N: usize>(&mut self)
		-> crate::Buffer64<N, &mut Self> where Self: Sized
	{
		crate::Buffer64::new(self)
	}

	#[inline]
	fn ranodm_into_buffer32<const N: usize>(self)
		-> crate::Buffer32<N, Self> where Self: Sized
	{
		crate::Buffer32::new(self)
	}

	#[inline]
	fn ranodm_buffer32<const N: usize>(&mut self)
		-> crate::Buffer32<N, &mut Self> where Self: Sized
	{
		crate::Buffer32::new(self)
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

pub struct Iter<R: Random> {
	inner: R,
}

impl<R: Random> Iter<R> {
	#[inline]
	pub fn new(inner: R) -> Self {
		Self {
			inner,
		}
	}

	#[inline]
	pub fn unwrap(self) -> R {
		self.inner
	}
}

impl<R: Random> Iterator for Iter<R> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

impl<R: Random> Random for Iter<R> {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		self.inner.random_f64()
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		self.inner.random_f32()
	}

	#[inline]
	fn random_u128(&mut self) -> u128 {
		self.inner.random_u128()
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.inner.random_u64()
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.inner.random_u32()
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		self.inner.random_u16()
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.inner.random_u8()
	}
}

#[cfg(test)]
mod test {
    use crate::Random;

	#[test]
	fn test_main() {
		let mut rng = crate::Static::new(|| 0.5);

		assert_eq!(rng.random_range(0.0..2.0), 1.0);
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

