// base64.rs
// Eryn Wells <eryn@erynwells.me>

static B64: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+\\";

pub struct Base64Encoder<T> {
    /// Input iterator
    input: T,
    /// Accumulator. Bits are read into here from the input and shifted out in `next()`.
    acc: u32,
    /// Number of bits to shift the accumulator for the next output byte.
    shift_bits: i8,
    /// Number of padding characters to emit after the accumulator has been drained.
    padding: i8,
}

impl<T> Base64Encoder<T> {
    pub fn new(input: T) -> Base64Encoder<T> {
        Base64Encoder {
            input: input,
            acc: 0,
            shift_bits: -1,
            padding: 0,
        }
    }
}

impl<'a, T> Iterator for Base64Encoder<T> where T: Iterator<Item=u8> + 'a {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.shift_bits < 0 && self.padding == 0 {
            self.get_input_bytes();
        }

        if self.shift_bits >= 0 {
            let char_index = ((self.acc >> self.shift_bits) & 0x3F) as usize;
            let out = B64.chars().nth(char_index);
            println!("out: acc:{:024b}, shift:{:2}, idx:{:08b}->{:?}", self.acc, self.shift_bits, char_index, out);
            self.shift_bits -= 6;
            out
        } else if self.padding > 0 {
            self.padding -= 1;
            Some('=')
        } else {
            None
        }
    }
}

impl<'a, T> Base64Encoder<T> where T: Iterator<Item=u8> + 'a {
    fn get_input_bytes(&mut self) {
        let input_bytes = self.take_from_input(3);
        let num_bits: i8 = input_bytes.len() as i8 * 8;

        if num_bits != 0 {
            // Shift over a few more bits to make sure the accumulator is divisible by 6.
            let makeup_shift = (24 - num_bits) % 6;
            self.acc = input_bytes.into_iter().fold(0, |acc, nxt| (acc << 8) + (nxt as u32)) << makeup_shift;
            self.shift_bits = (num_bits + makeup_shift) - 6;
            self.padding = (24 - num_bits) / 6;
        } else {
            self.acc = 0;
            self.shift_bits = -1;
            self.padding = -1;
        }

        println!("get: acc:{:024b}, shift:{:2}, padding:{}", self.acc, self.shift_bits, self.padding);
    }

    fn take_from_input(&mut self, n: usize) -> Vec<T::Item> {
        let mut input_bytes: Vec<T::Item> = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(x) = self.input.next() {
                input_bytes.push(x);
            } else {
                break;
            }
        }
        input_bytes
    }
}

pub trait Base64Encodable<T> {
    fn base64_encoded(self) -> Base64Encoder<T>;
}

impl<'a, T> Base64Encodable<T> for T where T: Iterator<Item=u8> + 'a {
    fn base64_encoded(self) -> Base64Encoder<T> { Base64Encoder::new(self) }
}

#[cfg(test)]
mod tests {
    use hex::*;
    use super::*;

    #[test]
    fn small_wikipedia_example() {
        let input = "4d616e";
        let ex_output = "TWFu";
        let output: String = input.chars().hex_decoded().base64_encoded().collect();
        assert_eq!(output, ex_output);
    }

    #[test]
    fn one_byte_padding() {
        println!("");
        let input = "6f6d";
        let ex_output = "b20=";
        let output: String = input.chars().hex_decoded().base64_encoded().collect();
        assert_eq!(output, ex_output);
    }

    #[test]
    fn two_byte_padding() {
        println!("");
        let input = "6f";
        let ex_output = "bw==";
        let output: String = input.chars().hex_decoded().base64_encoded().collect();
        assert_eq!(output, ex_output);
    }

    #[test]
    fn cryptopals() {
        println!("");
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let ex_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let output: String = input.chars().hex_decoded().base64_encoded().collect();
        assert_eq!(output, ex_output);
    }
}
