#[derive(Debug,PartialEq,Eq)]
pub struct Entry {
    pub key : Vec<u8>,
    pub val : Vec<u8>,
    pub del : bool,
}

impl Entry {
    pub fn new() -> Self {
        Entry {
            key: vec![],
            val: vec![],
            del: false,
        }
    }

    pub fn from<K,V>( key : K, val : V ) -> Self
        where K : Into<Vec<u8>>, V : Into<Vec<u8>>
    {
        Entry {
            key: key.into(),
            val: val.into(),
            del: false,
        }
    }
}

use crate::serial::{Encode, Decode,EncodeError,DecodeError};
use crate::crc32::{crc32_hash_fast, CRC32Cursor};
impl Encode for Entry {
    fn encoded_len(&self) -> usize {
        4 + 4 + 4 + 1 + self.key.len() + self.val.len()
    }
    // |  CRC32   | Del Flag | Key Size | Key     | Val Size | Val     |
    // | 4 bytes  | 1 byte   | 4 bytes  | N bytes | 4 bytes  | M bytes |
    fn encode(&self, output : &mut [u8]) -> Result<usize,EncodeError> {
        let needed = self.encoded_len();
        if output.len() < needed {
            return Err(EncodeError{needed})
        }
        //let k_size : u32 = self.key.len() as u32;
        //let v_size : u32 = self.val.len() as u32;

        let mut pos = 4;
        //pos += k_size.encode(&mut output[pos..])?;
        //pos += v_size.encode(&mut output[pos..])?;
        pos += (self.del as u8).encode(&mut output[pos..])?;
        pos += self.key.encode(&mut output[pos..])?;
        pos += self.val.encode(&mut output[pos..])?;
        crc32_hash_fast(&output[4..]).encode(output)?;
        Ok(pos)
    }
}

impl <'a> Decode<'a> for Entry {
    fn decode(input : &mut &'a [u8]) -> Result<Self, DecodeError> {
        let crc32 = u32::decode(input)?;
        // Creating CRC32 Reader
        let cursor = &mut CRC32Cursor::new(input);
        //let k_size : u32 = cursor.decode()?;
        //let v_size : u32 = cursor.decode()?;
        let del : u8 = cursor.decode()?;
        let key : Vec<u8> = cursor.decode()?;
        let val : Vec<u8> = cursor.decode()?;
        if crc32 != cursor.finalize() {
            return Err(DecodeError::Invalid("corrupted entry"))
        }
        Ok(Entry{del:del != 0,key,val})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_entry_encode_decode() {
        let entry = Entry::from( b"key", b"val" );
        let mut buffer = Vec::with_capacity(entry.encoded_len());
        entry.encode(&mut buffer).unwrap();
        let test = Entry::decode(&mut &buffer[..]).unwrap();
        assert_eq!(test, entry);
    }
}
