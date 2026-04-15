use crate::impl_packet_handler;
use crate::protocol::listener::{PacketAction, PacketListener};
use crate::protocol::{ConnectionState, PacketReader, Session};
use crate::protocol::packets::{MinecraftPacket, PacketHandler};
use self::packets::*;

// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Finish_Configuration
impl_packet_handler!(ConfigurationHandler, packet, session,
    c2s => {
        AcknowledgeFinishConfiguration => on_acknowledge_finish_configuration, { session.state = ConnectionState::Play }
    }
    s2c => {
        FinishConfiguration => on_finish_configuration
    }
);

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
