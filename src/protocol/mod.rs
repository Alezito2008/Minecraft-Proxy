mod codec;
mod varint;
mod packet_reader;

use core::fmt;

pub use codec::inspect_packet;
pub use varint::{read_string, read_ushort, read_varint};
pub use packet_reader::PacketReader;

pub enum Direction {
    ServerToClient,
    ClientToServer,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::ClientToServer => "Client -> Server",
            Self::ServerToClient => "Server -> Client"
        })
    }
}

pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

pub enum FilterResult {
    Send(Vec<u8>),
    Cancel,
    Incomplete,
}
