use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

use turbo::*;

// --- Type Aliases ---

pub type Vec2 = Vector2<f32>;
pub type Vec2Int = Vector2<i32>;

// --- Vec2 ---

#[derive(Default, PartialEq, Copy, PartialOrd)]
#[turbo::serialize]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }
}

// --- Vec2 Operators ---

impl<O> Add for Vector2<O>
where
    O: Copy + Add<Output = O>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<O> Sub for Vector2<O>
where
    O: Copy + Sub<Output = O>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<O> AddAssign for Vector2<O>
where
    O: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<O> SubAssign for Vector2<O>
where
    O: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<O> Mul for Vector2<O>
where
    O: Copy + Mul<Output = O>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self { x: self.x * other.x, y: self.y * other.y }
    }
}

impl<O> Div for Vector2<O>
where
    O: Copy + Div<Output = O>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self { x: self.x / other.x, y: self.y / other.y }
    }
}

impl<C> Mul<C> for Vector2<C>
where
    C: Copy + Mul<Output = C>,
{
    type Output = Self;
    fn mul(self, scalar: C) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

impl<C> Div<C> for Vector2<C>
where
    C: Copy + Div<Output = C>,
{
    type Output = Self;
    fn div(self, c: C) -> Self {
        Self { x: self.x / c, y: self.y / c }
    }
}

impl<C> MulAssign<C> for Vector2<C>
where
    C: Copy + MulAssign,
{
    fn mul_assign(&mut self, c: C) {
        self.x *= c;
        self.y *= c;
    }
}

impl<C> DivAssign<C> for Vector2<C>
where
    C: Copy + DivAssign,
{
    fn div_assign(&mut self, c: C) {
        self.x /= c;
        self.y /= c;
    }
}

// ---- Vec2 Float ----

impl Vec2 {
    pub const fn zero() -> Self { Self { x: 0.0, y: 0.0 } }

    pub fn abs(&self) -> Self {
        Self { x: self.x.abs(), y: self.y.abs() }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self.clone() / len
        } else {
            Self::zero()
        }
    }

    pub fn clamp_normalized(&self) -> Self {
        let len = self.length();
        if len > 1.0 {
            self.normalized()
        } else {
            self.clone()
        }
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        (*other - *self).length()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self { x: crate::lerp(self.x, other.x, t), y: crate::lerp(self.y, other.y, t) }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }

    pub fn random() -> Self {
        Self { x: random::f32(), y: random::f32() }
    }
}

// ---- Vec2 Int ----

impl Vec2Int {
    pub const fn zero() -> Self { Self { x: 0, y: 0 } }                             
}

impl Mul<f32> for Vec2Int {
    type Output = Self;
    
    fn mul(self, scale: f32) -> Self::Output {
        Self {
            x: (self.x as f32 * scale) as i32,
            y: (self.y as f32 * scale) as i32,
        }
    }
}

// ---- Vec Conversions ----

impl<T> From<(T, T)> for Vector2<T> {
    fn from(v: (T, T)) -> Self { Self::new(v.0, v.1) }
}

impl<T> From<Vector2<T>> for (T, T) {
    fn from(v: Vector2<T>) -> Self { (v.x, v.y) }
}

impl From<(u32, u32)> for Vec2 {
    fn from(v: (u32, u32)) -> Self { Self::new(v.0 as f32, v.1 as f32) }
}

impl From<(u32, u32)> for Vec2Int {
    fn from(v: (u32, u32)) -> Self { Self::new(v.0 as i32, v.1 as i32) }
}

impl From<(i32, i32)> for Vec2 {
    fn from(v: (i32, i32)) -> Self { 
        Self::new(v.0 as f32, v.1 as f32) 
    }
}

impl From<(f32, f32)> for Vec2Int {
    fn from(v: (f32, f32)) -> Self { 
        Self::new(v.0 as i32, v.1 as i32) 
    }
}

impl From<Vec2> for Vec2Int {
    fn from(v: Vec2) -> Self { Self::new(v.x as i32, v.y as i32) }
}

impl From<Vec2Int> for Vec2 {
    fn from(v: Vec2Int) -> Self { Self::new(v.x as f32, v.y as f32) }
}
