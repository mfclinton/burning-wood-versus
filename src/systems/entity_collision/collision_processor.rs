use std::collections::HashMap;

use crate::{entities::EntityState, lerp, CircleCollider, Collision, GameState, Vec2};

const MIN_PUSH_FORCE: f32 = 0.1;
const MAX_PUSH_FORCE: f32 = 5.0;

// --- Collision Processing ---

pub fn process_collisions(
    game_state: &mut GameState,
    collisions: Vec<Collision>,
) {
    for collision in collisions {
        match collision {
            Collision::PlayerToProjectile { player_id, projectile_id } => {
                if !game_state.players.contains_key(&player_id) || !game_state.projectiles.contains_key(&projectile_id) {
                    continue;
                }

                process_player_projectile_collision(game_state, player_id, projectile_id);
            }
            Collision::PlayerToPlayer { player1_id, player2_id } => {
                if !game_state.players.contains_key(&player1_id) || !game_state.players.contains_key(&player2_id) {
                    continue;
                }

                apply_player_push_forces(game_state, player1_id, player2_id);
            }
        }
    }
}

// --- Projectile Collision Helpers ---

pub fn process_player_projectile_collision(game_state: &mut GameState, player_id: String, projectile_id: String) -> Option<()> {
    // Get Player and Projectile
    let player = game_state.players.get_mut(&player_id)?;
    let projectile = game_state.projectiles.get_mut(&projectile_id)?;

    let projectile_data = projectile.data();

    // Apply Projectile Payload
    if !projectile_data.payload.apply_to_player(player) {
        return None;
    }

    // Remove Projectile
    game_state.remove_projectile(&projectile_id);

    Some(())
}

// --- Push Helpers ---

fn apply_player_push_forces(game_state: &mut GameState, player1_id: String, player2_id: String) {
    // Get Players
    let player1 = game_state.players.get(&player1_id).unwrap();
    let player2 = game_state.players.get(&player2_id).unwrap();

    let player1_data = player1.data();
    let player2_data = player2.data();

    // Calculate Push Forces
    let push_on_player1 = calculate_collider_push(&player1_data.collider, &player2_data.collider);
    let push_on_player2 = calculate_collider_push(&player2_data.collider, &player1_data.collider);

    // Apply Push Forces
    game_state.players.get_mut(&player1_id).unwrap().modify_velocity(push_on_player1);
    game_state.players.get_mut(&player2_id).unwrap().modify_velocity(push_on_player2);
}

fn calculate_collider_push(collider1: &CircleCollider, collider2: &CircleCollider) -> Vec2 {
    let total_radius = collider1.radius + collider2.radius;
    let distance = collider1.position.distance_to(&collider2.position);

    let overlap_percentage = (total_radius - distance).max(0.0) / total_radius;
    if overlap_percentage <= 0.0 {
        return Vec2::zero();
    }
        
    let direction = (collider1.position - collider2.position).normalized();
    let base_push_force = lerp(MIN_PUSH_FORCE, MAX_PUSH_FORCE, overlap_percentage);
    let radius_ratio = collider2.radius / (collider1.radius + collider2.radius);
    
    let scaled_push_force = base_push_force * radius_ratio;
    direction * scaled_push_force
}
