// Wrapper for single call
pub fn crc32_hash( bytes : &[u8] ) -> u32 {
    crc32_hash_update(0xFFFF_FFFF, bytes) ^ 0xFFFF_FFFF
}

// Algorithm based on CRC32-IEEE
// consulted at: https://wiki.osdev.org/CRC32
pub fn crc32_hash_update( mut crc : u32, bytes : &[u8] ) -> u32 {
    for &b in bytes {
        // Look Up on current crc XOR current b
        let idx = ( crc as u8 ^ b ) as usize;
        // XOR Look Up value with current crc shifted 8b
        crc = LOOK_UP[ idx ] ^ (crc >> 8);
    }
    crc
}

// CRC32 Polynomial
pub const POLY : u32 = 0xEDB8_8320;

// Filling the CRC32 Look Up Table
const LOOK_UP : [u32;256] = {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;
        while j < 8 {
            // 8 bit shift to the right
            // XOR with POLY if lsb is 1
            crc = if crc & 1 != 0 { (crc >> 1) ^ POLY }
            else { crc >> 1 };
            j = j + 1;
        }

        table[i] = crc;
        i = i + 1;
    }
    table
};
