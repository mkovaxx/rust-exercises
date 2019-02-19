use super::*;

#[test]
fn test_new_stack_is_empty() {
    let stack = &mut Stack::new();
    assert_eq!(stack.pop(), None);
}

#[quickcheck]
fn test_pop_after_push(n: i32) -> bool {
    let stack = &mut Stack::new();
    stack.push(n);
    stack.pop() == Some(n)
}

#[quickcheck]
fn test_reverses_elements(ns: Vec<i32>) -> bool {
    let stack = &mut Stack::new();
    for n in &ns {
        stack.push(n.clone());
    }
    let ps: Vec<i32> = (0..ns.len()).map(|_| stack.pop().unwrap()).collect();
    stack.pop() == None && ns == ps.iter().rev().map(|e| e.clone()).collect::<Vec<i32>>()
}
