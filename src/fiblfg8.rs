
#[inline(always)]
const fn rol(x: u8, c: bool) -> (u8, bool) {
	let carry = (0x80 & x) >> 7 != 0;
	(x << 1 | c as u8, carry)
}

#[inline(always)]
const fn adc(x: u8, y: u8, c: bool) -> (u8, bool) {
	let a = x.overflowing_add(y);
	let b = a.0.overflowing_add(c as u8);
	(b.0, a.1 || b.1)
}

#[inline]
pub const fn dornd(rand: &mut (u8, u8, u8, u8), carry: &mut bool) {
	let c = *carry;

	// lda RAND
	let a = rand.0;

	// rol A
	let (a, c) = rol(a, c);

	// tax
	let x = a;

	// adc RAND+2
	let (a, c) = adc(a, rand.2, c);

	// sta RAND
	rand.0 = a;

	// stx RAND+2
	rand.2 = x;


	// lda RAND+1
	let a = rand.1;

	// tax
	let x = a;

	// adc RAND+3
	let (a, c) = adc(a, rand.3, c);

	// sta RAND+1
	rand.1 = a;

	// stx RAND+3
	rand.3 = x;

	*carry = c;
}


/// [8bit lagged fibonacci generator](https://en.wikipedia.org/wiki/Lagged_Fibonacci_generator),
/// extracted from Elite's [source code](https://elite.bbcelite.com/cassette/main/subroutine/dornd.html).
#[derive(Clone)]
pub struct FibLFG8 {
	rand: (u8, u8, u8, u8),
	carry: bool,
}

impl FibLFG8 {
	#[inline]
	pub const fn new_raw(f0: u8, f1: u8, m0: u8, m1: u8, carry: bool) -> Self {
		Self {
			rand: (f1, m1, f0, m0),
			carry,
		}
	}

	#[inline]
	pub const fn new(seed: u32) -> Self {
		let [f1, m1, f0, m0] = seed.to_be_bytes();

		let f0 = crate::common::u8_or_1(f0);
		let f1 = crate::common::u8_or_1(f1);
		let m0 = crate::common::u8_or_1(m0);
		let m1 = crate::common::u8_or_1(m1);

		Self::new_raw(f0, f1, m0, m1, false)
	}

	#[inline]
	pub const fn rand(&mut self) -> &mut (u8, u8, u8, u8) {
		&mut self.rand
	}

	#[inline]
	pub const fn carry(&mut self) -> &mut bool {
		&mut self.carry
	}

	/// `RAND+2`
	#[inline]
	pub const fn f0(&mut self) -> &mut u8 {
		&mut self.rand.2
	}

	/// `RAND`
	#[inline]
	pub const fn f1(&mut self) -> &mut u8 {
		&mut self.rand.0
	}

	/// `RAND+3`
	#[inline]
	pub const fn m0(&mut self) -> &mut u8 {
		&mut self.rand.3
	}

	/// `RAND+1`
	#[inline]
	pub const fn m1(&mut self) -> &mut u8 {
		&mut self.rand.1
	}

	#[inline]
	pub const fn get_last(&self) -> u8 {
		self.rand.3
	}

	#[inline]
	pub const fn get(&mut self) -> u8 {
		dornd(&mut self.rand, &mut self.carry);
		self.rand.1
	}
}

impl crate::Random for FibLFG8 {
	#[inline]
	fn random_u64(&mut self) -> u64 {
		crate::common::u32_compose_u64(self.random_u32(), self.random_u32())
	}

	#[inline]
	fn random_u32(&mut self) -> u32 {
		crate::common::u16_compose_u32(self.random_u16(), self.random_u16())
	}

	#[inline]
	fn random_u16(&mut self) -> u16 {
		crate::common::u8_compose_u16(self.random_u8(), self.random_u8())
	}

	#[inline]
	fn random_u8(&mut self) -> u8 {
		self.get()
	}
}

impl Iterator for FibLFG8 {
	type Item = f64;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		use crate::Random;
		Some(self.random_f64())
	}
}

impl core::fmt::Debug for FibLFG8 {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "FibLFG8")
	}
}

#[cfg(test)]
mod test {
    use crate::FibLFG8;

	#[test]
	fn test_basic() {
		let mut rng = FibLFG8::new(0x0212c845);

		assert_eq!(rng.get(), 87);
		assert_eq!(rng.get(), 105);
		assert_eq!(rng.get_last(), 87);
		assert_eq!(rng.get(), 192);
		assert_eq!(rng.get_last(), 105);
		assert_eq!(rng.get(), 41);
		assert_eq!(rng.get(), 234);
	}
}


