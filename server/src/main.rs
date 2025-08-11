use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tungstenite::accept;
use std::fs;
use native_tls::TlsAcceptor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    server: String,
    sni: String,
    host: String,
    local_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert_pem = fs::read("certs/cert.pem")?;
    let key_pem = fs::read("certs/key.pem")?;
    let identity = native_tls::Identity::from_pkcs8(&cert_pem, &key_pem)?;
    let acceptor = TlsAcceptor::new(identity)?;
    let listener = TcpListener::bind("0.0.0.0:8443").await?;

    println!("Phantom Server запущен на wss://0.0.0.0:8443");

    while let Ok((stream, _)) = listener.accept().await {
        let acceptor = acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("TLS ошибка: {}", e);
                    return;
                }
            };

            let ws_stream = match accept(tls_stream) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("WebSocket ошибка: {}", e);
                    return;
                }
            };

            handle_client(ws_stream).await;
        });
    }

    Ok(())
}

async fn handle_client(mut ws_stream: tungstenite::WebSocketStream<tokio_native_tls::TlsStream<tokio::net::TcpStream>>) {
    use tungstenite::Message;

    loop {
        let msg = match ws_stream.next().await {
            Some(Ok(m)) => m,
            _ => break,
        };

        if let Message::Binary(data) = msg {
            // Расшифровка (в PoC — пропускаем, в реальности: AES-GCM)
            // Далее — отправка в интернет (реализация прокси — упрощена)
            // Это PoC: просто эхо (для демонстрации)
            if let Ok(mut out_stream) = tokio::net::TcpStream::connect("1.1.1.1:53").await {
                out_stream.write_all(&data).await.ok();
                let mut buf = vec![0u8; 1024];
                if let Ok(n) = out_stream.read(&mut buf).await {
                    let _ = ws_stream.send(Message::Binary(buf[..n].to_vec())).await;
                }
            }
        }
    }
}