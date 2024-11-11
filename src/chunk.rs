use std::fmt::{write, Display};
use crc::Crc;

use crate::chunk_type::ChunkType;

struct Chunk {
    length:     u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc:        u32,
}

impl Chunk {
    fn new(chunktype: ChunkType, data: Vec<u8>) -> Chunk {
        let x25: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let to_hash: Vec<u8> = chunktype
            .bytes()
            .iter()
            .chain(&data)
            .copied()
            .collect();

        let crc_value = x25.checksum(&to_hash);

        Chunk {
            length: data.len() as u32,
            chunk_type: chunktype,
            chunk_data: data,
            crc: crc_value,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> Result<String, &'static str> {
        let to_stringify: Vec<u8> = self.chunk_data.clone();

        if self.length() == 0 {
            Err("diocane")
        } else {
            Ok(String::from_utf8(to_stringify).unwrap())
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied().collect();

        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let slice_length = value.len();

        let length_slice: [u8; 4] = [
            value[0],
            value[1],
            value[2],
            value[3]
        ];
        let length_from_slice: u32 = u32::from_be_bytes(length_slice);

        let chunk_type_slice: [u8; 4] = [
            value[4],
            value[5],
            value[6],
            value[7]
        ];
        let chunk_type_from_slice: ChunkType = ChunkType::try_from(chunk_type_slice).unwrap();

        let mut chunk_data_vec: Vec<u8> = Vec::new();

        for byte in &value[8..(slice_length -4)] {
            chunk_data_vec.push(*byte)
        }

        let crc_slice: [u8; 4] = [
            value[slice_length - 4],
            value[slice_length - 3],
            value[slice_length - 2],
            value[slice_length - 1]
        ];
        let crc_from_slice: u32 = u32::from_be_bytes(crc_slice);

        let chunk = Chunk {
            length: length_from_slice,
            chunk_type: chunk_type_from_slice,
            chunk_data: chunk_data_vec,
            crc: crc_from_slice,
        };

        let x25: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let to_hash: Vec<u8> = chunk.chunk_type
            .bytes()
            .iter()
            .chain(chunk.chunk_data.iter())
            .copied()
            .collect();

        let crc_value = x25.checksum(&to_hash);

        if crc_value != chunk.crc {
            Err("Invalid bytes (probably crc is wrong)")
        } else {
            Ok(chunk)
        }



    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "chunk length:{}\nchunk type: {}\n, chunk message {}\nCRC u32: {}",
            self.length,
            self.chunk_type,
            self.data_as_string().unwrap(),
            self.crc
        )
    }
}

// unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}
