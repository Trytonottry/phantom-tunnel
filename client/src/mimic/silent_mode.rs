use tokio::time::{sleep, Duration};
use rand::Rng;

pub async fn youtube_background_traffic() {
    let mut rng = rand::thread_rng();
    loop {
        let packet_size = rng.gen_range(500..2000); // как видео-чанк
        let delay = rng.gen_range(800..2000);       // пауза между чанками

        // Отправляем "фиктивный" трафик
        send_dummy_request(packet_size).await;

        sleep(Duration::from_millis(delay)).await;
    }
}

async fn send_dummy_request(size: usize) {
    let data = vec![0u8; size];
    // Через текущий туннель — но не используется пользователем
    if let Some(active_tunnel) = get_active_tunnel() {
        let _ = active_tunnel.send(data).await;
    }
}