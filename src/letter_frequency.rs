// letter_frequency.rs
// Eryn Wells <eryn@erynwells.me>

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub type LetterSet = HashSet<String>;
pub type FreqMap = HashMap<String, f32>;

static ENGLISH_LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static ENGLISH_LETTER_FREQS: &'static [f32] = &[
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06996, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074                    // V-Z
];

pub fn english_letters() -> LetterSet {
    LetterSet::from_iter(ENGLISH_LETTERS.chars().map(|c| c.to_string()))
}

pub fn english_letter_freqs() -> FreqMap {
    let char_strings = ENGLISH_LETTERS.chars().map(|c| c.to_string());
    FreqMap::from_iter(char_strings.zip(ENGLISH_LETTER_FREQS.iter().map(|x| *x)))
}

pub trait LetterFreq {
    fn letter_freqs(&self, lang: &str) -> FreqMap;
    fn chi2_freqs(&self, lang: &str) -> f32;
}

impl<'a> LetterFreq for &'a str {
    fn letter_freqs(&self, lang: &str) -> FreqMap {
        assert_eq!(lang, "en", "only 'en' language is supported rn");
        let english_letters = english_letters();
        let mut freqs = FreqMap::new();
        for c in self.chars() {
            let c_str = c.to_uppercase().to_string();
            if english_letters.contains(&c_str) {
                *freqs.entry(c_str).or_insert(0f32) += 1f32;
            }
            println!("{:?}: {:?}", c, freqs);
        }
        freqs
    }

    fn chi2_freqs(&self, lang: &str) -> f32 {
        assert_eq!(lang, "en", "only 'en' language is supported rn");
        // Calculate chi-squared for this string, comparing actual frequencies vs. English letter frequencies.
        // https://en.wikipedia.org/wiki/Letter_frequency
        // https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
        let freqs = self.letter_freqs(lang);
        let english_freqs = english_letter_freqs();
        let num_letters = freqs.values().sum::<f32>();
        println!("freqs:{:?}, num:{}", freqs, num_letters);
        let score = english_freqs.into_iter()
            .map(|(c, sc)| (freqs.get(&c).map_or(0f32, |c| *c), sc * num_letters))
            .inspect(|c| println!("{:?}", c))
            .fold(0f32, |acc, (obs, exp)| acc + ((obs - exp).powf(2.0) / exp));
        println!("chi2 -> {}", score);
        score
    }
}

impl LetterFreq for String {
    fn letter_freqs(&self, lang: &str) -> FreqMap {
        self.as_str().letter_freqs(lang)
    }

    fn chi2_freqs(&self, lang: &str) -> f32 {
        self.as_str().chi2_freqs(lang)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_freqs() {
        let input = "I am a dog";
        let mut expected_freqs = FreqMap::new();
        expected_freqs.insert(String::from("I"), 1f32);
        expected_freqs.insert(String::from("A"), 2f32);
        expected_freqs.insert(String::from("M"), 1f32);
        expected_freqs.insert(String::from("D"), 1f32);
        expected_freqs.insert(String::from("O"), 1f32);
        expected_freqs.insert(String::from("G"), 1f32);

        let computed_freqs = input.letter_freqs("en");
        assert_eq!(computed_freqs, expected_freqs);

        let letters = computed_freqs.values().sum::<f32>();
        assert_eq!(letters, 7f32);
    }
}
