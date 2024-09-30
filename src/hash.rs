trait HashStep {
    fn step_hash_1(&mut self, byte: u8);
}

impl HashStep for u16 {
    #[inline(always)]
    fn step_hash_1(&mut self, byte: u8) {
        *self = self.rotate_left(3) ^ byte as u16;
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
            [d0, d1, d2, d3, d4, ..] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
                hash.step_hash_1(*d3);
                hash.step_hash_1(*d4);
            }
            [d0, d1, d2, d3] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
                hash.step_hash_1(*d3);
            }
            [d0, d1, d2] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
            }
            _ => {}
        }

        match self {
            [.., d4, d3, d2, d1, d0] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
                hash.step_hash_1(*d3);
                hash.step_hash_1(*d4);
            }
            [d3, d2, d1, d0] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
                hash.step_hash_1(*d3);
            }
            [d2, d1, d0] => {
                hash.step_hash_1(*d0);
                hash.step_hash_1(*d1);
                hash.step_hash_1(*d2);
            }
            _ => {}
        }

        hash
    }
}
