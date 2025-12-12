
/// general random number generation.
/// 
/// this type is implemented for all generators in this crate, and provides multiple
/// common utilities for working with the generators.
/// 
/// 
pub trait Random: Iterator<Item = f64> {
	/// returns a new f64.
	/// 
	/// this is the only required method in this trait, all other methods are derived from this.
	/// that being said, particularly for the random_[number]() methods, your type may
	/// be able to implement it better than a conversion from f64. for example, [`crate::XorShift32`]
	/// returns u32 values, and is thus able to wholly replace the [`Random::random_u32()`]
	/// default implementation with that.
	fn random_f64(&mut self) -> f64;

	/// by default, is derived from one call to [`Random::random_f32()`].
	#[inline]
	fn random_f32(&mut self) -> f32 {
		self.random_f64() as f32
	}

	/// by default, is derived from two calls to [`Random::random_u64()`].
	#[inline]
	fn random_u128(&mut self) -> u128 {
		let a = (self.random_u64() as u128) << 64;
		let b = self.random_u64() as u128;
		a | b
	}

	/// by default, is derived from two calls to [`Random::random_u32()`].
	#[inline]
	fn random_u64(&mut self) -> u64 {
		let a = (self.random_u32() as u64) << 32;
		let b = self.random_u32() as u64;
		a | b
	}

	/// by default, is derived from one call to [`Random::random_f64()`].
	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::f64_to_u32(self.random_f64())
	}

	/// by default, is derived from one call to [`Random::random_f64()`].
	#[inline]
	fn random_u16(&mut self) -> u16 {
		crate::common::f64_to_u16(self.random_f64())
	}

	/// by default, is derived from one call to [`Random::random_f64()`].
	#[inline]
	fn random_u8(&mut self) -> u8 {
		crate::common::f64_to_u8(self.random_f64())
	}

	/// derived from a call to [`Random::random_f64()`].
	#[inline]
	fn random_range(&mut self, range: core::ops::Range<f64>) -> f64 {
		range.start + self.random_f64() * (range.end - range.start)
	}

	#[inline]
	fn random_iter(&mut self) -> Iter<&mut Self> where Self: Sized {
		Iter {
			inner: self,
		}
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

pub struct Iter<T: Random> {
	inner: T,
}

impl<T: Random> Iter<T> {
	#[inline]
	pub fn new(inner: T) -> Self {
		Self {
			inner,
		}
	}
}

impl<T: Random> Iterator for Iter<T> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

impl<T: Random> Random for Iter<T> {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		self.inner.random_f64()
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

