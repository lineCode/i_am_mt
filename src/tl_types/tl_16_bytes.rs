use crate::{tl_types::TLType, utils::MyResult};

impl TLType for [u8; 16] {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let mut result = [0u8; 16];
        input.read_exact(&mut result)?;
        Ok(result)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_all(&self[..])?;
        Ok(16)
    }
}
