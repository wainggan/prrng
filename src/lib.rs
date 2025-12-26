#![doc = include_str!("../readme.md")]

#![no_std]

#[cfg(test)]
mod test;

pub mod common;

mod random;
#[doc(inline)]
pub use random::*;


mod utility;

#[doc(inline)]
pub use utility::iter::*;

#[doc(inline)]
pub use utility::r#static::*;

#[doc(inline)]
pub use utility::crush::*;

#[doc(inline)]
pub use utility::buffer::*;


mod algorithm;

#[doc(inline)]
pub use algorithm::wichhill::*;

#[doc(inline)]
pub use algorithm::xorshift32::*;

#[doc(inline)]
pub use algorithm::xorshift64::*;

#[doc(inline)]
pub use algorithm::xorshift128p::*;

#[doc(inline)]
pub use algorithm::xoshiro256ss::*;

#[doc(inline)]
pub use algorithm::chacha::*;

#[doc(inline)]
pub use algorithm::collatzweyl::*;

#[doc(inline)]
pub use algorithm::mtwister::*;

#[doc(inline)]
pub use algorithm::splitmix64::*;

#[doc(inline)]
pub use algorithm::pcg32::*;

#[doc(inline)]
pub use algorithm::lcg;

#[doc(inline)]
pub use algorithm::fiblfg8::*;

#[doc(inline)]
pub use algorithm::fiblfsr16::*;

