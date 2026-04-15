use mc_proxy::{MinecraftProxy, protocol};
use mc_proxy::protocol::listener::{PacketListener, PacketAction};

struct ExampleProxy;
impl ExampleProxy {
    fn new() -> Self {
        Self
    }
}

impl PacketListener for ExampleProxy {
    fn on_handshake(&mut self, p: &mut protocol::packets::Handshake) -> PacketAction {
        println!("Handshake packet:\n Server Address: {}\n Port: {}\n Protocol Version: {}\n Next State: {:?}",
            p.server_address,
            p.server_port,
            p.protocol_version,
            p.next_state
        );
        return PacketAction::Allow
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let test = ExampleProxy::new();

    let proxy = MinecraftProxy::new(1243, "127.0.0.1:25565", test);
    
    proxy.run().await
}
