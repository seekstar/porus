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

impl Arbitrary for ArbitrarySort {
    type Parameters = ();
    type Strategy = impl Strategy<Value = Self>;
    fn arbitrary_with(_: ()) -> Self::Strategy {
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
}

proptest! {
    #[test]
    fn sort(mut v: Vec::<usize>, sort: ArbitrarySort) {
        sort.sort(&mut v, PartialOrd::lt);
        prop_assert!(v.iter().is_sorted());
    }
}
