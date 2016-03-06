pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j = j - 1;
        }
    }
}

pub fn sort_with_copy<T: PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i - 1;

        while arr[j] > key {
            arr.swap(j + 1, j);

            if j == 0 {
                break;
            }

            j = j - 1;
        }

        arr[j + 1] = key;
    }
}

pub fn sort_with_binary_search<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        match arr[0..i].binary_search(&arr[i]) {
            Ok(_) => continue,
            Err(idx) => {
                for j in 0..(i - idx) {
                    arr.swap(i - j, i - j - 1);
                }
            }
        }
    }
}

#[test]
fn test_sort_simple() {
    let mut arr = [-5, 4, 1, -3, 2];

    sort(&mut arr);

    assert!(arr == [-5, -3, 1, 2, 4]);
}

#[test]
fn test_sort_with_copy() {
    let mut arr = [-5, 4, 1, -3, 2];

    sort_with_copy(&mut arr);

    assert!(arr == [-5, -3, 1, 2, 4]);
}

#[test]
fn test_sort_with_binary_search() {
    let mut arr = [-5, 4, 1, -3, 2];

    sort_with_binary_search(&mut arr);

    assert!(arr == [-5, -3, 1, 2, 4]);
}

#[bench]
fn bench_sort_simple(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut arr: Vec<u32> = (0..1000).rev().collect();

        sort(&mut arr);
    })
}

#[bench]
fn bench_sort_with_copy(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut arr: Vec<u32> = (0..1000).rev().collect();

        sort_with_copy(&mut arr);
    })
}

#[bench]
fn bench_sort_with_binary_search(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut arr: Vec<u32> = (0..1000).rev().collect();

        sort_with_binary_search(&mut arr);
    })
}
