//! this module provides a few common, simple utility functions you might
//! find useful when implementing your own [`crate::Random`] types.

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

/// ```
/// # use prrng::common::u64_normalize_f64;
/// assert_eq!(u64_normalize_f64(u64::MAX), 1.0 - f64::EPSILON);
/// assert_eq!(u64_normalize_f64(0), 0.0);
/// ```
#[inline(always)]
pub const fn u64_normalize_f64(x: u64) -> f64 {
	let x = x & 0x00_0f_ff_ff_ff_ff_ff_ff;
	let x = x | 0x3f_f0_00_00_00_00_00_00;
	f64::from_bits(x) - 1.0
}

/// ```
/// # use prrng::common::u32_normalize_f32;
/// assert_eq!(u32_normalize_f32(u32::MAX), 1.0 - f32::EPSILON);
/// assert_eq!(u32_normalize_f32(0), 0.0);
/// ```
#[inline(always)]
pub const fn u32_normalize_f32(x: u32) -> f32 {
	let x = x & 0x00_7f_ff_ff;
	let x = x | 0x3f_80_00_00;
	f32::from_bits(x) - 1.0
}

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

/// ```
/// # use prrng::common::u32_compose_u64;
/// assert_eq!(u32_compose_u64(0xf0f0f0f0, 0x70707070), 0xf0f0f0f070707070);
/// ```
#[inline(always)]
pub const fn u32_compose_u64(x: u32, y: u32) -> u64 {
	(x as u64) << 32 | y as u64
}

/// ```
/// # use prrng::common::u16_compose_u32;
/// assert_eq!(u16_compose_u32(0x1234, 0x5678), 0x12345678);
/// ```
#[inline(always)]
pub const fn u16_compose_u32(x: u16, y: u16) -> u32 {
	(x as u32) << 16 | y as u32
}

/// ```
/// # use prrng::common::u8_compose_u16;
/// assert_eq!(u8_compose_u16(0x12, 0x34), 0x1234);
/// ```
#[inline(always)]
pub const fn u8_compose_u16(x: u8, y: u8) -> u16 {
	(x as u16) << 8 | y as u16
}

/// convert a f64 to a u128.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_u128;
/// assert_eq!(f64_to_u128(0.0f64), 0u128);
/// assert_eq!(f64_to_u128(1.0f64), u128::MAX);
/// ```
#[inline(always)]
pub const fn f64_to_u128(x: f64) -> u128 {
	(x * u128::MAX as f64) as u128
}

/// convert a f64 to a u64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_u64;
/// assert_eq!(f64_to_u64(0.0f64), 0u64);
/// assert_eq!(f64_to_u64(1.0f64), u64::MAX);
/// ```
#[inline(always)]
pub const fn f64_to_u64(x: f64) -> u64 {
	(x * u64::MAX as f64) as u64
}

/// convert a f64 to a u32.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_u32;
/// assert_eq!(f64_to_u32(0.0f64), 0u32);
/// assert_eq!(f64_to_u32(1.0f64), u32::MAX);
/// ```
#[inline(always)]
pub const fn f64_to_u32(x: f64) -> u32 {
	(x * u32::MAX as f64) as u32
}

/// convert a f64 to a u16.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_u16;
/// assert_eq!(f64_to_u16(0.0f64), 0u16);
/// assert_eq!(f64_to_u16(1.0f64), u16::MAX);
/// ```
#[inline(always)]
pub const fn f64_to_u16(x: f64) -> u16 {
	(x * u16::MAX as f64) as u16
}

/// convert a f64 to a u8.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_u8;
/// assert_eq!(f64_to_u8(0.0f64), 0u8);
/// assert_eq!(f64_to_u8(1.0f64), u8::MAX);
/// ```
#[inline(always)]
pub const fn f64_to_u8(x: f64) -> u8 {
	(x * u8::MAX as f64) as u8
}

