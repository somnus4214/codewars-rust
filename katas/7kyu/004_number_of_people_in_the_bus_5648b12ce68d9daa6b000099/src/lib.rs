//! Title: Number of People in the Bus
//! Link: https://www.codewars.com/kata/5648b12ce68d9daa6b000099
//! Kata ID: 5648b12ce68d9daa6b000099
//! Rank: 7kyu
//! Completed: 2026-02-09
#![allow(dead_code)]

// fn number(bus_stops: &[(i32, i32)]) -> i32 {
//     let mut count = 0;
//     for (on, off) in bus_stops {
//         count += on - off;
//     }
//     count
// }

fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops.iter().fold(0, |acc, x| acc + x.0 - x.1)
}
#[test]
fn returns_expected() {
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 10), (12, 2), (6, 1), (7, 10)]),
        17
    );
    assert_eq!(
        number(&[(3, 0), (9, 1), (4, 8), (12, 2), (6, 1), (7, 8)]),
        21
    );
}
