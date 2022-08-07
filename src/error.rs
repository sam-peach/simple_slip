use std::fmt;

// pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug, Clone)]
pub enum SlipError {
  NoEndDelimiter,
  InvalidEncoding,
}

impl fmt::Display for SlipError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(match self {
      SlipError::NoEndDelimiter => "no 'END' (0xCO) delimiter byte found in buffer",
      SlipError::InvalidEncoding => "buffer not encoded to SLIP protocol",
    })
  }
}
