use hex::{AsHexBytes, HexResult};

static B64: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+\\";

pub fn base64(hex: &str) -> Result<String, String> {
    let mut out = String::from("");
    let mut num_bits = 0;
    let mut acc: u32 = 0;
    for (idx, c) in hex.hex_bytes().enumerate() {
        match c {
            HexResult::Byte(c) => {
                // Accumulate bytes until we have 6 chunks of 4.
                acc = (acc << 4) + (c as u32);
                num_bits += 4;
                if idx % 6 != 5 {
                    continue;
                }
        
                // Read out 4 chunks of 6.
                for i in (0..4).rev() {
                    let out_char_idx = ((acc >> (6 * i)) & 0x3F) as usize;
                    // TODO: I don't like this nth() call.
                    if let Some(out_char) = B64.chars().nth(out_char_idx) {
                        out.push(out_char);
                    } else {
                        return Err(format!("Couldn't make output char from {}", out_char_idx));
                    }
                }
                acc = 0;
                num_bits = 0;
            },
            HexResult::Invalid(c) => {
                return Err(format!("Invalid input: {}", c));
            },
        }
    }

    if acc != 0 {
        // Pad the string if we have bits remaining.
        acc = acc << (24 - num_bits);
        let padding = (24 - num_bits) / 6;
        for i in (0..4).rev() {
            let out_char_idx = ((acc >> (6 * i)) & 0x3F) as usize;
            if i < padding {
                out.push('=');
            } else if let Some(out_char) = B64.chars().nth(out_char_idx) {
                out.push(out_char);
            } else {
                return Err(format!("Couldn't make output char from {}", out_char_idx));
            }
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn small_wikipedia_example() {
        let input = "4d616e";
        let ex_output = "TWFu";
        println!("");
        let output = base64(input).expect("Error converting to base64");
        assert_eq!(output, ex_output);
    }

    #[test]
    fn one_byte_padding() {
        let input = "6f6d";
        let ex_output = "b20=";
        println!("");
        let output = base64(input).expect("Error converting to base64");
        assert_eq!(output, ex_output);
    }

    #[test]
    fn two_byte_padding() {
        let input = "6f";
        let ex_output = "bw==";
        println!("");
        let output = base64(input).expect("Error converting to base64");
        assert_eq!(output, ex_output);
    }

    #[test]
    fn cryptopals_input() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let ex_output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        println!("");
        let output = base64(input).expect("Error converting to base64");
        assert_eq!(output, ex_output);
    }
}
