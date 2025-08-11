use quinn::{ClientConfig, Endpoint, TransportConfig};
use rustls::{ClientConfig as RustlsClientConfig, RootCertStore};
use rustls_pemfile as pem;
use std::sync::Arc;

pub async fn connect_quic_ech(
    domain: &str,
    ech_config: &[u8],
) -> Result<quinn::Connection, Box<dyn std::error::Error>> {
    let mut root_store = RootCertStore::empty();
    root_store.add_parsable_certificates(&pem::certs(&mut std::io::BufReader::new(std::fs::File::open("certs/cert.pem")?))?);

    let mut rustls_config = RustlsClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    // Установка ECH
    rustls_config.alpn_protocols = vec![b"hq-29".to_vec()];
    // ECH: в rustls — экспериментально, нужно передавать ech_config

    let mut client_config = ClientConfig::new(Arc::new(rustls_config));
    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(30_000_u64.into()));
    client_config.transport_config(Arc::new(transport_config));

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    let connection = endpoint.connect((domain, 443), "ech.phantom.example")?.await?;
    Ok(connection)
}