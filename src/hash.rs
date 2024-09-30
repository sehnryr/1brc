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
        let mut hash: u16 = self.len() as u16;

        match self {
            [d0, d1, d2, ..] => {
                hash = hash.rotate_xor(*d0, 3);
                hash = hash.rotate_xor(*d1, 7);
                hash = hash.rotate_xor(*d2, 5);
            }
            _ => unreachable!(),
        }

        match self {
            [.., d2, d1, d0] => {
                hash = hash.rotate_xor(*d0, 3);
                hash = hash.rotate_xor(*d1, 5);
                hash = hash.rotate_xor(*d2, 7);
            }
            _ => unreachable!(),
        }

        hash
    }
}
