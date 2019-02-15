use super::*;
use quickcheck::TestResult;

#[test]
fn test_abba() {
    assert_eq!(is_palindrome("ABBA".to_string()), true);
}

#[test]
fn test_acdc() {
    assert_eq!(is_palindrome("ACDC".to_string()), false);
}

#[quickcheck]
fn test_even_length_palindromes(s: String) -> bool {
    let r = s.chars().rev().collect::<String>();
    let palindrome = s + &r;
    is_palindrome(palindrome)
}

#[quickcheck]
fn test_odd_length_palindromes(s: String, c: char) -> bool {
    let r = s.chars().rev().collect::<String>();
    let palindrome = s + &c.to_string() + &r;
    is_palindrome(palindrome)
}

#[quickcheck]
fn test_even_length_non_palindromes(s: String, t: String) -> TestResult {
    let l = std::cmp::min(s.len(), t.len());
    let u: String = s.chars().take(l).collect();
    let v: String = t.chars().take(l).collect();
    if u != v {
        let r: String = v.chars().rev().collect();
        let input = u + &r;
        TestResult::from_bool(is_palindrome(input) == false)
    } else {
        TestResult::discard()
    }
}

#[quickcheck]
fn test_odd_length_non_palindromes(s: String, t: String, c: char) -> TestResult {
    let l = std::cmp::min(s.len(), t.len());
    let u: String = s.chars().take(l).collect();
    let v: String = t.chars().take(l).collect();
    if u != v {
        let r: String = v.chars().rev().collect();
        let input = u + &c.to_string() + &r;
        TestResult::from_bool(is_palindrome(input) == false)
    } else {
        TestResult::discard()
    }
}
