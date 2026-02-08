//! Title: Remove exclamation marks
//! Link: https://www.codewars.com/kata/57a0885cbb9944e24c00008e
//! Kata ID: 57a0885cbb9944e24c00008e
//! Rank: 8kyu
//! Completed: 2026-02-08
#![allow(dead_code)]

fn remove_exclamation_marks(input: &str) -> String {
    input.replace("!", "")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_exclamation_marks;

    fn do_test(input: &str, expected: &str) {
        let actual = remove_exclamation_marks(input);
        assert_eq!(actual, expected, "\nYour result (left) did not match the expected output (right) for the input: {input:?}");
    }

    #[test]
    fn sample_test() {
        do_test("Hello World!", "Hello World");
        do_test("Hello World!!!", "Hello World");
        do_test("Hi! Hello!", "Hi Hello");
        do_test("", "");
        do_test("Oh, no!!!", "Oh, no");
    }
}
