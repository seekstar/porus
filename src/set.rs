use crate::collection::Collection;
use alloc::collections::btree_set::Range;
use core::ops::RangeBounds;

pub trait Set: Collection {
    type Elem;

    fn contains(&self, elem: &Self::Elem) -> bool;
}

#[allow(clippy::module_name_repetitions)]
pub trait SetMut: Set {
    fn insert(&mut self, elem: Self::Elem);
    fn remove(&mut self, elem: &Self::Elem);
}

#[allow(clippy::module_name_repetitions)]
pub trait SetOrd<'a>: Set<Elem: 'a + Ord> {
    type Range : Iterator<Item=&'a <Self as Set>::Elem>;

    fn range<R: RangeBounds<<Self as Set>::Elem>>(&'a self, range: R) -> Self::Range;
}

pub fn contains<S: Set>(set: &S, elem: &S::Elem) -> bool {
    Set::contains(set, elem)
}

pub fn insert<S: SetMut>(set: &mut S, elem: S::Elem) {
    SetMut::insert(set, elem);
}

pub fn remove<S: SetMut>(set: &mut S, elem: &S::Elem) {
    SetMut::remove(set, elem);
}

pub fn range<'a, S: SetOrd<'a>, R: RangeBounds<<S as Set>::Elem>>(set: &'a S, range: R) -> impl Iterator<Item=&'a <S as Set>::Elem> + 'a {
    SetOrd::range(set, range)
}

use alloc::collections::BTreeSet;

impl<T: Ord> Set for BTreeSet<T> {
    type Elem = T;

    fn contains(&self, elem: &T) -> bool {
        self.contains(elem)
    }
}

impl<T: Ord> SetMut for BTreeSet<T> {
    fn insert(&mut self, elem: T) {
        self.insert(elem);
    }

    fn remove(&mut self, elem: &T) {
        self.remove(elem);
    }
}

impl<'a, T: 'a + Ord> SetOrd<'a> for BTreeSet<T> {
    type Range = Range<'a, T>;

    fn range<R: RangeBounds<<Self as Set>::Elem>>(&'a self, range: R) -> Range<'a, T> {
        self.range(range)
    }
}
