use crate::hash::Hash;

pub struct RawRecord<'a> {
    city: &'a [u8],
    temperature: i32,
}

impl<'a> RawRecord<'a> {
    #[inline(always)]
    pub fn new(city: &'a [u8], temperature: i32) -> Self {
        Self { city, temperature }
    }

    #[inline(always)]
    pub fn city(&self) -> &'a [u8] {
        self.city
    }

    #[inline(always)]
    pub fn temperature(&self) -> i32 {
        self.temperature
    }
}

impl<'a> Hash for RawRecord<'a> {
    #[inline(always)]
    fn hash(&self) -> u16 {
        self.city.hash()
    }
}
