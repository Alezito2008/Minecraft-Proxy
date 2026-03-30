use crate::protocol::{Direction, PacketReader, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use crate::protocol::varint::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Login
pub struct LoginHandler;
impl PacketHandler for LoginHandler {
    fn handle_c2s(reader: &mut PacketReader, id: i32, state: &mut ConnectionState) {
        
    }

    fn handle_s2c(reader: &mut PacketReader, id: i32, state: &mut ConnectionState) {
        
    }
}