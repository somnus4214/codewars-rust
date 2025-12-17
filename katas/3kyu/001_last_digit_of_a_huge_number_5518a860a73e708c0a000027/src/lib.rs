//! Title: Last digit of a huge number
//! Link: https://www.codewars.com/kata/5518a860a73e708c0a000027
//! Kata ID: 5518a860a73e708c0a000027
//! Rank: 3kyu
//! Completed: 2025-12-17
#![allow(dead_code)]

pub fn last_digit(nums: &[u64]) -> u64 {
    if nums.is_empty() {
        return 1;
    }

    // exp_state:
    // 0 => 指数为 0
    // 1 => 指数为 1
    // 2 => 指数 >= 2
    let mut exp_state = 1u8;

    for &x in nums.iter().rev() {
        exp_state = match (x, exp_state) {
            (0, 0) => 1, // 0^0 = 1
            (0, _) => 0, // 0^n = 0
            (_, 0) => 1, // a^0 = 1
            (_, 1) => {
                if x == 1 {
                    1
                } else {
                    2
                }
            }
            (_, _) => {
                if x == 0 {
                    0
                } else {
                    2
                }
            }
        };
    }

    let base = nums[0] % 10;
    match base {
        0 => {
            if exp_state == 0 {
                1
            } else {
                0
            }
        }
        1 => 1,
        5 => 5,
        6 => 6,
        4 => {
            if exp_state == 1 {
                4
            } else {
                6
            }
        }
        9 => {
            if exp_state == 1 {
                9
            } else {
                1
            }
        }
        _ => {
            let cycle = [0, base, (base * base) % 10, (base * base * base) % 10, 1];
            let idx = if exp_state == 1 { 1 } else { 4 };
            cycle[idx]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::last_digit;

    fn reference_solution() {}

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(v: &[u64], expected: u64) {
        assert_eq!(last_digit(v), expected, "{ERR_MSG} with list = {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (a, b) in [
            (vec![], 1),
            (vec![0, 0], 1),
            (vec![0, 0, 0], 0),
            (vec![1, 2], 1),
            (vec![3, 4, 5], 1),
            (vec![4, 3, 6], 4),
            (vec![7, 6, 21], 1),
            (vec![12, 30, 21], 6),
            (vec![2, 2, 2, 0], 4),
            (vec![2, 2, 101, 2], 6),
            (vec![937640, 767456, 981242], 0),
            (vec![123232, 694022, 140249], 6),
            (vec![499942, 898102, 846073], 6),
        ] {
            dotest(&a, b);
        }
    }
}
