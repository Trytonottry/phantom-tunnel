use tokio::time::{sleep, Duration};
use rand::Rng;

pub async fn youtube_jitter() {
    let mut rng = rand::thread_rng();
    sleep(Duration::from_millis(rng.gen_range(200..800))).await;
}

pub async fn google_jitter() {
    sleep(Duration::from_millis(100)).await;
}