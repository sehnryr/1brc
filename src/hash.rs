trait HashStep {
    fn rotate_xor(&self, byte: u8, shift: u32) -> u16;
}

impl HashStep for u16 {
    #[inline(always)]
    fn rotate_xor(&self, byte: u8, shift: u32) -> u16 {
        self.rotate_left(shift) ^ byte as u16
    }
}

pub trait Hash {
    fn hash(&self) -> u16;
}

impl Hash for &[u8] {
    #[inline(always)]
    fn hash(&self) -> u16 {
        let len = self.len();
        let mut hash: u16 = len as u16;

        hash = hash.rotate_xor(self[0], 3);
        hash = hash.rotate_xor(self[1], 7);
        hash = hash.rotate_xor(self[2], 5);

        hash = hash.rotate_xor(self[len - 1], 3);
        hash = hash.rotate_xor(self[len - 2], 5);
        hash = hash.rotate_xor(self[len - 3], 7);

        hash
    }
}
