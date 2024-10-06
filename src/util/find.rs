use std::simd::{cmp::SimdPartialEq, u8x32};

type Result<T> = std::result::Result<T, ()>;

pub trait FindByte {
    fn find_byte(&self, byte: u8) -> Result<usize>;
}

impl FindByte for [u8] {
    #[inline(always)]
    fn find_byte(&self, byte: u8) -> Result<usize> {
        let simd_width = 32; // 256-bit wide SIMD register

        for (i, chunk) in self.chunks_exact(simd_width).enumerate() {
            // Load 32 bytes into a 256-bit SIMD register
            let chunk = u8x32::from_slice(chunk);

            // Compare each byte in the chunk with the needle
            let cmp = chunk.simd_eq(u8x32::splat(byte));

            // Convert the comparison result to a bitmask vector
            let masks = cmp.to_bitmask_vector();

            // Find the first non-zero mask which indicates a match
            for j in 0..simd_width {
                let mask = masks[j];
                if mask != 0 {
                    // Calculate the position of the match
                    return Ok(i * simd_width + j * 8 + mask.trailing_zeros() as usize);
                }
            }
        }

        // Handle the remaining bytes
        self.chunks_exact(simd_width)
            .remainder()
            .iter()
            .position(|&c| c == byte)
            .ok_or(())
    }
}
