use std::fmt::Debug;

use crate::utils::MyResult;

pub mod tl_16_bytes;
pub mod tl_32_bytes;
pub mod tl_bool;
pub mod tl_bytes;
pub mod tl_f64;
pub mod tl_i32;
pub mod tl_i64;
pub mod tl_string;
pub mod tl_u32;
pub mod tl_u64;
pub mod tl_vector;

pub trait TLType: Sized + Debug {
    fn tl_read(input: &mut std::io::Read) -> MyResult<Self>;
    fn tl_write(&self, output: &mut std::io::Write) -> MyResult<usize>;
}
