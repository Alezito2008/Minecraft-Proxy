use crate::protocol::{ConnectionState, Direction, FilterResult, PacketReader};
use crate::protocol::packets::{MinecraftPacket, Packet, Handshake};
use crate::protocol::{read_varint};

pub fn inspect_packet(
    buffer: &mut Vec<u8>,
    dir: &Direction,
    state: &mut ConnectionState
) -> FilterResult {
    // Leer largo total del paquete
    let (total_length, len_size) = match read_varint(&buffer) {
        Some(v) => v,
        None => return FilterResult::Incomplete
    };

    let total_packet_size = len_size + total_length as usize;
    if buffer.len() < total_packet_size {
        return FilterResult::Incomplete
    }

    let raw_packet = buffer.drain(..total_packet_size).collect::<Vec<u8>>();
    let (packet_id, id_size) = match read_varint(&raw_packet[len_size..]) {
        Some(v) => v,
        None => return FilterResult::Cancel // broken
    };

    let packet = Packet {
        id: packet_id,
        data: raw_packet[len_size + id_size..].to_vec(),
    };

    let mut reader = PacketReader::new(&packet.data);

    match (*state, packet.id, dir) {
        (ConnectionState::Handshaking, Handshake::ID, Direction::ClientToServer) => {
            if let Some(handshake) = Handshake::decode(&mut reader) {
                println!("Host: {}, Protocol: {}, Port: {}, Next: {:?}", handshake.server_address, handshake.protocol_version, handshake.server_port, handshake.next_state);
                *state = handshake.next_state;
            }
        }
        _ => {}
    }

    FilterResult::Send(raw_packet)
}