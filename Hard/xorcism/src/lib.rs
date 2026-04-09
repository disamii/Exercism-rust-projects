use std::borrow::Borrow;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    offset: usize,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        Self {
            key: key.as_ref(),
            offset: 0,
        }
    }

    // pub fn munge_in_place(&mut self, data: &mut [u8]) {
    //     for byte in data.into_iter() {
    //         *byte ^= self.key[self.offset];
    //         self.offset = (self.offset + 1) % self.key.len();
    //     }
    // }
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for byte in data.into_iter() {
            *byte ^= self.key[self.offset];
            self.offset = (self.offset + 1) % self.key.len();
        }
    }


    pub fn munge<'b, Data, T>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        Data: IntoIterator<Item = T> + 'b,
        T: Borrow<u8>,
    {
        data.into_iter().map(|byte| {
            let ret = byte.borrow() ^ self.key[self.offset];
            self.offset = (self.offset + 1) % self.key.len();
            ret
        })
    }


}

