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
        Ordering::Less => {
            if is_strict_sublist(first, second) {
                return Comparison::Sublist;
            }
        }
        Ordering::Greater => {
            if is_strict_sublist(second, first) {
                return Comparison::Superlist;
            }
        }
        Ordering::Equal => {
            if first == second {
                return Comparison::Equal;
            }
        }
    }

    Comparison::Unequal
}

fn is_strict_sublist<T: PartialEq>(needle: &[T], haystack: &[T]) -> bool {
    needle.is_empty()
        || haystack
            .windows(needle.len())
            .any(|candidate| candidate == needle)
}
