use crate::protocol::{PacketReader, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration
pub struct ConfigurationHandler;

impl PacketHandler for ConfigurationHandler {
    fn handle_c2s(_reader: &mut PacketReader, _id: i32, _state: &mut ConnectionState) {
        
    }

    fn handle_s2c(_reader: &mut PacketReader, _id: i32, _state: &mut ConnectionState) {
        
    }
}

pub mod packets {
    use super::*;
    use crate::protocol::varint::*;
    
}
