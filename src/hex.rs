use std::iter;
use std::str::Chars;

pub enum HexResult {
    Byte(u8),
    Invalid(char),
}

pub struct HexDecoder<'a> {
    input: Chars<'a>,
}

impl<'a> HexDecoder<'a> {
    fn new(input: &'a str) -> HexDecoder {
        HexDecoder { input: input.chars() }
    }
}

impl<'a> iter::Iterator for HexDecoder<'a> {
    type Item = HexResult;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.input.next() {
            if let Some(hex) = c.to_digit(16) {
                Some(HexResult::Byte(hex as u8))
            } else {
                Some(HexResult::Invalid(c))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let decoder = HexDecoder::new("123");
        let collected: Vec<u8> = decoder.map(|c| match c {
            HexResult::Byte(c) => c,
            _ => panic!(),
        }).collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
