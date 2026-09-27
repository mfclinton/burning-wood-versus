use turbo::*;

use crate::{common::Vec2Int, Vec2};

// --- Anchor Bounds ---

pub fn bounds_get_top_center(bounds: &Bounds) -> Vec2Int {
    Vec2Int::new(bounds.center_x(), bounds.top())
}

pub fn bounds_get_bottom_center(bounds: &Bounds) -> Vec2Int {
    Vec2Int::new(bounds.center_x(), bounds.bottom())
}

pub fn bounds_get_left_center(bounds: &Bounds) -> Vec2Int {
    Vec2Int::new(bounds.left(), bounds.center_y())
}

pub fn bounds_get_right_center(bounds: &Bounds) -> Vec2Int {
    Vec2Int::new(bounds.right(), bounds.center_y())
}

// --- Sampling Functions ---

pub fn sample_inside_bounds(bounds: &Bounds) -> Vec2 {
    Vec2::new(
        random::between(bounds.left() as f32, bounds.right() as f32),
        random::between(bounds.top() as f32, bounds.bottom() as f32),
    )
}

pub fn sample_outside_bounds(bounds: &Bounds, margin: f32) -> Vec2 {
    let side = random::between(0, 3);
    let (x, y) = match side {
        0 => (
            // Top
            random::between(bounds.left() as f32 - margin, bounds.right() as f32 + margin),
            bounds.top() as f32 - random::between(0.0, margin)
        ),
        1 => (
            // Bottom
            random::between(bounds.left() as f32 - margin, bounds.right() as f32 + margin),
            bounds.bottom() as f32 + random::between(0.0, margin)
        ),
        2 => (
            // Left
            bounds.left() as f32 - random::between(0.0, margin),
            random::between(bounds.top() as f32 - margin, bounds.bottom() as f32 + margin)
        ),
        _ => (
            // Right
            bounds.right() as f32 + random::between(0.0, margin),
            random::between(bounds.top() as f32 - margin, bounds.bottom() as f32 + margin)
        ),
    };
    Vec2::new(x, y)
}

// --- Bounds Positioning Functions ---

pub fn screen_relative_position(relative_pos: Vec2) -> Vec2Int {
    relative_position_in_bounds(&bounds::screen(), relative_pos)
}

pub fn relative_position_in_bounds(bounds: &Bounds, relative_pos: Vec2) -> Vec2Int {
    let rel_x = (bounds.w() as f32 * relative_pos.x) as i32;
    let rel_y = (bounds.h() as f32 * relative_pos.y) as i32;
    Vec2Int::new(bounds.x() as i32 + rel_x, bounds.y() as i32 + rel_y)
}

// --- Bounds Scaling Functions ---

pub fn inset_bounds_by_fraction(bounds: &Bounds, fraction: f32) -> Bounds {
    let width_inset = bounds.w() as f32 * fraction;
    let height_inset = bounds.h() as f32 * fraction;

    Bounds::new(
        bounds.x() as f32 + width_inset,
        bounds.y() as f32 + height_inset,
        bounds.w() as f32 - (width_inset * 2.0),
        bounds.h() as f32 - (height_inset * 2.0),
    )
}

pub fn inset_bounds_width_by_fraction(bounds: &Bounds, fraction: f32) -> Bounds {
    let inset_amount = bounds.w() as f32 * fraction;
    
    Bounds::new(
        bounds.x() as f32 + inset_amount,
        bounds.y(),
        bounds.w() as f32 - (inset_amount * 2.0),
        bounds.h(),
    )
}

pub fn inset_bounds_height_by_fraction(bounds: &Bounds, fraction: f32) -> Bounds {
    let inset_amount = bounds.h() as f32 * fraction;

    Bounds::new(
        bounds.x(),
        bounds.y() as f32 + inset_amount,
        bounds.w(),
        bounds.h() as f32 - (inset_amount * 2.0),
    )
}

pub fn scale_bounds_from_center(bounds: &Bounds, scale: f32) -> Bounds {
    let center: Vec2 = bounds.center().into();
    let new_dims: Vec2 = Vec2::from(bounds.wh()) * scale;

    let new_position = center - new_dims / 2.0;
    Bounds::new(
        new_position.x,
        new_position.y,
        new_dims.x,
        new_dims.y,
    )
}
