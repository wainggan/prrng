#![doc = include_str!("../readme.md")]

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

