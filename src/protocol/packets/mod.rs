mod handshaking;
mod login;
mod status;
mod play;

pub use handshaking::*;
pub use status::*;
pub use login::*;
pub use play::*;

use crate::protocol::{Direction, PacketReader};

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
    fn decode(reader: &mut PacketReader) -> Option<Self> where Self: Sized { None }
    fn encode(&self, buf: &mut Vec<u8>) {}
}

pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

pub trait PacketHandler {
    fn handle(reader: &mut PacketReader, dir: &Direction, id: i32, state: &mut ConnectionState);
}
