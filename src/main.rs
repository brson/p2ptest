use anyhow::Result;
use libp2p::{identity, ping, PeerId, Multiaddr};
use libp2p::swarm::{Swarm, SwarmEvent, NetworkBehaviour};
use libp2p::swarm::keep_alive;
use libp2p::futures::StreamExt;
use void::Void;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent")]
struct MyBehaviour {
    ping: ping::Behaviour,
    keep_alive: keep_alive::Behaviour,
}

#[derive(Debug)]
enum MyBehaviourEvent {
    Ping(ping::Event),
    KeepAlive(Void),
}

impl From<ping::Event> for MyBehaviourEvent {
    fn from(event: ping::Event) -> MyBehaviourEvent {
        MyBehaviourEvent::Ping(event)
    }
}

impl From<Void> for MyBehaviourEvent {
    fn from(event: Void) -> MyBehaviourEvent {
        MyBehaviourEvent::KeepAlive(event)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local peer id: {:?}", local_peer_id);

    let transport = libp2p::tokio_development_transport(local_key)?;

    let behaviour = MyBehaviour {
        ping: ping::Behaviour::new(ping::Config::new()),
        keep_alive: keep_alive::Behaviour::default(),
    };

    let mut swarm = Swarm::with_tokio_executor(transport, behaviour, local_peer_id);

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {}", addr)
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(event) => println!("{:?}", event),
            e => {
                println!("unhandled event: {:#?}", e);
            }
        }
    }
}
