use libp2p::{
    kad::{
        store::MemoryStore, GetRecordOk, Record,
    },
    Multiaddr, PeerId, Swarm,
};
use tokio::time::{sleep, Duration};

pub async fn discover_relays(
    bootstrap: Vec<(PeerId, Multiaddr)>,
) -> Result<Vec<PeerId>, Box<dyn std::error::Error>> {
    use libp2p::SwarmBuilder;
    use libp2p::identity::ed25519::Keypair;
    use libp2p::noise;
    use libp2p::tcp;
    use libp2p::yamux;
    use libp2p::kad;

    let local_keys = Keypair::generate();
    let local_pub_key = local_keys.public();
    let local_peer_id = PeerId::from(local_pub_key.clone());

    let transport = tcp::async_io::Transport::default()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(noise::Config::new(&local_pub_key.into())?)
        .multiplex(yamux::Config::default())
        .boxed();

    let mut swarm = SwarmBuilder::new(transport, DummyBehaviour::default(), local_peer_id)
        .build();

    for (peer_id, addr) in bootstrap {
        swarm.behaviour_mut().kademlia.add_address(&peer_id, addr);
    }

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    sleep(Duration::from_secs(2)).await;

    let mut found_peers = vec![];
    swarm
        .behaviour_mut()
        .kademlia
        .get_record(&"relay-node".into());

    // Обработка ответов
    while let Some(event) = swarm.next().now_or_never() {
        if let Some(peer_id) = extract_peer_from_event(event) {
            found_peers.push(peer_id);
        }
    }

    Ok(found_peers)
}

#[derive(libp2p::NetworkBehaviour)]
#[behaviour(out_event = "DhtEvent")]
struct DummyBehaviour {
    kademlia: kad::Behaviour<MemoryStore>,
}

impl Default for DummyBehaviour {
    fn default() -> Self {
        let local_key = libp2p::identity::ed25519::Keypair::generate();
        let local_peer_id = PeerId::from(local_key.public());
        let store = MemoryStore::new(local_peer_id);
        Self {
            kademlia: kad::Behaviour::new(local_peer_id, store),
        }
    }
}