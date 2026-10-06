// Wrapper for single call
pub fn crc32_hash_fast( bytes : &[u8] ) -> u32 {
    crc32_hash_fast_update(0xFFFF_FFFF, bytes) ^ 0xFFFF_FFFF
}

// Algorithm based on CRC32-IEEE
// consulted at: https://wiki.osdev.org/CRC32
use super::crc32_1_fold::crc32_hash_update;
pub fn crc32_hash_fast_update( mut crc : u32, bytes : &[u8] ) -> u32 {
    let mut chunks = bytes.chunks_exact(8);
    for chunk in &mut chunks {
        // Combine first 4 bytes with current crc
        let c = crc ^ u32::from_le_bytes([chunk[0],chunk[1],chunk[2],chunk[3]]);
        // Slicing-by-8 lookup method
        crc = LOOK_UP[0][ chunk[7] as usize ]
            ^ LOOK_UP[1][ chunk[6] as usize ]
            ^ LOOK_UP[2][ chunk[5] as usize ]
            ^ LOOK_UP[3][ chunk[4] as usize ]
            ^ LOOK_UP[4][ (c>>24) as u8 as usize ]
            ^ LOOK_UP[5][ (c>>16) as u8 as usize ]
            ^ LOOK_UP[6][ (c>> 8) as u8 as usize ]
            ^ LOOK_UP[7][ c       as u8 as usize ];
    }
    crc32_hash_update(crc, chunks.remainder())
}

// Filling the CRC32 Look Up Table
use super::crc32_1_fold::POLY;
const LOOK_UP : [[u32;256];8] = {
    let mut table = [[0u32; 256];8];
    // First Stage: Initialization
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

        table[0][i] = crc;
        i = i + 1;
    }

    // Second Stage: Precompute
    let mut t = 1;
    while t < 8 {
        let mut i = 0;
        while i < 256 {
            let prev = table[t-1][i];
            // T[i] at t = T[i] at (t-1) >> 8 XOR T[T[i] at t-1 as u8] at 0
            table[t][i] = (prev >> 8) ^ table[0][(prev as u8) as usize];
            i += 1;
        }
        t += 1;
    }
    table
};
