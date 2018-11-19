pub mod int_bytes;
pub mod prime_numbers;
pub mod rsa;

//TODO Replace failure::Error with MyError
pub type MyResult<T> = std::result::Result<T, failure::Error>;
