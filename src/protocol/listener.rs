use crate::protocol::packets::*;

pub enum PacketAction {
    Allow,
    Cancel,
}

macro_rules! add_listeners {
    ($($name:ident: $packet_type:ty),+$(,)?) => {
        $(
            fn $name (&mut self, _p: &mut $packet_type) -> PacketAction { PacketAction::Allow }
        )+
    };
}

pub trait PacketListener {
    add_listeners!(
        on_handshake: Handshake,
        on_login_start: LoginStart,
        on_encription_response: EncryptionResponse,
        on_login_acknowledged: LoginAcknowledged,
        on_encryption_request: EncryptionRequest,
        on_set_compression: SetCompression,
        on_login_success: LoginSuccess,
        on_status_request: StatusRequest,
        on_ping_packet_request: PingPacket,
        on_ping_packet_response: PingPacket,
        on_status_response: StatusResponse,
        on_chat_command: ChatCommand,
        on_command_suggestion_request: CommandSuggestionRequest,
        on_acknowledge_configuration: AcknowledgeConfiguration,
        on_start_configuration: StartConfiguration,
        on_spawn_entity: SpawnEntity,
        on_entity_position_sync: EntityPositionSync,
        on_update_entity_position: UpdateEntityPosition,
        on_acknowledge_finish_configuration: AcknowledgeFinishConfiguration,
        on_finish_configuration: FinishConfiguration,
    );
}