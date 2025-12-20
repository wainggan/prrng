
/// utility type for caching many [`crate::Random`] calls.
/// 
/// some prng algorithms can be slow. this type allows you to preemptively
/// fill a buffer full of random values, allowing for trivially fast
/// "generation" until the buffer has been fully consumed and needs to be refilled
/// with more random values.
/// 
/// internally, this uses a normal `[u32; N]` for the buffer, and therefore
/// should be heap-allocated (such as with `Box`) for larger buffers.
pub struct Buffer32<const N: usize, R: crate::Random> {
	inner: R,
	buf: [u32; N],
	index: usize,
}

impl<const N: usize, R: crate::Random> Buffer32<N, R> {
	#[inline]
	pub const fn new(inner: R) -> Self {
		Self {
			inner,
			buf: [0; N],
			index: N,
		}
	}

	#[inline]
	pub fn unwrap(self) -> R {
		self.inner
	}

	/// refills the buffer, regardless if it had been consumed or not.
	pub fn run(&mut self) {
		for i in &mut self.buf {
			*i = self.inner.random_u32();
		}
		self.index = 0;
	}

	/// returns the next value.
	/// if the buffer has been consumed, this returns `None`.
	#[inline]
	pub fn get_checked(&mut self) -> Option<u32> {
		if self.index >= N {
			None
		} else {
			let ret = self.buf[self.index];
			self.index += 1;
			Some(ret)
		}
	}

	/// returns the next value.
	/// 
	/// if the buffer has been consumed, the buffer will be automatically
	/// refilled here.
	/// see [`Self::get_checked()`] for a version that does not refill.
	#[inline]
	pub fn get(&mut self) -> u32 {
		if self.index >= N {
			self.run();
		}

		let ret = self.buf[self.index];
		self.index += 1;
		ret
	}
}

impl<const N: usize, R: crate::Random> crate::Random for Buffer32<N, R> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.get(), self.get())
	}
	
	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}
}

impl<const N: usize, R: crate::Random> Iterator for Buffer32<N, R> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}



/// utility type for caching many [`crate::Random`] calls.
pub struct Buffer64<const N: usize, R: crate::Random> {
	inner: R,
	buf: [u64; N],
	index: usize,
}

impl<const N: usize, R: crate::Random> Buffer64<N, R> {
	#[inline]
	pub fn new(inner: R) -> Self {
		Self {
			inner,
			buf: [0; N],
			index: N,
		}
	}

	pub fn run(&mut self) {
		for i in &mut self.buf {
			*i = self.inner.random_u64();
		}
		self.index = 0;
	}

	#[inline]
	pub fn get_checked(&mut self) -> Option<u64> {
		if self.index >= N {
			None
		} else {
			let ret = self.buf[self.index];
			self.index += 1;
			Some(ret)
		}
	}

	#[inline]
	pub fn get(&mut self) -> u64 {
		if self.index >= N {
			self.run();
		}

		let ret = self.buf[self.index];
		self.index += 1;
		ret
	}
}

impl<const N: usize, R: crate::Random> crate::Random for Buffer64<N, R> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		self.get()
	}
	
	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get() as u32
	}
}

impl<const N: usize, R: crate::Random> Iterator for Buffer64<N, R> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}


