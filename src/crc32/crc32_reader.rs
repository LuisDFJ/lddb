use crate::serial::{Decode,DecodeError};

use super::crc32_8_fold::crc32_hash_fast_update;

pub struct CRC32Cursor<'a> {
    buf: &'a [u8],
    crc: u32,
}

impl <'a> CRC32Cursor<'a> {
    pub fn new( buf : &'a[u8] ) -> Self {
        CRC32Cursor { buf, crc: 0xFFFF_FFFF }
    }
    pub fn decode<T: Decode<'a>>(&mut self) -> Result<T, DecodeError> {
        let before: &'a [u8] = self.buf;
        let mut cur = before;
        let val = T::decode(&mut cur)?;
        let size = before.len() - cur.len();
        self.crc = crc32_hash_fast_update(self.crc, &before[..size]);
        self.buf = cur;
        Ok(val)
    }
    pub fn finalize(&self) -> u32 {
        self.crc ^ 0xFFFF_FFFF
    }
    pub fn remaining(&self) -> &'a [u8] {
        self.buf
    }
}
