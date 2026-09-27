use turbo::*;

use crate::{entities::EntityState, ProjectileState};

pub fn update_projectile_physics(projectile: &mut ProjectileState) {
    update_projectile_physics_scaled(projectile, 1.0);
}

pub fn update_projectile_physics_scaled(projectile: &mut ProjectileState, time_scale: f32) {
    let projectile_data = projectile.data();

    let projectile_position = projectile_data.collider.position;
    let projectile_velocity = projectile_data.collider.velocity;

    // Update Position with time scaling
    let new_position = projectile_position + (projectile_velocity * time_scale);
    projectile.set_position(new_position);
}
