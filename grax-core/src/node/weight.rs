pub trait Balance<T> {
    fn balance(&self) -> &T;
    fn balance_mut(&mut self) -> &mut T;
}

impl<T> Balance<T> for T {
    fn balance(&self) -> &T {
        self
    }

    fn balance_mut(&mut self) -> &mut T {
        self
    }
}
