pub use self::insertion::sort as insertion_sort;
pub use self::insertion::sort_with_copy as insertion_sort_with_copy;
pub use self::insertion::sort_with_binary_search as insertion_sort_with_binary_search;

pub use self::selection::sort as selection_sort;

pub use self::merge::sort as merge_sort;

pub use self::heap::sort as heap_sort;

mod insertion;
mod selection;
mod merge;
mod heap;

#[bench]
fn bench_sort_default(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut arr: Vec<u32> = (0..1000).rev().collect();

        arr.sort();
    })
}
