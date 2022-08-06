use std::fmt;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug, Clone)]
pub enum Error {
  NoEndDelimiter,
  InvalidEncoding,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(match self {
      Error::NoEndDelimiter => "no 'END' (0xCO) delimiter byte found in buffer",
      Error::InvalidEncoding => "buffer not encoded to SLIP protocol",
    })
  }
}
