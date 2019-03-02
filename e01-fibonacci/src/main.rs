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
mod tests;
