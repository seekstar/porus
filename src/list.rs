use crate::collection::Collection;
use crate::iter::{DoubleEndedIter, ExactSizeIter, Iter};
use core::cell::Cell;
use core::cmp::Ordering::{Equal, Greater, Less};
use core::hint::unreachable_unchecked;
use core::mem::replace;
use core::mem::transmute;
use core::ops::Range;
use core::ops::{Bound, RangeBounds};

const fn midpoint(r: &Range<usize>) -> usize {
    usize::wrapping_shr(
        usize::saturating_add(r.start, usize::saturating_sub(r.end, 1)),
        1,
    )
}

const fn range_left(r: &Range<usize>, m: usize) -> Range<usize> {
    Range {
        start: r.start,
        end: m,
    }
}

const fn range_right(r: &Range<usize>, m: usize) -> Range<usize> {
    Range {
        start: usize::saturating_add(m, 1),
        end: r.end,
    }
}

pub trait List: Collection {
    type Elem;

    type Slice<'a>: Slice<&'a Self> + List<Elem = Self::Elem> = View<'a, Self> where
        Self: 'a;

    fn get(&self, index: usize) -> Option<&Self::Elem>;

    fn iter_ref(&self) -> ListIterRef<'_, Self> {
        ListIterRef {
            list: self,
            start: 0,
            end: Collection::size(self),
        }
    }

    fn iter(&self) -> ListIter<'_, Self>
    where
        Self::Elem: Clone,
    {
        ListIter {
            list: self,
            start: 0,
            end: Collection::size(self),
        }
    }

    fn slice<T: RangeBounds<usize>>(&self, bound: T) -> Self::Slice<'_> {
        Self::Slice::<'_>::new(self, &bound)
    }

    fn find(&self, elem: &Self::Elem) -> Option<usize>
    where
        Self::Elem: PartialEq,
    {
        let size = Collection::size(self);
        for i in 0..size {
            if List::get(self, i).unwrap() == elem {
                return Some(i);
            }
        }
        None
    }

    fn bsearch(&self, elem: &Self::Elem) -> Range<usize>
    where
        Self::Elem: Ord,
    {
        let mut r = 0..Collection::size(self);
        loop {
            if r.is_empty() {
                return r;
            }

            let m = midpoint(&r);
            match Ord::cmp(elem, List::get(self, m).unwrap()) {
                Equal => break,
                Less => r = range_left(&r, m),
                Greater => r = range_right(&r, m),
            }
        }

        let mut rl = range_left(&r, midpoint(&r));
        let mut rr = range_right(&r, midpoint(&r));

        while !rl.is_empty() {
            let m = midpoint(&rl);
            match Ord::cmp(elem, List::get(self, m).unwrap()) {
                Equal => rl = range_left(&rl, m),
                Greater => rl = range_right(&rl, m),
                Less => unsafe { unreachable_unchecked() },
            }
        }

        while !rr.is_empty() {
            let m = midpoint(&rr);
            match Ord::cmp(elem, List::get(self, m).unwrap()) {
                Equal => rr = range_right(&rr, m),
                Less => rr = range_left(&rr, m),
                Greater => unsafe { unreachable_unchecked() },
            }
        }

        Range {
            start: rl.start,
            end: rr.end,
        }
    }

    fn is_stable_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool, I: List<Elem = usize>>(
        &self,
        lt: F,
        index: &I,
    ) -> bool {
        List::iter(index).is_sorted_by(|&i, &j| {
            if !lt(List::get(self, i).unwrap(), List::get(self, j).unwrap()) && (i >= j) {
                None
            } else {
                Some(Less)
            }
        })
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>
    where
        Self: ListMut,
    {
        <Self as ListMut>::get_mut(self, index)
    }

    #[allow(clippy::iter_not_returning_iterator)]
    fn iter_mut(&mut self) -> ListIterMut<'_, Self>
    where
        Self: ListMut,
    {
        let end = Collection::size(self);
        ListIterMut {
            list: self,
            start: 0,
            end,
        }
    }

    fn slice_mut<T: RangeBounds<usize>>(&mut self, bound: T) -> <Self as ListMut>::SliceMut<'_>
    where
        Self: ListMut,
    {
        <Self as ListMut>::SliceMut::<'_>::new(self, &bound)
    }

    fn replace(&mut self, index: usize, elem: Self::Elem) -> Self::Elem
    where
        Self: ListMut,
    {
        replace(List::get_mut(self, index).unwrap(), elem)
    }

    fn set(&mut self, index: usize, elem: Self::Elem)
    where
        Self: ListMut,
    {
        List::replace(self, index, elem);
    }

    #[allow(clippy::transmute_ptr_to_ptr)]
    fn swap(&mut self, i: usize, j: usize)
    where
        Self: ListMut,
    {
        if i == j {
            return;
        }

        let p: &Cell<Self::Elem> = unsafe { transmute(List::get(self, i).unwrap()) };
        let q: &Cell<Self::Elem> = unsafe { transmute(List::get(self, j).unwrap()) };
        p.swap(q);
    }

    fn reverse(&mut self)
    where
        Self: ListMut,
    {
        let mut l = 0;
        let mut r = Collection::size(self);
        while l < r {
            r = usize::wrapping_sub(r, 1);
            List::swap(self, l, r);
            l = usize::wrapping_add(l, 1);
        }
    }

    fn rotate_left(&mut self, n: usize)
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        if let Some(n1) = usize::checked_rem(n, size) {
            let m = usize::wrapping_sub(size, n1);
            List::reverse(&mut List::slice_mut(self, ..m));
            List::reverse(&mut List::slice_mut(self, m..));
            List::reverse(self);
        }
    }

    fn rotate_right(&mut self, n: usize)
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        if let Some(m) = usize::checked_rem(n, size) {
            List::reverse(&mut List::slice_mut(self, ..m));
            List::reverse(&mut List::slice_mut(self, m..));
            List::reverse(self);
        }
    }

    fn bubble<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let mut count = 0;
        let size = Collection::size(self);
        for (i, j) in Iterator::rev((0..size).zip(1..size)) {
            if lt(List::get(self, j).unwrap(), List::get(self, i).unwrap()) {
                List::swap(self, i, j);
                count = usize::wrapping_add(count, 1);
            }
        }
        count
    }

    fn bubble_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        (0..size)
            .map(|i| List::bubble(&mut List::slice_mut(self, i..size), &lt))
            .sum()
    }

    fn bubble_sorted<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let mut count = 0;
        let size = Collection::size(self);
        for (i, j) in Iterator::rev((0..size).zip(1..size)) {
            if !lt(List::get(self, j).unwrap(), List::get(self, i).unwrap()) {
                break;
            }
            List::swap(self, i, j);
            count = usize::wrapping_add(count, 1);
        }
        count
    }

    fn insertion_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        (0..size)
            .map(|i| List::bubble_sorted(&mut List::slice_mut(self, 0..=i), &lt))
            .sum()
    }

    fn insertion_sort_g<F: Fn(&Self::Elem, &Self::Elem) -> bool>(
        &mut self,
        lt: F,
        g: usize,
    ) -> usize
    where
        Self: ListMut,
    {
        let mut count = 0;
        let size = Collection::size(self);
        for i in g..size {
            let mut j = i;
            while (j >= g)
                && lt(
                    List::get(self, j).unwrap(),
                    List::get(self, usize::wrapping_sub(j, g)).unwrap(),
                )
            {
                List::swap(self, j, usize::wrapping_sub(j, g));
                count = usize::wrapping_add(count, 1);
                j = usize::wrapping_sub(j, g);
            }
        }
        count
    }

    fn shell_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool, G: List<Elem = usize>>(
        &mut self,
        lt: F,
        gaps: &G,
    ) -> usize
    where
        Self: ListMut,
    {
        let mut count = 0;
        for g in List::iter(gaps) {
            count = usize::wrapping_add(count, List::insertion_sort_g(self, &lt, g));
        }
        count
    }

    fn selection_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let mut count = 0;
        let size = Collection::size(self);
        for i in 0..size {
            if let Some(min) = (i..size).min_by(|&x, &y| {
                if lt(List::get(self, y).unwrap(), List::get(self, x).unwrap()) {
                    Greater
                } else {
                    Less
                }
            }) {
                if min != i {
                    List::swap(self, i, min);
                    count = usize::wrapping_add(count, 1);
                }
            }
        }
        count
    }

    fn partition<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F) -> usize
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        let mut i = 0;

        if let Some(pivot) = usize::checked_sub(size, 1) {
            for j in 0..pivot {
                if lt(List::get(self, j).unwrap(), List::get(self, pivot).unwrap()) {
                    List::swap(self, j, i);
                    i = usize::wrapping_add(i, 1);
                }
            }

            List::swap(self, i, pivot);
        }

        i
    }

    fn quick_sort_aux<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: &F)
    where
        Self: ListMut,
    {
        let size = Collection::size(self);
        if size < 2 {
            return;
        }

        let p = List::partition(self, lt);
        List::quick_sort_aux(&mut List::slice_mut(self, ..p), lt);
        List::quick_sort_aux(&mut List::slice_mut(self, usize::wrapping_add(p, 1)..), lt);
    }

    fn quick_sort<F: Fn(&Self::Elem, &Self::Elem) -> bool>(&mut self, lt: F)
    where
        Self: ListMut,
    {
        List::quick_sort_aux(self, &lt);
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ListIter<'a, T: 'a + List + ?Sized>
where
    T::Elem: Clone,
{
    list: &'a T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + List + ?Sized> Iterator for ListIter<'a, T>
where
    T::Elem: Clone,
{
    type Item = T::Elem;

    fn next(&mut self) -> Option<Self::Item> {
        (self.start < self.end).then(|| {
            let index = self.start;
            self.start = usize::wrapping_add(self.start, 1);
            Clone::clone(List::get(self.list, index).unwrap())
        })
    }
}

impl<'a, T: 'a + List + ?Sized> ExactSizeIterator for ListIter<'a, T>
where
    T::Elem: Clone,
{
    fn len(&self) -> usize {
        usize::saturating_sub(self.end, self.start)
    }
}

impl<'a, T: 'a + List + ?Sized> DoubleEndedIterator for ListIter<'a, T>
where
    T::Elem: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.start < self.end).then(|| {
            self.end = usize::wrapping_sub(self.end, 1);
            let index = self.end;
            Clone::clone(List::get(self.list, index).unwrap())
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct ListIterRef<'a, T: 'a + List + ?Sized> {
    list: &'a T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + List + ?Sized> Iter for ListIterRef<'a, T> {
    type Item<'b>
    = &'b <T as List>::Elem
    where
        Self: 'b;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        (self.start < self.end).then(move || {
            let index = self.start;
            self.start = usize::wrapping_add(self.start, 1);
            List::get(self.list, index).unwrap()
        })
    }
}

impl<'a, T: 'a + List + ?Sized> ExactSizeIter for ListIterRef<'a, T> {
    fn len(&self) -> usize {
        usize::saturating_sub(self.end, self.start)
    }
}

impl<'a, T: 'a + List + ?Sized> DoubleEndedIter for ListIterRef<'a, T> {
    fn next_back(&mut self) -> Option<<Self as Iter>::Item<'_>> {
        (self.start < self.end).then(|| {
            self.end = usize::wrapping_sub(self.end, 1);
            let index = self.end;
            List::get(self.list, index).unwrap()
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub trait ListMut: List {
    type SliceMut<'a>: SliceMut<&'a mut Self> + ListMut<Elem = Self::Elem>
    where
        Self: 'a,
    = ViewMut<'a, Self>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem>;
}

#[allow(clippy::module_name_repetitions)]
pub struct ListIterMut<'a, T: 'a + ListMut + ?Sized> {
    list: &'a mut T,
    start: usize,
    end: usize,
}

impl<'a, T: 'a + ListMut + ?Sized> Iter for ListIterMut<'a, T> {
    type Item<'b>
    = &'b mut <T as List>::Elem
    where
        Self: 'b;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        (self.start < self.end).then(move || {
            let index = self.start;
            self.start = usize::wrapping_add(self.start, 1);
            List::get_mut(self.list, index).unwrap()
        })
    }
}

pub trait Slice<L> {
    fn new<T: RangeBounds<usize>>(list: L, bound: &T) -> Self;
}

fn start_bound<T: RangeBounds<usize>>(bound: &T) -> usize {
    match RangeBounds::start_bound(bound) {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => usize::checked_add(x, 1).expect("start_bound overflow"),
    }
}

fn end_bound<T: RangeBounds<usize>>(bound: &T, default: usize) -> usize {
    match RangeBounds::end_bound(bound) {
        Bound::Unbounded => default,
        Bound::Included(&x) => usize::checked_add(x, 1).expect("end_bound overflow"),
        Bound::Excluded(&x) => x,
    }
}

pub struct View<'a, L: 'a + List + ?Sized> {
    list: &'a L,
    start: usize,
    size: usize,
}

impl<'a, L: 'a + List + ?Sized> Collection for View<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + List + ?Sized> List for View<'a, L> {
    type Elem = <L as List>::Elem;
    type Slice<'b>
    = View<'b, L>
    where
        Self: 'b;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        (index < self.size).then(|| {
            List::get(
                self.list,
                usize::checked_add(self.start, index).expect("index overflow"),
            )
            .unwrap()
        })
    }
}

impl<'a, 'b, L: 'a + List + ?Sized> Slice<&'a View<'b, L>> for View<'a, L> {
    fn new<T: RangeBounds<usize>>(list: &'a View<'b, L>, bound: &T) -> Self {
        let start = start_bound(bound);
        let end = end_bound(bound, Collection::size(list));

        Self {
            list: list.list,
            start: usize::checked_add(list.start, start).expect("slice start overflow"),
            size: usize::saturating_sub(end, start),
        }
    }
}

impl<'a, L: 'a + List + ?Sized> Slice<&'a L> for View<'a, L> {
    fn new<T: RangeBounds<usize>>(list: &'a L, bound: &T) -> Self {
        let start = start_bound(bound);
        let end = end_bound(bound, Collection::size(list));

        Self {
            list,
            start,
            size: usize::saturating_sub(end, start),
        }
    }
}

pub trait SliceMut<L> {
    fn new<T: RangeBounds<usize>>(list: L, bound: &T) -> Self;
}

pub struct ViewMut<'a, L: 'a + ListMut + ?Sized> {
    list: &'a mut L,
    start: usize,
    size: usize,
}

impl<'a, L: 'a + ListMut + ?Sized> Collection for ViewMut<'a, L> {
    fn size(&self) -> usize {
        self.size
    }
}

impl<'a, L: 'a + ListMut + ?Sized> List for ViewMut<'a, L> {
    type Elem = <L as List>::Elem;
    type Slice<'b>
    = View<'b, L>
    where
        Self: 'b;

    fn get(&self, index: usize) -> Option<&Self::Elem> {
        (index < self.size).then(|| {
            List::get(
                self.list,
                usize::checked_add(self.start, index).expect("index overflow"),
            )
            .unwrap()
        })
    }
}

impl<'a, 'b, L: 'a + ListMut + ?Sized> Slice<&'a ViewMut<'b, L>> for View<'a, L> {
    fn new<T: RangeBounds<usize>>(list: &'a ViewMut<'b, L>, bound: &T) -> Self {
        let start = start_bound(bound);
        let end = end_bound(bound, Collection::size(list));

        Self {
            list: list.list,
            start: usize::checked_add(list.start, start).expect("slice start overflow"),
            size: usize::saturating_sub(end, start),
        }
    }
}

impl<'a, L: 'a + ListMut + ?Sized> ListMut for ViewMut<'a, L> {
    type SliceMut<'b>
    = ViewMut<'b, L>
    where
        Self: 'b;

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Elem> {
        (index < self.size).then(move || {
            List::get_mut(
                self.list,
                usize::checked_add(self.start, index).expect("index overflow"),
            )
            .unwrap()
        })
    }
}

impl<'a, 'b, L: 'a + ListMut + ?Sized> SliceMut<&'a mut ViewMut<'b, L>> for ViewMut<'a, L> {
    fn new<T: RangeBounds<usize>>(list: &'a mut ViewMut<'b, L>, bound: &T) -> Self {
        let start = start_bound(bound);
        let end = end_bound(bound, Collection::size(list));

        Self {
            list: list.list,
            start: usize::checked_add(list.start, start).expect("slice start overflow"),
            size: usize::saturating_sub(end, start),
        }
    }
}

impl<'a, L: 'a + ListMut + ?Sized> SliceMut<&'a mut L> for ViewMut<'a, L> {
    fn new<T: RangeBounds<usize>>(list: &'a mut L, bound: &T) -> Self {
        let start = start_bound(bound);
        let end = end_bound(bound, Collection::size(list));

        Self {
            list,
            start,
            size: usize::saturating_sub(end, start),
        }
    }
}

use alloc::vec::Vec;

impl<T> List for Vec<T> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
}

impl<T> ListMut for Vec<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

use alloc::collections::VecDeque;

impl<T> List for VecDeque<T> {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
}

impl<T> ListMut for VecDeque<T> {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }
}

impl<T, const N: usize> List for [T; N] {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
}

impl<T, const N: usize> ListMut for [T; N] {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_mut_slice().get_mut(index)
    }
}

impl<T> List for [T] {
    type Elem = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }
}

impl<T> ListMut for [T] {
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }
}
