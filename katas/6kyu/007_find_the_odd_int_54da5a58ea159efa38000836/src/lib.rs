//! Title: Find the odd int
//! Link: https://www.codewars.com/kata/54da5a58ea159efa38000836
//! Kata ID: 54da5a58ea159efa38000836
//! Rank: 6kyu
//! Completed: 2026-03-25
#![allow(dead_code)]
// use std::collections::HashMap;
// fn find_odd(arr: &[i32]) -> i32 {
//     let mut map = HashMap::new();
//     for i in arr {
//         *map.entry(i).or_insert(0) += 1;
//     }
//     for (k, v) in map {
//         if v % 2 == 1 {
//             return *k;
//         }
//     }
//     return 0;
// }

fn find_odd(arr: &[i32]) -> i32 {
    arr.iter().fold(0_i32, |a, v| a ^ v)
}
#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&vec![
                20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
            ]),
            5
        );
        assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }
}
