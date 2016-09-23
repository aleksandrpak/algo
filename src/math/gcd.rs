use std::mem;

use super::{Numeric, Primitive};

pub fn euclid_gcd<T>(mut u: T, mut v: T) -> T
where T: Copy + Numeric
{
    let mut t;
    let zero = T::zero();
    while v != zero {
        t = u;
        u = v;
        v = t % v;
    }

    u.abs()
}

pub fn binary_gcd<T>(mut u: T, mut v: T) -> T
where T: Copy + Primitive
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
            mem::swap(&mut u, &mut v);
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
