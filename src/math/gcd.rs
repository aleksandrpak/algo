use std::num::{Zero, One};
use std::ops::{Shl, Shr, BitAnd, BitOr, Add, Sub};

pub fn gcd<T>(a: T, b: T) -> T where
    T: Copy,
    T: Zero,
    T: One,
    T: PartialEq,
    T: PartialOrd,
    T: Shl<T, Output=T>,
    T: Shr<T, Output=T>,
    T: BitAnd<Output=T>,
    T: BitOr<Output=T>,
    T: Add<Output=T>,
    T: Sub<Output=T> {
    let zero = T::zero();

    if a == b {
        return a;
    }

    if a == zero || b == zero {
        return a + b;
    }

    let mut u = a;
    let mut v = b;
    let mut shift = T::zero();
    let one = T::one();

    while ((u | v) & one) == zero {
        u = u >> one;
        v = v >> one;

        shift = shift + one;
    }

    while (u & one) == zero {
        u = u >> one;
    }

    while v != zero {
        while (v & one) == zero {
            v = v >> one; 
        }

        if u > v {
            let t = v;
            v = u;
            u = t;
        }

        v = v - u;
    }

    u << shift
}

#[test]
fn test_zero() {
    assert_eq!(0, gcd(0, 0));
}

#[test]
fn test_same() {
    assert_eq!(10, gcd(10, 10));
}

#[test]
fn test_simple() {
    assert_eq!(7, gcd(14, 21));
}

#[test]
fn test_prime() {
    assert_eq!(1, gcd(132512537, 132512351));
}

#[bench]
fn bench_primes(b: &mut ::test::Bencher) {
    b.iter(|| {
        gcd(132512537, 132512351)
    })
}
