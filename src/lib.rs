//! # Random white noise turned into random numbers
//!
//! The [ANU QRNG] project offers true random numbers to anyone on the internet.
//! By their description:
//!
//! > The random numbers are generated in real-time in our lab
//! > by measuring the quantum fluctuations of the vacuum.
//!
//! The random numbers are offered in a variety of formats,
//! including as [five seconds of white noise][whitenoise].
//!
//! This can be turned back into random bytes and used wherever one need [`rand::Rng`](https://docs.rs/rand/0.8.3/rand/trait.Rng.html).
//!
//! # Example
//!
//! ```rust,no_run
//! use noiserand::NoiseRand;
//! use rand::Rng;
//!
//! let mut rng = NoiseRand::new();
//! let x: u32 = rng.gen();
//! println!("{}", x);
//! ```
//!
//! # Listen to randomness.
//!
//! You can run the included example that turns a stream of random bytes back into noise:
//!
//! ```text
//! cargo run --example play
//! ```
//!
//! # Should I really use that?
//!
//! I mean ... it's guaranteed to be random.
//!
//! It's also slow.
//! On the first request for random data it will fetch a 5s audio sample
//! from the [ANU QRNG] project, parse the WAVE file
//! and then use the samples as random numbers.
//! After about 588 kB of random data generated,
//! it sends off another HTTP request to fill the buffer again.
//!
//! Please don't overload the servers.
//! They do however accept donations, see [FAQ: Donating to ANU QRNG][donation].
//!
//! [ANU QRNG]: https://qrng.anu.edu.au/
//! [whitenoise]: https://qrng.anu.edu.au/random-white-noise/
//! [donation]: https://qrng.anu.edu.au/contact/faq/#giving

use std::io::Cursor;

use isahc::prelude::*;
use rand_core::{impls, Error, RngCore};

const WHITENOISE_URL: &str =
    "https://qrng.anu.edu.au/wp-content/plugins/colours-plugin/get_audio_whiteNoise.php";
const PREFIX: &[u8] = b"data:audio/wav;base64,";

/// A random number generator seeded by noise.
///
/// This is based on random numbers provided by the [ANU QRNG] project.
/// By their description:
///
/// > The random numbers are generated in real-time in our lab
/// > by measuring the quantum fluctuations of the vacuum.
///
/// [ANU QRNG]: https://qrng.anu.edu.au/
pub struct NoiseRand {
    buffer: Vec<i16>,
}

impl NoiseRand {
    pub fn new() -> Self {
        let buffer = vec![];
        Self { buffer }
    }

    fn fill_buffer(&mut self) {
        log::trace!("trying to fill the buffer");
        let mut response = isahc::get(WHITENOISE_URL).unwrap();
        let mut buffer = Cursor::new(vec![]);
        let len = response.copy_to(&mut buffer).unwrap();
        if len <= PREFIX.len() as u64 {
            panic!("didn't get enough data");
        }
        let buffer = buffer.into_inner();
        let bytes = base64::decode(&buffer[PREFIX.len()..]).expect("not base64 encoded");
        log::trace!("got a total of {} bytes", bytes.len());
        let mut reader = hound::WavReader::new(Cursor::new(bytes)).unwrap();
        log::trace!("WAVE file decoded, expect {} samples", reader.len());

        self.buffer = reader.samples().flatten().collect();
    }
}

impl RngCore for NoiseRand {
    fn next_u32(&mut self) -> u32 {
        log::trace!(
            "4 byte requested, {} samples left in the buffer",
            self.buffer.len()
        );
        if self.buffer.len() < 2 {
            self.fill_buffer();
        }

        (self.buffer.pop().unwrap() as u32) << 16 | self.buffer.pop().unwrap() as u32
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
