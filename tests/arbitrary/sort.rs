use porus::list::ListMut;
use porus::prelude::list;
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum ArbitrarySort {
    BubbleSort,
    InsertionSort,
    SelectionSort,
    ShellSort(Vec<usize>),
    QuickSort,
}

impl ArbitrarySort {
    fn sort<E, L: ListMut<Elem = E>, F: Fn(&E, &E) -> bool>(&self, list: &mut L, lt: F) {
        match self {
            ArbitrarySort::BubbleSort => {
                list::bubble_sort(list, lt);
            }
            ArbitrarySort::InsertionSort => {
                list::insertion_sort(list, lt);
            }
            ArbitrarySort::SelectionSort => {
                list::selection_sort(list, lt);
            }
            ArbitrarySort::ShellSort(gaps) => {
                list::shell_sort(list, lt, gaps);
            }
            ArbitrarySort::QuickSort => {
                list::quick_sort(list, lt);
            }
        }
    }
}

fn arbitrary_sort() -> impl Strategy<Value = ArbitrarySort> {
    prop_oneof![
        Just(ArbitrarySort::BubbleSort),
        Just(ArbitrarySort::InsertionSort),
        Just(ArbitrarySort::SelectionSort),
        Just(ArbitrarySort::ShellSort(vec![
            797161, 265720, 88573, 29524, 9841, 3280, 1093, 364, 121, 40, 13, 4, 1
        ])),
        Just(ArbitrarySort::QuickSort)
    ]
}

fn arbitrary_stable_sort() -> impl Strategy<Value = ArbitrarySort> {
    prop_oneof![
        Just(ArbitrarySort::BubbleSort),
        Just(ArbitrarySort::InsertionSort),
    ]
}

proptest! {
    #[test]
    fn sort(mut v: Vec::<usize>, sort in arbitrary_sort()) {
        sort.sort(&mut v, PartialOrd::lt);
        prop_assert!(v.iter().is_sorted());
    }

    #[test]
    fn stable_sort(mut v: Vec::<usize>, sort in arbitrary_stable_sort()) {
        let s: &mut Vec<usize> = &mut (0..v.len()).collect();
        sort.sort(s, |&i, &j| list::get(&v, i) < list::get(&v, j));
        prop_assert!(list::is_stable_sort(&v, PartialOrd::lt, s));
    }
}
