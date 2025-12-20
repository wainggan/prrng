
/// this crate provides a few prng implementations. this is not one of those.
/// 
/// `Static` is an 'algorithm' that simply generates values you provide yourself.
/// 
/// ```
/// # use prrng::Static;
/// let mut rng = Static::new(|| 4.0); // chosen by fair dice roll.
/// assert_eq!(rng.get(), 4.0);
/// assert_eq!(rng.get(), 4.0);
/// assert_eq!(rng.get(), 4.0);
/// ```
/// 
/// this may be useful for testing, as `Static` also implements [`crate::Random`].
/// this lets you compose it with any type expecting `Random`.
/// 
/// ```
/// # use prrng::Static;
/// # use prrng::Random;
/// fn important(rng: impl Random) {
///     // ...
/// }
/// 
/// fn main() {
///     let mut rng = Static::new(|| 2.0);
///     important(&mut rng);
///     
///     // `Static` takes an `FnMut`
///     let mut i = 0u32;
///     let mut rng = Static::new(|| {
///         i += 1;
///         i as f64 / u32::MAX as f64
///     });
///     important(&mut rng);
/// }
/// ```
/// 
/// never trust safe code.
/// 
/// ```no_run
/// # use prrng::Static;
/// # use prrng::Random;
/// fn safe(slice: &[u8], mut rng: impl Random) {
///     unsafe {
///         let index = rng.random_range(0.0..slice.len() as f64) as usize;
///         // safety: ensure that we only index inside the slice.
///         let value = slice.get_unchecked(index); // (definetely UB)
///         println!("{}", value);
///     }
/// }
/// 
/// fn main() {
///     let mut rng = Static::new(|| 2.0);
///     safe(&[0], &mut rng);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Static<T: FnMut() -> f64> {
	cb: T,
}

impl<T: FnMut() -> f64> Static<T> {
	/// construct a new [`Static`].
	/// 
	/// ## examples
	/// 
	/// ```
	/// # use prrng::Static;
	/// let mut rng = Static::new(|| 0.0);
	/// ```
	#[inline]
	pub fn new(cb: T) -> Self {
		Self {
			cb,
		}
	}

	/// returns the next value by calling the inner `FnMut`.
	/// 
	/// ## examples
	/// 
	/// ```
	/// # use prrng::Static;
	/// let mut i = 0.0;
	/// let mut rng = Static::new(|| {
	///     i += 1.0;
	///     i
	/// });
	/// 
	/// assert_eq!(rng.get(), 1.0);
	/// assert_eq!(rng.get(), 2.0);
	/// assert_eq!(rng.get(), 3.0);
	/// ```
	#[inline]
	pub fn get(&mut self) -> f64 {
		(self.cb)()
	}
}

impl<T: FnMut() -> f64> crate::Random for Static<T> {
	#[inline]
	fn random_f64(&mut self) -> f64 {
		self.get()
	}

	#[inline]
	fn random_f32(&mut self) -> f32 {
		self.get() as f32
	}

	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::f64_to_u32(self.get())
	}
}

impl<T: FnMut() -> f64> Iterator for Static<T> {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		Some(self.get())
	}
}

