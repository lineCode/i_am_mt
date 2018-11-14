use std::io::Cursor;

use crate::{
    tl_types::TLType,
    utils::{read_bytes::TlReadBytes, MyResult, TlWriteBytes},
};

#[derive(Debug, PartialEq)]
pub struct TLLong(i64);

impl TLType for TLLong {
    fn read(input: &mut Cursor<&[u8]>) -> MyResult<Self> {
        Ok(input.read_i64().map(TLLong)?)
    }

    fn write(&self, output: &mut TlWriteBytes) -> MyResult<usize> {
        output.write_i64(self.0)?;
        Ok(8)
    }
}

impl TLLong {
    pub fn as_i64(&self) -> i64 {
        self.0
    }

    pub fn as_u63(&self) -> u64 {
        self.0 as u64
    }
}

#[test]
fn test_read_tl_long() {
    let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut cursor = Cursor::new(buffer.as_ref());
    assert_eq!(
        0x0807060504030201u64,
        TLLong::read(&mut cursor).unwrap().as_u63()
    );
}

#[test]
fn test_write_tl_long() {
    let data = TLLong(0x0807060504030201i64);
    let mut buffer = vec![];
    assert_eq!(8, data.write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
