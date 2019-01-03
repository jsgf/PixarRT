#![allow(clippy::float_cmp)]

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(all(feature = "vec-simd", target_arch = "x86_64", any(target_feature = "avx", target_feature = "sse4.1")))]
mod vec_x86_64_simd;
#[cfg(all(feature = "vec-simd", target_arch = "x86_64", any(target_feature = "avx", target_feature = "sse4.1")))]
use self::vec_x86_64_simd as arch;

#[cfg(not(all(feature = "vec-simd", target_arch = "x86_64", any(target_feature = "avx", target_feature = "sse4.1"))))]
mod vec_portable;
#[cfg(not(all(feature = "vec-simd", target_arch = "x86_64", any(target_feature = "avx", target_feature = "sse4.1"))))]
use self::vec_portable as arch;

pub use self::arch::V;

impl<T> AddAssign<T> for V
where
    V: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = self.add(rhs);
    }
}

impl<T> MulAssign<T> for V
where
    V: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = self.mul(rhs);
    }
}

impl<T> SubAssign<T> for V
where
    V: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = self.sub(rhs);
    }
}

#[test]
fn init3() {
    let v = V::new(1., 2., 3.);

    assert_eq!(v.x(), 1.);
    assert_eq!(v.y(), 2.);
    assert_eq!(v.z(), 3.);
}

#[test]
fn init1() {
    let v = V::from(1.1);

    assert_eq!(v.x(), 1.1);
    assert_eq!(v.y(), 1.1);
    assert_eq!(v.z(), 1.1);
}

#[test]
fn add() {
    let a = V::new(1., 2., 3.);
    let b = V::new(3., 2., 1.);

    let v = a + b;

    assert_eq!(v.x(), 4.0);
    assert_eq!(v.y(), 4.0);
    assert_eq!(v.z(), 4.0);
}

#[test]
fn mul() {
    let a = V::new(2., 4., 8.);
    let b = V::new(1., 0.5, 0.25);

    let v = a * b;

    assert_eq!(v.x(), 2.0);
    assert_eq!(v.y(), 2.0);
    assert_eq!(v.z(), 2.0);
}

#[test]
fn dot() {
    let a = V::new(2., 4., 8.);
    let b = V::new(1., 0.5, 0.25);

    let v = a % b;

    assert_eq!(v, 6.0);
}
