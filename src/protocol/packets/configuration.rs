use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{ConnectionState, PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration
pub struct ConfigurationHandler;

impl PacketHandler for ConfigurationHandler {
    fn handle_c2s<L: crate::protocol::listener::PacketListener>(
            _reader: &mut PacketReader,
            id: i32,
            session: &mut Session,
            listener: &mut L
        ) -> crate::protocol::listener::PacketAction {
        match id {
            AcknowledgeFinishConfiguration::ID => {
                println!("Acknowledge finish configuration");
                session.state = ConnectionState::Play;
                return listener.on_acknowledge_finish_configuration(&mut AcknowledgeFinishConfiguration);
            }
            _ => {}
        }

        PacketAction::Allow
    }

    fn handle_s2c<L: PacketListener>(
            _reader: &mut PacketReader,
            id: i32,
            _session: &mut Session,
            listener: &mut L
        ) -> PacketAction {
        match id {
            FinishConfiguration::ID => {
                println!("Finish configuration");
                return listener.on_finish_configuration(&mut FinishConfiguration);
            }
            _ => {}
        }

        PacketAction::Allow
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
