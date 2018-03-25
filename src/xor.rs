pub fn fixed(a: &str, b: &str) -> Result<String, String> {
    let a_decoded = a.chars().map(|c| c.to_digit(16).unwrap() as u16);
    let b_decoded = b.chars().map(|c| c.to_digit(16).unwrap() as u16);
    let xor = a_decoded.zip(b_decoded).map(|(a, b)| a ^ b as char);
    String::from_iter(xor)
}

#[cfg(test)]
mod tests {
    use super::fixed;

    #[test]
    fn cryptopals() {
        let a = "1c0111001f010100061a024b53535009181c";
        let b = "686974207468652062756c6c277320657965";
        let ex_output = "746865206b696420646f6e277420706c6179";
        let output = fixed(a, b);
        assert_eq!(output, ex_output);
    }
}
