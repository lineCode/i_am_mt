use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{tl_types::TLType, utils::MyResult};

const DIVIDING_SIZE: u8 = 254;

const fn padding_size(current: u64, step: u64) -> u64 {
    (step - (current % step)) % step
}

#[derive(Debug, PartialEq)]
pub struct TLString(Vec<u8>);

impl TLString {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_ref()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let _ = TLStringType::new_by_size(bytes.len()); // Check length
        TLString(bytes)
    }
}

impl TLType for TLString {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let first_byte = input.read_u8()?;
        let string_type = TLStringType::new_by_first_byte(first_byte);
        let length = match string_type {
            TLStringType::Short => first_byte as usize,
            TLStringType::Long => input.read_u24::<LittleEndian>()? as usize,
        };
        let mut result = vec![0u8; length];
        input.read_exact(&mut result)?;

        let padding_size = padding_size((length + string_type.length()) as u64, 4);
        for _ in 0..padding_size {
            let padding_byte = input.read_u8()?;
            assert_eq!(0, padding_byte);
        }

        Ok(TLString(result))
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        let length = self.0.len();
        let string_type = TLStringType::new_by_size(length);

        match string_type {
            TLStringType::Short => {
                output.write_u8(length as u8)?;
            }
            TLStringType::Long => {
                output.write_u8(DIVIDING_SIZE)?;
                output.write_u24::<LittleEndian>(length as u32)?;
            }
        };

        output.write_all(self.0.as_ref())?;

        let padding_size = padding_size((string_type.length() + length) as u64, 4) as usize;
        for _ in 0..padding_size {
            output.write_u8(0)?;
        }

        Ok(padding_size + string_type.length() + length)
    }
}

#[test]
fn test_read_tl_string() {
    use std::io::Cursor;

    let buffer = [8, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', 0, 0, 0];
    let mut cursor = Cursor::new(&buffer);
    assert_eq!(
        b"12345678",
        TLString::tl_read(&mut cursor).unwrap().as_bytes()
    );
}

#[test]
fn test_write_tl_string() {
    let data = TLString(b"12345678".to_vec());
    let mut buffer = vec![];
    assert_eq!(12, data.tl_write(&mut buffer).unwrap());
    assert_eq!(
        buffer,
        vec![8, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', 0, 0, 0]
    );
}

enum TLStringType {
    Short,
    Long,
}

impl TLStringType {
    fn new_by_size(length: usize) -> Self {
        if length < DIVIDING_SIZE as usize {
            TLStringType::Short
        } else if length < 2usize.pow(24) {
            TLStringType::Long
        } else {
            unreachable!()
        }
    }

    fn new_by_first_byte(lead_byte: u8) -> Self {
        if lead_byte < DIVIDING_SIZE {
            TLStringType::Short
        } else if lead_byte == DIVIDING_SIZE {
            TLStringType::Long
        } else {
            unreachable!()
        }
    }

    fn length(&self) -> usize {
        match self {
            TLStringType::Short => 1,
            TLStringType::Long => 4,
        }
    }
}
