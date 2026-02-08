//! Title: Is he gonna survive?
//! Link: https://www.codewars.com/kata/59ca8246d751df55cc00014c
//! Kata ID: 59ca8246d751df55cc00014c
//! Rank: 8kyu
//! Completed: 2026-02-08
#![allow(dead_code)]

fn hero(bullets: u16, dragons: u16) -> bool {
    bullets >= dragons * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(hero(10, 5), true);
        assert_eq!(hero(7, 4), false);
        assert_eq!(hero(4, 5), false);
        assert_eq!(hero(100, 40), true);
        assert_eq!(hero(1500, 751), false);
        assert_eq!(hero(0, 1), false);
    }
}
