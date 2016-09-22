use std::ops::{Shl, Shr, BitAnd, BitOr, Add, Sub, Mul, Div, Rem};

pub use self::gcd::euclid_gcd as gcd;
pub use self::gcd::{euclid_gcd, binary_gcd};

pub use self::lcm::lcm;

mod gcd;
mod lcm;

/// Trait encompassing all numeric types usable by this module.
pub trait Numeric: PartialEq + PartialOrd +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> +
    Rem<Output = Self> + Sized
{
    /// Return the absolute value of `self`. No-op for unsigned types.
    fn abs(self) -> Self;

    /// Return the equivalent of `0` for this type.
    fn zero() -> Self;

    /// Return the equivalent of `1` for this type.
    fn one() -> Self;
}

/// Trait encompassing all numeric types supporting bitwise operations.
pub trait Primitive: Numeric + Shl<Self, Output = Self> + Shr<Self, Output = Self> +
BitAnd<Output = Self> + BitOr<Output = Self> {}

macro_rules! impl_numeric_signed {
    ($($ty:ty),*) => (
        $(impl Numeric for $ty {
            fn abs(self) -> Self {
                // Every signed primitive has an `.abs()` method with guaranteed lowering
                // to one instruction.
                self.abs()
            }

            fn zero() -> Self {
                0 as $ty
            }

            fn one() -> Self {
                1 as $ty
            }
        })*
    )
}

macro_rules! impl_numeric_unsigned {
    ($($ty:ty),*) => (
        $(impl Numeric for $ty {
            /// No-op
            fn abs(self) -> Self {
                self
            }

            fn zero() -> Self {
                0 as $ty
            }

            fn one() -> Self {
                1 as $ty
            }
        })*
    )
}

macro_rules! impl_primitive {
    ($($ty:ty),*) => (
        $(impl Primitive for $ty {})*
    )
}

impl_numeric_signed! { f32, f64, i8, i16, i32, i64 }
impl_numeric_unsigned! { u8, u16, u32, u64 }
impl_primitive! { i8, i16, i32, i64, u8, u16, u32, u64 }