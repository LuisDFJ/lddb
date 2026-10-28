use super::{Encode,Decode,EncodeError,DecodeError};

impl <T: Encode + ?Sized> Encode for &T {
    fn encoded_len(&self) -> usize { (**self).encoded_len() }
    fn encode(&self, output : &mut [u8]) -> Result<usize,EncodeError> {
        (**self).encode(output)
    }
}

impl <T: Encode> Encode for [T] {
    fn encoded_len(&self) -> usize { 
        4 + self.iter().map(Encode::encoded_len).sum::<usize>()
    }
    fn encode(&self, output : &mut [u8]) -> Result<usize, EncodeError> {
        let needed = self.encoded_len();
        if output.len() < needed {
            return Err( EncodeError{ needed } );
        }
        let len = (needed - 4) as u32;
        let mut pos = len.encode(output)?;
        for item in self {
            pos += item.encode(&mut output[pos..])?;
        }
        Ok(pos)
    }
}

impl <T: Encode> Encode for Vec<T> {
    fn encoded_len(&self) -> usize { self.as_slice().encoded_len() }
    fn encode(&self, output : &mut [u8]) -> Result<usize, EncodeError> {
        self.as_slice().encode(output)
    }
}

impl <'a, T: Decode<'a>> Decode<'a> for Vec<T> {
    fn decode(input : &mut &'a [u8]) -> Result<Self, DecodeError> {
        let needed = u32::decode(input)? as usize;
        if input.len() < needed {
            return Err(DecodeError::Incomplete {needed:needed-input.len()});
        }
        let (mut body, rest) = input.split_at(needed);

        let mut out = Vec::new();
        while !body.is_empty() {
            let before = body.len();
            let item = T::decode(&mut body)?;
            if body.len() == before {
                return Err(DecodeError::Invalid("element consumed no bytes"));
            }
            out.push(item);
        }

        *input = rest;
        Ok(out)
    }
}
