use std::io::Read;
use std::fs::File;

use crate::storage::Entry;
use crate::serial::{Encode,Decode};


pub struct LogReader {
    buf: Vec<u8>,
    offset: usize,
    done: bool,
}

impl LogReader {
    pub fn new(reader: &mut File) -> Self {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        Self {
            buf,
            offset: 0,
            done: false
        }
    }

    pub fn get_offset(&self) -> usize { self.offset }

    pub fn next(&mut self) -> Option<Entry> {
        if self.done { return None; }
        let entry = Entry::decode(&mut &self.buf[..]).ok()?;
        self.offset += entry.encoded_len();
        Some(entry)
    }
}
