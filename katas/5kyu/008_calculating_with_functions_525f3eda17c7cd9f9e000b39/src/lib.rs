//! Title: Calculating with Functions
//! Link: https://www.codewars.com/kata/525f3eda17c7cd9f9e000b39
//! Kata ID: 525f3eda17c7cd9f9e000b39
//! Rank: 5kyu
//! Completed: 2026-02-08
#![allow(dead_code)]

macro_rules! zero {
    () => {
        0
    };
    ($x: expr) => {
        $x(0)
    };
}

macro_rules! one {
    () => {
        1
    };
    ($x: expr) => {
        $x(1)
    };
}

macro_rules! two {
    () => {
        2
    };
    ($x: expr) => {
        $x(2)
    };
}

macro_rules! three {
    () => {
        3
    };
    ($x: expr) => {
        $x(3)
    };
}

macro_rules! four {
    () => {
        4
    };
    ($x: expr) => {
        $x(4)
    };
}

macro_rules! five {
    () => {
        5
    };
    ($x: expr) => {
        $x(5)
    };
}

macro_rules! six {
    () => {
        6
    };
    ($x: expr) => {
        $x(6)
    };
}

macro_rules! seven {
    () => {
        7
    };
    ($x: expr) => {
        $x(7)
    };
}

macro_rules! eight {
    () => {
        8
    };
    ($x: expr) => {
        $x(8)
    };
}

macro_rules! nine {
    () => {
        9
    };
    ($x: expr) => {
        $x(9)
    };
}

pub fn plus(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

pub fn minus(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x - y
}

pub fn times(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x * y
}

pub fn divided_by(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x / y
}
#[cfg(test)]
mod example_tests {
    use super::*;
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(actual: i32, expected: i32, operation: &str) {
        assert_eq!(actual, expected, "{ERR_MSG} with operation: {operation}")
    }

    #[test]
    fn fixed_tests() {
        dotest(seven!(times(five!())), 35, "7 * 5");
        dotest(four!(plus(nine!())), 13, "4 + 9");
        dotest(eight!(minus(three!())), 5, "8 - 3");
        dotest(six!(divided_by(two!())), 3, "6 / 2");
        dotest(one!(times(zero!())), 0, "1 * 0");
    }
}
