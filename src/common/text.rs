use std::ops::{Mul, MulAssign};

use turbo::*;

use crate::{common::Font, Anchor, CircleCollider, Renderable, ScaleMode, Vec2, Vec2Int};

// --- Text ---

#[derive(Debug, Clone, Copy)]
pub struct Text<'a> {
    pub content: &'a str,
    pub font: &'static Font,
    pub scale: f32,
    pub scale_mode: ScaleMode,
}

impl<'a> Text<'a> {
    pub const fn new(content: &'a str, font: &'static Font) -> Self {
        Self {
            content,
            font,
            scale: 1.0,
            scale_mode: ScaleMode::Absolute,
        }
    }
    
    // Core
    pub fn render(&self, xy: Vec2, color: u32, fixed: bool) {
        text!(
            self.content,
            xy = xy.into(),
            color = color,
            font = self.font.name,
            scale = self.font_scale(),
            fixed = fixed,
        );
    }

    // Helpers
    pub fn font_scale(&self) -> f32 {
        let scaled_size = self.scaled_size();
        let native_size = self.native_size();
        scaled_size.x / native_size.x
    }
}

// --- Renderable Implementation ---

impl Renderable for Text<'_> {
    fn native_size(&self) -> Vec2 {
        utils::text::measure(self.font.name, 1.0, self.content).into()
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

// --- Text Multiplication Operators ---

impl<'a> Mul<f32> for Text<'a> {
    type Output = Text<'a>;

    fn mul(self, scale: f32) -> Self::Output {
        self.with_scale(self.scale * scale, self.scale_mode)
    }
}

impl<'a> MulAssign<f32> for Text<'a> {
    fn mul_assign(&mut self, scale: f32) {
        *self = self.with_scale(self.scale * scale, self.scale_mode);
    }
}
