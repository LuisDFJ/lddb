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

use super::StorageError;
pub trait SerDesEntry {
    fn encode<W>(&self, writer : &mut W) -> Result<(), StorageError>
        where W : std::io::Write;
    fn decode<R>(&mut self, reader : &mut R) -> Result<(), StorageError>
        where R : std::io::Read;
}

use std::io::{Read};
use crate::crc32::{crc32_hash_fast, CRC32Reader};
impl SerDesEntry for Entry {
    // | Key Size | Val Size | Del Flag | Key     | Val     |
    // | 4 bytes  | 4 bytes  | 1 byte   | N bytes | M bytes |
    fn encode<W>(&self, writer : &mut W) -> Result<(), StorageError>
        where W : std::io::Write
    {
        let k_size : u32 = self.key.len().try_into()?;
        let v_size : u32 = self.val.len().try_into()?;
        let mut msg : Vec<u8> = Vec::new();
        // Key/Value Size Encoding
        msg.extend(k_size.to_le_bytes());
        msg.extend(v_size.to_le_bytes());
        // Del Flag Encoding
        msg.push( self.del.into() );
        // Key/Value Encoding
        msg.extend(self.key.clone());
        msg.extend(self.val.clone());
        // CRC32 IEEE checksum
        let crc32 : u32 = crc32_hash_fast(&msg);
        // Write and Flush
        writer.write_all(&crc32.to_le_bytes())?;
        writer.write_all(&msg)?;
        writer.flush()?;
        Ok(())
    }

    fn decode<R>(&mut self, reader : &mut R) -> Result<(), StorageError>
        where R : std::io::Read
    {
        // CRC32 Read
        let crc32  = read_crc32(reader)?;
        // Creating CRC32 Reader
        let reader = &mut CRC32Reader::new(reader);
        // Key/Value Size Decoding
        let k_size = read_u32(reader)?;
        let v_size = read_u32(reader)?;
        self.del = read_bool(reader)?;
        self.val.resize(v_size as usize, 0);
        self.key.resize(k_size as usize, 0);
        reader.read_exact(&mut self.key)?;
        reader.read_exact(&mut self.val)?;
        if crc32 != reader.finalize() {
            return Err(StorageError::BadCRC32)
        }
        Ok(())
    }
}

fn read_crc32<R>(reader : &mut R) -> Result<u32,StorageError>
    where R : std::io::Read
{
    let mut buffer = [0u8;4];
    let n = reader.read(&mut buffer)?;
    match n {
        0 => Err(StorageError::EOF),
        4 => Ok(u32::from_le_bytes(buffer)),
        _ => Err(StorageError::UnexpectedEOF),
    }
}

fn read_u32<R>(reader : &mut R) -> Result<u32,StorageError>
    where R : std::io::Read
{
    let mut buffer = [0u8;4];
    reader.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

fn read_bool<R>(reader : &mut R) -> Result<bool,StorageError>
    where R : std::io::Read
{
    let mut buffer = [0u8];
    reader.read_exact(&mut buffer)?;
    Ok(buffer[0] != 0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_entry_encode_decode() {
        let entry = Entry::from( b"key", b"val" );
        let mut buffer = vec![];
        entry.encode(&mut buffer).unwrap();

        let mut test = Entry::new();
        let mut reader = &buffer[..];
        test.decode(&mut reader).unwrap();

        assert_eq!(test, entry);
    }
}
