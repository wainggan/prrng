//! utility types for caching [`crate::Random`] calls.
//! 
//! some prng algorithms can be slow. this type allows you to preemptively
//! fill a buffer full of random values, allowing for trivially fast
//! "generation" until the buffer has been fully consumed and needs to be refilled
//! with more random values.
//! 
//! internally, this uses a normal rust array for the buffer, and therefore
//! should be heap-allocated (such as with `Box`) when using larger buffers.

/// cache values `T`. see the [module level documentation](self) for more information.
/// 
/// note that this type only implements [`crate::Random`] if `T` is either `u32` or `u64`.
#[derive(Clone)]
pub struct Buffer<T: crate::FromRandom, const N: usize, R: crate::Random> {
	inner: R,
	buf: BufferDropable<T, N>,
}

impl<T: crate::FromRandom, const N: usize, R: crate::Random> Buffer<T, N, R> {
	/// construct a new `Buffer`.
	#[inline]
	pub const fn new(inner: R) -> Self {
		Self {
			inner,
			buf: BufferDropable::new(),
		}
	}

	/// consume `self`, returning the inner rng.
	#[inline]
	pub fn unwrap(self) -> R {
		self.inner
	}

	/// whether the buffer is consumed or not.
	#[inline]
	pub fn consumed(&self) -> bool {
		self.buf.index >= N
	}

	/// refills the buffer, regardless if it had been consumed or not.
	pub fn run(&mut self) {
		for i in &mut self.buf.buf {
			*i = core::mem::MaybeUninit::new(self.inner.random());
		}
		self.buf.index = 0;
	}

	/// returns the next value.
	/// if the buffer has been consumed, this returns `None`.
	/// 
	/// ```
	/// # use prrng::XorShift64;
	/// use prrng::Random;
	/// let mut rng = XorShift64::new(1).random_into_buffer::<u64, 4>();
	/// 
	/// assert!(matches!(rng.get_checked(), None));
	/// 
	/// rng.run();
	/// 
	/// assert!(matches!(rng.get_checked(), Some(_)));
	/// assert!(matches!(rng.get_checked(), Some(_)));
	/// assert!(matches!(rng.get_checked(), Some(_)));
	/// assert!(matches!(rng.get_checked(), Some(_)));
	/// assert!(matches!(rng.get_checked(), None));
	/// ```
	#[inline]
	pub fn get_checked(&mut self) -> Option<T> {
		if self.buf.index >= N {
			None
		} else {
			let ret = &self.buf.buf[self.buf.index];
			self.buf.index += 1;
			Some(unsafe {
				// safety:
				// 1. invariant: `buf[0..index]` is uninit
				// 2. therefore, `buf[index..N]` is init
				// 3. struct is created with `index == N` (nothing is init)
				// 4. `index` can only decrement in `run()`, where buf is initialized
				// 5. therefore, in this function, we can only be
				// at this branch if `index` is pointing to valid data
				// 6. copy made here is never touched by us again
				// (since `index` is incremented)
				ret.assume_init_read()
			})
		}
	}

	/// returns the next value.
	/// 
	/// if the buffer has been consumed, the buffer will be automatically
	/// refilled here.
	/// see [`Self::get_checked()`] for a version that does not refill.
	/// 
	/// ```
	/// # use prrng::XorShift64;
	/// use prrng::Random;
	/// let mut rng = XorShift64::new(1).random_into_buffer::<u64, 4>();
	/// 
	/// assert!(rng.consumed());
	/// 
	/// rng.get(); // rng.run() called here
	/// 
	/// assert!(!rng.consumed());
	/// 
	/// rng.get();
	/// rng.get();
	/// rng.get();
	/// 
	/// assert!(rng.consumed());
	/// 
	/// rng.get(); // rng.run() called here again
	/// 
	/// assert!(!rng.consumed());
	/// ```
	#[inline]
	pub fn get(&mut self) -> T {
		if self.buf.index >= N {
			self.run();
		}

		let ret = &self.buf.buf[self.buf.index];
		self.buf.index += 1;
		unsafe {
			// safety:
			// 1. invariant: `buf[0..index]` is uninit
			// 2. therefore, `buf[index..N]` is init
			// 3. struct is created with `index == N` (nothing is init)
			// 4. `index` can only decrement in `run()`, where buf is initialized
			// 5. therefore, in this function, we can only be
			// at this point if `index` is pointing to valid data
			// 6. copy made here is never touched by us again
			// (since `index` is incremented)
			ret.assume_init_read()
		}
	}
}

impl<T: crate::FromRandom, const N: usize, R: crate::Random + core::fmt::Debug> core::fmt::Debug for Buffer<T, N, R> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Buffer<[{}; {}]>({:?})", core::any::type_name::<T>(), N, self.inner)
	}
}

// used for safer drop semantics
struct BufferDropable<T: crate::FromRandom, const N: usize>{
	buf: [core::mem::MaybeUninit<T>; N],
	// any indice < `index` is uninit
	index: usize,
}

impl<T: crate::FromRandom, const N: usize> BufferDropable<T, N> {
	const fn new() -> Self {
		Self {
			buf: [const { core::mem::MaybeUninit::uninit() }; N],
			index: N,
		}
	}
}

impl<T: crate::FromRandom + Clone, const N: usize> Clone for BufferDropable<T, N> {
	fn clone(&self) -> Self {
		let mut buf = Self::new();

		let mut i = self.index;
		while i < N {
			buf.buf[i] = unsafe {
				// safety:
				// this loop keeps `i` within the range of `index..N`, which
				// as discussed earlier, is init, and therefore safe to assume_init.
				core::mem::MaybeUninit::new(self.buf[i].assume_init_ref().clone())
			};
			i += 1;
		}

		// doing this after out of paranoia
		buf.index = self.index;

		buf
	}
}

impl<T: crate::FromRandom, const N: usize> Drop for BufferDropable<T, N> {
	fn drop(&mut self) {
		for i in &mut self.buf[self.index..N] {
			unsafe {
				// safety:
				// this loop keeps `i` within the range of `index..N`, which
				// as discussed earlier, is init, and therefore safe to assume_init.
				i.assume_init_drop();
			}
		}
	}
}

impl<const N: usize, R: crate::Random> crate::RandomImpl for Buffer<u64, N, R> {
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

impl<const N: usize, R: crate::Random> crate::RandomImpl for Buffer<u32, N, R> {
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

/// cache `u8` values. see the [module level documentation](self) for more information.
#[derive(Clone)]
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

impl<const N: usize, R: crate::Random + core::fmt::Debug> core::fmt::Debug for Buffer8<N, R> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "Buffer8<[u8; {}]>({:?})", N, self.inner)
	}
}


#[cfg(test)]
mod test {
	#[test]
	fn test_miri() {
		extern crate std;

		// avoid a scary warning
		#[allow(dead_code)]
		#[derive(Clone)]
		struct Wrap(std::boxed::Box<u64>);

		impl crate::FromRandom for Wrap {
			fn from_random(random: &mut impl crate::Random) -> Self {
				Wrap(std::boxed::Box::new(random.random()))
			}
		}

		use crate::XorShift64;
		use crate::Random;
		let mut rng = XorShift64::new(1).random_into_buffer::<Wrap, 4>();

		rng.get(); // refill
		rng.get();

		let mut rng2 = rng.clone();

		let a = rng2.get();
		let b = rng2.get();

		drop(rng2);

		assert_eq!(rng.get().0, a.0);
		assert_eq!(rng.get().0, b.0);
		rng.get(); // refill
	}
}

