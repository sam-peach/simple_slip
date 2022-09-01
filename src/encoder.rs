use super::*;

/// Encodes data following the SLIP protocol.
///
/// Given a buffer of unencoded data, this data will be encoded following
/// the SLIP protocol, allocated into a new `Vec<u8>`, and returned
/// to the calling scope.
///
/// # Example:
///
/// ```rust
/// use simple_slip::encode;
///
/// let input: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
/// let expected: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];
///
/// let result: Vec<u8> = encode(&input).unwrap();
///
/// assert_eq!(result, expected);
/// ```
pub fn encode(raw_buffer: &[u8]) -> Result<Vec<u8>, SlipError> {
  let mut encoded_buffer: Vec<u8> = vec![END];

  for byte in raw_buffer {
    match *byte {
      END => {
        let esc_end: [u8; 2] = [ESC, ESC_END];
        encoded_buffer.extend(esc_end);
      }
      ESC => {
        let esc_esc: [u8; 2] = [ESC, ESC_ESC];
        encoded_buffer.extend(esc_esc);
      }
      _ => encoded_buffer.push(*byte),
    }
  }

  encoded_buffer.push(END);

  Ok(encoded_buffer)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn encode_data() {
    let input: Vec<u8> = vec![0x01, ESC, 0x49, END, 0x15];
    let expected: Vec<u8> = vec![END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15, END];

    let res: Vec<u8> = encode(&input).unwrap();
    assert_eq!(res, expected);
  }
}
