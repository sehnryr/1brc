pub trait Hash {
    fn hash(&self) -> u16;
}

impl Hash for &[u8] {
    #[inline(always)]
    fn hash(&self) -> u16 {
        let mut hash: u16 = self.len() as u16;

        for &byte in self.iter().take(5) {
            hash = hash.rotate_left(3) ^ byte as u16;
        }

        for &byte in self.iter().rev().take(5) {
            hash = hash.rotate_left(3) ^ byte as u16;
        }

        hash
    }
}
