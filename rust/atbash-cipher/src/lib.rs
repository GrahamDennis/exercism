/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain.chars()
        .filter_map(transpose)
        .enumerate()
        .flat_map(|(idx, c)| if idx > 0 && idx % 5 == 0 { vec![' ', c] } else { vec![c] })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher.chars()
        .filter_map(transpose)
        .collect()
}

fn transpose(c: char) -> Option<char> {
    match c {
        '0'...'9' => Some(c),
        'a'...'z' => Some((b'a' + b'z' - c as u8) as char),
        'A'...'Z' => Some((b'a' + b'Z' - c as u8) as char),
        _ => None
    }
}