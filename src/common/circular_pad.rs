use turbo::*;
use crate::{common::{draw, Anchor, Renderable, Vec2Int}, sprites, Sprite, Vec2};

#[derive(Default)]
#[turbo::serialize]
pub struct CircularPad {
    pub center: Vec2,
}

impl CircularPad {
    pub fn update(&mut self) {
        let pointer = pointer::screen();
        
        // Update Center
        if pointer.just_pressed() {
            self.center = Vec2::from(pointer.xy());
        }
    }

    // Rendering
    pub fn render(&self) {
        let pointer = pointer::screen();
        if !pointer.pressed() {
            return;
        }

        // Sprites
        let base_sprite = self.get_base_sprite();
        let knob_sprite = self.get_knob_sprite();

        // Draw Base
        let center_bounds = base_sprite.get_bounds_anchored_at(self.center.into(), Anchor::Center);
        sprite!(
            base_sprite.name,
            bounds = center_bounds,
            fixed = true,
            color = 0xFFFFFF77
        );

        // Draw Knob
        let knob_pos = self.center + self.get_knob_delta();
        let knob_bounds = knob_sprite.get_bounds_anchored_at(knob_pos.into(), Anchor::Center);
        sprite!(
            knob_sprite.name,
            bounds = knob_bounds,
            fixed = true,
            color = 0xFFFFFFCC
        );
    }

    // Public Functions
    pub fn get_movement_vector(&self) -> Vec2 {
        let pointer = pointer::screen();
        if !pointer.pressed() {
            return Vec2::zero();
        }

        let base_radius = self.get_base_radius();
        let knob_delta = self.get_knob_delta();
        (knob_delta / base_radius).clamp_normalized()
    }


    // Sprite Getters
    fn get_base_sprite(&self) -> Sprite {
        sprites::UI_CIRCULAR_PAD_BASE
    }

    fn get_knob_sprite(&self) -> Sprite {
        sprites::UI_CIRCULAR_PAD_KNOB
    }

    // Helpers
    fn get_knob_delta(&self) -> Vec2 {
        let pointer = pointer::screen();
        if !pointer.pressed() {
            return Vec2::zero();
        }

        let base_radius = self.get_base_radius();
        let mut knob_delta = Vec2::from(pointer.xy()) - self.center;
        if knob_delta.length() > base_radius {
            knob_delta = knob_delta.normalized() * base_radius;
        }

        knob_delta
    }

    fn get_base_radius(&self) -> f32 {
        (self.get_base_sprite().scaled_size_int().x as f32 / 2.0) * 0.8
    }
}