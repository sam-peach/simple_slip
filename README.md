# Simple SLIP

A simple, lightweight implementation of [RFC 1055](https://tools.ietf.org/html/rfc1055) SLIP encoding for Rust!

## What is SLIP encoding?

SLIP (serial line internet protocol) encoding is a very simple way of packaging so it can be transmitted to some other receiver. I'd highly recommend reading the [Wikipedia article](https://en.wikipedia.org/wiki/Serial_Line_Internet_Protocol) on the topic for some more insight!

## Examples

SLIP is used in encoding data to be sent and decoding data to be read.

### Encoding

**NOTE: Each packet will _start_ and _end_ with an `END` (0xC0) byte.**

```rust
use simple_slip::encode;

let input: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];
let expected: Vec<u8> = vec![0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];

let result: Vec<u8> = encode(&input).unwrap();

assert_eq!(result, expected);
```

### Decoding

**NOTE: Each packet will start decoding from the second occurrence of the `END` (0xC0) byte.**

**The following data array would only decode `0x01` as it's the only byte after the second `END` (0xC0) byte:**

```
[0xA1, 0xA2, 0xA3, 0xC0, 0xC0, 0x01] --decode--> [0x01]
```

```rust
use simple_slip::decode;

let input: Vec<u8> = vec![0xA1, 0xA2, 0xA3, 0xC0, 0xC0, 0x01, 0xDB, 0xDD, 0x49, 0xDB, 0xDC, 0x15, 0xC0];
let expected: Vec<u8> = vec![0x01, 0xDB, 0x49, 0xC0, 0x15];

let result: Vec<u8> = decode(&input).unwrap();

assert_eq!(result, expected);
```
