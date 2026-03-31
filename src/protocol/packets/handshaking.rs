use crate::protocol::{PacketReader, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Handshaking
pub struct HandshakeHandler;
impl PacketHandler for HandshakeHandler {
    fn handle_c2s(
        reader: &mut PacketReader,
        id: i32,
        state: &mut ConnectionState
    ) {
        match id {
            Handshake::ID => {
                if let Some(handshake) = Handshake::decode(reader) {
                    println!("Handshake: Host: {}, Protocol: {}, Port: {}, Intent: {:?}", handshake.server_address, handshake.protocol_version, handshake.server_port, handshake.next_state);
                    *state = handshake.next_state;
                }
            },
            _ => {}
        }
    }
}

pub mod packets {
    use super::*;
    use crate::protocol::varint::*;

    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: ConnectionState,
    }
    
    impl MinecraftPacket for Handshake {
        const ID: i32 = 0x0;
    
        fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
            Some(Self {
                protocol_version: reader.read_varint()?,
                server_address: reader.read_string()?,
                server_port: reader.read_ushort()?,
                next_state: ConnectionState::from(reader.read_varint()?)
            })
        }
    
        fn encode(&self, buf: &mut Vec<u8>) {
            write_varint(self.protocol_version, buf);
            write_string(&self.server_address, buf);
            write_ushort(self.server_port, buf);
            write_varint(self.next_state.into(), buf);
        }
    }
}

