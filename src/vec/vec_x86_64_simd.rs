use std::arch::x86_64::*;
use std::ops::{Add, Mul, Sub, Not, Rem};

// layout:
// | 0.0 | x | y | z |
// msb             lsb
#[derive(Copy, Clone, Debug)]
pub struct V(__m128);

macro_rules! _MM_SHUFFLE {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        ($z << 6) | ($y << 4) | ($x << 2) | $w
    };
}

impl V {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> V {
        V(unsafe { _mm_set_ps(0.0, x, y, z) })
    }

    #[inline]
    pub fn x(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE!(0, 0, 0, 2))) }
    }
    #[inline]
    pub fn y(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE!(0, 0, 0, 1))) }
    }
    #[inline]
    pub fn z(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, _MM_SHUFFLE!(0, 0, 0, 0))) }
    }

    pub fn cross<T>(self, rhs: T) -> Self
    where
        V: From<T>,
    {
        let rhs = V::from(rhs);

        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();

        V::new(x, y, z)

        //let res = unsafe {
        //    _mm_permute_ps(
        //        _mm_sub_ps(
        //            _mm_mul_ps(self.0, _mm_permute_ps(rhs.0, _MM_SHUFFLE!(3, 0, 2, 1))),
        //            _mm_mul_ps(rhs.0, _mm_permute_ps(self.0, _MM_SHUFFLE!(3, 0, 2, 1))),
        //        ),
        //        _MM_SHUFFLE!(3, 0, 2, 1),
        //    )
        //};
        //
        //V(res)
    }
}

impl From<f32> for V {
    #[inline]
    fn from(f: f32) -> Self {
        V(unsafe { _mm_set1_ps(f) })
    }
}

impl From<(f32, f32, f32)> for V {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        V::new(x, y, z)
    }
}

impl From<(f32, f32)> for V {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        V::new(x, y, 0.0)
    }
}

impl Default for V {
    fn default() -> V {
        V (unsafe { _mm_setzero_ps() })
    }
}

impl<T> Add<T> for V
where
    V: From<T>,
{
    type Output = V;

    #[inline]
    fn add(self, other: T) -> V {
        let other = V::from(other);
        V(unsafe { _mm_add_ps(self.0, other.0) })
    }
}

impl<T> Sub<T> for V
where
    V: From<T>,
{
    type Output = V;

    #[inline]
    fn sub(self, other: T) -> V {
        let other = V::from(other);
        V(unsafe { _mm_sub_ps(self.0, other.0) })
    }
}

impl<T> Mul<T> for V
where
    V: From<T>,
{
    type Output = V;

    #[inline]
    fn mul(self, other: T) -> V {
        let other = V::from(other);
        V(unsafe { _mm_mul_ps(self.0, other.0) })
    }
}

impl<T> Rem<T> for V
where
    V: From<T>,
{
    type Output = f32;

    #[inline]
    fn rem(self, rhs: T) -> f32 {
        let rhs = V::from(rhs);
        unsafe {
            let res = _mm_dp_ps(self.0, rhs.0, 0x71);
            _mm_cvtss_f32(res)
        }
    }
}

impl Not for V {
    type Output = V;

    #[inline]
    fn not(self) -> V {
        self * V::from((self % self).sqrt().recip())
    }
}
