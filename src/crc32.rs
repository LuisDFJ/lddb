mod crc32_1_fold;
mod crc32_8_fold;
mod crc32_reader;

pub use self::crc32_1_fold::crc32_hash;
pub use self::crc32_8_fold::crc32_hash_fast;
pub use self::crc32_reader::CRC32Reader;


#[cfg(test)]
mod test {
    use std::io::Read;

use super::*;
    #[test]
    fn test_crc32_hash() {
        // Results compared with CRC-32/ISO-HDLC at https://crccalc.com
        assert_eq!(crc32_hash(b"123456789"), 0xCBF4_3926 );
        assert_eq!(crc32_hash(b"hello world"), 0x0D4A_1185 );
        assert_eq!(crc32_hash(b"this is a test"), 0x0D1E_E7EA );
    }
    #[test]
    fn test_crc32_hash_fast() {
        let test_text : Vec<&[u8]> = vec![
            b"hello world",
            b"128394 919288 1928383 nnajdjadh",
            b"this is a very complicated test",
            b"testing that both crc32_hash and crc32_hash_fast match results",
        ];
        for t in test_text {
            assert_eq!(crc32_hash(t), crc32_hash_fast(t));
        }
    }
    #[test]
    fn test_crc32_reader() {
        let text = b"128394 919288 1928383 nnajdjadh";
        let mut reader = &text[..];
        let mut crc32_reader = CRC32Reader::new(&mut reader);

        let mut buf = vec![];
        crc32_reader.read_to_end(&mut buf).unwrap();
        assert_eq!( crc32_reader.finalize(), crc32_hash(text) );
    }
}
