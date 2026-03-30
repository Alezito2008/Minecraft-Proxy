use crate::protocol::*;
use crate::protocol::packets::*;

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
            next_state: match reader.read_varint()? {
                1 => ConnectionState::Status,
                2 => ConnectionState::Login,
                3 => ConnectionState::Transfer,
                _ => {
                    println!("WARNING: Unkown connection state received.");
                    ConnectionState::Unknown
                }
            },
        })
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        let state_int: i32 = match self.next_state {
            ConnectionState::Status => 1,
            ConnectionState::Login => 2,
            ConnectionState::Transfer => 3,
            _ => return
        };

        write_varint(self.protocol_version, buf);
        write_string(&self.server_address, buf);
        write_ushort(self.server_port, buf);
        write_varint(state_int, buf);
    }
}
