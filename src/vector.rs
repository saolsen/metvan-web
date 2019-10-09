#[derive(Debug, Clone, Copy)]
pub struct V2 {
    pub x: f32,
    pub y: f32,
}

impl V2 {
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn mag(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn mag2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&self) -> V2 {
        let mag = self.mag();
        if mag != 0.0 {
            V2 {
                x: self.x / mag,
                y: self.y / mag,
            }
        } else {
            self.clone()
        }
    }
}

use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

impl Add for V2 {
    type Output = V2;

    fn add(self, other: V2) -> V2 {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for V2 {
    type Output = V2;

    fn sub(self, other: V2) -> V2 {
        V2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for V2 {
    type Output = V2;

    fn mul(self, other: f32) -> V2 {
        V2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Mul<V2> for f32 {
    type Output = V2;

    fn mul(self, other: V2) -> V2 {
        V2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl MulAssign<f32> for V2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

impl Div<f32> for V2 {
    type Output = V2;

    fn div(self, other: f32) -> V2 {
        V2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign<f32> for V2 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
    }
}

pub const fn v2(x: f32, y: f32) -> V2 {
    V2 { x, y }
}
