use super::Numeric;

pub fn lcm<T>(u: T, v: T) -> T
    where T: Copy + Numeric
{
    let gcd = super::gcd(u, v);
    let product = u * v;

    product.abs() / gcd
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
