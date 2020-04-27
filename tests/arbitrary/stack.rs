use porus::prelude::Stack;
use proptest::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
enum ArbitraryStack<T: Debug + Clone> {
    Vec(Vec<T>),
}

impl<T: Debug + Clone> ArbitraryStack<T> {
    fn as_mut(&mut self) -> &mut dyn Stack<Elem = T> {
        match self {
            ArbitraryStack::Vec(v) => v as _,
        }
    }
}

fn arbitrary_stack<T: Debug + Clone>() -> impl Strategy<Value = ArbitraryStack<T>> {
    prop_oneof![Just(ArbitraryStack::Vec(Vec::<T>::new()))]
}

proptest! {
    #[test]
    fn stack(v: Vec::<usize>, mut stack in arbitrary_stack()) {
        let s = stack.as_mut();
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
