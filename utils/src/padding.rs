use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PaddingError(String);

impl Display for PaddingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = format!("Padding Error: {}", self.0);
        write!(f, "{e}")
    }
}

/// Do nothing. Included for compatibility.
pub fn none_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), PaddingError> {
    if bytes.len() % block_size as usize != 0 {
        Err(PaddingError(format!(
            "encrypted data must be in chunks of {} bytes",
            block_size
        )))
    } else {
        Ok(())
    }
}

/// Do nothing. Included for compatibility.
pub fn strip_none_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), PaddingError> {
    if bytes.len() % block_size as usize != 0 {
        Err(PaddingError(format!(
            "encrypted data must be in chunks of {} bytes",
            block_size
        )))
    } else {
        Ok(())
    }
}

/// Bit padding adds the byte 0b10000000 (or 0x80) to the end of the input and then fills the rest with null bytes to reach a multiple of the block size.
pub fn bit_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), PaddingError> {
    bytes.push(0x80);
    while bytes.len() % block_size as usize != 0 {
        bytes.push(0x00)
    }
    Ok(())
}

/// Remove bit padding.
pub fn strip_bit_padding(bytes: &mut Vec<u8>) -> Result<(), PaddingError> {
    loop {
        let p = bytes.pop();
        if p == Some(0x00) {
            continue;
        } else if p == Some(0x80) || p == None {
            return Ok(());
        } else {
            return Err(PaddingError(format!(
                "invalid bit padding, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
}

/// PKCS5 padding adds n bytes each with value n to reach a multiple of the block size.
pub fn pkcs5_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), PaddingError> {
    let n_padding = (block_size as usize - (bytes.len() % block_size as usize))
        .try_into()
        .unwrap();
    for _ in 0..n_padding {
        bytes.push(n_padding)
    }
    Ok(())
}

/// Remove PKCS5 padding.
pub fn strip_pkcs5_padding(bytes: &mut Vec<u8>) -> Result<(), PaddingError> {
    let n_padding = *bytes.iter().last().ok_or(PaddingError(String::from(
        "PKCS padded ciphertext cannot have zero length",
    )))?;
    for _ in 0..n_padding {
        let p = bytes.pop();
        if p == Some(n_padding) {
            continue;
        } else if p == None {
            return Err(PaddingError(String::from(
                "invalid PKCS padding, ran out of ciphertext",
            )));
        } else {
            return Err(PaddingError(format!(
                "invalid PKCS padding, found byte {:02x} for ",
                p.unwrap()
            )));
        }
    }
    Ok(())
}

/// ANSI X9.23 padding adds n-1 null bytes and then a final byte with a value of n to reach a multiple of the block size.
pub fn ansi923_padding(bytes: &mut Vec<u8>, block_size: u32) -> Result<(), PaddingError> {
    let n_padding = (block_size as usize - (bytes.len() % block_size as usize))
        .try_into()
        .unwrap();
    for _ in 0..(n_padding - 1) {
        bytes.push(0)
    }
    bytes.push(n_padding);
    Ok(())
}

// Remove ANSI X9.23 padding.
pub fn strip_ansi923_padding(bytes: &mut Vec<u8>) -> Result<(), PaddingError> {
    let n_padding = bytes.pop().ok_or(PaddingError(String::from(
        "ANSI X9.23 padded ciphertext cannot have zero length",
    )))?;

    for _ in 0..(n_padding - 1) {
        let p = bytes.pop();
        if p == Some(0) {
            continue;
        } else if p == None {
            return Err(PaddingError(String::from(
                "invalid ANSI X9.23 padding, ran out of ciphertext",
            )));
        } else {
            return Err(PaddingError(format!(
                "invalid ANSI X9.23 padding, found byte {:02x}",
                p.unwrap()
            )));
        }
    }
    Ok(())
}

/// Pad with the 0x00 byte until the block size is reached. If the input already has a length equal to the block size no padding is added. Zero padding is not reversible.
pub fn zero_padding(bytes: &mut Vec<u8>, block_size: u32) {
    while bytes.len() % block_size as usize != 0 {
        bytes.push(0)
    }
}

/// Merkele-Damgård strengthening pads the input with 0x80, then with zeroes, and then appends the length of the original input. Length is appended as a 64-bit little endian value. This form of padding is intended for hash functions so no inverse is provided.
pub fn md_strengthening_64_le(bytes: &mut Vec<u8>, block_size: u32) {
    // Length in bits before padding
    let b_len = (bytes.len().wrapping_mul(8)) as u64;
    // push a byte with a leading 1 to the bytes
    bytes.push(0x80);
    // push zeros until the length is eight bytes less than the block size.
    while (bytes.len() % block_size as usize) != (block_size - 8) as usize {
        bytes.push(0)
    }
    // Append the eight bytes of length
    for b in b_len.to_le_bytes() {
        bytes.push(b)
    }
}

/// Merkele-Damgård strengthening pads the input with 0x80, then with zeroes, and then appends the length of the original input. Length is appended as a 64-bit big endian value. This form of padding is intended for hash functions so no inverse is provided.
pub fn md_strengthening_64_be(bytes: &mut Vec<u8>, block_size: u32) {
    // Length in bits before padding
    let b_len = (bytes.len().wrapping_mul(8)) as u64;
    // push a byte with a leading 1 to the bytes
    bytes.push(0x80);
    // push zeros until the length is eight bytes less than the block size.
    while (bytes.len() % block_size as usize) != (block_size - 8) as usize {
        bytes.push(0)
    }
    // Append the eight bytes of length
    for b in b_len.to_be_bytes() {
        bytes.push(b)
    }
}

#[cfg(test)]
mod padding_tests {

    use super::*;

    #[test]
    fn test_bit_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        bit_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x80, 0x00, 0x00, 0x00], bytes);
        strip_bit_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_bit_padding_full_block() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80];
        bit_padding(&mut bytes, 8).unwrap();
        assert_eq!(
            vec![
                0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ],
            bytes
        );
        strip_bit_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_pkcs_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        pkcs5_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x04, 0x04, 0x04, 0x04], bytes);
        strip_pkcs5_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_pkcs_padding_full_blocks() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80];
        pkcs5_padding(&mut bytes, 8).unwrap();
        assert_eq!(
            vec![
                0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
                0x08, 0x08
            ],
            bytes
        );
        strip_pkcs5_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_ansi_padding() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0x80];
        ansi923_padding(&mut bytes, 8).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80, 0x00, 0x00, 0x00, 0x04], bytes);
        strip_ansi923_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0x80], bytes);
    }

    #[test]
    fn test_ansi_padding_full_block() {
        let mut bytes = vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80];
        ansi923_padding(&mut bytes, 8).unwrap();
        assert_eq!(
            vec![
                0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x08
            ],
            bytes
        );
        strip_ansi923_padding(&mut bytes).unwrap();
        assert_eq!(vec![0x01, 0x02, 0xff, 0xff, 0xff, 0xff, 0xff, 0x80], bytes);
    }
}
