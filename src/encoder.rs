use super::*;

pub fn encode(raw_buffer: &[u8]) -> Result<Vec<u8>> {
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

  Ok(encoded_buffer)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn encode_data() {
    let input: Vec<u8> = vec![0x01, ESC, 0x49, END, 0x15];
    let expected: Vec<u8> = vec![END, 0x01, ESC, ESC_ESC, 0x49, ESC, ESC_END, 0x15];

    let res: Vec<u8> = encode(&input).unwrap();
    assert_eq!(res, expected);
  }
}
