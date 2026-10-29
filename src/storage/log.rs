mod log_reader;

use super::Entry;
use crate::errors::DBError;
use crate::serial::{Encode};
pub trait Logger {
    fn open( &mut self ) -> Result<(),DBError>;
    fn write( &mut self, entry : &Entry ) -> Result<(), DBError>;
    fn read( &mut self ) -> Result<Option<Entry>, DBError>;
    fn sync( &mut self ) -> Result<(), DBError>;
    fn write_lock( &mut self ) -> Result<(), DBError>;
}

use std::fs::File;
use std::path::PathBuf;
use self::log_reader::LogReader;

pub struct Log {
    filename : PathBuf,
    fp : Option<File>,
    reader: Option<LogReader>,
}

impl Log {
    pub fn new( filename : &str ) -> Self {
        Log {
            filename : filename.into(),
            fp : None,
            reader: None,
        }
    }
}


use std::fs::OpenOptions;
use std::io::{Seek, Write};
impl Logger for Log {
    fn open( &mut self ) -> Result<(),DBError> {
        if let Some(p) = self.filename.parent() {
            // If p == "" then file is in "./" dir
            let mut dir : PathBuf = p.into();
            if p == "" { dir.push("./"); }
            // Check if dir is valid
            if dir.is_dir() {
                self.fp = Some(OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&self.filename)?);
                
                // Sync Directory Data and Metadata
                //      This step is mandatory for Linux/Unix
                OpenOptions::new()
                    .read(true)
                    .open(dir)?
                    .sync_all()?;
                return Ok(())
            }
        }
        Err(DBError::Custom("wrong path"))
    }

    fn write( &mut self, entry : &Entry ) -> Result<(), DBError> {
        if let Some(fp) = self.fp.as_mut() {
            let mut scratch = Vec::with_capacity(entry.encoded_len());
            // TODO: Check len written
            entry.encode(&mut scratch);
            fp.write_all(&scratch)?;
            Ok(())
        } else {
            Err(DBError::Custom("uninitialized log"))
        }
    }

    fn read( &mut self ) -> Result<Option<Entry>, DBError> {
        if self.reader.is_none() {
            if let Some(fp) = self.fp.as_mut() {
                self.reader = Some(LogReader::new(fp))
            } else {
                return Err(DBError::Custom("uninitialized log"));
            }
        }
        if let Some(reader) = self.reader.as_mut() {
            Ok(reader.next())
        } else {
            Err(DBError::Custom("uninitialized log"))
        }
    }

    fn sync( &mut self ) -> Result<(), DBError> {
        if let Some(fp) = self.fp.as_mut() {
            fp.sync_all()?;
            Ok(())
        } else {
            Err(DBError::Custom("uninitialized log"))
        }
    }

    fn write_lock( &mut self ) -> Result<(), DBError> {
        if let Some(fp) = self.fp.as_mut() {
            if let Some(reader) = self.reader.as_mut() {
                let offset = reader.get_offset();
                fp.seek(std::io::SeekFrom::Start(offset as u64));
            }
            self.reader = None;
            Ok(())
        } else {
            Err(DBError::Custom("uninitialized log"))
        }
    }
}
