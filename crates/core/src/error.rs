use std::io;
use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
}
