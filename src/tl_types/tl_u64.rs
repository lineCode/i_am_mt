use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{tl_types::TLType, utils::MyResult};

impl TLType for u64 {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        Ok(input.read_u64::<LittleEndian>()?)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_u64::<LittleEndian>(self.clone())?;
        Ok(8)
    }
}

#[test]
fn test_read_tl_i64() {
    use std::io::Cursor;

    let buffer = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(0x0807060504030201u64, u64::tl_read(&mut cursor).unwrap());
}

#[test]
fn test_write_tl_long() {
    let data = 0x0807060504030201u64;
    let mut buffer = vec![];
    assert_eq!(8, data.tl_write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}
