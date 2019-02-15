#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

// NOTE: fib(48) wouldn't fit in a u32, so assume n <= 47.
fn fib(n: u32) -> u32 {
    // TODO: Remove the next line and implement the function.
    compile_error!("You need to implement fib().");
}

static COUNT: u32 = 10;

fn main() {
    println!("The first {} fibonacci numbers:", COUNT);
    for i in 0..COUNT {
        println!("fib({}) = {}", i, fib(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::TestResult;

    #[test]
    fn test_base_cases() {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
    }

    #[quickcheck]
    fn test_recurrence_for_values_at_most_20(n: u32) -> TestResult {
        if n < 2 || n > 10 {
            TestResult::discard()
        } else {
            TestResult::from_bool(recurrence_property(n))
        }
    }

    // NOTE: Naive implementations won't be able to pass this.
    #[quickcheck]
    fn test_recurrence_for_values_at_most_47(n: u32) -> TestResult {
        if n < 2 || n > 47 {
            TestResult::discard()
        } else {
            TestResult::from_bool(recurrence_property(n))
        }
    }

    fn recurrence_property(n: u32) -> bool {
        fib(n) == fib(n-1) + fib(n-2)
    }
}
