pub trait Hash {
    fn hash(&self) -> u16;
}

impl Hash for &[u8] {
    #[inline(always)]
    fn hash(&self) -> u16 {
        let len = self.len();
        let mut hash = len as u16 ^ self[0] as u16;
        hash = hash.rotate_left(4) ^ self[1] as u16;
        hash = hash.rotate_left(7) ^ self[2] as u16;
        hash = hash.rotate_left(11) ^ self[len - 3] as u16;
        hash
    }
}
