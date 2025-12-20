
#[derive(Clone)]
pub struct Iter<R: crate::Random> {
	inner: R,
}

impl<R: crate::Random> Iter<R> {
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

impl<R: crate::Random> Iterator for Iter<R> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

impl<R: crate::Random> crate::Random for Iter<R> {
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

impl<R: crate::Random + core::fmt::Debug> core::fmt::Debug for Iter<R> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Iter({:?})", self.inner)
	}
}

