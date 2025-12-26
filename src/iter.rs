
/// iterator of [`crate::Random`].
/// 
/// this type simply implements [`Iterator`], where `T` is the return value
/// of [`Iterator::next()`] using [`crate::FromRandom`].
/// 
/// ```
/// # use prrng::Iter;
/// # use prrng::XorShift32;
/// let mut rng = XorShift32::new(1);
/// 
/// // either explicitly wrap it
/// let iter = Iter::<(), _>::new(&mut rng);
/// 
/// // or use the `Random` trait
/// use prrng::Random;
/// let iter = rng.random_iter::<()>();
/// ```
/// 
/// notably, this type *also* implements `Random`. this likely isn't useful.
#[derive(Clone)]
pub struct Iter<T: crate::FromRandom, R: crate::Random> {
	inner: R,
	_marker: core::marker::PhantomData<T>,
}

impl<T: crate::FromRandom, R: crate::Random> Iter<T, R> {
	/// construct a new `Iter` from an rng.
	#[inline]
	pub fn new(inner: R) -> Self {
		Self {
			inner,
			_marker: core::marker::PhantomData,
		}
	}

	/// consume `self` and return the inner rng.
	#[inline]
	pub fn unwrap(self) -> R {
		self.inner
	}
}

impl<T: crate::FromRandom, R: crate::Random> Iterator for Iter<T, R> {
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(self.inner.random())
	}
}

impl<T: crate::FromRandom, R: crate::Random> crate::Random for Iter<T, R> {
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

impl<T: crate::FromRandom, R: crate::Random + core::fmt::Debug> core::fmt::Debug for Iter<T, R> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Iter({:?})", self.inner)
	}
}

