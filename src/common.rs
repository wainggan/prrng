//! this module provides a few common, simple utility functions you might
//! find useful when implementing your own [`crate::Random`] types.

/// convert a f64 to a f64. only included for completeness.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_f64;
/// assert_eq!(f64_to_f64(0.0f64), 0.0f64);
/// assert_eq!(f64_to_f64(1.0f64), 1.0f64);
/// ```
#[inline]
pub const fn f64_to_f64(x: f64) -> f64 {
	x
}

/// convert a f64 to a f32. only included for completeness.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::f64_to_f32;
/// assert_eq!(f64_to_f32(0.0f64), 0.0f32);
/// assert_eq!(f64_to_f32(1.0f64), 1.0f32);
/// ```
#[inline]
pub const fn f64_to_f32(x: f64) -> f32 {
	x as f32
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
pub const fn f64_to_u8(x: f64) -> u8 {
	(x * u8::MAX as f64) as u8
}

/// convert a u128 to a f64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::u128_to_f64;
/// assert_eq!(u128_to_f64(0u128), 0.0);
/// assert_eq!(u128_to_f64(u128::MAX), 1.0);
/// ```
#[inline]
pub const fn u128_to_f64(x: u128) -> f64 {
	x as f64 / u128::MAX as f64
}

/// convert a u64 to a f64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::u64_to_f64;
/// assert_eq!(u64_to_f64(0u64), 0.0);
/// assert_eq!(u64_to_f64(u64::MAX), 1.0);
/// ```
#[inline]
pub const fn u64_to_f64(x: u64) -> f64 {
	x as f64 / u64::MAX as f64
}

/// convert a u32 to a f64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::u32_to_f64;
/// assert_eq!(u32_to_f64(0u32), 0.0);
/// assert_eq!(u32_to_f64(u32::MAX), 1.0);
/// ```
#[inline]
pub const fn u32_to_f64(x: u32) -> f64 {
	x as f64 / u32::MAX as f64
}

/// convert a u16 to a f64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::u16_to_f64;
/// assert_eq!(u16_to_f64(0u16), 0.0);
/// assert_eq!(u16_to_f64(u16::MAX), 1.0);
/// ```
#[inline]
pub const fn u16_to_f64(x: u16) -> f64 {
	x as f64 / u16::MAX as f64
}

/// convert a u8 to a f64.
/// 
/// ## examples
/// 
/// ```
/// # use prrng::common::u8_to_f64;
/// assert_eq!(u8_to_f64(0u8), 0.0);
/// assert_eq!(u8_to_f64(u8::MAX), 1.0);
/// ```
#[inline]
pub const fn u8_to_f64(x: u8) -> f64 {
	x as f64 / u8::MAX as f64
}

