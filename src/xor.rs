use hex::{AsHexBytes, HexDigest};

#[derive(Debug)]
pub enum FixedXORError {
    Bad,
}

pub fn fixed(a: &str, b: &str) -> Result<String, FixedXORError> {
    let a_decoded = a.hex_bytes().valid();
    let b_decoded = b.hex_bytes().valid();
    let xor: Vec<u8> = a_decoded.zip(b_decoded).map(|(x, y)| x ^ y).collect();
    Ok(xor.hex_digest())
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
        assert_eq!(output.unwrap(), ex_output);
    }
}
