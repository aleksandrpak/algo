pub use self::fibonacci::FibonacciHeap;
pub use self::unsafe_fibonacci::FibonacciHeap as UnsafeFibonacciHeap;

mod fibonacci;
mod unsafe_fibonacci;

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
