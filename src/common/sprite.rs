use std::ops::{Mul, MulAssign};

use turbo::*;

use crate::{Vec2, Vec2Int, CircleCollider, Renderable, Anchor, ScaleMode};

// --- Sprite ---

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub name: &'static str,
    pub native_size: Vec2Int,
    pub scale: f32,
    pub scale_mode: ScaleMode,
}

impl Sprite {
    pub const fn new(name: &'static str, width: u32, height: u32) -> Self {
        Self { 
            name, 
            native_size: Vec2Int { x: width as i32, y: height as i32 },
            scale: 1.0,
            scale_mode: ScaleMode::Absolute,
        }
    }
}

// --- Renderable Implementation ---

impl Renderable for Sprite {
    fn native_size(&self) -> Vec2 {
        self.native_size.into()
    }
    
    fn scale(&self) -> f32 {
        self.scale
    }
    
    fn scale_mode(&self) -> ScaleMode {
        self.scale_mode
    }
    
    fn with_scale(self, scale: f32, mode: ScaleMode) -> Self {
        Self {
            scale,
            scale_mode: mode,
            ..self
        }
    }
}

// --- Sprite Multiplication Operators ---

impl Mul<f32> for Sprite {
    type Output = Sprite;
    
    fn mul(self, scale: f32) -> Self::Output {
        self.with_scale(self.scale * scale, self.scale_mode)
    }
}

impl MulAssign<f32> for Sprite {
    fn mul_assign(&mut self, scale: f32) {
        *self = self.with_scale(self.scale * scale, self.scale_mode);
    }
}
