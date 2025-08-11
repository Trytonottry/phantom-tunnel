use rand::Rng;

pub fn add_noise( Vec<u8>, min: usize, max: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let pad_len = rng.gen_range(min..=max);
    let mut padded = data;
    for _ in 0..pad_len {
        padded.push(rng.gen());
    }
    padded
}