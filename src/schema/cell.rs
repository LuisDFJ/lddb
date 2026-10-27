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
            Cell::I64(i) => writer.write_all(&i.to_le_bytes())?,
            Cell::Str(s) => {
                let l : u32 = s.len().try_into()?;
                writer.write_all(&l.to_le_bytes())?;
                writer.write_all(s)?;
            },
        };
        Ok(())
    }
}

impl Decode for Cell {
    fn decode<R>(&mut self, reader : &mut R) -> Result<(), DBError>
        where R : std::io::Read
    {
        match self {
            Cell::I64(_) => {
                let mut buf = [0u8;8];
                reader.read_exact(&mut buf)?;
                *self = Cell::I64(i64::from_le_bytes(buf));
            },
            Cell::Str(_) => {
                // Read Lenght
                let mut buf = [0u8;4];
                reader.read_exact(&mut buf)?;
                let l = u32::from_le_bytes(buf);
                // Read Buffer
                let mut buf : Vec<u8> = Vec::new();
                buf.resize(l as usize, 0);
                reader.read_exact(&mut buf)?;
                *self = Cell::Str(buf);
            },
        }
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

        let mut j = i.from();
        let mut reader = &buffer[..];
        j.decode(&mut reader).unwrap();

        assert_eq!(i,j);
    }
}

