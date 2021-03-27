# noiserand

[![Crates.io version](https://img.shields.io/crates/v/noiserand.svg?style=flat-square)](https://crates.io/crates/noiserand)
[![docs.rs docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/noiserand)
[![License: MIT](https://img.shields.io/github/license/badboy/noiserand?style=flat-square)](LICENSE)

## Random white noise turned into random numbers

The [ANU QRNG] project offers true random numbers to anyone on the internet.
By their description:

> The random numbers are generated in real-time in our lab
> by measuring the quantum fluctuations of the vacuum.

The random numbers are offered in a variety of formats,
including as [five seconds of white noise][whitenoise].

This can be turned back into random bytes and used wherever one need [`rand::Rng`](https://docs.rs/rand/0.8.3/rand/trait.Rng.html).

## Example

```rust
use noiserand::NoiseRand;
use rand::Rng;

let mut rng = NoiseRand::new();
let x: u32 = rng.gen();
println!("{}", x);
```

## Listen to randomness.

You can run the included example that turns a stream of random bytes back into noise:

```
cargo run --example play
```

## Should I really use that?

I mean ... it's guaranteed to be random.

It's also slow.
On the first request for random data it will fetch a 5s audio sample
from the [ANU QRNG] project, parse the WAVE file
and then use the samples as random numbers.
After about 588 kB of random data generated,
it sends off another HTTP request to fill the buffer again.

Please don't overload the servers.
They do however accept donations, see [FAQ: Donating to ANU QRNG][donation].

[ANU QRNG]: https://qrng.anu.edu.au/
[whitenoise]: https://qrng.anu.edu.au/random-white-noise/
[donation]: https://qrng.anu.edu.au/contact/faq/#giving

# License


MIT. See [LICENSE](LICENSE).
