use porus::prelude::{list, BinaryHeap, Heap};
use proptest::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum ArbitraryHeap {
    BinaryHeap,
}

impl ArbitraryHeap {
    fn allocate<'a, T: 'a + Ord>(&self) -> Box<dyn 'a + Heap<Elem = T>> {
        match self {
            ArbitraryHeap::BinaryHeap => Box::new(BinaryHeap::new()) as _,
        }
    }
}

fn arbitrary_heap() -> impl Strategy<Value = ArbitraryHeap> {
    prop_oneof![Just(ArbitraryHeap::BinaryHeap),]
}

proptest! {
    #[test]
    fn heap(mut v: Vec::<usize>, heap in arbitrary_heap()) {
        let mut b = heap.allocate();
        let h = b.as_mut();

        prop_assert_eq!(None, Heap::peek(h));

        for e in v.iter() {
            Heap::push(h, *e);
        }

        list::quick_sort(&mut v, PartialOrd::gt);

        for e in v.iter() {
            prop_assert_eq!(Some(e), Heap::peek(h));
            prop_assert_eq!(Some(*e), Heap::pop(h));
        }

        prop_assert_eq!(None, Heap::peek(h));
    }
}
