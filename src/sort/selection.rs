pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..(len - 1) {
        let mut min_idx = i;

        for j in (i + 1)..len {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        arr.swap(min_idx, i);
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
