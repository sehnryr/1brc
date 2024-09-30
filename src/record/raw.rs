use crate::hash::Hash;

pub struct RawRecord<'a> {
    pub city: &'a [u8],
    pub temperature: i32,
}

impl<'a> RawRecord<'a> {
    #[inline(always)]
    pub fn new(city: &'a [u8], temperature: i32) -> Self {
        Self { city, temperature }
    }
}

impl<'a> Hash for RawRecord<'a> {
    #[inline(always)]
    fn hash(&self) -> u16 {
        self.city.hash()
    }
}
