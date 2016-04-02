use std::num::{Zero, One};
use std::ops::{Shl, Shr, BitAnd, BitOr, Add, Sub, Neg, Rem};

pub fn euclid_gcd<T>(mut u: T, mut v: T) -> T
where T: Copy,
      T: Zero,
      T: PartialEq,
      T: PartialOrd,
      T: Rem<Output = T>,
      T: Neg<Output = T>
{
    let mut t;
    let zero = T::zero();
    while v != zero {
        t = u;
        u = v;
        v = t % v;
    }

    if u < zero {
        -u
    } else {
        u
    }
}

pub fn binary_gcd<T>(mut u: T, mut v: T) -> T
where T: Copy,
      T: Zero,
      T: One,
      T: PartialEq,
      T: PartialOrd,
      T: Shl<T, Output = T>,
      T: Shr<T, Output = T>,
      T: BitAnd<Output = T>,
      T: BitOr<Output = T>,
      T: Add<Output = T>,
      T: Sub<Output = T>
{
    if u == v {
        return u;
    }

    let zero = T::zero();
    if u == zero || v == zero {
        return u + v;
    }

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
fn test_euclid_zero() {
    assert_eq!(0, euclid_gcd(0, 0));
}

#[test]
fn test_euclid_same() {
    assert_eq!(10, euclid_gcd(10, 10));
}

#[test]
fn test_euclid_simple() {
    assert_eq!(7, euclid_gcd(14, 21));
}

#[test]
fn test_euclid_prime() {
    assert_eq!(1, euclid_gcd(132512537, 132512351));
}
#[test]
fn test_binary_zero() {
    assert_eq!(0, binary_gcd(0, 0));
}

#[test]
fn test_binary_same() {
    assert_eq!(10, binary_gcd(10, 10));
}

#[test]
fn test_binary_simple() {
    assert_eq!(7, binary_gcd(14, 21));
}

#[test]
fn test_binary_prime() {
    assert_eq!(1, binary_gcd(132512537, 132512351));
}

#[bench]
fn bench_euclid_primes(b: &mut ::test::Bencher) {
    b.iter(|| euclid_gcd(132512537, 132512351))
}

#[bench]
fn bench_binary_primes(b: &mut ::test::Bencher) {
    b.iter(|| binary_gcd(132512537, 132512351))
}
