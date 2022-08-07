use super::*;

/// Decodes data following the SLIP protocol.
///
/// Given a buffer of encoded data, this data will be decoded following
/// the SLIP protocol, allocated into a new `Vec<u8>`, and returned
/// to the calling scope.
///
/// # Example:
///
/// ```rust
/// use simple_slip::decode;
///
/// let input: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15];
/// let expected: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
///
/// let result: Vec<u8> = decode(&input).unwrap();
///
/// assert_eq!(result, expected);
/// ```
pub fn decode(encoded_buffer: &[u8]) -> Result<Vec<u8>, SlipError> {
  let mut read_idx = find_delimiter(encoded_buffer)?;
  let mut write_idx = 0;

  let decoded_buffer_size = calc_decode_buffer_size(&encoded_buffer[read_idx..]);

  let mut decoded_buffer: Vec<u8> = vec![0; decoded_buffer_size];

  while read_idx < encoded_buffer.len() {
    let byte = encoded_buffer[read_idx];

    match byte {
      ESC => {
        let next_byte = encoded_buffer[read_idx + 1];
        let (latest_read_idx, latest_write_idx) =
          unescape(&next_byte, read_idx, write_idx, &mut decoded_buffer)?;

        read_idx = latest_read_idx;
        write_idx = latest_write_idx;
      }
      END => read_idx += 1,
      _ => {
        decoded_buffer[write_idx] = byte;

        read_idx += 1;
        write_idx += 1;
      }
    }
  }

  Ok(decoded_buffer)
}

fn find_delimiter(buffer: &[u8]) -> Result<usize, SlipError> {
  for (i, _) in buffer.iter().enumerate() {
    if buffer[i] == END {
      return Ok(i);
    }
  }

  Err(SlipError::NoEndDelimiter)
}

fn calc_decode_buffer_size(encoded_buffer: &[u8]) -> usize {
  encoded_buffer.into_iter().fold(
    0,
    |acc, byte| {
      if will_be_decoded(byte) {
        acc + 1
      } else {
        acc
      }
    },
  )
}

fn will_be_decoded(byte: &u8) -> bool {
  *byte == ESC || ![END, ESC_ESC, ESC_END].contains(byte)
}

fn unescape(
  val: &u8,
  read_idx: usize,
  write_idx: usize,
  write_buffer: &mut Vec<u8>,
) -> Result<(usize, usize), SlipError> {
  match *val {
    ESC_ESC => write_buffer[write_idx] = ESC,
    ESC_END => write_buffer[write_idx] = END,
    _ => return Err(SlipError::InvalidEncoding),
  }

  Ok((read_idx + 2, write_idx + 1))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decodes_data() {
    let input: Vec<u8> = vec![
      0xA1, 0xA2, 0xA3, END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15,
    ];
    let expected: Vec<u8> = vec![0x01, ESC, 0x49, END, 0x15];

    let res: Vec<u8> = decode(&input).unwrap();
    assert_eq!(res, expected);
  }

  #[test]
  fn errors_when_no_delimiter() {
    let error_input: [u8; 10] = [
      0xA1, 0xA2, 0xA3, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15,
    ];
    assert!(decode(&error_input).is_err());
  }
}
