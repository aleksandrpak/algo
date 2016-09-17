use std::cmp::Ordering;

// The threshold below which qselect_inplace_by() should just sort the slice.
const SORT_THRESH: usize = 6;

/// Use the Quickselect algorithm to select the `k`th smallest
/// element from `data`. 
///
/// Does not modify `data`. Allocates an additional vector for scratch space; see `qselect_inplace`
/// for an in-place version.
///
/// ##Panics
/// If `k` is greater than `data.len()`.
pub fn qselect<T: Ord>(data: &[T], k: usize) -> &T {
    qselect_by(data, k, <T as Ord>::cmp)
}

/// Use the Quickselect algorithm, given an ordering function, to select the `k`th
/// smallest element from `data`.
///
/// Does not modify `data`. Allocates an additional vector for scratch space; see `qselect_inplace_by`
/// for an in-place version.
///
/// ##Panics
/// If `k` is greater than `data.len()`.
pub fn qselect_by<T, F: FnMut(&T, &T) -> Ordering>(data: &[T], k: usize, mut ord_fn: F) -> &T {
    let len = data.len();
    assert!(k < len, "Called qselect with k = {} and data length: {}", k, len);

    let mut refs: Vec<_> = data.iter().collect();
    *qselect_inplace_by(&mut refs, k, |l, r| ord_fn(l, r))
}

/// Use the Quickselect algorithm to select the `k`th smallest element from `data`. As part of the
/// algorithm, `k` is moved to its final sorted position and the rest of the array is (at least) partially
/// sorted.
///
/// For a version that uses extra space but does not reorder the slice, use `qselect`.
///
/// ##Panics
/// If `k` is greater than `data.len()`.
pub fn qselect_inplace<T: Ord>(data: &mut [T], k: usize) -> &mut T {
    qselect_inplace_by(data, k, <T as Ord>::cmp)
}

/// Use the Quickselect algorithm, given an ordering function,
/// to select the `k`th smallest element from `data`. As part of the
/// algorithm, `k` is moved to its final sorted position and the rest of the array is (at least) partially
/// sorted.
///
/// For a version that uses extra space but does not reorder the slice, use `qselect_by`.
///
/// ##Panics
/// If `k` is greater than `data.len()`.
pub fn qselect_inplace_by<T, F: FnMut(&T, &T) -> Ordering>(data: &mut [T], k: usize, mut ord_fn: F) -> &mut T {
    let len = data.len();

    assert!(k < len, "Called qselect_inplace with k = {} and data length: {}", k, len);

    if len < SORT_THRESH {
        data.sort_by(&mut ord_fn);
        return &mut data[k];
    }

    let pivot_idx = partition_by(data, &mut ord_fn);

    if k == pivot_idx {
        &mut data[pivot_idx]
    } else if k < pivot_idx {
        qselect_inplace_by(&mut data[..pivot_idx], k, ord_fn)
    } else {
        qselect_inplace_by(&mut data[pivot_idx + 1..], k - pivot_idx - 1, ord_fn)
    }
}

/// Given an ordering function, pick an arbitrary pivot and partition the slice, returning the pivot.
fn partition_by<T, F: FnMut(&T, &T) -> Ordering>(data: &mut [T], mut ord_fn: F) -> usize {
    let len = data.len();

    let pivot_idx = {
        let first = (&data[0], 0);
        let mid = (&data[len / 2], len / 2);
        let last = (&data[len - 1], len - 1);

        super::median_of_3_by(&first, &mid, &last, |left, right| ord_fn(left.0, right.0)).1
    };

    data.swap(pivot_idx, len - 1);

    let mut curr = 0;

    for i in 0 .. len - 1 {
        if ord_fn(&data[i], &data[len - 1]) == Ordering::Less {
            data.swap(i, curr);
            curr += 1;
        }
    }

    data.swap(curr, len - 1);

    curr
}

#[test]
fn test_qsel() {
    macro_rules! test_qsel (
        ($($elem:expr),+; $k:expr; $expect:expr) => (
            let mut data = [$($elem),+];
            let selected = qselect_inplace(&mut data, $k);
            assert_eq!(*selected, $expect);
        )
    );

    test_qsel!(1, 2, 3, 4, 5; 3; 4);
    test_qsel!(5, 4, 3, 2, 1; 1; 2);
    test_qsel!(2, 2, 3, 3, 4; 1; 2);
}
