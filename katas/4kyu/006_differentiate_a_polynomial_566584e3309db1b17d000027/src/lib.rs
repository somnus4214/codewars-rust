//! Title: Differentiate a polynomial
//! Link: https://www.codewars.com/kata/566584e3309db1b17d000027
//! Kata ID: 566584e3309db1b17d000027
//! Rank: 4kyu
//! Completed: 2025-12-11
#![allow(dead_code)]

pub fn differentiate(expr: &str, point: i64) -> i64 {
    let expr = expr.replace("-", "+-");
    let mut result: i64 = 0;
    for s in expr.split("+") {
        if !s.contains("x") || s.is_empty() {
            continue;
        }
        let mut n1 = 0;
        let mut n2 = 0;
        let n3;
        for (i, ss) in s.split("x").enumerate() {
            if i == 0 {
                if ss.is_empty() {
                    n1 = 1;
                } else if ss == "-" {
                    n1 = -1;
                } else {
                    n1 = ss.parse::<i64>().unwrap();
                }
            } else {
                if ss.is_empty() {
                    n2 = 1;
                } else {
                    n2 = ss[1..].parse::<i64>().unwrap();
                }
            }
        }
        if n2 - 1 == 0 {
            n3 = 1;
        } else {
            n3 = point.pow((n2 - 1) as u32);
        }

        result += n1 * n2 * n3;
    }
    result
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

#[cfg(test)]
mod review_1;
