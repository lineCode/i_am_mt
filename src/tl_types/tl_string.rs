use crate::{
    tl_types::{tl_bytes::TLBytes, TLType},
    utils::MyResult,
};

impl TLType for String {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self> {
        let tl_bytes: TLBytes = TLBytes::tl_read(input)?;
        let bytes = tl_bytes.into_bytes();
        Ok(String::from_utf8(bytes)?)
    }

    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize> {
        let bytes = self.as_bytes().to_vec();
        let tl_bytes = TLBytes::from_bytes(bytes);
        tl_bytes.tl_write(output)
    }
}
