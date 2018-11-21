use crate::{tl_types::TLType, utils::MyResult};

impl TLType for [u8; 32] {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let mut result = [0u8; 32];
        input.read_exact(&mut result)?;
        Ok(result)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        output.write_all(&self[..])?;
        Ok(32)
    }
}
