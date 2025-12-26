//! this module provides a few common, simple utility functions you might
//! find useful when implementing your own [`crate::RandomImpl`] types.

/// construct a `u64` with [`crate::RandomImpl::random_bytes()`] with
/// little-endian ordering.
#[inline(always)]
pub fn u64_from_bytes<R: crate::RandomImpl>(random: &mut R) -> u64 {
	let mut bytes = [0u8; _];
	random.random_bytes(&mut bytes);
	u64::from_le_bytes(bytes)
}

/// construct a `u32` with [`crate::RandomImpl::random_bytes()`] with
/// little-endian ordering.
#[inline(always)]
pub fn u32_from_bytes<R: crate::RandomImpl>(random: &mut R) -> u32 {
	let mut bytes = [0u8; _];
	random.random_bytes(&mut bytes);
	u32::from_le_bytes(bytes)
}

/// fill a buffer with values from [`crate::RandomImpl::random_u64()`], with
/// little-endian ordering.
/// 
/// ```
/// # use prrng::common::bytes_from_u64;
/// # use prrng::XorShift64;
/// let mut rng = XorShift64::new(1);
/// 
/// {
///     let mut rng = rng.clone();
///     // demonstrating the first two values of this generator:
///     assert_eq!(rng.get(), 0x00_00_00_00_40_82_20_41);
///     assert_eq!(rng.get(), 0x10_00_41_06_0c_01_14_41);
/// }
/// 
/// // 8 + 4 bytes
/// let mut buf = [0u8; 12];
/// 
/// bytes_from_u64(&mut rng, &mut buf);
/// 
/// assert_eq!(
///     buf,
///     [
///         // first value
///         0x41, 0x20, 0x82, 0x40, 0x00, 0x00, 0x00, 0x00,
///         // second value (cut off)
///         0x41, 0x14, 0x01, 0x0c,
///     ],
/// );
/// ```
#[inline(always)]
pub fn bytes_from_u64<R: crate::RandomImpl>(random: &mut R, dst: &mut [u8]) {
	let (chunks, extra) = dst.as_chunks_mut();

	for chunk in chunks {
		*chunk = random.random_u64().to_le_bytes();
	}

	if extra.is_empty() {
		return;
	}

	let last = random.random_u64().to_le_bytes();

	for (o, i) in extra.iter_mut().zip(last.iter()) {
		*o = *i;
	}
}

/// fill a buffer with values from [`crate::RandomImpl::random_u32()`], with
/// little-endian ordering.
/// 
/// ```
/// # use prrng::common::bytes_from_u32;
/// # use prrng::XorShift32;
/// let mut rng = XorShift32::new(1);
/// 
/// {
///     let mut rng = rng.clone();
///     // demonstrating the first two values of this generator:
///     assert_eq!(rng.get(), 0x00_04_20_21);
///     assert_eq!(rng.get(), 0x04_08_06_01);
/// }
/// 
/// // 4 + 2 bytes
/// let mut buf = [0u8; 6];
/// 
/// bytes_from_u32(&mut rng, &mut buf);
/// 
/// assert_eq!(
///     buf,
///     [
///         // first value
///         0x21, 0x20, 0x04, 0x00,
///         // second value (cut off)
///         0x01, 0x06,
///     ],
/// );
/// ```
#[inline(always)]
pub fn bytes_from_u32<R: crate::RandomImpl>(random: &mut R, dst: &mut [u8]) {
	let (chunks, extra) = dst.as_chunks_mut();

	for chunk in chunks {
		*chunk = random.random_u32().to_le_bytes();
	}

	if extra.is_empty() {
		return;
	}

	let last = random.random_u32().to_le_bytes();

	for (o, i) in extra.iter_mut().zip(last.iter()) {
		*o = *i;
	}
}

/// construct a `u128` from two `u64`.
/// 
/// ```
/// # use prrng::common::u64_compose_u128;
/// assert_eq!(
///     u64_compose_u128(
///         0xf0f0f0f0f0f0f0f0,
///         0x7070707070707070,
///     ),
///     0xf0f0f0f0f0f0f0f07070707070707070,
/// );
/// ```
#[inline(always)]
pub const fn u64_compose_u128(x: u64, y: u64) -> u128 {
	(x as u128) << 64 | y as u128
}

/// construct a `u64` from two `u32`.
/// 
/// ```
/// # use prrng::common::u32_compose_u64;
/// assert_eq!(u32_compose_u64(0xf0f0f0f0, 0x70707070), 0xf0f0f0f070707070);
/// ```
#[inline(always)]
pub const fn u32_compose_u64(x: u32, y: u32) -> u64 {
	(x as u64) << 32 | y as u64
}

/// construct a `u32` from two `u16`.
/// 
/// ```
/// # use prrng::common::u16_compose_u32;
/// assert_eq!(u16_compose_u32(0x1234, 0x5678), 0x12345678);
/// ```
#[inline(always)]
pub const fn u16_compose_u32(x: u16, y: u16) -> u32 {
	(x as u32) << 16 | y as u32
}

/// construct a `u16` from two `u8`.
/// 
/// ```
/// # use prrng::common::u8_compose_u16;
/// assert_eq!(u8_compose_u16(0x12, 0x34), 0x1234);
/// ```
#[inline(always)]
pub const fn u8_compose_u16(x: u8, y: u8) -> u16 {
	(x as u16) << 8 | y as u16
}

#[inline(always)]
pub(crate) const fn f64_to_u32(x: f64) -> u32 {
	(x * u32::MAX as f64) as u32
}

#[inline(always)]
pub(crate) const fn u64_normalize_f64(x: u64) -> f64 {
	let x = x & 0x00_0f_ff_ff_ff_ff_ff_ff;
	let x = x | 0x3f_f0_00_00_00_00_00_00;
	f64::from_bits(x) - 1.0
}

#[inline(always)]
pub(crate) const fn u32_normalize_f32(x: u32) -> f32 {
	let x = x & 0x00_7f_ff_ff;
	let x = x | 0x3f_80_00_00;
	f32::from_bits(x) - 1.0
}

#[inline(always)]
pub(crate) const fn u64_or_1(x: u64) -> u64 {
	if x == 0 {
		1
	} else {
		x
	}
}

#[inline(always)]
pub(crate) const fn u32_or_1(x: u32) -> u32 {
	if x == 0 {
		1
	} else {
		x
	}
}

#[inline(always)]
pub(crate) const fn u16_or_1(x: u16) -> u16 {
	if x == 0 {
		1
	} else {
		x
	}
}

#[inline(always)]
pub(crate) const fn u8_or_1(x: u8) -> u8 {
	if x == 0 {
		1
	} else {
		x
	}
}


#[cfg(test)]
mod test {
    use crate::common::{f64_to_u32, u32_normalize_f32, u64_normalize_f64};

	#[test]
	fn test_private() {
		assert_eq!(u64_normalize_f64(u64::MAX), 1.0 - f64::EPSILON);
		assert_eq!(u64_normalize_f64(0), 0.0);
		
		assert_eq!(u32_normalize_f32(u32::MAX), 1.0 - f32::EPSILON);
		assert_eq!(u32_normalize_f32(0), 0.0);

		assert_eq!(f64_to_u32(0.0f64), 0u32);
		assert_eq!(f64_to_u32(1.0f64), u32::MAX);
	}
}

