pub trait StackInterface {
    fn new() -> Self;
    fn pop(&mut self) -> Option<i32>;
    fn push(&mut self, n: i32);
}
