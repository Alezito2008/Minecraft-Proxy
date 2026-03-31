use crate::protocol::{PacketReader, ConnectionState};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration
pub struct ConfigurationHandler;

impl PacketHandler for ConfigurationHandler {
    fn handle_c2s(_reader: &mut PacketReader, id: i32, state: &mut ConnectionState) {
        match id {
            AcknowledgeFinishConfiguration::ID => {
                println!("Acknowledge finish configuration");
                *state = ConnectionState::Play;
            }
            _ => {}
        }
    }

    fn handle_s2c(_reader: &mut PacketReader, id: i32, _state: &mut ConnectionState) {
        match id {
            FinishConfiguration::ID => {
                println!("Finish configuration");
            }
            _ => {}
        }
    }
}

pub mod packets {
    use super::*;
    
    pub struct FinishConfiguration;

    impl MinecraftPacket for FinishConfiguration {
        const ID: i32 = 0x03;
    }

    pub struct AcknowledgeFinishConfiguration;

    impl MinecraftPacket for AcknowledgeFinishConfiguration {
        const ID: i32 = 0x03;
    }
}
