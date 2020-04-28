use porus::prelude::{SinglyLinkedList, Stack};
use proptest::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum ArbitraryStack {
    Vec,
    SinglyLinkedList,
}

impl ArbitraryStack {
    fn allocate<'a, T: 'a>(&self) -> Box<dyn 'a + Stack<Elem = T>> {
        match self {
            ArbitraryStack::Vec => Box::new(Vec::<T>::new()) as _,
            ArbitraryStack::SinglyLinkedList => Box::new(SinglyLinkedList::<T>::new()) as _,
        }
    }
}

fn arbitrary_stack() -> impl Strategy<Value = ArbitraryStack> {
    prop_oneof![
        Just(ArbitraryStack::Vec),
        Just(ArbitraryStack::SinglyLinkedList)
    ]
}

proptest! {
    #[test]
    fn stack(v: Vec::<usize>, stack in arbitrary_stack()) {
        let mut b = stack.allocate();
        let s = b.as_mut();
        prop_assert!(Stack::is_empty(s));
        prop_assert_eq!(None, Stack::top(s));
        for e in v.iter() {
            Stack::push(s, *e);
            prop_assert!(!Stack::is_empty(s));
            prop_assert_eq!(Some(e), Stack::top(s));
        }

        for e in v.iter().rev() {
            prop_assert_eq!(Some(e), Stack::top(s));
            prop_assert_eq!(Some(*e), Stack::pop(s));
        }

        prop_assert_eq!(None, Stack::top(s));
        prop_assert!(Stack::is_empty(s));
    }
}
