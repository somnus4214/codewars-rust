//! Title: Are they the \
//! Link: https://www.codewars.com/kata/550498447451fbbd7600041c
//! Kata ID: 550498447451fbbd7600041c
//! Rank: 6kyu
//! Completed: 2026-02-08
#![allow(dead_code)]

fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    // your code
    if a.len() != b.len() {
        return false;
    }
    if a.is_empty() && b.is_empty() {
        return true;
    }
    let mut a_squared: Vec<i64> = a.iter().map(|&x| x * x).collect();
    a_squared.sort();
    let mut b_sorted = b.clone();
    b_sorted.sort();
    a_squared == b_sorted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(a: Vec<i64>, b: Vec<i64>, exp: bool) -> () {
        assert_eq!(comp(a, b), exp)
    }

    #[test]
    fn tests_comp() {
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 11,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, true);
        let a1 = vec![121, 144, 19, 161, 19, 144, 19, 11];
        let a2 = vec![
            11 * 21,
            121 * 121,
            144 * 144,
            19 * 19,
            161 * 161,
            19 * 19,
            144 * 144,
            19 * 19,
        ];
        testing(a1, a2, false);
    }
}
