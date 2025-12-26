
extern crate std;
use std::prelude::rust_2024::*;

#[test]
fn test_debug() {
	use std::fmt::Write;

	fn cmp(left: impl std::fmt::Debug, right: &str) {
		let mut string = String::new();
		write!(&mut string, "{:?}", left).unwrap();
		assert_eq!(string, right);
	}

	cmp(
		crate::ChaCha::new(
			[
				0, 0, 0, 0, 0, 0, 0, 0,
			],
			[
				0, 0, 0,
			],
			0,
		),
		"ChaCha12",
	);

	cmp(
		crate::ChaCha::<20>::new_n(
			[
				0, 0, 0, 0, 0, 0, 0, 0,
			],
			[
				0, 0, 0,
			],
			0,
		),
		"ChaCha20",
	);

	cmp(
		crate::XorShift32::new(0),
		"XorShift32",
	);

	cmp(
		crate::XorShift64::new(0),
		"XorShift64",
	);

	cmp(
		crate::XorShift128p::new([0, 0]),
		"XorShift128p",
	);

	cmp(
		crate::XorShift256ss::new([0, 0, 0, 0]),
		"XorShift256ss",
	);

	cmp(
		crate::WichHill::new([0, 0, 0]),
		"WichHill",
	);


	cmp(
		crate::CollatzWeyl64::new_one(0),
		"CollatzWeyl64",
	);

	cmp(
		crate::CollatzWeyl128_64::new_one(0),
		"CollatzWeyl128_64",
	);

	cmp(
		crate::CollatzWeyl128::new_one(0),
		"CollatzWeyl128",
	);

	cmp(
		crate::FibLFG8::new(0),
		"FibLFG8",
	);

	cmp(
		crate::FibLFSR16::new(0),
		"FibLFSR16",
	);

	cmp(
		crate::MTwister::new(0),
		"MTwister",
	);

	cmp(
		crate::Pcg32::new(0, 1),
		"Pcg32",
	);

	cmp(
		crate::SplitMix64::new(0),
		"SplitMix64",
	);

	cmp(
		crate::Iter::<(), _>::new(crate::XorShift32::new(0)),
		"Iter<()>(XorShift32)",
	);

	cmp(
		crate::Buffer::<(), 4, _>::new(crate::XorShift32::new(0)),
		"Buffer<[(); 4]>(XorShift32)",
	);

	cmp(
		crate::Buffer8::<4, _>::new(crate::XorShift32::new(0)),
		"Buffer8<[u8; 4]>(XorShift32)",
	);

	#[derive(Debug)]
	struct DebugHash;
	impl std::hash::Hasher for DebugHash {
		fn write(&mut self, _bytes: &[u8]) {}
		fn finish(&self) -> u64 { 0 }
	}

	cmp(
		crate::Crush::<4, _, _>::new(
			crate::XorShift32::new(0),
			DebugHash,
		),
		"Crush4(XorShift32, DebugHash)",
	);

	cmp(
		crate::Static::new(|| 0.0),
		"Static",
	);
}
