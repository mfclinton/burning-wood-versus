use turbo::*;

use crate::{common::{bounds_get_bottom_center, bounds_get_top_center, Anchor, Renderable, ScaleMode, Text}, config, entities::EntityState, sprites, EntityRenderer, PlayerState, Sprite, Vec2, JERSEY_20};

use super::health_effects::{check_and_spawn_health_effects, render_health_effects};

// --- Player Renderer ---

struct PlayerSprites {
    sprite: Sprite,
    sprite_scale: f32,
}

impl PlayerSprites {
    fn for_health_ratio(health_ratio: f32) -> Self {
        match health_ratio {
            ratio if ratio > 0.833 => Self {
                sprite: sprites::FIRE_6,
                sprite_scale: 0.08,
            },
            ratio if ratio > 0.666 => Self {
                sprite: sprites::FIRE_5,
                sprite_scale: 0.1,
            },
            ratio if ratio > 0.5 => Self {
                sprite: sprites::FIRE_4,
                sprite_scale: 0.20625,
            },
            ratio if ratio > 0.333 => Self {
                sprite: sprites::FIRE_3,
                sprite_scale: 0.21875,
            },
            ratio if ratio > 0.166 => Self {
                sprite: sprites::FIRE_2,
                sprite_scale: 0.3125,
            },
            _ => Self {
                sprite: sprites::FIRE_1,
                sprite_scale: 0.425,
            },
        }
    }
}

// --- Entity Renderer Implementation ---

impl EntityRenderer for PlayerState {
    // Render
    fn _render(&self) {
        self.draw_player();
        
        self.check_and_spawn_health_effects();
        self.render_health_effects();
    }

    fn _render_debug(&self) {
        self.render_debug_color(0xFF000099);
    }

    // Animation
    fn is_death_animation_complete(&self) -> bool {
        true
    }
}

// --- Helper Methods ---

impl PlayerState {
    fn check_and_spawn_health_effects(&self) {
        let player_data = self.data();
        let player_id = &player_data.id;
        let current_health = player_data.health;
        let player_position = player_data.collider.position;

        check_and_spawn_health_effects(player_id, current_health, player_position);
    }

    fn render_health_effects(&self) {
        let player_data = self.data();
        let player_id = &player_data.id;
        let player_radius = player_data.collider.radius;

        render_health_effects(player_id, player_radius);
    }

    // Drawing
    fn draw_player(&self) {
        let player_data = self.data();
        
        // Get Sprites
        let health_ratio = player_data.health as f32 / config::PLAYER_MAX_HEALTH as f32;
        let sprites = PlayerSprites::for_health_ratio(health_ratio);
        let mut sprite = sprites.sprite * sprites.sprite_scale;
        sprite *= player_data.collider.radius;

        // Render Sprite
        let sprite_bounds = sprite.get_bottom_center_aligned_on_collider(&player_data.collider);
        sprite!(
            sprite.name,
            bounds = sprite_bounds
        );
    }

    pub fn draw_player_username_with_crown(&self, show_crown: bool) {
        let player_data = self.data();
        
        let position = self.get_position();
        let radius = self.get_radius();
        let username = &player_data.username;

        // Username
        let mut username_text = Text::new(username, &JERSEY_20);
        username_text = username_text.with_scale(0.005, ScaleMode::HeightRelative);

        let username_position = Vec2::new(position.x, position.y - radius - 10.0);
        let username_text_bounds = username_text.get_bounds_anchored_at(username_position.into(), Anchor::Center);
        
        username_text.render(username_text_bounds.xy().into(), 0xFFFFFFFF, false);

        // Crown
        if show_crown {
            let player_data = self.data();
            let center = player_data.collider.position;
            let radius = player_data.collider.radius;
            
            let mut crown_sprite = sprites::FIRE_FIRST_PLACE_CROWN;
            crown_sprite *= radius * 0.03; // Same scale as items
            
            let crown_bounds = crown_sprite.get_bounds_anchored_at(bounds_get_top_center(&username_text_bounds).into(), Anchor::BottomCenter);
            sprite!(
                crown_sprite.name,
                bounds = crown_bounds
            );
        }

        // Items
        self.draw_player_item(&username_text_bounds);
    }

    fn draw_player_item(&self, username_text_bounds: &Bounds) {
        let player_data = self.data();
        let items: Vec<_> = player_data.held_items.iter().flatten().collect();

        if items.is_empty() {
            return;
        }

        let radius = player_data.collider.radius;
        let item_scale = radius * 0.025;
        let item_count = items.len() as f32;

        for (i, item_state) in items.iter().enumerate() {
            let mut item_bounds = (item_state.get_sprite() * item_scale).get_bounds_anchored_at(bounds_get_bottom_center(username_text_bounds), Anchor::TopCenter);
            item_bounds = item_bounds.translate_y_by_fraction(0.05);
            
            let offset_fraction = if item_count == 1.0 {
                0.0
            } else {
                let item_index_from_center = i as f32 - (item_count - 1.0) / 2.0;
                item_index_from_center * 1.1
            };
            
            item_bounds = item_bounds.translate_x_by_fraction(offset_fraction);
            item_state.draw_item_with_bounds(item_bounds, false, false);
        }
    }

    // Debug
    pub fn render_debug_color(&self, color: u32) {
        let player_data = self.data();
        player_data.collider.render_circle_collider(color);
    }
}