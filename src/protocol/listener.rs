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
    );
}