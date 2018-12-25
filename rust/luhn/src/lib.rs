/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !valid_chars(code) {
        return false;
    }

    let checksum: u32 = code
        .chars()
        .flat_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, v)| if i % 2 == 1 { digit_double(v) } else { v })
        .sum();

    checksum % 10 == 0
}

fn digit_double(d: u32) -> u32 {
    let result = 2 * d;
    if result > 9 {
        result - 9
    } else {
        result
    }
}

fn valid_chars(s: &str) -> bool {
    non_whitespace(s).all(|c| c.is_digit(10)) && non_whitespace(s).count() > 1
}

fn non_whitespace<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(|c| !c.is_whitespace())
}
