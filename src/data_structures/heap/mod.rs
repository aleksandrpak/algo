pub use self::fibonacci::FibonacciHeap;

mod fibonacci;

#[bench]
fn bench_push_pop_binary(b: &mut ::test::Bencher) {
    b.iter(|| {
        let mut heap = ::std::collections::BinaryHeap::new();

        for i in 1..10001 {
            heap.push(i);
        }

        for _ in 1..10001 {
            heap.pop();
        }
    })
}
