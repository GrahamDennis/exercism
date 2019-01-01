use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match first.len().cmp(&second.len()) {
        Ordering::Less => if is_strict_sublist(first, second) { return Comparison::Sublist },
        Ordering::Equal => if first.eq(second) { return Comparison::Equal },
        Ordering::Greater => if is_strict_sublist(second, first) { return Comparison::Superlist },
    }

    Comparison::Unequal
}

fn is_strict_sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    for idx in 0..=(haystack.len() - needle.len()) {
        if haystack[idx..].starts_with(needle) {
            return true
        }
    }

    false
}