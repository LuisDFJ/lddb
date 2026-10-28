mod arrays;
mod integers;

#[derive(Debug)]
pub enum DecodeError {
    Incomplete { needed: usize },
    Invalid(&'static str),
}

#[derive(Debug)]
pub struct EncodeError { pub needed: usize }

pub trait Encode {
    fn encoded_len(&self) -> usize;
    fn encode(&self, output : &mut [u8]) -> Result<usize, EncodeError>;
}

pub trait Decode<'a> : Sized {
    fn decode(input : &mut &'a [u8]) -> Result<Self, DecodeError>;
}

