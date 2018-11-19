pub trait IntFromIntoBytes<Bytes>: Sized {
    fn into_le_bytes(self) -> Bytes {
        Self::into_bytes::<byteorder::LittleEndian>(self)
    }
    fn into_be_bytes(self) -> Bytes {
        Self::into_bytes::<byteorder::BigEndian>(self)
    }
    fn into_bytes<B: byteorder::ByteOrder>(self) -> Bytes;
    fn from_le_bytes(bytes: Bytes) -> Self {
        Self::from_bytes::<byteorder::LittleEndian>(bytes)
    }
    fn from_be_bytes(bytes: Bytes) -> Self {
        Self::from_bytes::<byteorder::BigEndian>(bytes)
    }
    fn from_bytes<B: byteorder::ByteOrder>(bytes: Bytes) -> Self;
}

impl IntFromIntoBytes<[u8; 4]> for u32 {
    fn into_bytes<B: byteorder::ByteOrder>(self) -> [u8; 4] {
        let mut result = [0u8; 4];
        B::write_u32(&mut result, self);
        result
    }

    fn from_bytes<B: byteorder::ByteOrder>(bytes: [u8; 4]) -> Self {
        B::read_u32(&bytes)
    }
}

impl IntFromIntoBytes<[u8; 8]> for u64 {
    fn into_bytes<B: byteorder::ByteOrder>(self) -> [u8; 8] {
        let mut result = [0u8; 8];
        B::write_u64(&mut result, self);
        result
    }

    fn from_bytes<B: byteorder::ByteOrder>(bytes: [u8; 8]) -> Self {
        B::read_u64(&bytes)
    }
}

impl IntFromIntoBytes<[u8; 16]> for u128 {
    fn into_bytes<B: byteorder::ByteOrder>(self) -> [u8; 16] {
        let mut result = [0u8; 16];
        B::write_u128(&mut result, self);
        result
    }

    fn from_bytes<B: byteorder::ByteOrder>(bytes: [u8; 16]) -> Self {
        B::read_u128(&bytes)
    }
}
