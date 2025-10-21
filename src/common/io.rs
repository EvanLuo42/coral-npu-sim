pub enum Poll<T> {
    Pending,
    Ready(T),
}

pub trait Future {
    type Output;
    type Input;

    fn poll(&mut self, context: &mut Self::Input) -> Poll<Self::Output>;
}
