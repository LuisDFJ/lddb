use crate::errors::DBError;
use std::io::{Read,Write};

pub trait Encode {
    fn encode<W : Write>(&self, writer : &mut W) -> Result<(), DBError>;
}

pub trait Decode {
    fn decode<R : Read>(&mut self, reader : &mut R) -> Result<(), DBError>;
}

impl <T: Encode + ?Sized> Encode for &T {
    fn encode<W : Write>(&self, writer : &mut W) -> Result<(), DBError> {
        (**self).encode(writer)
    }
}

impl <T: Encode> Encode for [T] {
    fn encode<W : Write>(&self, writer : &mut W) -> Result<(), DBError> {
        let mut buffer = Vec::new();
        for item in self {
            item.encode(&mut buffer)?;
        }
        writer.write_all(&buffer)?;
        Ok(())
    }
}

impl <T: Encode> Encode for Vec<T> {
    fn encode<W : Write>(&self, writer : &mut W) -> Result<(), DBError> {
        self.as_slice().encode(writer)
    }
}

