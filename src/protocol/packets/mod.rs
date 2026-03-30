mod handshaking;
mod login;
mod status;
mod play;

pub use handshaking::*;
use crate::protocol::PacketReader;

#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Transfer,
    Configuration,
    Play,
    Unknown,
}

#[allow(unused)]
pub trait MinecraftPacket {
    const ID: i32;
    fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized;
    fn encode(&self, buf: &mut Vec<u8>) {}
}

pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

