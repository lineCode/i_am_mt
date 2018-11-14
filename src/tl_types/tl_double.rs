use std::io::Cursor;

use crate::utils::{read_bytes::TlReadBytes, MyResult, TlWriteBytes};

use super::TLType;

#[derive(Debug, PartialEq)]
pub struct TLDouble(f64);

impl TLType for TLDouble {
    fn read(input: &mut Cursor<&[u8]>) -> MyResult<Self> {
        Ok(input.read_f64().map(TLDouble)?)
    }

    fn write(&self, output: &mut TlWriteBytes) -> MyResult<usize> {
        output.write_f64(self.0)?;
        Ok(8)
    }
}

impl TLDouble {
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

#[test]
fn test_read_tl_double() {
    let buffer = vec![24, 45, 68, 84, 251, 33, 9, 64];
    let mut cursor = Cursor::new(buffer.as_ref());
    assert_eq!(
        std::f64::consts::PI,
        TLDouble::read(&mut cursor).unwrap().as_f64()
    );
}

#[test]
fn test_write_tl_double() {
    let data = TLDouble(std::f64::consts::PI);
    let mut buffer = vec![];
    assert_eq!(8, data.write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![24, 45, 68, 84, 251, 33, 9, 64]);
}
