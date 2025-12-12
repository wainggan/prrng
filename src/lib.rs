//! this crate provides a few prng algorithms, easily composable with
//! each other via the the [`Random`] trait.
//! 
//! - [WichHill]
//! - [XorShift32]
//! - [XorShift64]
//! - [XorShift128p]
//! - [FibLFSR16]
//! 
//! ```
//! # use prrng::XorShift32;
//! # use prrng::Random;
//! let mut rng = XorShift32::new(0);
//! 
//! assert_eq!(rng.get(), 270369);
//! assert_eq!(rng.get(), 67634689);
//! 
//! assert_eq!(rng.next(), Some(0.6164041025602268));
//! assert_eq!(rng.next(), Some(0.07161863499125899));
//! 
//! assert_eq!(rng.random_range(8.0..16.0) as u32, 12);
//! assert_eq!(rng.random_range(8.0..16.0) as u32, 9);
//! ```

#![no_std]

pub mod common;

mod random;
#[doc(inline)]
pub use random::*;

mod r#static;
#[doc(inline)]
pub use r#static::Static;

mod wichhill;
#[doc(inline)]
pub use wichhill::WichHill;

mod xorshift32;
#[doc(inline)]
pub use xorshift32::XorShift32;

mod xorshift64;
#[doc(inline)]
pub use xorshift64::XorShift64;

mod xorshift128p;
#[doc(inline)]
pub use xorshift128p::XorShift128p;

mod fiblfsr16;
#[doc(inline)]
pub use fiblfsr16::FibLFSR16;

mod fiblfg8;
#[doc(inline)]
pub use fiblfg8::FibLFG8;
#[doc(inline)]
pub use fiblfg8::dornd;

