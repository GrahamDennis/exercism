use std::collections::HashSet;
use std::cmp::{min, max};
use std::f64::consts::FRAC_1_SQRT_2;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();
    // a >= 1
    let minimum_a = 1;
    // b >= 2
    let minimum_b = 2;

    // a + b + c = sum and a < b < c implies c > sum/3
    // c >= sum/3 + 1
    let minimum_c = sum/3 + 1;
    let maximum_c = sum - minimum_b - minimum_a;

    for c in minimum_c..=maximum_c {
        // b > (sum - c)/2
        // b > c / sqrt(2)
        let minimum_b_given_c = max((sum - c)/2 + 1, (f64::from(c) * FRAC_1_SQRT_2) as u32 + 1);
        // b < c and b <= sum - c - minimum_a
        let maximum_b_given_c = min(c - 1, sum - c - minimum_a);
        for b in minimum_b_given_c..=maximum_b_given_c {
            let a = sum - c - b;

            if a*a + b*b == c*c {
                result.insert([a, b, c]);
            }
        }
    }

    return result;
}
