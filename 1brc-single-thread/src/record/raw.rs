use super::hash::Hash;

pub struct RawRecord<'a> {
    city: &'a [u8],
    temperature: i64,
}

impl<'a> RawRecord<'a> {
    #[inline(always)]
    pub fn new(city: &'a [u8], temperature: i64) -> Self {
        Self { city, temperature }
    }

    #[inline(always)]
    pub fn city(&self) -> &'a [u8] {
        self.city
    }

    #[inline(always)]
    pub fn temperature(&self) -> i64 {
        self.temperature
    }
}

impl<'a> Hash for RawRecord<'a> {
    #[inline(always)]
    fn hash(&self) -> u16 {
        self.city.hash()
    }
}
