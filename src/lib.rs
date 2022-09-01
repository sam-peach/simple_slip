//! Simple SLIP
//!
//! A simple, lightweight implementation of [RFC 1055](https://tools.ietf.org/html/rfc1055) SLIP encoding.
//!
//! ## What is SLIP encoding?
//!
//! SLIP (serial line internet protocol) encoding is a very simple way of packaging so it can be transmitted to some other receiver. I'd
//! highly recommend reading the [Wikipedia article](https://en.wikipedia.org/wiki/Serial_Line_Internet_Protocol) on the topic for some more insight!
//!
//! ## Examples
//!
//! SLIP is used in encoding data to be sent and decoding data to be read.
//!
//! ### Encoding
//!
//! ```rust
//! use simple_slip::encode;
//!
//! let input: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
//! let expected: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15];
//!
//! let result: Vec<u8> = encode(&input).unwrap();
//!
//! assert_eq!(result, expected);
//! ```
//!
//! ### Decoding
//!
//! ```rust
//! use simple_slip::decode;
//!
//! let input: Vec<u8> = vec![0xA1, 0xA2, 0xA3, 0xC0, 0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];
//! let expected: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
//!
//! let result: Vec<u8> = decode(&input).unwrap();
//!
//! assert_eq!(result, expected);
//! ```
//!
//! Decoding multiple packets
//!
//! ```rust
//! use simple_slip::decode_packets;
//!
//! let input: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0,
//!                           0xC0, 0x02, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0,
//!                           0xC0, 0x03, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0, 0xC0, 0x01];
//!
//! let expected: Vec<Vec<u8>> = vec![
//!                              vec![1, 219, 73, 192, 21],
//!                              vec![2, 219, 73, 192, 21],
//!                              vec![3, 219, 73, 192, 21]
//!                             ];
//!
//! let (result, remainder): (Vec<Vec<u8>>, Vec<u8>) = decode_packets(&input);
//!
//! assert_eq!(result, expected);
//! assert_eq!(remainder, [0xC0, 0x01]);
//! ```

mod constants;
mod decoder;
mod encoder;
mod error;

pub use constants::*;
pub use decoder::{decode, decode_packets};
pub use encoder::encode;
pub use error::SlipError;
