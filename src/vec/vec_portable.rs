use std::ops::{Add, Mul, Not, Rem, Sub};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct V([f32; 4]);

impl V {
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> V {
        V([x, y, z, 0.0])
    }

    #[inline]
    pub const fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub const fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub const fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn cross<T>(self, rhs: T) -> Self
    where
        V: From<T>,
    {
        let rhs = V::from(rhs);

        V([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
            0.0,
        ])
    }
}

impl From<f32> for V {
    #[inline]
    fn from(f: f32) -> Self {
        V([f, f, f, 0.0])
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
        V([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
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
        V([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
        ])
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
        V([
            self.0[0] * other.0[0],
            self.0[1] * other.0[1],
            self.0[2] * other.0[2],
            self.0[3] * other.0[3],
        ])
    }
}

impl<T> Rem<T> for V
where
    V: From<T>,
{
    type Output = f32;

    #[inline]
    fn rem(self, other: T) -> f32 {
        let m = self * other;

        m.0[0] + m.0[1] + m.0[2]
    }
}

impl Not for V {
    type Output = V;

    #[inline]
    fn not(self) -> V {
        self * V::from(1.0 / (self % self).sqrt())
    }
}
