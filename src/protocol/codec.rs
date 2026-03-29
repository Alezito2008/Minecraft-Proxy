use crate::protocol::{Direction, FilterResult, Packet, PacketReader};
use crate::protocol::{read_string, read_ushort, read_varint};

pub fn inspect_packet(buffer: &mut Vec<u8>, dir: &Direction) -> FilterResult {
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
        data: raw_packet[len_size + id_size..].to_vec()
    };

    if packet_id == 0x00 && matches!(dir, Direction::ClientToServer) {
        let mut packet_reader = PacketReader::new(&packet.data);
        let _ = (|| -> Option<()> {
            let protocol_version =  packet_reader.read_varint()?;
            let server_addr = packet_reader.read_string()?;
            let server_port = packet_reader.read_ushort()?;
            let intent = packet_reader.read_varint()?;

            println!("Protocol Version: {protocol_version} | Server: {server_addr} | Port: {server_port} | Intent: {intent}");
            return Some(());
        })();
    }

    FilterResult::Send(raw_packet)
}