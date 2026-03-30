//! Title: Snail
//! Link: https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1
//! Kata ID: 521c2db8ddc89b9b7a0000c1
//! Rank: 4kyu
//! Completed: 2026-03-25
#![allow(dead_code)]

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    // enjoy
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    if matrix.len() == 1 {
        return vec![matrix[0][0]];
    }
    let mut top = 0;
    let mut bottom = matrix.len() - 1;
    let mut left = 0;
    let mut right = matrix[0].len() - 1;
    let mut result = Vec::new();
    while top <= bottom && left <= right {
        for col in left..=right {
            result.push(matrix[top][col]);
        }
        top += 1;
        for row in top..=bottom {
            result.push(matrix[row][right]);
        }
        right -= 1;
        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom][col]);
            }
            bottom -= 1;
        }
        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row][left]);
            }
            left += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
