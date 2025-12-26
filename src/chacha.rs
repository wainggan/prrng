
/// [chacha](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant)
/// cryptographically secure psuedo-rng.
/// 
/// the internal state is a 4x4 matrix, specified
/// (by [rfc 8439](https://www.rfc-editor.org/rfc/rfc8439), from which this
/// implementation is derived from) to have this shape:
/// 
/// ```text
/// | "expa" | "nd 3" | "2-by" | "te k" |
/// | key    | key    | key    | key    |
/// | key    | key    | key    | key    |
/// | count  | nonce  | nonce  | nonce  |
/// ```
/// 
/// the top row, ascii bytes "expand 32-byte k", is
/// [for transparency](https://en.wikipedia.org/wiki/Nothing-up-my-sleeve_number).
/// for encryption, `key` is intended to be a constant shared secret between
/// a sender and reciever throughout a session, `nonce` should be changed
/// every message, and `count` is intended to be changed every 64 bytes.
/// the intention here is, for every 64 bytes, a new `ChaCha` instance is instantiated
/// with an incremented `count` value. one should then call [`Self::run()`] to
/// complete `N` rounds of the algorithm, then use the values of [`Self::inner()`]
/// for encryption.
/// 
/// ```
/// # use prrng::SplitMix64;
/// # use prrng::Random;
/// # use prrng::ChaCha;
/// # extern crate std;
/// # use std::prelude::rust_2024::*; 
/// fn encrypt_block(key: [u32; 8], nonce: [u32; 3], block: u32, bytes: &mut [u8]) {
///     // new ChaCha12 instance for every block
///     let mut rng = ChaCha::new(key, nonce, block);
/// 
///     // run 12 rounds
///     rng.run();
/// 
///     // xor every byte
///     // (this example assumes `bytes` is at most 64 bytes)
///     for (i, o) in bytes.iter_mut().zip(rng.inner_bytes().iter()) {
///         *i ^= *o;
///     }
/// }
/// 
/// let mut pretend_this_is_secure = SplitMix64::new(1);
/// 
/// // please use a good source of entropy for this. see the crate `getrandom`.
/// let key = pretend_this_is_secure.random();
/// let nonce = pretend_this_is_secure.random();
/// 
/// let mut message = b"meow meow meow meow meow meow".to_vec();
/// 
/// // encrypt message in 64 byte chunks
/// for (block, bytes) in message.chunks_mut(64).enumerate() {
///     encrypt_block(key, nonce, block.try_into().unwrap(), bytes);
/// }
/// 
/// // message has been encrypted!
/// assert_ne!(message, b"meow meow meow meow meow meow");
/// 
/// // decrypt message in 64 byte chunks
/// for (block, bytes) in message.chunks_mut(64).enumerate() {
///     encrypt_block(key, nonce, block.try_into().unwrap(), bytes);
/// }
/// 
/// // message has been retrieved!
/// assert_eq!(message, b"meow meow meow meow meow meow");
/// ```
/// 
/// of course, this generator can also just be used as a rather good prng.
#[derive(Clone)]
pub struct ChaCha<const N: u8 = 12> {
	seed: [u32; 16],
	serialized: u8,
}

impl ChaCha {
	/// construct a new `ChaCha12`.
	/// see [`Self::new_n()`] for a generic constructor method.
	/// 
	/// see [`ChaCha`]'s documentation for how initialization should work.
	#[inline]
	pub fn new(key: [u32; 8], nonce: [u32; 3], block: u32) -> Self {
		Self::new_n(key, nonce, block)
	}
}

impl<const N: u8> ChaCha<N> {
	/// construct a new `ChaCha` instance.
	/// unlike [`Self::new_n()`], this method does not have organized arguments.
	/// 
	/// see [`ChaCha`]'s documentation for how initialization should work.
	#[inline]
	pub const fn new_raw(seed: [u32; 16]) -> Self {
		Self {
			seed,
			serialized: 16,
		}
	}

	/// construct a new `ChaCha`.
	/// 
	/// see [`ChaCha`]'s documentation for how initialization should work.
	#[inline]
	pub const fn new_n(key: [u32; 8], nonce: [u32; 3], block: u32) -> Self {
		Self::new_raw([
			0x61707865,
			0x3320646e,
			0x79622d32,
			0x6b206574,
			key[0],
			key[1],
			key[2],
			key[3],
			key[4],
			key[5],
			key[6],
			key[7],
			block,
			nonce[0],
			nonce[1],
			nonce[2],
		])
	}

	/// get the internal state, which is also this algorithm's output
	/// following a call to [`Self::run()`].
	#[inline]
	pub fn inner(&self) -> [u32; 16] {
		self.seed
	}

	/// get the internal state as bytes, which is also this algorithm's output
	/// following a call to [`Self::run()`].
	#[inline]
	pub fn inner_bytes(&self) -> [u8; 64] {
		// todo: surely there's a better way of doing this :(
		let mut ret = [0; 64];
		let (iter, _) = ret.as_chunks_mut::<4>();

		for (o, i) in iter.iter_mut().zip(self.seed.iter()) {
			*o = i.to_le_bytes();
		}

		ret
	}

	/// complete `N` rounds of the `ChaCha` algorithm.
	pub fn run(&mut self) {
		let mut x = self.seed;

		macro_rules! qr {
			($a:expr, $b:expr, $c:expr, $d:expr) => {
				$a = $a.wrapping_add($b);
				$d ^= $a;
				$d = $d.rotate_left(16);

				$c = $c.wrapping_add($d);
				$b ^= $c;
				$b = $b.rotate_left(12);

				$a = $a.wrapping_add($b);
				$d ^= $a;
				$d = $d.rotate_left(8);

				$c = $c.wrapping_add($d);
				$b ^= $c;
				$b = $b.rotate_left(7);
			};
		}

		for _ in 0..N / 2 {
			qr!(x[0], x[4], x[8], x[12]);
			qr!(x[1], x[5], x[9], x[13]);
			qr!(x[2], x[6], x[10], x[14]);
			qr!(x[3], x[7], x[11], x[15]);

			qr!(x[0], x[5], x[10], x[15]);
			qr!(x[1], x[6], x[11], x[12]);
			qr!(x[2], x[7], x[8], x[13]);
			qr!(x[3], x[4], x[9], x[14]);
		}

		#[expect(clippy::needless_range_loop, reason = "resulting code-gen is good like this")]
		for i in 0..self.seed.len() {
			self.seed[i] = self.seed[i].wrapping_add(x[i]);
		}

		self.serialized = 0;
	}

	/// returns the next value of this generator, returning `None` if the
	/// current state is "consumed". a call to [`Self::run()`] resets the
	/// consumed status.
	/// see [`Self::get()`] for a version that automatically calls `run()`.
	#[inline]
	pub fn get_checked(&mut self) -> Option<u32> {
		if self.serialized >= 16 {
			None
		} else {
			let ret = self.seed[self.serialized as usize];
			self.serialized += 1;
			Some(ret)
		}
	}

	/// returns the next value of this generator. if the current state is
	/// "consumed", this method calls [`Self::run()`].
	/// see [`Self::get_checked()`] for a version that returns `None` instead.
	#[inline]
	pub fn get(&mut self) -> u32 {
		if self.serialized >= 16 {
			self.run();
		}

		let ret = self.seed[self.serialized as usize];
		self.serialized += 1;
		ret
	}
}

impl<const N: u8> crate::Random for ChaCha<N> {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.get(), self.get())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		self.get()
	}
}

impl<const N: u8> core::fmt::Debug for ChaCha<N> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "ChaCha{}", N)
	}
}

