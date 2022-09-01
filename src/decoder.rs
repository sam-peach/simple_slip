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
/// let input: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];
/// let expected: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
///
/// let result: Vec<u8> = decode(&input).unwrap();
///
/// assert_eq!(result, expected);
/// ```
pub fn decode(encoded_buffer: &[u8]) -> Result<Vec<u8>, SlipError> {
  let trim_start = find_delimiter(encoded_buffer)?;
  let trim_end = find_last_delimiter(encoded_buffer)?;

  let mut read_idx = trim_start;
  let mut write_idx = 0;

  let decoded_buffer_size = calc_decode_buffer_size(&encoded_buffer[trim_start..trim_end]);

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

/// Decodes data following the SLIP protocol into delimited packets.
///
/// Given a buffer of encoded data, this data will be decoded following
/// the SLIP protocol, allocated into a new 2D vector, and returned
/// to the calling scope.
///
/// It will also return the remainder from what can't be decoded at the end of the input.
/// This can the be appended with the next input buffer to continue the decoding process.
/// For example:
///
/// An input of `[0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0, 0xC0, 0x01]` will not be able
/// to confidently decode the last to bytes: `[0xC0, 0x01]` and they will be return as a remainder.
///
/// If we then append the next input to the remainder: `[0xC0, 0x01] + [0x02, ...]`, this will enable us to
/// keep decoding packets across the input buffers boundary.
///
/// # Example:
///
/// ```rust
/// use simple_slip::decode_packets;
///
/// let input: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0,
///                           0xC0, 0x02, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0,
///                           0xC0, 0x03, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0, 0xC0, 0x01];
///
/// let expected: Vec<Vec<u8>> = vec![
///                              vec![1, 219, 73, 192, 21],
///                              vec![2, 219, 73, 192, 21],
///                              vec![3, 219, 73, 192, 21]
///                             ];
///
/// let (result, remainder): (Vec<Vec<u8>>, Vec<u8>) = decode_packets(&input);
///
/// assert_eq!(result, expected);
/// assert_eq!(remainder, [0xC0, 0x01]);

/// ```
pub fn decode_packets(encoded_buffer: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
  let mut parent_decoded_buffer: Vec<Vec<u8>> = Vec::new();

  let mut idx = 0;
  let mut trim_start = find_delimiter(&encoded_buffer).unwrap();

  while idx < encoded_buffer.len() {
    match find_next_delimiter(encoded_buffer, trim_start + 1) {
      Some(trim_end) => {
        let decoded_buffer_size = calc_decode_buffer_size(&encoded_buffer[trim_start..trim_end]);
        let mut local_decoded_buffer: Vec<u8> = vec![0; decoded_buffer_size];
        simple_decode(
          &encoded_buffer[trim_start..trim_end],
          &mut local_decoded_buffer,
        );

        parent_decoded_buffer.push(local_decoded_buffer);

        idx = trim_end + 1;
        trim_start = idx;
      }
      None => {
        return (
          parent_decoded_buffer,
          (&encoded_buffer[trim_start..]).to_vec(),
        )
      }
    }
  }

  (parent_decoded_buffer, (&[]).to_vec())
}

fn simple_decode(encoded_buffer: &[u8], decoded_buffer: &mut Vec<u8>) {
  let mut read_idx = 0;
  let mut write_idx = 0;
  while read_idx < encoded_buffer.len() {
    let byte = encoded_buffer[read_idx];

    match byte {
      ESC => {
        let next_byte = encoded_buffer[read_idx + 1];
        let (latest_read_idx, latest_write_idx) =
          unescape(&next_byte, read_idx, write_idx, decoded_buffer).unwrap();

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
}

fn find_delimiter(buffer: &[u8]) -> Result<usize, SlipError> {
  for (i, _) in buffer.iter().enumerate() {
    if buffer[i] == END {
      if buffer[i + 1] == END {
        return Ok(i + 1);
      } else {
        return Ok(i);
      }
    }
  }

  Err(SlipError::NoEndDelimiter)
}

fn find_next_delimiter(buffer: &[u8], start_idx: usize) -> Option<usize> {
  let mut idx = start_idx;
  while idx < buffer.len() {
    if buffer[idx] == END {
      return Some(idx);
    }

    idx += 1;
  }

  None
}

fn find_last_delimiter(buffer: &[u8]) -> Result<usize, SlipError> {
  for (i, _) in buffer.iter().rev().enumerate() {
    let idx = (buffer.len() - 1) - i;
    if buffer[idx] == END {
      if buffer[idx - 1] == END {
        return Ok(idx - 1);
      } else {
        return Ok(idx);
      }
    }
  }

  Err(SlipError::NoEndDelimiter)
}

fn calc_decode_buffer_size(encoded_buffer: &[u8]) -> usize {
  let mut sum = 0;
  let mut idx = 0;

  while idx < encoded_buffer.len() {
    let byte = encoded_buffer[idx];

    if byte == ESC {
      sum += 1;
      idx += 2;
      continue;
    } else if byte != END {
      sum += 1;
    }

    idx += 1;
  }

  return sum;
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
      0xA1, 0xA2, 0xA3, END, END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15, END,
    ];
    let expected: Vec<u8> = vec![0x01, ESC, 0x49, END, 0x15];

    let res: Vec<u8> = decode(&input).unwrap();

    assert_eq!(res, expected);
  }

  #[test]
  fn decodes_large_buffer() {
    let input: Vec<u8> = vec![
      0xA1, 0xA2, 0xA3, END, END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15, END, END, 0x01,
      ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15, END, END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END,
      0x15, END, END, 0x01, ESC, ESC_ESC, 0x49, ESC,
    ];

    let expected_packets: Vec<Vec<u8>> = vec![
      vec![0x01, ESC, 0x49, END, 0x15],
      vec![0x01, ESC, 0x49, END, 0x15],
      vec![0x01, ESC, 0x49, END, 0x15],
    ];
    let expected_remainder: &[u8] = &input[31..];

    let (packets, remainder): (Vec<Vec<u8>>, Vec<u8>) = decode_packets(&input);
    assert_eq!(packets, expected_packets);
    assert_eq!(remainder, expected_remainder);
  }

  #[test]
  fn errors_when_no_delimiter() {
    let error_input: [u8; 10] = [
      0xA1, 0xA2, 0xA3, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15,
    ];
    assert!(decode(&error_input).is_err());
  }
}
