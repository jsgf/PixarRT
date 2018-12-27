use crate::float::Float;
use std::ops::{Add, Mul, Not, Rem};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct V {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl From<Float> for V {
    #[inline]
    fn from(f: Float) -> Self {
        V { x: f, y: f, z: f }
    }
}

impl From<f32> for V {
    #[inline]
    fn from(f: f32) -> Self {
        let f = Float::from(f);
        V { x: f, y: f, z: f }
    }
}

impl From<(Float, Float, Float)> for V {
    #[inline]
    fn from((x, y, z): (Float, Float, Float)) -> Self {
        V { x, y, z }
    }
}

impl From<(f32, f32, f32)> for V {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        V::from((Float::from(x), Float::from(y), Float::from(z)))
    }
}

impl From<(Float, Float)> for V {
    #[inline]
    fn from((x, y): (Float, Float)) -> Self {
        V {
            x,
            y,
            z: Float::ZERO,
        }
    }
}

impl From<(f32, f32)> for V {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        V::from((Float::from(x), Float::from(y), Float::ZERO))
    }
}

impl Default for V {
    fn default() -> V {
        V::from(0.)
    }
}

impl<T> Add<T> for V where V: From<T> {
    type Output = V;

    #[inline]
    fn add(self, other: T) -> V {
        let other = V::from(other);
        V {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Mul<T> for V where V: From<T> {
    type Output = V;

    #[inline]
    fn mul(self, other: T) -> V {
        let other = V::from(other);
        V {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T> Rem<T> for V where V: From<T>{
    type Output = Float;

    #[inline]
    fn rem(self, other: T) -> Float {
        let other = V::from(other);
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Not for V {
    type Output = V;

    #[inline]
    fn not(self) -> V {
        self * V::from(Float::ONE / (self % self).sqrt())
    }
}
