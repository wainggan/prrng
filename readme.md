
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

    assert_eq!(rng.next(), Some(0.6164041025602268));
    assert_eq!(rng.next(), Some(0.07161863499125899));

    assert_eq!(rng.random_range(8.0..16.0) as u32, 12);
    assert_eq!(rng.random_range(8.0..16.0) as u32, 9);
}
```

## road to 1.0.0

-[ ] use api in a project to see if it is usable
-[ ] finish documentation
-[ ] add more algorithms
    -[ ] [acorn](https://en.wikipedia.org/wiki/ACORN_(random_number_generator))
    -[ ] [blum blum shub](https://en.wikipedia.org/wiki/Blum_Blum_Shub)
    -[ ] [mersenne twister](https://en.wikipedia.org/wiki/Mersenne_Twister)
    -[ ] [more xorshift](https://en.wikipedia.org/wiki/Xorshift)
    -[ ] [collatz weyl](https://arxiv.org/abs/2312.17043)
    -[ ] [salsa20](https://en.wikipedia.org/wiki/Salsa20)
    -[ ] [isaac](https://en.wikipedia.org/wiki/ISAAC_(cipher))
-[ ] benchmark for fun

