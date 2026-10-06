pub struct CRC32Reader<'a, R>
where R : std::io::Read
{
    reader: &'a mut R,
    crc   : u32,
}

impl <'a,R> CRC32Reader<'a,R>
    where R : std::io::Read
{
    pub fn new( reader : &'a mut R ) -> Self {
        CRC32Reader { reader, crc: 0xFFFF_FFFF }
    }
    pub fn finalize( &self ) -> u32 {
        self.crc ^ 0xFFFF_FFFF
    }
}

use super::crc32_8_fold::crc32_hash_fast_update;
impl <'a,R> std::io::Read for CRC32Reader<'a,R>
    where R : std::io::Read
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.reader.read(buf)?;
        if size > 0 {
            self.crc = crc32_hash_fast_update(self.crc, &buf[..size]);
        }
        Ok(size)
    }
}
