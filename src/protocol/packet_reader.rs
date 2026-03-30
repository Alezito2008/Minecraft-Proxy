use crate::protocol::{read_string, read_ushort, read_varint};

pub struct PacketReader<'a> {
    data: &'a Vec<u8>,
    offset: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        Self {
            data: data,
            offset: 0
        }
    }

    pub fn read_varint(&mut self) -> Option<i32> {
        let (value, size) = read_varint(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_string(&mut self) -> Option<String> {
        let (value, size) = read_string(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_ushort(&mut self) -> Option<u16> {
        let (value, size) = read_ushort(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }
}
