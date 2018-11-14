use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};

pub trait TlWriteBytes {
    fn write_u8(&mut self, n: u8) -> io::Result<()>;
    fn write_u24(&mut self, n: u32) -> io::Result<()>;
    fn write_i32(&mut self, n: i32) -> io::Result<()>;
    fn write_u32(&mut self, n: u32) -> io::Result<()>;
    fn write_i64(&mut self, n: i64) -> io::Result<()>;
    fn write_u64(&mut self, n: u64) -> io::Result<()>;
    fn write_f64(&mut self, n: f64) -> io::Result<()>;
    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()>;
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

    fn write_i32(&mut self, n: i32) -> io::Result<()> {
        <Self as WriteBytesExt>::write_i32::<LittleEndian>(self, n)
    }

    #[inline]
    fn write_u32(&mut self, n: u32) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u32::<LittleEndian>(self, n)
    }

    fn write_i64(&mut self, n: i64) -> io::Result<()> {
        <Self as WriteBytesExt>::write_i64::<LittleEndian>(self, n)
    }

    #[inline]
    fn write_u64(&mut self, n: u64) -> io::Result<()> {
        <Self as WriteBytesExt>::write_u64::<LittleEndian>(self, n)
    }

    fn write_f64(&mut self, n: f64) -> io::Result<()> {
        <Self as WriteBytesExt>::write_f64::<LittleEndian>(self, n)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        <Self as Write>::write_all(self, bytes)
    }
}
