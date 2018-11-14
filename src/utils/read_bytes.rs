use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub trait TlReadBytes: ReadBytesExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u24(&mut self) -> io::Result<u32>;
    fn read_i32(&mut self) -> io::Result<i32>;
    fn read_u31(&mut self) -> io::Result<u32>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_i64(&mut self) -> io::Result<i64>;
    fn read_u63(&mut self) -> io::Result<u64>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_f64(&mut self) -> io::Result<f64>;
    fn read_8_bytes(&mut self) -> io::Result<[u8; 8]>;
    fn read_16_bytes(&mut self) -> io::Result<[u8; 16]>;
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

    fn read_i32(&mut self) -> io::Result<i32> {
        <Self as ReadBytesExt>::read_i32::<LittleEndian>(self)
    }

    fn read_u31(&mut self) -> io::Result<u32> {
        let result = <Self as TlReadBytes>::read_i32(self)?;
        if result < 0 {
            unreachable!()
        } else {
            Ok(result as u32)
        }
    }

    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        <Self as ReadBytesExt>::read_u32::<LittleEndian>(self)
    }

    fn read_i64(&mut self) -> io::Result<i64> {
        <Self as ReadBytesExt>::read_i64::<LittleEndian>(self)
    }

    fn read_u63(&mut self) -> io::Result<u64> {
        let result = <Self as TlReadBytes>::read_i64(self)?;
        if result < 0 {
            unreachable!()
        } else {
            Ok(result as u64)
        }
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        <Self as ReadBytesExt>::read_u64::<LittleEndian>(self)
    }

    fn read_f64(&mut self) -> io::Result<f64> {
        <Self as ReadBytesExt>::read_f64::<LittleEndian>(self)
    }

    fn read_8_bytes(&mut self) -> io::Result<[u8; 8]> {
        let mut result = [0u8; 8];
        self.read_exact(&mut result)?;
        Ok(result)
    }

    #[inline]
    fn read_16_bytes(&mut self) -> io::Result<[u8; 16]> {
        let mut result = [0u8; 16];
        self.read_exact(&mut result)?;
        Ok(result)
    }
}
