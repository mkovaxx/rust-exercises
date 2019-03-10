use super::*;
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

const DIGITS: &str = "0123456789";

#[test]
fn test_single_digits() {
    for (n, c) in DIGITS.chars().enumerate() {
        assert_eq!(to_decimal(n as u32), c.to_string());
    }
}

#[test]
fn test_overflow() {
    assert_eq!(from_decimal(&"4294967296".to_string()), None);
}

#[quickcheck]
fn test_lossless_encoding(n: u32) -> bool {
    from_decimal(&to_decimal(n)) == Some(n)
}

#[quickcheck]
fn test_only_uses_digits(n: u32) -> bool {
    to_decimal(n).chars().all(|c| c.is_ascii_digit())
}

#[quickcheck]
fn test_leading_0s_are_irrelevant(n: u32, count: u8) -> bool {
    let leading_0s: String = std::iter::repeat('0').take(count as usize).collect();
    from_decimal(&(leading_0s + &to_decimal(n))) == Some(n)
}

#[quickcheck]
fn test_trailing_0_is_times_10(n: u32) -> TestResult {
    if n > 214748364 {
        TestResult::discard()
    } else {
        TestResult::from_bool(from_decimal(&(to_decimal(n) + "0")) == Some(10 * n))
    }
}

#[quickcheck]
fn test_fails_on_invalid(s: String) -> TestResult {
    if s.chars().all(|c| c.is_ascii_digit()) {
        TestResult::discard()
    } else {
        TestResult::from_bool(from_decimal(&s) == None)
    }
}
