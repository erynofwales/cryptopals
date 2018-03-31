use std::char;
use std::iter;
use std::str::Chars;

pub trait HexDecodable<T> {
    fn hex_decoded(self) -> HexDecoder<T>;
}

pub struct HexDecoder<T> {
    input: T,
}

impl<T> HexDecoder<T> {
    pub fn new(input: T) -> HexDecoder<T> {
        HexDecoder { input: input }
    }
}

impl<'a, T> Iterator for HexDecoder<T> where T: Iterator<Item=char> + 'a {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let c0 = match self.input.next() {
            Some(ch) => match ch.to_digit(16) {
                Some(digit) => digit as u8,
                None => return None
            },
            None => return None
        };

        let c1 = match self.input.next() {
            Some(ch) => match ch.to_digit(16) {
                Some(digit) => Some(digit as u8),
                None => return None
            },
            None => None
        };

        let out_char = if let Some(c1) = c1 {
            (c0 << 4) + (c1 & 0x0F)
        } else {
            c0 & 0x0F
        };

        Some(out_char)
    }
}

impl<'a, T> HexDecodable<T> for T where T: Iterator<Item=char> + 'a {
    fn hex_decoded(self) -> HexDecoder<T> { HexDecoder::new(self) }
}

pub trait HexEncodable<T> {
    fn hex_encoded(self) -> HexEncoder<T>;
}

pub struct HexEncoder<T> {
    input: T,
    current_byte: u8,
}

impl<T> HexEncoder<T> {
    pub fn new(input: T) -> HexEncoder<T> {
        HexEncoder { input: input, current_byte: 0 }
    }
}

impl<'a, T> Iterator for HexEncoder<T> where T: Iterator<Item=u8> + 'a {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_byte == 0 {
            // Get a new byte.
            match self.input.next() {
                Some(byte) => {
                    // Swap the order of the nibbles to make the code below nicer.
                    self.current_byte = (byte << 4) + (byte >> 4);
                }
                None => return None
            }
        }
        // Convert the lowest order nibble to a hex digit.
        match char::from_digit((self.current_byte & 0xF) as u32, 16) {
            Some(ch) => {
                self.current_byte = self.current_byte >> 4;
                return Some(ch);
            },
            None => panic!("This will never happen???")
        }
    }
}

impl<'a, T> HexEncodable<T> for T where T: Iterator<Item=u8> + 'a {
    fn hex_encoded(self) -> HexEncoder<T> { HexEncoder::new(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_encode() {
        let collected: String = "123".bytes().hex_encoded().collect();
        assert_eq!(collected, "313233");
    }

    #[test]
    fn simple_decode() {
        let decoded: Vec<u8> = "313233".chars().hex_decoded().collect();
        assert_eq!(decoded, vec![0x31u8, 0x32u8, 0x33u8]);
    }
}
