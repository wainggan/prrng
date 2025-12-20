
/// [chacha](https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant) psuedo-rng.
/// 
/// implementation is based on [rfc 7539](https://www.rfc-editor.org/rfc/rfc7539).
#[derive(Clone)]
pub struct ChaCha<const N: u8 = 12> {
	seed: [u32; 16],
	serialized: u8,
}

impl ChaCha {
	#[inline]
	pub fn new(key: [u32; 8], nonce: [u32; 3], block: u32) -> Self {
		Self::new_n(key, nonce, block)
	}
}

impl<const N: u8> ChaCha<N> {
	#[inline]
	pub const fn new_raw(seed: [u32; 16]) -> Self {
		Self {
			seed,
			serialized: 0,
		}
	}

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

	pub fn run(&mut self) {
		let mut x = self.seed;

		macro_rules! qr {
			($a:expr, $b:expr, $c:expr, $d:expr) => {
				$a += $b;
				$d ^= $a;
				$d = $d.rotate_left(16);

				$c += $d;
				$b ^= $c;
				$b = $b.rotate_left(12);

				$a += $b;
				$d ^= $a;
				$d = $d.rotate_left(8);

				$c += $d;
				$b ^= $c;
				$b = $b.rotate_left(7);
			};
		}

		for _ in 0..N {
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
			self.seed[i] += x[i];
		}

		self.serialized = 0;
	}

	#[inline]
	pub fn get(&mut self) -> u32 {
		if self.serialized > 16 {
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

impl<const N: u8> Iterator for ChaCha<N> {
	type Item = f64;
	
	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl<const N: u8> core::fmt::Debug for ChaCha<N> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "ChaCha{}", N)
	}
}

