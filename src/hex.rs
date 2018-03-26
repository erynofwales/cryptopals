use std::char;
use std::iter;
use std::str::Chars;

pub enum HexResult {
    Byte(u8),
    Invalid(char),
}

pub struct HexBytes<'a> {
    input: Chars<'a>,
}

impl<'a> HexBytes<'a> {
    fn new(input: &'a str) -> HexBytes {
        HexBytes { input: input.chars() }
    }

    pub fn valid(self) -> ValidHexBytes<'a> {
        ValidHexBytes::new(self)
    }
}

impl<'a> iter::Iterator for HexBytes<'a> {
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

/*
 * ValidHexBytes --
 */

pub struct ValidHexBytes<'a> {
    input: HexBytes<'a>,
}

impl<'a> ValidHexBytes<'a> {
    fn new(input: HexBytes) -> ValidHexBytes {
        ValidHexBytes { input: input }
    }
}

impl<'a> iter::Iterator for ValidHexBytes<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.next() {
                Some(hex_c) => match hex_c {
                    HexResult::Byte(c) => return Some(c),
                    _ => continue,
                },
                None => return None,
            }
        }
    }
}

/*
 * HexDigest --
 */

pub trait HexDigest {
    fn hex_digest(self) -> String;
}

impl<'a> HexDigest for HexBytes<'a> {
    fn hex_digest(self) -> String {
        self.valid().hex_digest()
    }
}

impl<'a> HexDigest for ValidHexBytes<'a> {
    fn hex_digest(self) -> String {
        self.map(|x| char::from_digit(x as u32, 16).unwrap()).collect()
    }
}

impl HexDigest for Vec<u8> {
    fn hex_digest(self) -> String {
        self.into_iter().map(|x| char::from_digit(x as u32, 16).unwrap()).collect()
    }
}

impl<'a> HexDigest for &'a [u8] {
    fn hex_digest(self) -> String {
        self.into_iter().map(|x| char::from_digit(*x as u32, 16).unwrap()).collect()
    }
}

/*
 * AsHexbytes --
 */

pub trait AsHexBytes<'a> {
    fn hex_bytes(&self) -> HexBytes<'a>;
}

impl<'a> AsHexBytes<'a> for &'a str {
    fn hex_bytes(&self) -> HexBytes<'a> { HexBytes::new(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let collected: Vec<u8> = "123".hex_bytes().map(|c| match c {
            HexResult::Byte(c) => c,
            _ => panic!(),
        }).collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }
}
