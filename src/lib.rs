static B64: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+\\";

pub fn base64(hex: &str) -> Result<String, String> {
    let mut out = String::from("");
    let mut acc: u32 = 0;
    for (idx, c) in hex.char_indices() {
        if let Some(c_digit) = c.to_digit(16) {
            // Accumulate bytes until we have 24 of them. (We can do this with any number of bytes % 6.
            acc = (acc << 4) + c_digit;
            println!("idx={}, c='{}'({:2}), acc={:024b}", idx, c, c_digit, acc);
            if idx % 6 != 5 {
                continue;
            }
        
            // Read out bytes in chunks of 6.
            for i in (0..4).rev() {
                let out_char_idx = ((acc >> (6 * i)) & 0x3F) as usize;
                println!("  i:{}, shift:{}, out_char_idx:{:08b}", i, 6*i, out_char_idx); 
                // TODO: I don't like this nth() call.
                if let Some(out_char) = B64.chars().nth(out_char_idx) {
                    out.push(out_char);
                } else {
                    return Err(format!("Couldn't make output char from {} (shifted: {})", out_char_idx, 6 * i));
                }
            }
            acc = 0;
        } else {
            return Err(format!("Invalid input: {}", c));
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn simple_single() {
        let input = "4d616e";
        let ex_output = "TWFu";
        println!("");
        let output = base64(input).expect("Error converting to base64");
        assert_eq!(output, ex_output);
    }

    #[test]
    fn padding() {
        let input = "6f6d";
        let ex_output = "b20=";
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
