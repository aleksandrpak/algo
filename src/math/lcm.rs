use std::num::Zero;
use std::ops::{Mul, Div, Neg, Rem};

pub fn lcm<T>(u: T, v: T) -> T
    where T: Copy,
          T: Zero,
          T: PartialEq,
          T: PartialOrd,
          T: Mul<Output = T>,
          T: Div<Output = T>,
          T: Rem<Output = T>,
          T: Neg<Output = T>
{

    let gcd = super::gcd(u, v);
    let product = u * v;
    let zero = T::zero();

    if product < zero {
        -product / gcd
    } else {
        product / gcd
    }
}

#[test]
fn test_zero() {
    assert_eq!(0, lcm(0, 1));
}

#[test]
fn test_same() {
    assert_eq!(4, lcm(4, 4));
}

#[test]
fn test_simple() {
    assert_eq!(42, lcm(21, 6));
}

#[test]
fn test_negative() {
    assert_eq!(42, lcm(21, -6));
}

#[test]
fn test_prime() {
    assert_eq!(77, lcm(7, 11));
}
