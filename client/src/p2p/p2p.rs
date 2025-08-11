use libp2p::{
    futures::StreamExt,
    identity, noise, tcp, yamux, PeerId, Swarm
};
use tokio;

#[tokio::main]
async fn spawn_p2p_client() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = identity::ed25519::Keypair::generate();
    let local_pub_key = local_key.public();
    let transport = tcp::async_io::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_pub_key.into())?)
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed();

    let mut swarm = Swarm::new(transport, DummyBehaviour, PeerId::from(local_pub_key.clone()));

    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse()?)?;

    // Подключиться к публичному узлу-мосту
    let remote: Multiaddr = "/ip4/192.168.1.10/tcp/4001/p2p/QmFoo".parse()?;
    swarm.dial(remote)?;

    loop {
        swarm.next().await;
    }
}

struct DummyBehaviour;