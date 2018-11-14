use std::io::Cursor;

use crate::utils::{MyResult, TlWriteBytes};
use std::fmt::Debug;

pub mod tl_double;
pub mod tl_int;
pub mod tl_long;
pub mod tl_string;

pub trait TLType: Sized + PartialEq + Debug {
    fn read(input: &mut Cursor<&[u8]>) -> MyResult<Self>;
    fn write(&self, output: &mut TlWriteBytes) -> MyResult<usize>;
}
