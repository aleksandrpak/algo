pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j = j - 1;
        }
    }
}

#[test]
fn test_insertion_sort_simple() {
    let mut arr = [-5, 4, 1, -3, 2];

    sort(&mut arr);

    assert!(arr == [-5, -3, 1, 2, 4]);
}
