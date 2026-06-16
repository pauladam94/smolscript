use std::collections::VecDeque;

pub trait IntoOwned {
    type Output;
    fn into_owned(&self) -> Self::Output;
}

impl<T, O> IntoOwned for VecDeque<T>
where
    T: IntoOwned<Output = O>,
{
    type Output = VecDeque<O>;

    fn into_owned(&self) -> Self::Output {
        self.iter().map(|x| x.into_owned()).collect()
    }
}
