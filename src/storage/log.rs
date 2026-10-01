use std::fs::File;

pub struct Log {
    filename : String,
    fp : Option<File>,
}

impl Log {
    pub fn new( filename : &str ) -> Self {
        Log {
            filename : filename.to_string(),
            fp : None
        }
    }
}

use super::{StorageError, SerDesEntry, Entry};
pub trait Logger {
    fn open( &mut self ) -> Result<(),StorageError>;
    //fn close( &mut self ) -> Result<(), StorageError>;
    fn write( &mut self, entry : &Entry ) -> Result<(), StorageError>;
    fn read( &mut self, entry : &mut Entry ) -> Result<bool, StorageError>;
}

use std::fs::OpenOptions;
impl Logger for Log {
    fn open( &mut self ) -> Result<(),StorageError> {
        self.fp = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.filename)?);
        Ok(())
    }

    fn write( &mut self, entry : &Entry ) -> Result<(), StorageError> {
        if let Some(fp) = self.fp.as_mut() {
            entry.encode( fp )?;
            Ok(())
        } else {
            Err(StorageError::Custom("uninitialized log"))
        }
    }

    fn read( &mut self, entry : &mut Entry ) -> Result<bool, StorageError> {
        if let Some(fp) = self.fp.as_mut() {
            match entry.decode( fp ) {
                Ok(_) => Ok(false),
                Err(err) => match err {
                    StorageError::EOF => Ok(true),
                    _ => Err(err),
                },
            }
        } else {
            Err(StorageError::Custom("uninitialized log"))
        }
    }
}

