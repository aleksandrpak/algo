//! Algorithms for selecting a particular element of an unsorted slice.

use std::cmp::Ordering;
use std::mem;

pub use self::qselect::*;

mod qselect;

/// Get the median of `data` (the element that would be at `data.len() / 2` if the slice was sorted).
///
/// Allocates scratch space so it doesn't have to modify `data`.
///
/// ##Panics
/// If `data` is empty (`data.len() / 2` is out of bounds).
pub fn median<T: Ord>(data: &[T]) -> &T {
    qselect(data, data.len() / 2)
}

/// Given an ordering function, get the median of `data` (the element that would be at
/// `data.len() / 2` if the slice was sorted).
///
/// Allocates scratch space so it doesn't have to modify `data`.
///
/// ##Panics
/// If `data` is empty (`data.len() / 2` is out of bounds).
pub fn median_by<T, F: FnMut(&T, &T) -> Ordering>(data: &[T], ord_fn: F) -> &T {
    qselect_by(data, data.len() / 2, ord_fn)
}

/// Given an ordering function, get the median of `data` (the element that would be at
/// `data.len() / 2` if the slice was sorted).
///
/// This performs the selection operation in-place (`data` will be partially [or fully] sorted
/// in the process).
///
/// ##Panics
/// If `data` is empty (`data.len() / 2` is out of bounds).
pub fn median_inplace<T: Ord>(data: &mut [T]) -> &mut T {
    let median_idx = data.len() / 2;
    qselect_inplace(data, median_idx)
}

/// Get the median of `data` (the element that would be at `data.len() / 2` if the slice was sorted).
///
/// This performs the selection operation in-place (`data` will be partially [or fully] sorted
/// in the process).
///
/// ##Panics
/// If `data` is empty (`data.len() / 2` is out of bounds).
pub fn median_inplace_by<T, F: FnMut(&T, &T) -> Ordering>(data: &mut [T], ord_fn: F) -> &mut T {
    let median_idx = data.len() / 2;
    qselect_inplace_by(data, median_idx, ord_fn)
}

/// Of the three values passed, return the median.
pub fn median_of_3<T: Ord>(x: T, y: T, z: T) -> T {
    median_of_3_by(x, y, z, <T as Ord>::cmp)
}

/// Given an ordering function, of the three values passed, return the median.
pub fn median_of_3_by<T, F: FnMut(&T, &T) -> Ordering>(mut x: T, mut y: T, mut z: T, mut ord_fn: F) -> T {
    in_order_by(&mut x, &mut y, &mut ord_fn);
    in_order_by(&mut x, &mut z, &mut ord_fn);
    in_order_by(&mut y, &mut z, &mut ord_fn);

    y
}

/// If `x > y`, swap `x` and `y`.
#[inline]
pub fn in_order<T: Ord>(x: &mut T, y: &mut T) {
    if x > y {
        mem::swap(x, y);
    }
}

/// Given an ordering function, if `x > y`, swap `x` and `y`.
#[inline]
pub fn in_order_by<T, F: FnMut(&T, &T) -> Ordering>(x: &mut T, y: &mut T, mut ord_fn: F) {
    if ord_fn(x, y) == Ordering::Greater {
        mem::swap(x, y);
    }
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