use std::env;

fn to_decimal(n: u32) -> String {
    // TODO: Remove the next line and implement the function.
    compile_error!("You need to implement to_decimal().");
}

fn from_decimal(s: &String) -> Option<u32> {
    // TODO: Remove the next line and implement the function.
    compile_error!("You need to implement from_decimal().");
}

fn main() {
    if env::args().len() > 1 {
        for arg in env::args().skip(1) {
            if let Some(n) = from_decimal(&arg) {
                println!("{} + 1 = {}", to_decimal(n), to_decimal(n + 1));
            } else {
                println!("'{}' is not a valid `u32`", &arg);
            }
        }
    } else {
        println!("INC: Incremented Number Computer");
    }
}

#[cfg(test)]
mod tests;
