
# prrng

collection of psuedo-random number generators.

this crate provides a few prng algorithms, easily composable with each other via the the [`Random`] trait.

```rust
use prrng::XorShift32;
use prrng::Random;

fn main() {
    let mut rng = XorShift32::new(0);

    assert_eq!(rng.get(), 270369);
    assert_eq!(rng.get(), 67634689);

    assert_eq!(rng.next(), Some(0.7912035671411848));
    assert_eq!(rng.next(), Some(0.5683147178403836));

    assert_eq!(rng.random_range(8.0..16.0) as u32, 9);
    assert_eq!(rng.random_range(8.0..16.0) as u32, 11);
}
```

