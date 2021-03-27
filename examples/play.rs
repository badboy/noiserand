//! # Turn the random data back into noise.
//!
//! This uses the `NoiseRand` random generator,
//! which itself is seeded by white noise,
//! to turn random bytes back into white noise.
//!
//! **Caution**: Don't turn your speakers too loud.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use noiserand::NoiseRand;
use rand::Rng;

fn main() {
    env_logger::init();

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find output device");

    let config = device
        .default_output_config()
        .expect("failed to select output config");

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig)
where
    T: cpal::Sample,
{
    let channels = config.channels as usize;

    let mut rng = NoiseRand::new();
    let _: f32 = rng.gen();

    // Select the next sample randomly.
    let mut next_value = move || rng.gen();
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
        )
        .expect("can't build output stream");
    stream.play().expect("can't play");

    std::thread::sleep(std::time::Duration::from_millis(2300));
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let value: T = cpal::Sample::from::<f32>(&next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
