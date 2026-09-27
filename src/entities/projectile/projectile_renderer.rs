use turbo::*;

use crate::{common::{Renderable, Vec2, Vec2Int}, entities::{projectile, EntityDirtyState, EntityState}, sprites, EntityRenderer, ProjectileState, ProjectileType, Sprite};

// --- Projectile Renderer ---

struct ProjectileSprites {
    idle: Sprite,
    summon: Sprite,
    destroy: Sprite,
    sprite_scale: f32,
    offset_p: Vec2,
}

impl ProjectileSprites {
    fn for_type(projectile_type: ProjectileType) -> Self {
        match projectile_type {
            ProjectileType::Wood => Self {
                idle: sprites::PROJECTILES_WOOD_IDLE,
                summon: sprites::PROJECTILES_WOOD_SUMMON,
                destroy: sprites::PROJECTILES_WOOD_DESTROY,
                sprite_scale: 0.18,
                offset_p: Vec2::new(0.0, 0.0),
            },
            ProjectileType::Water => Self {
                idle: sprites::PROJECTILES_WATER_IDLE,
                summon: sprites::PROJECTILES_WATER_SUMMON,
                destroy: sprites::PROJECTILES_WATER_DESTROY,
                sprite_scale: 0.2,
                offset_p: Vec2::new(0.0, -0.1),
            },
            ProjectileType::Coal => Self {
                idle: sprites::PROJECTILES_COAL_IDLE,
                summon: sprites::PROJECTILES_COAL_SUMMON,
                destroy: sprites::PROJECTILES_COAL_DESTROY,
                sprite_scale: 0.08,
                offset_p: Vec2::new(0.0, 0.0),
            },
            ProjectileType::Chest => Self {
                idle: sprites::PROJECTILES_CHEST_IDLE,
                summon: sprites::PROJECTILES_CHEST_SUMMON,
                destroy: sprites::PROJECTILES_CHEST_DESTROY,
                sprite_scale: 0.1,
                offset_p: Vec2::new(0.0, 0.0),
            },
        }
    }
}

// --- Entity Renderer Implementation ---

impl EntityRenderer for ProjectileState {
    fn _render(&self) {
        let projectile_data = self.data();
        let projectile_dirty = self.dirty();
        let projectile_id = self.get_id();
        
        // Get Sprites
        let sprites = ProjectileSprites::for_type(projectile_data.projectile_type);
        let mut sprite = sprites.idle * sprites.sprite_scale;
        sprite *= projectile_data.collider.radius;

        // Animations
        let anim = animation::get(projectile_id.as_str());
        if projectile_dirty.just_created() {
            anim.use_sprite(sprites.summon.name);
            anim.set_repeat(1);
        }
        if projectile_dirty.just_destroyed() {
            anim.use_sprite(sprites.destroy.name);
            anim.set_speed(3.0);
            anim.set_repeat(1);
            anim.set_fill_forwards(true);
        }

        // Render Sprite
        let mut sprite_bounds = sprite.get_centered_on_collider(&projectile_data.collider);

        let offset: Vec2Int = (sprites.offset_p * Vec2::from(sprite.scaled_size_int())).into();
        sprite_bounds = sprite_bounds.translate(offset.x, offset.y);
        
        sprite!(
            animation_key = projectile_id.as_str(),
            default_sprite = sprite.name,
            bounds = sprite_bounds
        );
    }

    // Animation
    fn is_death_animation_complete(&self) -> bool {
        let projectile_data = self.data();
        let sprites = ProjectileSprites::for_type(projectile_data.projectile_type);
        
        let projectile_id = self.get_id();
        let anim = animation::get(projectile_id.as_str());

        let is_anim_done = anim.done();
        let is_destroy_sprite = anim.sprite_name() == sprites.destroy.name;
        is_anim_done && is_destroy_sprite
    }

    // Debug
    fn _render_debug(&self) {
        let debug_color = Self::get_debug_color(self.data().projectile_type);
        self.render_debug_color(debug_color);
    }
}

// --- Helper Methods ---

impl ProjectileState {
    fn get_debug_color(projectile_type: ProjectileType) -> u32 {
        match projectile_type {
            ProjectileType::Wood => 0x8B451399,  // Brown
            ProjectileType::Water => 0x0077BE99, // Blue
            ProjectileType::Coal => 0x36454F99,  // Dark gray  
            ProjectileType::Chest => 0xFFD70099, // Gold
        }
    }

    pub fn render_debug_color(&self, color: u32) {
        let projectile_data = self.data();
        projectile_data.collider.render_circle_collider(color);
    }
}