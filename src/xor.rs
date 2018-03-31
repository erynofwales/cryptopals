use std::iter::{Iterator, Map, Zip};

//#[derive(Debug)]
//pub enum FixedXORError {
//    Bad,
//}

pub trait FixedXORable<T> {
    fn fixed_xor(self, other: T) -> Map<Zip<T, T>, fn((u8, u8)) -> u8>;
}

impl<'a, T> FixedXORable<T> for T
    where T: Iterator<Item=u8> + 'a,
{
    fn fixed_xor(self, other: T) -> Map<Zip<T, T>, fn((u8, u8)) -> u8> {
        fn xor(tup: (u8, u8)) -> u8 { tup.0 ^ tup.1 }
        self.zip(other).map(xor)
    }
}

//pub fn fixed(a: &str, b: &str) -> Result<String, FixedXORError> {
//    let a_decoded = a.hex_bytes().valid();
//    let b_decoded = b.hex_bytes().valid();
//    let xor: Vec<u8> = a_decoded.zip(b_decoded).map(|(x, y)| x ^ y).collect();
//    Ok(xor.hex_digest())
//}
//
//#[cfg(test)]
//mod tests {
//    use hex::AsHexBytes;
//    use std::char;
//    use super::fixed;
//
//    #[test]
//    fn cryptopals() {
//        let a = "1c0111001f010100061a024b53535009181c";
//        let b = "686974207468652062756c6c277320657965";
//        let ex_output = "746865206b696420646f6e277420706c6179";
//        let output = fixed(a, b);
//        assert_eq!(output.unwrap(), ex_output);
//    }
//
//    static ENGLISH_LETTER_FREQ: &'static str = "EARIOTNSLCUDPMHGBFYWKVXZJQ";
//
//    fn letter_freq_score(input: &str) -> f32 {
//        let mut score: f32 = 0.0;
//        score
//    }
//
//    #[test]
//    fn cryptopals13() {
//        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
//        for key in 32u32..127 {
//            let possible_output = input.hex_bytes().valid()
//                .map(|c| char::from_u32(c as u32 ^ key))
//                .take_while(|c| c.is_some())
//                .map(|c| c.unwrap())
//                .collect::<String>();
//            println!("{}: {}", key, possible_output);
//        }
//    }
//}
