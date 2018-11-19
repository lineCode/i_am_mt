use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{tl_types::TLType, utils::MyResult};

impl TLType for u32 {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        Ok(input.read_u32::<LittleEndian>()?)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_u32::<LittleEndian>(self.clone())?;
        Ok(4)
    }
}

#[test]
fn test_read_tl_u32() {
    use std::io::Cursor;

    let buffer = [1u8, 2, 3, 4];
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(0x04030201u32, u32::tl_read(&mut cursor).unwrap());
}

#[test]
fn test_write_tl_i32() {
    let data = 0x04030201u32;
    let mut buffer = vec![];
    assert_eq!(4, data.tl_write(&mut buffer).unwrap());
    assert_eq!(buffer, vec![1, 2, 3, 4]);
}
