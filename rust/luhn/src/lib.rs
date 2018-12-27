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
        .map(|(i, v)| digit_checksum(i, v))
        .sum();

    checksum % 10 == 0
}

fn digit_checksum(reverse_idx: usize, digit: u32) -> u32 {
    if reverse_idx % 2 == 0 {
        return digit;
    }

    let doubled = 2 * digit;
    if doubled > 9 {
        doubled - 9
    } else {
        doubled
    }
}

fn valid_chars(s: &str) -> bool {
    non_whitespace(s).all(|c| c.is_digit(10)) && non_whitespace(s).count() > 1
}

fn non_whitespace<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(|c| !c.is_whitespace())
}
