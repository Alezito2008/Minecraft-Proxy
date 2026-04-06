use crate::protocol::varint::*;

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

    pub fn add_padding(&mut self, padding: usize) {
        self.offset += padding;
    }

    pub fn read_varint(&mut self) -> Option<i32> {
        let (value, size) = read_varint(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_varlong(&mut self) -> Option<i64> {
        let (value, size) = read_varlong(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_string(&mut self) -> Option<String> {
        let (value, size) = read_string(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        let (value, size) = read_bool(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_long(&mut self) -> Option<i64> {
        let (value, size) = read_long(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_float(&mut self) -> Option<f32> {
        let (value, size) = read_float(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_double(&mut self) -> Option<f64> {
        let (value, size) = read_double(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_ushort(&mut self) -> Option<u16> {
        let (value, size) = read_ushort(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }

    pub fn read_uuid(&mut self) -> Option<u128> {
        let (value, size) = read_uuid(&self.data[self.offset..])?;
        self.offset += size;
        Some(value)
    }
}
