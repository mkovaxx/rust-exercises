pub trait StackInterface {
    fn new() -> Self
    where
        Self: Sized;
    fn pop(&mut self) -> Option<i32>;
    fn push(&mut self, n: i32);
}
