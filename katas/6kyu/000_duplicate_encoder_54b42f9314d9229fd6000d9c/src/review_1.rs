//! Review #1
//! Date: 2025-12-03

fn duplicate_encode(word: &str) -> String {
    let mut enc = std::collections::HashMap::new();
    for c in word.to_lowercase().chars() {
        *enc.entry(c).or_insert(0) += 1;
    }
    word.to_lowercase()
        .chars()
        .map(|c| match *enc.get(&c).unwrap() {
            1 => '(',
            _ => ')',
        })
        .collect()
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}
