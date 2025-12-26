//! utility type for caching many [`crate::Random`] calls.
//! 
//! some prng algorithms can be slow. this type allows you to preemptively
//! fill a buffer full of random values, allowing for trivially fast
//! "generation" until the buffer has been fully consumed and needs to be refilled
//! with more random values.
//! 
//! internally, this uses a normal rust array for the buffer, and therefore
//! should be heap-allocated (such as with `Box`) when using larger buffers.

/// cache `u32` values. see the [module level documentation](self) for more information.
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

impl<const N: usize, R: crate::Random> crate::RandomImpl for Buffer32<N, R> {
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

/// cache `u64` values. see the [module level documentation](self) for more information.
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

impl<const N: usize, R: crate::Random> crate::RandomImpl for Buffer64<N, R> {
	#[inline]
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

/// cache `u8` values. see the [module level documentation](self) for more information.
pub struct Buffer8<const N: usize, R: crate::Random> {
	inner: R,
	buf: [u8; N],
	index: usize,
}

impl<const N: usize, R: crate::Random> Buffer8<N, R> {
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
		self.inner.random_bytes(&mut self.buf);
		self.index = 0;
	}

	/// returns the next value.
	/// if the buffer has been consumed, this returns `None`.
	#[inline]
	pub fn get_checked(&mut self) -> Option<u8> {
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
	pub fn get(&mut self) -> u8 {
		if self.index >= N {
			self.run();
		}

		let ret = self.buf[self.index];
		self.index += 1;
		ret
	}
}

impl<const N: usize, R: crate::Random> crate::RandomImpl for Buffer8<N, R> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u64_from_bytes(self)
	}
	
	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::u32_from_bytes(self)
	}

	#[inline]
	fn random_bytes(&mut self, dst: &mut [u8]) {
		for i in dst {
			*i = self.get();
		}
	}
}

