//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

/// returns two numbers
const fn sum_two_numbers(first: i32, second: i32) -> i32 {
    return first.saturating_add(second);
}

#[cfg_attr(coverage_nightly, coverage(off))]
/// main function
fn main() {
    let first: i32 = 5;
    let second: i32 = 10;
    let result = sum_two_numbers(first, second);
    println!("The sum of {first} and {second} is {result}");
}

#[cfg(test)]
/// the tests
mod tests {
    use super::*;

    #[test]
    /// tests this and that
    fn test_sum_two_numbers() {
        assert_eq!(sum_two_numbers(5, 10), 15);
    }
}
