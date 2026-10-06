use super::Log;
use std::collections::HashMap;

pub struct KeyValue {
    log : Log,
    mem : HashMap<Vec<u8>, Vec<u8>>
}

impl KeyValue {
    pub fn new( filename : &str ) -> Self {
        KeyValue {
            log : Log::new( filename ),
            mem : HashMap::new()
        }
    }
}

use std::hash::Hash;
use super::StorageError;

pub trait InMemory<Key,Val>
where
    Key : Hash + Eq
{
    fn open(&mut self) -> Result<(), StorageError>;
    //fn close(&mut self) -> Result<(), StorageError>;
    fn get<K>(&self, key : K) -> Result<Option<&Val>, StorageError>
        where K : Into<Key>;
    fn set<K,V>(&mut self, key : K, val : V) -> Result<bool, StorageError>
        where K : Into<Key>, V : Into<Val>;
    fn del<K>(&mut self, key : K) -> Result<bool, StorageError>
        where K : Into<Key>;
}

use super::{Logger, Entry};
impl InMemory<Vec<u8>,Vec<u8>> for KeyValue {
    fn open(&mut self) -> Result<(), StorageError> {
        self.log.open()?;

        loop {
            let mut entry = Entry::new();
            if self.log.read(&mut entry)? { break }
            if entry.del {
                self.mem.remove( &entry.key );
            } else {
                self.mem.insert( entry.key, entry.val );
            }
        }

        Ok(())
    }
    //fn close(&mut self) -> Result<(), StorageError> { Ok(()) }
    fn get<K>(&self, key : K) -> Result<Option<&Vec<u8>>, StorageError>
        where K : Into<Vec<u8>>
    {
        Ok(self.mem.get(&key.into()))
    }
    fn set<K,V>(&mut self, key : K, val : V) -> Result<bool, StorageError>
        where K : Into<Vec<u8>>, V : Into<Vec<u8>>
    {
        let mut entry = Entry::from(key, val);
        self.log.write(&mut entry)?;
        match self.mem.insert(entry.key, entry.val) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    fn del<K>(&mut self, key : K) -> Result<bool, StorageError>
        where K : Into<Vec<u8>>
    {
        let mut entry = Entry::from(key,[]);
        entry.del = true;
        self.log.write(&mut entry)?;
        match self.mem.remove(&entry.key) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_kv_set_get_del() {
        let filename = ".test_db";
        let _ = fs::remove_file(filename);
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();
            let res = kv.set(b"a", b"val").unwrap();
            assert_eq!(res, false);
            let res = kv.set(b"a", b"asd").unwrap();
            assert_eq!(res, true);
            let res = kv.set(b"b", b"bcd").unwrap();
            assert_eq!(res, false);

            let res = kv.get(b"a").unwrap().unwrap();
            assert_eq!(res, b"asd");
            let res = kv.get(b"b").unwrap().unwrap();
            assert_eq!(res, b"bcd");

            let res = kv.del(b"b").unwrap();
            assert_eq!(res, true);
            let res = kv.del(b"b").unwrap();
            assert_eq!(res, false);
            let res = kv.get(b"b").unwrap();
            assert_eq!(res, None);
        }
        let _ = fs::remove_file(filename);
    }

    use std::fs;
    #[test]
    fn test_kv_log() {
        let filename = ".test_db_a";
        let _ = fs::remove_file(filename);
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();

            kv.set(b"k1", b"v1").unwrap();
            kv.set(b"k2", b"v2").unwrap();
            kv.set(b"k3", b"v3").unwrap();
            kv.set(b"k1", b"v4").unwrap();

            kv.del(b"k2").unwrap();
        }
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();
            assert_eq!(kv.get(b"k1").unwrap().unwrap(), b"v4");
            assert_eq!(kv.get(b"k2").unwrap(), None);
            assert_eq!(kv.get(b"k3").unwrap().unwrap(), b"v3");
        }
        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_kv_checksum() {
        let filename = ".test_db_b";
        let _ = fs::remove_file(filename);
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();
            kv.set(b"k1", b"v1").unwrap();
            kv.set(b"k2", b"v2").unwrap();
            kv.set(b"k3", b"v3").unwrap();
            kv.set(b"k1", b"v4").unwrap();
            kv.del(b"k2").unwrap();
        }
        // Corrupting last entry
        let mut data = fs::read(filename).unwrap();
        data.pop();
        let l = data.len() - 1;
        data[l] = 34u8;
        fs::write(filename, data).unwrap();
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();
            assert_eq!(kv.get(b"k1").unwrap().unwrap(), b"v4");
            assert_eq!(kv.get(b"k2").unwrap().unwrap(), b"v2");
            assert_eq!(kv.get(b"k3").unwrap().unwrap(), b"v3");
            kv.set(b"k4", b"v4").unwrap();
            kv.set(b"k5", b"v5").unwrap();
            kv.del(b"k2").unwrap();
        }
        {
            let mut kv = KeyValue::new(filename);
            kv.open().unwrap();
            assert_eq!(kv.get(b"k1").unwrap().unwrap(), b"v4");
            assert_eq!(kv.get(b"k2").unwrap(), None);
            assert_eq!(kv.get(b"k3").unwrap().unwrap(), b"v3");
            assert_eq!(kv.get(b"k4").unwrap().unwrap(), b"v4");
            assert_eq!(kv.get(b"k5").unwrap().unwrap(), b"v5");
        }
        let _ = fs::remove_file(filename);
    }
}
