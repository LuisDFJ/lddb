const TAG_I64 : u8 = 0;
const TAG_STR : u8 = 1;

#[derive(Debug,PartialEq)]
pub enum Cell {
    I64(i64),
    Str(Vec<u8>),
}

impl Cell {
    pub fn from(&self) -> Cell {
        match self {
            Cell::I64(_) => Cell::I64(0),
            Cell::Str(_) => Cell::Str(b"".into())
        }
    }
}

use crate::serial::{Encode, Decode};
use crate::errors::DBError;
impl Encode for Cell {
    fn encode<W>(&self, writer : &mut W) -> Result<(), DBError>
        where W : std::io::Write
    {
        match self {
            Cell::I64(i) => {
                TAG_I64.encode(writer)?;
                i.encode(writer)
            },
            Cell::Str(s) => {
                TAG_STR.encode(writer)?;
                s.encode(writer)
            },
        }
    }
}

impl Decode for Cell {
    fn decode<R>(&mut self, reader : &mut R) -> Result<(), DBError>
        where R : std::io::Read
    {
        let mut tag = 0u8;
        tag.decode(reader)?;
        match tag {
            TAG_I64 => {
                let mut i = 0i64;
                i.decode(reader)?;
                *self = Cell::I64(i);
            },
            TAG_STR => {
                let mut s = Vec::new();
                s.decode(reader)?;
                println!("Hello {:?}", s);
                *self = Cell::Str(s);
            },
            _ => return Err(DBError::UnexpectedType),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_cell_i64() {
        let mut buffer = vec![];
        let i = Cell::I64(1234);
        i.encode(&mut buffer).unwrap();

        let mut j = i.from();
        let mut reader = &buffer[..];
        j.decode(&mut reader).unwrap();

        assert_eq!(i,j);
    }

    #[test]
    fn test_cell_str() {
        let mut buffer = vec![];
        let i = Cell::Str(b"1234 1234".into());
        i.encode(&mut buffer).unwrap();

        let mut j = Cell::I64(0);
        let mut reader = &buffer[..];
        j.decode(&mut reader).unwrap();

        assert_eq!(i,j);
    }
}

