use crate::protocol::{Direction, FilterResult, Packet, varint::read_varint};

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
    let (packet_id, id_size) = read_varint(&raw_packet[len_size..]).unwrap();

    let _packet = Packet {
        id: packet_id,
        data: raw_packet[len_size + id_size..].to_vec()
    };

    if packet_id < 0x10 && packet_id != 0x0 {
        println!("[{dir}] ID: 0x{:02X}", packet_id);
    }

    FilterResult::Send(raw_packet)
}