pub use self::{read_bytes::TlReadBytes, write_bytes::TlWriteBytes};

pub mod read_bytes;
pub mod write_bytes;

//TODO Replace failure::Error with MyError
pub type MyResult<T> = std::result::Result<T, failure::Error>;
