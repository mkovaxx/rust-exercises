use std::env;

mod interface;
use crate::interface::StackInterface;

struct Stack {
    // TODO: Add field to hold stack elements.
}

impl StackInterface for Stack {
    // TODO: Implement methods defined by StackInterface.
}

fn main() {
    if env::args().len() > 1 {
        let mut stack = Stack::new();
        for token in env::args().skip(1) {
            let result = match token.as_ref() {
                "+" => eval_operator(&mut stack, &|op0, op1| op0 + op1)
                    .expect("Not enough operands on stack for operator '+'"),
                "-" => eval_operator(&mut stack, &|op0, op1| op0 - op1)
                    .expect("Not enough operands on stack for operator '-'"),
                "x" => eval_operator(&mut stack, &|op0, op1| op0 * op1)
                    .expect("Not enough operands on stack for operator 'x'"),
                other => other
                    .parse::<i32>()
                    .expect(format!("Unable to parse '{}' as i32", other).as_ref()),
            };
            stack.push(result);
        }
        println!("Result: {}", stack.pop().unwrap());
    } else {
        println!("Reverse Polish Notation calculator with i32s and + - x");
    }
}

type BinaryOperator = Fn(i32, i32) -> i32;

fn eval_operator(stack: &mut StackInterface, operator: &BinaryOperator) -> Option<i32> {
    stack
        .pop()
        .and_then(|op1| stack.pop().map(|op0| operator(op0, op1)))
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests;
