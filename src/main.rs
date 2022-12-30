use anyhow::Result;
use libp2p::{identity, PeerId};
use libp2p::ping;

#[tokio::main]
async fn main() -> Result<()> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("local peer id: {:?}", local_peer_id);

    let transport = libp2p::tokio_development_transport(local_key)?;

    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));

    Ok(())
}
