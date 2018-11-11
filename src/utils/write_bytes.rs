use std::io;

use byteorder::{LittleEndian, WriteBytesExt};

pub trait TlWriteBytes {
    fn write_u8(&mut self, n: u8) -> io::Result<()>;
    fn write_u24(&mut self, n: u32) -> io::Result<()>;
    fn write_u32(&mut self, n: u32) -> io::Result<()>;
    fn write_u64(&mut self, n: u64) -> io::Result<()>;
}

impl<T: WriteBytesExt> TlWriteBytes for T {
    #[inline]
    fn write_u8(&mut self, n: u8) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u8(self, n)
    }

    #[inline]
    fn write_u24(&mut self, n: u32) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u24::<LittleEndian>(self, n)
    }

    #[inline]
    fn write_u32(&mut self, n: u32) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u32::<LittleEndian>(self, n)
    }

    #[inline]
    fn write_u64(&mut self, n: u64) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u64::<LittleEndian>(self, n)
    }
}
