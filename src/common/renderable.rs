use std::ops::{Mul, MulAssign};

use turbo::*;

use crate::{Vec2, Vec2Int, CircleCollider};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleMode {
    Absolute,
    WidthRelative,
    HeightRelative,
    Fit,
    Fill,
    WidthRelativeContain,
    HeightRelativeContain,
}

pub trait Renderable: Sized {
    // Abstract Functions
    fn native_size(&self) -> Vec2;
    fn scale(&self) -> f32;
    fn scale_mode(&self) -> ScaleMode;
    fn with_scale(self, scale: f32, mode: ScaleMode) -> Self;
    
    // Scaling
    fn scaled_size(&self) -> Vec2 {
        let canvas_bounds = bounds::screen();
        let screen_size = Vec2::new(canvas_bounds.w() as f32, canvas_bounds.h() as f32);
        let native_size = self.native_size();

        let scaled_size = match self.scale_mode() {
            ScaleMode::Absolute => native_size * self.scale(),
            ScaleMode::WidthRelative => {
                let scaled_width = screen_size.x * self.scale();
                let aspect_ratio = native_size.y / native_size.x;
                Vec2::new(scaled_width, scaled_width * aspect_ratio)
            }
            ScaleMode::HeightRelative => {
                let scaled_height = screen_size.y * self.scale();
                let aspect_ratio = native_size.x / native_size.y;
                Vec2::new(scaled_height * aspect_ratio, scaled_height)
            }
            ScaleMode::WidthRelativeContain => {
                let width_scaled = screen_size.x * self.scale();
                let height_if_width_scaled = width_scaled * (native_size.y / native_size.x);
                
                let scale_factor = if height_if_width_scaled > screen_size.y {
                    screen_size.y / height_if_width_scaled
                } else {
                    1.0
                };
                
                Vec2::new(width_scaled, height_if_width_scaled) * scale_factor
            }
            ScaleMode::HeightRelativeContain => {
                let height_scaled = screen_size.y * self.scale();
                let width_if_height_scaled = height_scaled * (native_size.x / native_size.y);
                
                let scale_factor = if width_if_height_scaled > screen_size.x {
                    screen_size.x / width_if_height_scaled
                } else {
                    1.0
                };
                
                Vec2::new(width_if_height_scaled, height_scaled) * scale_factor
            }
            ScaleMode::Fit => {
                let scale_factors = screen_size / native_size;
                let fit_scale = scale_factors.x.min(scale_factors.y) * self.scale();
                native_size * fit_scale
            }
            ScaleMode::Fill => {
                let scale_factors = screen_size / native_size;
                let fill_scale = scale_factors.x.max(scale_factors.y) * self.scale();
                native_size * fill_scale
            }
        };

        scaled_size
    }

    fn scaled_size_int(&self) -> Vec2Int {
        self.scaled_size().into()
    }

    // Anchoring
    fn get_anchor_offset(&self, anchor: Anchor) -> Vec2Int {
        let size = self.scaled_size_int();
        match anchor {
            Anchor::TopLeft => Vec2Int::new(0, 0),
            Anchor::TopCenter => Vec2Int::new(-size.x / 2, 0),
            Anchor::TopRight => Vec2Int::new(-size.x, 0),
            Anchor::CenterLeft => Vec2Int::new(0, -size.y / 2),
            Anchor::Center => Vec2Int::new(-size.x / 2, -size.y / 2),
            Anchor::CenterRight => Vec2Int::new(-size.x, -size.y / 2),
            Anchor::BottomLeft => Vec2Int::new(0, -size.y),
            Anchor::BottomCenter => Vec2Int::new(-size.x / 2, -size.y),
            Anchor::BottomRight => Vec2Int::new(-size.x, -size.y),
        }
    }

    // Bounds Functions
    fn get_bounds(&self) -> Bounds {
        let scaled_size = self.scaled_size_int();
        Bounds::with_size(scaled_size.x as u32, scaled_size.y as u32)
    }

    fn get_bounds_anchored_at(&self, position: Vec2Int, anchor: Anchor) -> Bounds {
        let scaled_size = self.scaled_size_int();
        let offset = self.get_anchor_offset(anchor);
        let final_position = position + offset;
        Bounds::new(final_position.x, final_position.y, scaled_size.x as u32, scaled_size.y as u32)
    }

    // Collider Functions
    fn get_centered_on_collider(&self, collider: &CircleCollider) -> Bounds {
        self.get_bounds_anchored_at(collider.position.into(), Anchor::Center)
    }

    fn get_bottom_center_aligned_on_collider(&self, collider: &CircleCollider) -> Bounds {
        let bottom_center: Vec2Int = (collider.position + Vec2::new(0.0, collider.radius)).into();
        self.get_bounds_anchored_at(bottom_center, Anchor::BottomCenter)
    }
}
