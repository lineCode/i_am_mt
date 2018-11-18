//TODO Replace failure::Error with MyError
pub type MyResult<T> = std::result::Result<T, failure::Error>;
