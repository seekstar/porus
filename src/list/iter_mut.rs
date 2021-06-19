use super::{get_mut, List, ListMut};
use crate::collection;
use crate::iter::Iter;

#[allow(clippy::module_name_repetitions)]
pub struct ListIterMut<'a, T: ListMut> {
    list: &'a mut T,
    start: usize,
    end: usize,
}

impl<'a, T: ListMut> Iter for ListIterMut<'a, T> {
    type Item<'b>
    where
        Self: 'b,
    = &'b mut <T as List>::Elem;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        (self.start < self.end).then(move || {
            let index = self.start;
            self.start = usize::wrapping_add(self.start, 1);
            get_mut(self.list, index)
        })
    }
}

pub fn iter_mut<T: ListMut>(list: &mut T) -> ListIterMut<'_, T> {
    let end = collection::size(list);
    ListIterMut {
        list,
        start: 0,
        end,
    }
}
