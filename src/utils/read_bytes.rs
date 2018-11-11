use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait TlReadBytes {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u24(&mut self) -> io::Result<u32>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_u64(&mut self) -> io::Result<u64>;
}

impl<T: ReadBytesExt> TlReadBytes for T {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        <Self as ReadBytesExt>::read_u8(self)
    }

    #[inline]
    fn read_u24(&mut self) -> io::Result<u32> {
        <Self as ReadBytesExt>::read_u24::<LittleEndian>(self)
    }

    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        <Self as ReadBytesExt>::read_u32::<LittleEndian>(self)
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        <Self as ReadBytesExt>::read_u64::<LittleEndian>(self)
    }
}
