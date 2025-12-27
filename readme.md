
# prrng

a collection of psuedo-random number generators.

this crate provides a few prng algorithms, easily composable with each other via the [`Random`] trait.

```rust
use prrng::XorShift32;

fn main() {
    let mut rng = XorShift32::new(1);

    assert_eq!(rng.get(), 270369u32);
    assert_eq!(rng.get(), 67634689u32);

    use prrng::Random;

    let mut iter = rng.random_iter();
    assert_eq!(iter.next(), Some(0.7912035671411848));
    assert_eq!(iter.next(), Some(0.5683147178403836));

    assert_eq!(rng.random::<u64>(), 2716289712455752882);
    assert_eq!(rng.random::<(u8, bool)>(), (37, false));
}
```

prrng is for fun, and mainly intended to have minimal and extremely simple implementations of various rng algorithms, including popular ones like some xorshift variants or ChaCha, and including esoteric ones like a recreation of the infamous RANDU function, or even the rng used in BBC Elite. all while being completely `no_std`, mostly `const fn`, and dependency-free.

everything here is best effort.


## rust version support

what is that?

