use super::{Encode,Decode,EncodeError,DecodeError};

macro_rules! impl_codec_for_int {
    ($($t:ty),* $(,)?) => {
        $(
            impl Encode for $t {
                fn encoded_len(&self) -> usize { std::mem::size_of::<$t>() }
                fn encode(&self, output : &mut [u8]) -> Result<usize, EncodeError> {
                    let needed = self.encoded_len();
                    if output.len() < needed { 
                        return Err(EncodeError{ needed });
                    }
                    output[0..needed].copy_from_slice(&self.to_le_bytes());
                    Ok(needed)
                }
            }

            impl <'a> Decode<'a> for $t {
                fn decode(input : &mut &'a [u8]) -> Result<Self, DecodeError> {
                    let needed = std::mem::size_of::<$t>();
                    if input.len() < needed {
                        return Err(DecodeError::Incomplete { needed:needed-input.len() });
                    }
                    let (head, rest) = input.split_at(needed);
                    *input = rest;
                    Ok(<$t>::from_le_bytes(head.try_into().unwrap()))
                }
            }
        )*
    };
}

impl_codec_for_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
