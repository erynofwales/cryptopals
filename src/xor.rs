use std::iter;

pub type FixedByteXOR<T, U> = iter::Map<iter::Zip<T, U>, fn((u8, u8)) -> u8>;
pub type SingleByteXOR<T> = FixedByteXOR<T, iter::Repeat<u8>>;

pub trait FixedXOR<T, U> {
    fn fixed_xor(self, other: U) -> FixedByteXOR<T, U>;
}

impl<'a, 'b, T, U> FixedXOR<T, U> for T
    where T: iter::Iterator<Item=u8> + 'a,
          U: iter::Iterator<Item=u8> + 'b
{
    fn fixed_xor(self, other: U) -> FixedByteXOR<T, U> {
        fn xor(tup: (u8, u8)) -> u8 { tup.0 ^ tup.1 }
        self.zip(other).map(xor)
    }
}

pub trait ByteXOR<T> {
    fn byte_xor(self, byte: u8) -> SingleByteXOR<T>;
}

impl<'a, T> ByteXOR<T> for T where T: iter::Iterator<Item=u8> + 'a {
    fn byte_xor(self, byte: u8) -> SingleByteXOR<T> {
        self.fixed_xor(iter::repeat(byte))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use letter_frequency::LetterFreq;
    use hex::{HexDecodable, HexEncodable};
    use std::f32;

}
