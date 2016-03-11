pub fn sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        sort(&mut arr[..(len / 2)]);
        sort(&mut arr[(len / 2)..]);

        merge(arr);
    }
}

fn merge<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    let left_len = len / 2;
    let right_len = len - left_len;

    let mut left = Vec::<T>::with_capacity(left_len);
    let mut right = Vec::<T>::with_capacity(right_len);

    left.extend_from_slice(&arr[..left_len]);
    right.extend_from_slice(&arr[left_len..]);

    let mut i = 0;
    let mut j = 0;

    for k in 0..len {
        if i < left_len && (j == right_len || left[i] < right[j]) {
            arr[k] = left[i];
            i = i + 1;
        } else {
            arr[k] = right[j];
            j = j + 1;
        }
    }
}

#[test]
fn test_sort() {
    let mut arr = [-5, 4, 1, -3, 2];

    sort(&mut arr);

    assert!(arr == [-5, -3, 1, 2, 4]);
}

#[bench]
fn bench_sort(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut arr: Vec<u32> = (0..1000).rev().collect();

        sort(&mut arr);
    })
}
