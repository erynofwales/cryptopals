use std::iter;

pub type FixedXOR<T, U> = iter::Map<iter::Zip<T, U>, fn((u8, u8)) -> u8>;
pub type SingleByteXOR<T> = FixedXOR<T, iter::Repeat<u8>>;

pub trait FixedXORable<T, U> {
    fn fixed_xor(self, other: U) -> FixedXOR<T, U>;
}

impl<'a, 'b, T, U> FixedXORable<T, U> for T
    where T: iter::Iterator<Item=u8> + 'a,
          U: iter::Iterator<Item=u8> + 'b
{
    fn fixed_xor(self, other: U) -> FixedXOR<T, U> {
        fn xor(tup: (u8, u8)) -> u8 { tup.0 ^ tup.1 }
        self.zip(other).map(xor)
    }
}

pub trait SingleByteXORable<T> {
    fn single_byte_xor(self, byte: u8) -> SingleByteXOR<T>;
}

impl<'a, T> SingleByteXORable<T> for T
    where T: FixedXORable<T, iter::Repeat<u8>>
{
    fn single_byte_xor(self, byte: u8) -> SingleByteXOR<T> {
        self.fixed_xor(iter::repeat(byte))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use letter_frequency::LetterFreq;
    use hex::{HexDecodable, HexEncodable};
    use std::f32;
    use std::ascii::AsciiExt;

    #[test]
    fn cryptopals() {
        println!();
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let ex_output = "746865206b696420646f6e277420706c6179";
        let output: String = a.chars().hex_decoded()
            .fixed_xor(b.chars().hex_decoded())
            .hex_encoded().collect();
        assert_eq!(output, ex_output);
    }

    #[test]
    fn cryptopals13() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let mut best_key = 0u8;
        let mut best_score = f32::INFINITY;
        let mut best_output: Option<String> = None;
        for key in 32u8..127 {
            let possible_output = input.chars().hex_decoded()
                .single_byte_xor(key)
                .map(char::from)
                .collect::<String>();
            let score = possible_output.chi2_freqs("en");
            println!("{}: {:?} -> {}", key, possible_output, score);
            if !score.is_nan() && score < best_score {
                best_score = score;
                best_output = Some(possible_output);
                best_key = key;
            }
        }
        let best_output = best_output.expect("expected output");
        println!("{}: {:?} -> {}", best_key, best_output, best_score);
        assert!(best_output.to_lowercase().contains("bacon"));
    }
}
