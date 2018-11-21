use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{tl_types::TLType, utils::MyResult};

const TL_VECTOR_ID: u32 = 0x1cb5_c415;

impl<T: TLType> TLType for Vec<T> {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let id_code = input.read_u32::<LittleEndian>()?;
        assert_eq!(TL_VECTOR_ID, id_code);
        let length = input.read_i32::<LittleEndian>()?;
        assert_eq!(true, length >= 0);
        let mut result: Vec<T> = Vec::with_capacity(length as usize);
        for _ in 0..length {
            let item = T::tl_read(input)?;
            result.push(item);
        }
        Ok(result)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_u32::<LittleEndian>(TL_VECTOR_ID)?;
        output.write_u32::<LittleEndian>(self.len() as u32)?;
        let mut size = 8usize;
        for i in self {
            size += i.tl_write(output)?;
        }
        Ok(size)
    }
}

#[test]
fn test_read_tl_vector_int() {
    use std::io::Cursor;
    let buffer = [
        0x15, 0xc4, 0xb5, 0x1c, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x03, 0x00, 0x00, 0x00,
    ];
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(vec![1, 2, 3], Vec::<i32>::tl_read(&mut cursor).unwrap())
}

#[test]
fn test_write_tl_vector_int() {
    let data = vec![1i32, 2, 3];
    let mut buffer = vec![];
    assert_eq!(20, data.tl_write(&mut buffer).unwrap());
    assert_eq!(
        buffer,
        vec![
            0x15, 0xc4, 0xb5, 0x1c, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
        ]
    )
}
