use turbo::*;
use crate::{Vec2, Sprite, EntityState};

// --- Entity Renderer Trait ---

pub trait EntityRenderer: EntityState {
    // Render
    fn render(&self, debug_mode: bool) {
        self._render();

        if debug_mode {
            self._render_debug();
        }
    }

    fn _render(&self);

    // Animation
    fn is_death_animation_complete(&self) -> bool;

    // Debug
    fn _render_debug(&self) {
        
    }
}
