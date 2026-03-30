//! Title: Square Every Digit
//! Link: https://www.codewars.com/kata/546e2562b03326a88e000020
//! Kata ID: 546e2562b03326a88e000020
//! Rank: 7kyu
//! Completed: 2026-02-09
#![allow(dead_code)]

fn square_digits(num: u64) -> u64 {
    let mut s = String::new();
    for ch in num.to_string().chars() {
        let d = ch.to_digit(10).unwrap();
        s.push_str(&(d * d).to_string());
    }
    s.parse::<u64>().unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
