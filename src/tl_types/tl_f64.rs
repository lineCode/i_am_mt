use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{tl_types::TLType, utils::MyResult};

impl TLType for f64 {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        Ok(input.read_f64::<byteorder::LittleEndian>()?)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_f64::<byteorder::LittleEndian>(self.clone())?;
        Ok(8)
    }
}

#[test]
fn test_read_f64() {
    use std::io::Cursor;

    let buffer = [24u8, 45, 68, 84, 251, 33, 9, 64];
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(std::f64::consts::PI, f64::tl_read(&mut cursor).unwrap());
}

#[test]
fn test_write_tl_f64() {
    let data = std::f64::consts::PI;
    let mut buffer = vec![];
    assert_eq!(8, data.tl_write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![24, 45, 68, 84, 251, 33, 9, 64]);
}
