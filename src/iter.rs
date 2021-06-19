pub trait Iter {
    type Item<'b>
    where
        Self: 'b;
    fn next(&mut self) -> Option<Self::Item<'_>>;
}

#[allow(clippy::module_name_repetitions)]
pub trait IntoIter {
    type IntoIter: Iter;
    fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> Iter for I {
    type Item<'b>
    where
        Self: 'b,
    = <Self as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item<'_>> {
        Iterator::next(self)
    }
}

impl<I: IntoIterator> IntoIter for I {
    type IntoIter = <Self as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}
