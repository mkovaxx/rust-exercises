use std::env;

fn is_palindrome(s: String) -> bool {
    // TODO: Remove the next line and implement the function.
    compile_error!("You need to implement is_palindrome().");
}

fn main() {
    println!("Which arguments are palindromes?");
    for s in env::args().skip(1) {
        let verdict = if is_palindrome(s.clone()) { "yes" } else { "no" };
        println!("'{}': {}", s, verdict);
    }
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests;
