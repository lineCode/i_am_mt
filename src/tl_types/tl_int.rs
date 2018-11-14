use std::io::Cursor;

use crate::utils::{read_bytes::TlReadBytes, MyResult, TlWriteBytes};

use super::TLType;

#[derive(Debug, PartialEq)]
pub struct TLInt(i32);

impl TLType for TLInt {
    fn read(input: &mut Cursor<&[u8]>) -> MyResult<Self> {
        Ok(input.read_i32().map(TLInt)?)
    }

    fn write(&self, output: &mut TlWriteBytes) -> MyResult<usize> {
        output.write_i32(self.0)?;
        Ok(4)
    }
}

impl TLInt {
    pub fn as_i32(&self) -> i32 {
        self.0
    }

    pub fn as_u31(&self) -> u32 {
        self.0 as u32
    }
}

#[test]
fn test_read_tl_int() {
    let buffer = vec![1, 2, 3, 4];
    let mut cursor = Cursor::new(buffer.as_ref());
    assert_eq!(0x04030201u32, TLInt::read(&mut cursor).unwrap().as_u31());
}

#[test]
fn test_write_tl_int() {
    let data = TLInt(0x04030201i32);
    let mut buffer = vec![];
    assert_eq!(4, data.write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![1, 2, 3, 4]);
}
