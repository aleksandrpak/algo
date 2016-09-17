use std::mem;

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
    let len = data.len();
    assert!(k < len, "Called qselect with k = {} and data length: {}", k, len);

    let mut refs: Vec<_> = data.iter().collect();
    *qselect_inplace(&mut refs, k)
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
    let len = data.len();

    assert!(k < len, "Called qselect_inplace with k = {} and data length: {}", k, len);

    if len < SORT_THRESH { 
        data.sort();  
        return &mut data[k];
    }

    let pivot_idx = partition(data);

    if k == pivot_idx {
        &mut data[pivot_idx]
    } else if k < pivot_idx {
        qselect_inplace(&mut data[..pivot_idx], k)
    } else {
        qselect_inplace(&mut data[pivot_idx + 1..], k - pivot_idx - 1)
    }
}

fn partition<T: Ord>(data: &mut [T]) -> usize {
    let len = data.len();

    let pivot_idx = {
        let first = (&data[0], 0);
        let mid = (&data[len / 2], len / 2);
        let last = (&data[len - 1], len - 1);

        median_of_3(&first, &mid, &last).1
    };

    data.swap(pivot_idx, len - 1);

    let mut curr = 0;

    for i in 0 .. len - 1 {
        if data[i] < data[len - 1] {
            data.swap(i, curr);
            curr += 1;
        }
    }

    data.swap(curr, len - 1);

    curr
}


/// Of the three values passed, return the median.
pub fn median_of_3<T: Ord>(mut x: T, mut y: T, mut z: T) -> T {
    in_order(&mut x, &mut y);
    in_order(&mut x, &mut z);
    in_order(&mut y, &mut z);
    
    y
}

/// If `x > y`, swap `x` and `y`.
#[inline]
pub fn in_order<T: Ord>(x: &mut T, y: &mut T) {
    if x > y {
        mem::swap(x, y);
    }
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

#[test]
fn test_median() {
    macro_rules! test_median {
        ($x:expr, $y:expr, $z:expr; $expected:expr) => (
            let x = $x;
            let y = $y;
            let z = $z;
            let result = median_of_3(&x, &y, &z);
            assert_eq!(*result, $expected);
        )
    }
    
    test_median!(1, 2, 3; 2);
    test_median!(1, 3, 2; 2);
    test_median!(2, 1, 3; 2);
    test_median!(2, 3, 1; 2);
    test_median!(3, 1, 2; 2);
    test_median!(3, 2, 1; 2);
}
