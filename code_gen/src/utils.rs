use std::{fmt::Display, str::FromStr};

use proc_macro2::{Ident, Span, TokenStream};
use serde::{Deserialize, Deserializer};

pub fn deserialize_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn ident(name: impl AsRef<str>) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

pub type MyResult<T> = std::result::Result<T, failure::Error>;

pub fn i32_suffixed(num: i32) -> TokenStream {
    let text = format!("{:08x}", num.abs());
    let (be, le) = text.split_at(4);
    let text = if num < 0 {
        format!("-0x{}_{}i32", be, le)
    } else {
        format!("0x{}_{}i32", be, le)
    };
    text.parse().unwrap()
}
