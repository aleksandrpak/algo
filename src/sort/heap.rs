pub fn sort<T: PartialOrd>(arr: &mut [T]) {
    heapify(arr);

    let mut end = arr.len() - 1;

    while end > 0 {
        arr.swap(0, end);
        end -= 1;

        sift_down(arr, 0, end);
    }
}

fn heapify<T: PartialOrd>(arr: &mut [T]) {
    let end = arr.len() - 1;
    let mut start = parent(end);

    loop {
        sift_down(arr, start, end);

        if start == 0 {
            break;
        }

        start -= 1;
    }
}

fn sift_down<T: PartialOrd>(arr: &mut [T], start: usize, end: usize) {
    let mut root = start;
    let mut left = left_child(root);

    while left <= end {
        let mut swap = root;
        let right = right_child(root);

        if arr[swap] < arr[left] {
            swap = left;
        }

        if right <= end && arr[swap] < arr[right] {
            swap = right;
        }

        if swap == root {
            break;
        } else {
            arr.swap(root, swap);
            root = swap;
            left = left_child(root);
        }
    }
}

fn parent(i: usize) -> usize {
    ((i - 1) as f32 / 2.0).floor() as usize
}

fn left_child(i: usize) -> usize {
    2 * i + 1
}

fn right_child(i: usize) -> usize {
    2 * i + 2
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
