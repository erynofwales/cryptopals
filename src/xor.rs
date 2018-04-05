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
    use hex::{HexDecodable, HexEncodable};

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

    static ENGLISH_LETTER_FREQS: &'static [f32] = &[
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
        0.06094, 0.06996, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
        0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
        0.00978, 0.02360, 0.00150, 0.01974, 0.00074                    // V-Z
    ];

    fn letter_freq_score(input: &str) -> f32 {
        let mut freqs: Vec<f32> = iter::repeat(0.0f32).take(26).collect();
        let mut num_alphabetic_chars = 0f32;
        for c in input.chars() {
            num_alphabetic_chars += 1f32;
            if !c.is_alphabetic() { continue; }
            let c = c.to_ascii_uppercase();
            freqs[c as usize - 'A' as usize] += 1f32;
        }
        let freqs = freqs.into_iter().map(|sc| sc / num_alphabetic_chars);
        // Calculate chi-squared for this string, comparing actual frequencies vs. English letter
        // frequencies.
        let score = freqs.zip(ENGLISH_LETTER_FREQS.iter())
                         .fold(0f32, |acc, (obs, exp)| acc + (obs - exp).powf(2.0) / exp);
        score
    }

    #[test]
    fn cryptopals13() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let mut best_key = 0u8;
        let mut best_score = -1f32;
        let mut best_output: Option<String> = None;
        for key in 32u8..127 {
            let possible_output = input.chars().hex_decoded()
                .single_byte_xor(key)
                .map(|x| char::from(x))
                .collect::<String>();
            let score = letter_freq_score(&possible_output);
            if score > best_score {
                best_score = score;
                best_output = Some(possible_output);
                best_key = key;
            }
        }
        let best_output = best_output.expect("expected output");
        println!("{}: {} -> {}", best_key, best_output, best_score);
    }
}
