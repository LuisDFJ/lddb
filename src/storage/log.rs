use std::fs::File;
use std::path::PathBuf;

pub struct Log {
    filename : PathBuf,
    fp : Option<File>,
}

impl Log {
    pub fn new( filename : &str ) -> Self {
        Log {
            filename : filename.into(),
            fp : None,
        }
    }
}

use super::{StorageError, SerDesEntry, Entry};
pub trait Logger {
    fn open( &mut self ) -> Result<(),StorageError>;
    fn write( &mut self, entry : &Entry ) -> Result<(), StorageError>;
    fn read( &mut self, entry : &mut Entry ) -> Result<bool, StorageError>;
}

use std::fs::OpenOptions;
use std::io::{Seek,SeekFrom,BufReader,BufWriter};
impl Logger for Log {
    fn open( &mut self ) -> Result<(),StorageError> {
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
        Err(StorageError::Custom("wrong path"))
    }

    fn write( &mut self, entry : &Entry ) -> Result<(), StorageError> {
        if let Some(fp) = self.fp.as_mut() {
            //{
                //let mut writer = BufWriter::new(&mut *fp);
                //entry.encode( &mut writer )?;
            //}
            // Encode entry to file
            entry.encode(fp)?;
            // Sync File to Disk
            fp.sync_all()?;
            Ok(())
        } else {
            Err(StorageError::Custom("uninitialized log"))
        }
    }

    fn read( &mut self, entry : &mut Entry ) -> Result<bool, StorageError> {
        if let Some(fp) = self.fp.as_mut() {
            let offset = fp.stream_position()?;
            match entry.decode( fp ) {
                Ok(_) => Ok(false),
                Err(err) => match err {
                    StorageError::EOF => Ok(true),
                    StorageError::UnexpectedEOF |
                    StorageError::BadCRC32 => {
                        fp.seek(SeekFrom::Start(offset))?;
                        Ok(true)
                    }
                    _ => Err(err),
                },
            }
        } else {
            Err(StorageError::Custom("uninitialized log"))
        }
    }
}
