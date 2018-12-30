use lazy_static::lazy_static;
use std::collections::HashMap;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    transpose(plain)
        .enumerate()
        .flat_map(|(idx, c)| if idx > 0 && idx % 5 == 0 { vec![' ', c] } else { vec![c] })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    transpose(cipher).collect()
}

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

lazy_static! {
    static ref REVERSED_ALPHABET: String = ALPHABET.chars().rev().collect();

    static ref CIPHER: HashMap<char, char> = {
        let mut m = HashMap::new();
        for (from, to) in ALPHABET.chars().zip(REVERSED_ALPHABET.chars()) {
            m.insert(from, to);
        }
        m
    };
}

fn transpose<'a>(input: &'a str) -> impl Iterator<Item=char> + 'a {
    input.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| *CIPHER.get(&c).unwrap_or(&c))
}