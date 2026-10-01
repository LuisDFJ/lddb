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
        // Write and Flush
        writer.write_all(&msg)?;
        writer.flush()?;
        Ok(())
    }

    fn decode<R>(&mut self, reader : &mut R) -> Result<(), StorageError>
        where R : std::io::Read
    {
        // Key/Value Size Decoding
        let k_size = read_u32(reader)?;
        let v_size = read_u32(reader)?;
        let del = read_bool(reader)?;
        let rk = reader.take(k_size as u64).read_to_end(&mut self.key)?;
        let rv = reader.take(v_size as u64).read_to_end(&mut self.val)?;
        if rk != k_size as usize || rv != v_size as usize { return Err(StorageError::EOF) }
        self.del = del;
        Ok(())
    }
}

fn read_u32<R>(reader : &mut R) -> Result<u32,StorageError>
    where R : std::io::Read
{
    let mut buffer = [0u8;4];
    let r = reader.read(&mut buffer)?;
    if r != 4 { return Err(StorageError::EOF) }
    let n = u32::from_le_bytes(buffer);
    Ok(n)
}

fn read_bool<R>(reader : &mut R) -> Result<bool,StorageError>
    where R : std::io::Read
{
    let mut buffer = [0u8];
    let r = reader.read(&mut buffer)?;
    if r != 1 { return Err(StorageError::EOF) }
    let b = buffer[0] != 0;
    Ok(b)
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

        assert_eq!(test.key, b"key".to_vec());
        assert_eq!(test.val, b"val".to_vec());
        assert_eq!(test.del, false);
    }
}
