use porus::prelude::{Chunk, SinglyLinkedList, Stack};
use proptest::prelude::*;
use std::alloc::Global;
use std::cell::Cell;
use std::fmt::Debug;

pub struct Item<'a, T> {
    value: T,
    counter: &'a Cell<usize>,
}

impl<'a, T> Drop for Item<'a, T> {
    fn drop(&mut self) {
        self.counter.replace(self.counter.get() + 1);
    }
}

#[derive(Debug, Clone)]
enum ArbitraryPool {
    Global,
    Chunk,
}

impl ArbitraryPool {
    fn allocate_stack<'a, T: 'a>(&self) -> Box<dyn 'a + Stack<Elem = T>> {
        match self {
            ArbitraryPool::Global => Box::new(SinglyLinkedList::<T>::new_in(Global)) as _,
            ArbitraryPool::Chunk => {
                Box::new(SinglyLinkedList::<T, _, _>::new_in(Chunk::<_>::new())) as _
            }
        }
    }
}

fn arbitrary_pool() -> impl Strategy<Value = ArbitraryPool> {
    prop_oneof![Just(ArbitraryPool::Global), Just(ArbitraryPool::Chunk)]
}

proptest! {
    #[test]
    fn pool(v: Vec::<usize>, pool in arbitrary_pool()) {
        let counter = Cell::new(0);

        {
            let mut b = pool.allocate_stack();
            let s = b.as_mut();
            prop_assert!(Stack::is_empty(s));
            assert!(matches!(Stack::top(s), None));

            for e in v.iter() {
                Stack::push(s, Item {value: *e, counter: &counter});
                prop_assert!(!Stack::is_empty(s));
                assert!(matches!(Stack::top(s), Some(&Item {value: v, counter: c}) if v == *e && c == &counter));
            }

            for e in v.iter().rev() {
                assert!(matches!(Stack::top(s), Some(&Item {value: v, counter: c}) if v == *e && c == &counter));
                assert!(matches!(Stack::pop(s), Some(Item {value: v, counter: c}) if v == *e && c == &counter));
            }

            assert!(matches!(Stack::top(s), None));
            prop_assert!(Stack::is_empty(s));
        }

        assert!(counter.into_inner() == v.len());
    }
}
