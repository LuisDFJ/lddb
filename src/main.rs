use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::fmt::Display;
use std::io::{Read,Write};

#[derive(Debug)]
enum DBError {
    IO(std::io::Error),
    TryInt(std::num::TryFromIntError),
    Unkown,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::IO(err) => write!(f, "I/O Failure: {}", err),
            DBError::TryInt(err) => write!(f, "Try Int Failure: {}", err),
            _ => write!(f, "Unkown Failure"),
        }
    }
}

impl Error for DBError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DBError::IO(err) => Some(err),
            DBError::TryInt(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for DBError {
    fn from(value: std::io::Error) -> Self {
        DBError::IO(value)
    }
}

impl From<std::num::TryFromIntError> for DBError {
    fn from(value: std::num::TryFromIntError) -> Self {
        DBError::TryInt(value)
    }
}

struct KV {
    mem : HashMap<Vec<u8>, Vec<u8>>
}

impl KV {
    fn new() -> Self {
        KV {
            mem : HashMap::new()
        }
    }
}

trait InMemory<Key,Val>
where
    Key : Hash + Eq
{
    fn open(&mut self) -> Result<(),DBError>;
    fn close(&mut self) -> Result<(), DBError>;
    fn get<K>(&self, key : K) -> Result<Option<&Val>, DBError>
        where K : Into<Key>;
    fn set<K,V>(&mut self, key : K, val : V) -> Result<bool, DBError>
        where K : Into<Key>, V : Into<Val>;
    fn del<K>(&mut self, key : K) -> Result<bool, DBError>
        where K : Into<Key>;
}

impl InMemory<Vec<u8>,Vec<u8>> for KV {
    fn open(&mut self) -> Result<(), DBError> { Ok(()) }
    fn close(&mut self) -> Result<(), DBError> { Ok(()) }
    fn get<K>(&self, key : K) -> Result<Option<&Vec<u8>>, DBError>
        where K : Into<Vec<u8>>
    {
        Ok(self.mem.get(&key.into()))
    }
    fn set<K,V>(&mut self, key : K, val : V) -> Result<bool, DBError>
        where K : Into<Vec<u8>>, V : Into<Vec<u8>>
    {
        match self.mem.insert(key.into(), val.into()) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    fn del<K>(&mut self, key : K) -> Result<bool, DBError>
        where K : Into<Vec<u8>>
    {
        match self.mem.remove(&key.into()) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

struct Entry {
    key : Vec<u8>,
    val : Vec<u8>,
}

impl Entry {
    fn new() -> Self {
        Entry {
            key: vec![],
            val: vec![],
        }
    }

    fn from<K,V>( key : K, val : V ) -> Self
        where K : Into<Vec<u8>>, V : Into<Vec<u8>>
    {
        Entry {
            key: key.into(),
            val: val.into(),
        }
    }
}

trait SerDes {
    fn encode<W>(&self, writer : &mut W) -> Result<(), DBError>
        where W : std::io::Write;
    fn decode<R>(&mut self, reader : &mut R) -> Result<(), DBError>
        where R : std::io::Read;
}

impl SerDes for Entry {
    // | 4b | 4b | X bytes | Y bytes |
    // | X  | Y  | Key     | Val     |
    fn encode<W>(&self, writer : &mut W) -> Result<(), DBError>
        where W : std::io::Write
    {
        let k_size : u32 = self.key.len().try_into()?;
        let v_size : u32 = self.val.len().try_into()?;
        let mut msg : Vec<u8> = Vec::new();
        msg.extend(k_size.to_le_bytes());
        msg.extend(v_size.to_le_bytes());
        msg.extend(self.key.clone());
        msg.extend(self.val.clone());
        writer.write_all(&msg)?;
        writer.flush()?;
        Ok(())
    }

    fn decode<R>(&mut self, reader : &mut R) -> Result<(), DBError>
        where R : std::io::Read
    {
        let k_size = read_u32(reader)?;
        let v_size = read_u32(reader)?;
        reader.take(k_size as u64).read_to_end(&mut self.key)?;
        reader.take(v_size as u64).read_to_end(&mut self.val)?;
        Ok(())
    }
}

fn read_u32<R>(reader : &mut R) -> Result<u32,DBError>
    where R : std::io::Read
{
    let mut buffer = [0u8;4];
    let r = reader.read(&mut buffer)?;
    if r != 4 { return Err(DBError::Unkown) }
    let n = u32::from_le_bytes(buffer);
    Ok(n)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_kv_set_get_del() {
        let mut kv = KV::new();
        let res = kv.set(b"a", b"val").unwrap();
        assert_eq!(res, false);
        let res = kv.set(b"a", b"asd").unwrap();
        assert_eq!(res, true);
        let res = kv.set(b"b", b"bcd").unwrap();
        assert_eq!(res, false);

        let res = kv.get(b"a").unwrap().unwrap();
        assert_eq!(res, &b"asd".to_vec());
        let res = kv.get(b"b").unwrap().unwrap();
        assert_eq!(res, &b"bcd".to_vec());

        let res = kv.del(b"b").unwrap();
        assert_eq!(res, true);
        let res = kv.del(b"b").unwrap();
        assert_eq!(res, false);
        let res = kv.get(b"b").unwrap();
        assert_eq!(res, None);
    }

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
    }
}
