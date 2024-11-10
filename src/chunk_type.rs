use core::str;
use std::{str::FromStr, u8};

fn byte_to_bits(byte: u8) -> [u8; 8] {
    let mut bits = [0u8; 8];

    for i in 0..=7 {
        let shifted_byte = byte >> i;
        let cur_bit = shifted_byte & 1;
        bits[7 - i] = cur_bit;
    }

    bits
}

/// ChunkType is defined as a 4 byte string with the 5th
/// bit of every byte having a special meaning, thus being
/// the "property bit". I have named the bytes with their
/// property bit name.
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    ancillary_byte:      u8,
    private_byte:        u8,
    reserved_byte:       u8,
    safe_to_copy_byte:   u8,
}

impl ChunkType {

    fn bytes(&self) -> [u8; 4] {
        let bytes: [u8; 4] = [
            self.ancillary_byte,
            self.private_byte,
            self.reserved_byte,
            self.safe_to_copy_byte
        ];

        bytes
    }

    fn is_valid(&self) -> bool {
        let mut x = self.bytes().len();

        loop {
            x -= 1;

            if x == 2 {
                if self.bytes()[x].is_ascii_lowercase() {
                    break false;
                } else {
                    continue;
                }
            } else if !self.bytes()[x].is_ascii_alphabetic() {
                break false;
            } else if x == 0 {
                break true;
            } else {
                continue;
            }
        }

    }


    fn is_critical(&self) -> bool {
        self.ancillary_byte.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.private_byte.is_ascii_uppercase()

    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved_byte.is_ascii_uppercase()

    }

    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy_byte.is_ascii_lowercase()
    }

}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {

        let chunk_type = ChunkType {
            ancillary_byte:     value[0],
            private_byte:       value[1],
            reserved_byte:      value[2],
            safe_to_copy_byte:  value[3],
        };

        let mut x = chunk_type.bytes().len();

        loop {
            x -= 1;

            if !chunk_type.bytes()[x].is_ascii_alphabetic() {
                break Err("The chunk type can't contain non-alphabetic bytes");
            } else if x == 0 {
                break Ok(chunk_type);
            } else {
                continue;
            }
        }

    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buf = self.bytes();

        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid utf8 sequence: {}", e),
        };

        write!(f, "{}", s)
    }

}

impl FromStr for ChunkType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let chunk_type = ChunkType {
            ancillary_byte:     bytes[0],
            private_byte:       bytes[1],
            reserved_byte:      bytes[2],
            safe_to_copy_byte:  bytes[3],
        };

        let mut x = chunk_type.bytes().len();

        loop {
            x -= 1;

            if !chunk_type.bytes()[x].is_ascii_alphabetic() {
                break Err("The chunk type can't contain non-alphabetic bytes".into());
            } else if x == 0 {
                break Ok(chunk_type);
            } else {
                continue;
            }
        }

    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
