use std::cmp::{Eq, Ord, Ordering};
use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Copy, Clone, Debug)]
pub struct Float(f32);

impl Float {
    pub const ZERO: Float = Float(0.0);
    pub const ONE: Float = Float(1.0);
    pub const MAX: Float = Float(std::f32::MAX);

    #[inline]
    pub fn sqrt(self) -> Self {
        Float(self.0.sqrt())
    }

    #[inline]
    pub fn abs(self) -> Self {
        Float(self.0.abs())
    }

    #[inline]
    pub fn powf(self, e: f32) -> Self {
        Float(self.0.powf(e))
    }

    #[inline]
    pub fn signum(self) -> Self {
        Float(self.0.signum())
    }
}

impl<T> PartialEq<T> for Float
where
    T: Copy,
    Float: From<T>,
{
    #[inline]
    fn eq(&self, rhs: &T) -> bool {
        let rhs = Float::from(*rhs);
        self.0.eq(&rhs.0)
    }
}

impl Eq for Float {}

impl<T> PartialOrd<T> for Float
where
    T: Copy,
    Float: From<T>,
{
    #[inline]
    fn partial_cmp(&self, rhs: &T) -> Option<Ordering> {
        let rhs = Float::from(*rhs);
        self.0.partial_cmp(&rhs.0)
    }
}

impl Ord for Float {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl From<f32> for Float {
    #[inline]
    fn from(f: f32) -> Self {
        Float(f)
    }
}

impl From<Float> for f32 {
    #[inline]
    fn from(f: Float) -> Self {
        f.0
    }
}

impl From<Float> for u8 {
    #[inline]
    fn from(f: Float) -> Self {
        f.0 as u8
    }
}

macro_rules! binop {
    ($name:ident, $func:ident, $assname:ident, $assfunc:ident, $op:expr) => {
        impl<T> $name<T> for Float
        where
            Float: From<T>,
        {
            type Output = Float;

            #[inline]
            fn $func(self, rhs: T) -> Self::Output {
                let rhs = Float::from(rhs);
                Float($op(self.0, rhs.0))
            }
        }
        impl<T> $assname<T> for Float
        where
            Float: From<T>,
        {
            #[inline]
            fn $assfunc(&mut self, rhs: T) {
                let rhs = Float::from(rhs);
                *self = Float($op(self.0, rhs.0))
            }
        }
    };
}

binop!(Add, add, AddAssign, add_assign, |l, r| l + r);
binop!(Sub, sub, SubAssign, sub_assign, |l, r| l - r);
binop!(Mul, mul, MulAssign, mul_assign, |l, r| l * r);
binop!(Div, div, DivAssign, div_assign, |l, r| l / r);
binop!(Rem, rem, RemAssign, rem_assign, |l, r| l % r);

impl Neg for Float {
    type Output = Float;

    #[inline]
    fn neg(self) -> Self {
        Float(-self.0)
    }
}

impl Deref for Float {
    type Target = f32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
