use noiserand::NoiseRand;
use rand::Rng;

fn main() {
    env_logger::init();

    let mut rng = NoiseRand::new();
    let x: u32 = rng.gen();
    println!("{}", x);
    println!("{:?}", rng.gen::<(f64, bool)>());
}
