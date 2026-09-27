use turbo::*;

use crate::{
    common::{Renderable, Vec2}, sprites, Anchor, ScaleMode
};

pub fn draw_title_background() {
    draw_base_background();
    draw_background_elements();
}

// --- Drawing Functions ---

fn draw_base_background() {
    let bg_bounds = get_bg_bounds();
    sprite!(sprites::UI_TITLESCREEN_BG.name, bounds = bg_bounds, fixed = true);
}

fn draw_background_elements() {
    let bg_bounds = get_bg_bounds();

    let bg_origin = Vec2::from(bg_bounds.xy());
    let bg_dims = Vec2::from(bg_bounds.wh());
    let bg_scale = bg_dims.x / sprites::UI_TITLESCREEN_BG.native_size().x;

    // Flames
    let flame_sprite = sprites::UI_TITLESCREEN_FLAME.with_scale(bg_scale, ScaleMode::Absolute);

    let flame1_pos = bg_origin + bg_dims * Vec2::new(0.134, 0.15);
    let flame1_bounds = flame_sprite.get_bounds_anchored_at(flame1_pos.into(), Anchor::Center);
    sprite!(sprites::UI_TITLESCREEN_FLAME.name, bounds = flame1_bounds, fixed = true);

    let flame2_pos = bg_origin + bg_dims * Vec2::new(0.866, 0.15);
    let flame2_bounds = flame_sprite.get_bounds_anchored_at(flame2_pos.into(), Anchor::Center);
    sprite!(sprites::UI_TITLESCREEN_FLAME.name, bounds = flame2_bounds, fixed = true);
    
    // Eyes
    let eyes_sprite = sprites::UI_TITLESCREEN_EYES.with_scale(bg_scale, ScaleMode::Absolute);
    let eyes_pos = bg_origin + bg_dims * Vec2::new(0.298, 0.92);
    let eyes_bounds = eyes_sprite.get_bounds_anchored_at(eyes_pos.into(), Anchor::Center);
    sprite!(sprites::UI_TITLESCREEN_EYES.name, bounds = eyes_bounds, fixed = true);
    
    // Lights
    let lights_sprite = sprites::UI_TITLESCREEN_LIGHTS.with_scale(bg_scale, ScaleMode::Absolute);
    let lights_pos = bg_origin + bg_dims * Vec2::new(0.5, 0.5);
    let lights_bounds = lights_sprite.get_bounds_anchored_at(lights_pos.into(), Anchor::Center);
    sprite!(sprites::UI_TITLESCREEN_LIGHTS.name, bounds = lights_bounds, fixed = true);
}

// --- Helpers ---

fn get_bg_bounds() -> Bounds {
    let bg_sprite = sprites::UI_TITLESCREEN_BG.with_scale(1.0, ScaleMode::Fill);
    bg_sprite.get_bounds_anchored_at(bounds::screen().center().into(), Anchor::Center)
}