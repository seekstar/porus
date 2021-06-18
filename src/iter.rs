pub trait Iter<'a> {
    type Item<'b>
    where
        'a: 'b;
    fn next<'b>(&'b mut self) -> Option<Self::Item<'b>>
    where
        'a: 'b;
}

#[allow(clippy::module_name_repetitions)]
pub trait IntoIter<'a> {
    type IntoIter: Iter<'a>;
    fn into_iter(self) -> Self::IntoIter;
}

impl<'a, I: Iterator> Iter<'a> for I
where
    <Self as Iterator>::Item: 'a,
{
    type Item<'b>
    where
        'a: 'b,
    = <Self as Iterator>::Item;
    fn next<'b>(&'b mut self) -> Option<Self::Item<'b>>
    where
        'a: 'b,
    {
        Iterator::next(self)
    }
}

impl<'a, I: IntoIterator> IntoIter<'a> for I
where
    <<Self as IntoIterator>::IntoIter as Iterator>::Item: 'a,
{
    type IntoIter = <Self as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}
