use std::ops::Add;

pub fn max_subarray<T>(arr: &[T]) -> (usize, usize, T)
    where T: Copy,
          T: Add<Output = T>,
          T: PartialOrd
{
    let len = arr.len();
    max_subarray_rec(arr, 0, len - 1)
}

fn max_subarray_rec<T>(arr: &[T], low: usize, high: usize) -> (usize, usize, T)
    where T: Copy,
          T: Add<Output = T>,
          T: PartialOrd
{
    if low == high {
        (low, high, arr[low])
    } else {
        let mid = (high + low) / 2;

        let (left_low, left_high, left_max) = max_subarray_rec(arr, low, mid);
        let (right_low, right_high, right_max) = max_subarray_rec(arr, mid + 1, high);
        let (cross_low, cross_high, cross_max) = cross_max_subarray(arr, low, mid, high);

        if left_max >= right_max && left_max >= cross_max {
            (left_low, left_high, left_max)
        } else if right_max >= left_max && right_max >= cross_max {
            (right_low, right_high, right_max)
        } else {
            (cross_low, cross_high, cross_max)
        }
    }
}

fn cross_max_subarray<T>(arr: &[T], low: usize, mid: usize, high: usize) -> (usize, usize, T)
    where T: Copy,
          T: Add<Output = T>,
          T: PartialOrd
{
    let (left_idx, left_max) = cross_part_max(arr, (low...mid).rev());
    let (right_idx, right_max) = cross_part_max(arr, (mid + 1)...high);

    (left_idx, right_idx, left_max + right_max)
}

fn cross_part_max<T, I>(arr: &[T], mut range: I) -> (usize, T)
    where T: Copy,
          T: Add<Output = T>,
          T: PartialOrd,
          I: Sized + Iterator<Item = usize>
{
    let mut idx = range.next().unwrap();
    let mut max = arr[idx];
    let mut sum = max;

    for i in range {
        sum = sum + arr[i];

        if sum > max {
            max = sum;
            idx = i;
        }
    }

    (idx, max)
}

#[test]
fn test_simple() {
    let arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];

    let (low, high, sum) = max_subarray(&arr);

    assert_eq!(low, 7);
    assert_eq!(high, 10);
    assert_eq!(sum, 43);
}

#[bench]
fn bench_simple(b: &mut ::test::Bencher) {
    b.iter(|| {
        let arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        max_subarray(&arr);
    })
}
