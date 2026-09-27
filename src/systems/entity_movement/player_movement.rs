use turbo::*;

use crate::{config, entities::EntityState, PlayerState, Vec2};

pub fn apply_player_movement_scaled(player: &mut PlayerState, direction: Vec2, time_scale: f32) {    
    let base_acceleration = player.scaled_acceleration();
    let acceleration = base_acceleration * time_scale;
    
    // Apply movement
    let movement_velocity = Vec2 {
        x: direction.x * acceleration,
        y: direction.y * acceleration,
    };
    
    // Update and Clamp Velocity
    let player_data = player.data();

    let new_velocity = player_data.collider.velocity + movement_velocity;
    player.set_velocity(new_velocity);
}

pub fn update_player_physics_scaled(player: &mut PlayerState, map_bounds: &Bounds, time_scale: f32) {    
    let player_data = player.data();

    let player_position = player_data.collider.position;
    let player_velocity = player_data.collider.velocity;

    // Position Update
    let new_position = player_position + (player_velocity * time_scale);
    player.set_position(new_position);

    clamp_player_to_bounds(player, map_bounds);

    // Apply Damping
    let base_damping = player.scaled_damping();
    let damping_factor = base_damping.powf(time_scale);
    let new_velocity = player_velocity * damping_factor;

    // Clamp Velocity
    let clamped_velocity = if new_velocity.length() < config::PLAYER_MIN_VELOCITY_THRESHOLD {
        Vec2::new(0.0, 0.0)
    } else {
        new_velocity
    };
    
    player.set_velocity(clamped_velocity);
}

fn clamp_player_to_bounds(player: &mut PlayerState, map_bounds: &Bounds,) {
    let player_data = player.data();

    let player_radius = player_data.collider.radius;
    let player_position = player_data.collider.position;
    let player_velocity = player_data.collider.velocity;

    // Bounds
    let min_x = map_bounds.left() as f32 + player_radius;
    let max_x = map_bounds.right() as f32 - player_radius;
    let min_y = map_bounds.top() as f32 + player_radius;
    let max_y = map_bounds.bottom() as f32 - player_radius;
    
    // Stop Velocity
    let new_velocity = Vec2 {
        x: if player_position.x < min_x || player_position.x > max_x { 0.0 } else { player_velocity.x },
        y: if player_position.y < min_y || player_position.y > max_y { 0.0 } else { player_velocity.y },
    };
    player.set_velocity(new_velocity);

    // Clamp Position
    let new_position = Vec2::new(
        player_position.x.clamp(min_x, max_x),
        player_position.y.clamp(min_y, max_y),
    );
    player.set_position(new_position);
}
