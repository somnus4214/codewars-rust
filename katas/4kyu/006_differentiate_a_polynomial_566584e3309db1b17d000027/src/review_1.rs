//! Review #1
//! Date: 2025-12-11
#![allow(dead_code)]
use regex::Regex;
pub fn differentiate(expr: &str, point: i64) -> i64 {
    let mut r = 0;
    for c in Regex::new(r"(-?\d*)x\^?(\d*)").unwrap().captures_iter(expr) {
        let sx = &c[1];
        let sy = &c[2];
        let x = if sx == "" {
            1
        } else if sx == "-" {
            -1
        } else {
            sx.parse::<i64>().unwrap()
        };
        let y = if sy == "" {
            1
        } else if sy == "-" {
            -1
        } else {
            sy.parse::<i64>().unwrap()
        };
        r += x * y * point.pow(y as u32 - 1);
    }
    r
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::differentiate;

    #[test]
    fn sample_tests() {
        assert_eq!(differentiate("12x+2", 3), 12);
        assert_eq!(differentiate("x^2-x", 3), 5);
        assert_eq!(differentiate("-5x^2+10x+4", 3), -20);
    }
}
