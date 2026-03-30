use crate::protocol::{ConnectionState, Direction, PacketReader};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use crate::protocol::varint::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Status
// https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping
pub struct StatusHandler;
impl PacketHandler for StatusHandler {
    fn handle_c2s(reader: &mut PacketReader, id: i32, _state: &mut ConnectionState) {
        match id {
            StatusRequest::ID => {
                println!("Server Status Requested");
            }
            PingPacket::ID => {
                if let Some(ping_request) = PingPacket::decode(reader) {
                    println!("Sent ping request with payload: {}", ping_request.payload);
                }
            }
            _ => {}
        }
    }

    fn handle_s2c(reader: &mut PacketReader, id: i32, _state: &mut ConnectionState) {
        match id {
            StatusResponse::ID => {
                if let Some(status_response) = StatusResponse::decode(reader) {
                    println!("Status Response: {}", status_response.json_response);
                }
            }
            PingPacket::ID => {
                if let Some(ping_request) = PingPacket::decode(reader) {
                    println!("Received pong response with payload: {}", ping_request.payload);
                }
            }
            _ => {}
        }
    }
}

pub struct StatusRequest;

impl MinecraftPacket for StatusRequest {
    const ID: i32 = 0x00;
}

pub struct StatusResponse {
    pub json_response: String
}

impl MinecraftPacket for StatusResponse {
    const ID: i32 = 0x00;

    fn decode(reader: &mut crate::protocol::PacketReader) -> Option<Self> where Self: Sized {
        Some(Self {
            json_response: reader.read_string()?
        })
    }
}

pub struct PingPacket {
    pub payload: i64,
}

impl MinecraftPacket for PingPacket {
    const ID: i32 = 0x01;

    fn decode(reader: &mut crate::protocol::PacketReader) -> Option<Self> where Self: Sized {
        Some(Self {
            payload: reader.read_long()?
        })
    }

    fn encode(&self, buf: &mut Vec<u8>) {
        write_varlong(self.payload, buf);
    }
}
