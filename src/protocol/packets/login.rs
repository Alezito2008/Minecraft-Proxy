use crate::protocol::{Direction, PacketReader, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use crate::protocol::varint::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login
pub struct LoginHandler;
impl PacketHandler for LoginHandler {
    fn handle_c2s(reader: &mut PacketReader, id: i32, state: &mut ConnectionState) {
        match id {
            LoginStart::ID => {
                if let Some(login_start) = LoginStart::decode(reader) {
                    println!("Login start: Name: {}, UUID: {}", login_start.name, login_start.uuid);
                }
            },
            EncryptionResponse::ID => {
                println!("Sent encryption response packet");
            }
            _ => {}
        }
    }

    fn handle_s2c(reader: &mut PacketReader, id: i32, state: &mut ConnectionState) {
        match id {
            EncryptionRequest::ID => {
                println!("Received encryption request packet");
            },
            SetCompression::ID => {
                if let Some(set_compression) = SetCompression::decode(reader) {
                    println!("Set compression threshold: {}", set_compression.threshold);
                }
            }
            _ => {}
        }
    }
}

pub struct LoginStart {
    pub name: String,
    pub uuid: u128,
}

impl MinecraftPacket for LoginStart {
    const ID: i32 = 0x00;

    fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
        Some(Self {
            name: reader.read_string()?,
            uuid: reader.read_uuid()?,
        })
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        write_string(&self.name, buf);
        write_uuid(self.uuid, buf);
    }
}

pub struct EncryptionRequest; // TODO

impl MinecraftPacket for EncryptionRequest {
    const ID: i32 = 0x01;
}

pub struct EncryptionResponse; // TODO

impl MinecraftPacket for EncryptionResponse {
    const ID: i32 = 0x01;
}

pub struct SetCompression {
    pub threshold: i32
}

impl MinecraftPacket for SetCompression {
    const ID: i32 = 0x03;

    fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized {
        Some(Self {
            threshold: reader.read_varint()?,
        })
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        write_varint(self.threshold, buf);
    }
}
