use turbo::{camera::x, utils::color, *};
use crate::{common::{bounds_get_bottom_center, Anchor, ClientTimer, Renderable, ScaleMode, Sprite, Text}, config, entities::{EntityRenderer, EntityState}, Span, Vec2, JERSEY_20};

// --- UI Transitions ---

pub fn draw_fade(t: f32, span: Span, base_color: u32, from: f32, to: f32, easing: fn(f32) -> f32) {
    let alpha_fraction = span.value(t, from, to, easing);
    let fade_alpha = (alpha_fraction * 255.0) as u32;
    
    let color = (base_color & 0xFFFFFF00) | fade_alpha;
    rect!(
        x = 0, y = 0,
        w = bounds::screen().w(),
        h = bounds::screen().h(),
        color = color,
        fixed = true
    );
}

pub fn draw_fade_to_black(t: f32, span: Span) {
    draw_fade(t, span, 0x000000, 1.0, 0.0, |x| x);
}

// --- UI Buttons ---

pub fn draw_button(bounds: &Bounds, is_selected: bool, recently_pressed: bool,
                   normal_sprite: &Sprite, selected_sprite: &Sprite, color: u32) {
    let sprite = if is_selected { selected_sprite } else { normal_sprite };
    sprite!(sprite.name, bounds = bounds, fixed = true, color = color);

    if is_selected {
        let bg = if recently_pressed { 0x55555555 } else { 0x00000000 };
        rect!(bounds = bounds, color = bg, border_size = 2, fixed = true);
    }
}
