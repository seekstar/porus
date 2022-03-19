pub trait Iter {
    type Item<'a>
    where
        Self: 'a;
    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn rev(self) -> Rev<Self>
    where
        Self: Sized + DoubleEndedIter,
    {
        Rev { it: self }
    }
}

impl<I: Iterator> Iter for I {
    type Item<'a> = <Self as Iterator>::Item where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        Iterator::next(self)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait IntoIter {
    type IntoIter: Iter;

    fn into_iter(self) -> Self::IntoIter;
}

impl<I: IntoIterator> IntoIter for I {
    type IntoIter = <Self as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ExactSizeIter: Iter {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<I: ExactSizeIterator> ExactSizeIter for I {
    fn len(&self) -> usize {
        ExactSizeIterator::len(self)
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait DoubleEndedIter: Iter {
    fn next_back(&mut self) -> Option<<Self as Iter>::Item<'_>>;
}

impl<I: DoubleEndedIterator> DoubleEndedIter for I {
    fn next_back(&mut self) -> Option<<Self as Iter>::Item<'_>> {
        DoubleEndedIterator::next_back(self)
    }
}

pub struct Rev<I: DoubleEndedIter> {
    it: I,
}

impl<I: DoubleEndedIter + ExactSizeIter> ExactSizeIter for Rev<I> {
    fn len(&self) -> usize {
        ExactSizeIter::len(&self.it)
    }
}

impl<I: DoubleEndedIter> Iter for Rev<I> {
    type Item<'a> = <I as Iter>::Item<'a> where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        DoubleEndedIter::next_back(&mut self.it)
    }
}

impl<I: DoubleEndedIter> DoubleEndedIter for Rev<I> {
    fn next_back(&mut self) -> Option<Self::Item<'_>> {
        Iter::next(&mut self.it)
    }
}
