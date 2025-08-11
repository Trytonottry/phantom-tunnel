use tokio::time::{sleep, Duration};
use rand::Rng;

pub async fn google_meet_traffic() {
    let mut rng = rand::thread_rng();
    loop {
        let size = rng.gen_range(800..1200);
        send_dummy_packet(size).await;
        sleep(Duration::from_millis(20)).await; // 50 пакетов/сек
    }
}