#![doc = include_str!("../readme.md")]

#![no_std]

pub mod common;

mod random;
#[doc(inline)]
pub use random::*;

mod iter;
#[doc(inline)]
pub use iter::Iter;

mod r#static;
#[doc(inline)]
pub use r#static::Static;

mod crush;
#[doc(inline)]
pub use crush::Crush;

pub mod buffer;

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

mod xoshiro256ss;
#[doc(inline)]
pub use xoshiro256ss::XorShift256ss;

mod chacha;
#[doc(inline)]
pub use chacha::ChaCha;

mod collatzweyl;
#[doc(inline)]
pub use collatzweyl::CollatzWeyl64;
#[doc(inline)]
pub use collatzweyl::CollatzWeyl128_64;
#[doc(inline)]
pub use collatzweyl::CollatzWeyl128;

mod mtwister;
#[doc(inline)]
pub use mtwister::MTwister;

mod splitmix64;
#[doc(inline)]
pub use splitmix64::SplitMix64;

mod pcg32;
#[doc(inline)]
pub use pcg32::Pcg32;

pub mod lcg;

mod fiblfsr16;
#[doc(inline)]
pub use fiblfsr16::FibLFSR16;

mod fiblfg8;
#[doc(inline)]
pub use fiblfg8::FibLFG8;
#[doc(inline)]
pub use fiblfg8::dornd;

