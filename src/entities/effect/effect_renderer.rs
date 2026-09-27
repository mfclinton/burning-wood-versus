use turbo::*;

use crate::{common::Renderable, sprites, EffectState, EffectType, EntityRenderer, EntityState, Sprite};

// --- Effect Renderer ---

struct EffectSprites {
    sprite: Sprite,
    sprite_scale: f32,
}

impl EffectSprites {
    fn for_type(effect_type: &EffectType) -> Self {
        match effect_type {
            EffectType::DamageField { .. } => Self {
                sprite: sprites::EFFECTS_GOLDENIDOL_ACTIVE,
                sprite_scale: 0.04,
            },
            EffectType::HealthModifier { .. } => Self {
                sprite: sprites::EFFECTS_GASCAN_USE,
                sprite_scale: 0.04,
            },
            EffectType::ProjectileSpawner { .. } => Self {
                sprite: sprites::EFFECTS_PROJECTILE_SPAWNER,
                sprite_scale: 0.04,
            },
            EffectType::MagnetField { .. } => Self {
                sprite: sprites::EFFECTS_MAGNET_FIELD,
                sprite_scale: 0.0325,
            },
            EffectType::HealthField { .. } => Self {
                sprite: sprites::EFFECTS_CAMP_FIRE,
                sprite_scale: 0.04,
            },
            EffectType::LifeDrainField { .. } => Self {
                sprite: sprites::EFFECTS_LIFE_DRAIN_FIELD,
                sprite_scale: 0.04,
            },
            EffectType::DelayedMultiProjectileSpawner { .. } => Self {
                sprite: sprites::EFFECTS_WATERBOMB_USE,
                sprite_scale: 0.04,
            },
        }
    }
}

// --- Entity Renderer Implementation ---

impl EntityRenderer for EffectState {
    // Render
    fn _render(&self) {
        let effect_data = self.data();
        
        // Get Sprites
        let sprites = EffectSprites::for_type(&effect_data.effect_type);
        let mut sprite = sprites.sprite * sprites.sprite_scale;
        sprite *= effect_data.collider.radius;

        // Temp
        self._render_debug();

        // Render Sprite
        let sprite_bounds = sprite.get_centered_on_collider(&effect_data.collider);
        sprite!(
            sprite.name,
            bounds = sprite_bounds
        );
    }

    // Animation
    fn is_death_animation_complete(&self) -> bool {
        true
    }

    // Debug
    fn _render_debug(&self) {
        let effect_data = self.data();
        let debug_color = Self::get_debug_color(&effect_data.effect_type);
        effect_data.collider.render_circle_collider(debug_color);
    }
}

// --- Helper Methods ---

impl EffectState {
    fn get_debug_color(effect_type: &EffectType) -> u32 {
        match effect_type {
            EffectType::DamageField { .. } => 0xFF000099, // Red
            EffectType::HealthModifier { .. } => 0x00FF0099, // Green
            EffectType::ProjectileSpawner { .. } => 0xFFFF0099, // Yellow
            EffectType::MagnetField { .. } => 0x0000FF99, // Blue
            EffectType::HealthField { .. } => 0xFF00FF99, // Magenta
            EffectType::LifeDrainField { .. } => 0x800080FF, // Purple
            EffectType::DelayedMultiProjectileSpawner { .. } => 0x00FFFF99, // Cyan
        }
    }
}