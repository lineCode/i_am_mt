use std::io::{Cursor, Read};

use crate::utils::{read_bytes::TlReadBytes, MyResult, TlWriteBytes};

use super::TLType;

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
}

impl TLType for TLString {
    fn read(input: &mut Cursor<&[u8]>) -> MyResult<Self> {
        let begin_position = input.position();
        let first_bytes = input.read_u8()?;

        let string_type = TLStringType::new_by_lead_byte(first_bytes);
        let length = match string_type {
            TLStringType::Short => first_bytes as usize,
            TLStringType::Long => input.read_u24()? as usize,
        };
        let mut result = vec![0u8; length];
        input.read_exact(&mut result)?;

        let current_position = input.position();
        let padding_size = padding_size(current_position - begin_position, 4);
        for _ in 0..padding_size {
            let padding_byte = input.read_u8()?;
            assert_eq!(0, padding_byte);
        }

        Ok(TLString(result))
    }

    fn write(&self, output: &mut TlWriteBytes) -> MyResult<usize> {
        let length = self.0.len();
        let string_type = TLStringType::new(length);

        match string_type {
            TLStringType::Short => {
                output.write_u8(length as u8)?;
            }
            TLStringType::Long => {
                output.write_u8(DIVIDING_SIZE)?;
                output.write_u24(length as u32)?;
            }
        };

        output.write_bytes(self.0.as_ref())?;

        let padding_size = padding_size((string_type.size() + length) as u64, 4) as usize;
        for _ in 0..padding_size {
            output.write_u8(0)?;
        }

        Ok(padding_size + string_type.size() + length)
    }
}

#[test]
fn test_read_tl_string() {
    let buffer = vec![8, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', 0, 0, 0];
    let mut cursor = Cursor::new(buffer.as_ref());
    assert_eq!(b"12345678", TLString::read(&mut cursor).unwrap().as_bytes());
}

#[test]
fn test_write_tl_long() {
    let data = TLString(b"12345678".to_vec());
    let mut buffer = vec![];
    assert_eq!(12, data.write(&mut buffer).unwrap());
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
    fn new(length: usize) -> Self {
        if length < DIVIDING_SIZE as usize {
            TLStringType::Short
        } else if length < 2usize.pow(24) {
            TLStringType::Long
        } else {
            unreachable!()
        }
    }

    fn new_by_lead_byte(lead_byte: u8) -> Self {
        if lead_byte < DIVIDING_SIZE {
            TLStringType::Short
        } else if lead_byte == DIVIDING_SIZE {
            TLStringType::Long
        } else {
            unreachable!()
        }
    }

    fn size(&self) -> usize {
        match self {
            TLStringType::Short => 1,
            TLStringType::Long => 4,
        }
    }
}
