use std::{fmt::Display, hash::Hash, ops::{Add, Mul}};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output=T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Copy> Vec2<T> where T: Mul<Output = T> {
    pub fn s_mul(&self, s: T) -> Vec2<T> {
        Vec2 { 
            x: self.x*s,
            y: self.y*s
        }
    }
}


impl Vec2<usize> {
    pub fn zero() -> Self {
        Vec2 {
            x: 0,
            y: 0
        }
    }
}

