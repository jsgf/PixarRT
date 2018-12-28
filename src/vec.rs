use std::ops::{Add, AddAssign, Mul, MulAssign, Not, Rem};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct V {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V {
    pub const fn new(x: f32, y: f32, z: f32) -> V {
        V { x, y, z }
    }
}

impl From<f32> for V {
    #[inline]
    fn from(f: f32) -> Self {
        V { x: f, y: f, z: f }
    }
}

impl From<(f32, f32, f32)> for V {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        V { x, y, z }
    }
}

impl From<(f32, f32)> for V {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        V { x, y, z: 0.0 }
    }
}

impl Default for V {
    fn default() -> V {
        V::from(0.)
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
        V {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign<T> for V
where
    V: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = self.add(rhs);
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
        V {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
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

impl<T> Rem<T> for V
where
    V: From<T>,
{
    type Output = f32;

    #[inline]
    fn rem(self, other: T) -> f32 {
        let other = V::from(other);
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Not for V {
    type Output = V;

    #[inline]
    fn not(self) -> V {
        self * V::from(1.0 / (self % self).sqrt())
    }
}
