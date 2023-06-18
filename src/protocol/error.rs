use std::{io, string::FromUtf8Error};

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("Insufficient bytes")]
    TooShort,
    #[error("Too many bytes (max {0} bytes)")]
    TooLong(usize),
    #[error("Invalid data")]
    Invalid,
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Invalid UTF-8")]
    InvalidUtf8(#[from] FromUtf8Error),
}
