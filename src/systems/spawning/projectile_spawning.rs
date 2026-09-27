use turbo::*;

use crate::{config, create_uuid, entities::EntityState, sample_inside_bounds, sample_outside_bounds, CircleCollider, ProjectileState, ProjectileType, Vec2};

const OUTSIDE_MARGIN: f32 = 100.0;

const PROJECTILE_MIN_SPEED: f32 = 1.0;
const PROJECTILE_MAX_SPEED: f32 = 4.0;
const PROJECTILE_MAX_LIFETIME: u64 = 600;

const PROJECTILE_CLEANUP_GRACE_PERIOD_TICKS: u64 = (5000 / config::SERVER_INTERVAL_RATE_MS) as u64;

// --- Projectile Spawning ---

pub fn spawn_random_projectile(map_bounds: &Bounds, current_tick: u64) -> ProjectileState {
    // Positions
    let inside_pos = sample_inside_bounds(map_bounds);
    let outside_pos = sample_outside_bounds(map_bounds, OUTSIDE_MARGIN);

    // Velocity
    let dir = (inside_pos - outside_pos).normalized();
    let speed = random::between(PROJECTILE_MIN_SPEED, PROJECTILE_MAX_SPEED);
    
    let velocity = dir * speed;

    // Random Type
    let projectile_type = ProjectileType::random();

    // Random Payload
    let payload = projectile_type.random_payload();

    // Lifetime
    let lifetime_ticks = PROJECTILE_MAX_LIFETIME;

    let mut new_projectile = ProjectileState::new(create_uuid(current_tick), outside_pos, projectile_type, payload, current_tick, lifetime_ticks);
    new_projectile.set_velocity(velocity);

    new_projectile
}

// --- Helpers ---

pub fn is_projectile_expired(projectile: &ProjectileState, current_tick: u64) -> bool {
    let projectile_data = projectile.data();
    current_tick >= projectile_data.spawn_tick + projectile_data.lifetime_ticks
}

pub fn is_projectile_out_of_bounds_and_moving_away(projectile: &ProjectileState, map_bounds: &Bounds) -> bool {
    let position = projectile.get_position();
    let velocity = projectile.get_velocity();
    
    let is_out_of_bounds = !map_bounds.intersects_position(position.x, position.y);
    if !is_out_of_bounds {
        return false;
    }
    
    let map_center: Vec2 = map_bounds.center().into();
    let direction_to_center = (map_center - position).normalized();
    let velocity_normalized = velocity.normalized();
    
    let dot_product = direction_to_center.dot(velocity_normalized);
    dot_product < 0.0
}

pub fn check_cull_projectile_early(projectile: &mut ProjectileState, map_bounds: &Bounds) {
    if is_projectile_out_of_bounds_and_moving_away(projectile, map_bounds) {
        let projectile_data = projectile.data_mut();
        projectile_data.lifetime_ticks = projectile_data.lifetime_ticks.min(PROJECTILE_CLEANUP_GRACE_PERIOD_TICKS);
    }
}
