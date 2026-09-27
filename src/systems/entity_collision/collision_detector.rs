use std::collections::HashMap;

use crate::{circle_circle_collision, entities::EntityState, Collision, PlayerState, ProjectileState};

// --- Collision Detection ---

pub fn check_player_projectile_collisions(
    players: &HashMap<String, PlayerState>,
    projectiles: &HashMap<String, ProjectileState>,
) -> Vec<Collision> {
    let mut collisions = Vec::new();
    for (player_id, player) in players {
        if player.just_destroyed() { continue; }
        let player_data = player.data();

        for (projectile_id, projectile) in projectiles {
            if projectile.just_destroyed() { continue; }
            let projectile_data = projectile.data();

            if circle_circle_collision(&player_data.collider, &projectile_data.collider) {
                collisions.push(Collision::PlayerToProjectile {
                    player_id: player_id.clone(),
                    projectile_id: projectile_id.clone(),
                });
            }
        }
    }
    
    collisions
}

pub fn check_player_player_collisions(
    players: &HashMap<String, PlayerState>
) -> Vec<Collision> {
    let mut collisions = Vec::new();

    for (i, (player1_id, player1)) in players.iter().enumerate() {
        if player1.just_destroyed() { continue; }
        let player1_data = player1.data();

        for (player2_id, player2) in players.iter().skip(i + 1) {
            if player2.just_destroyed() { continue; }
            let player2_data = player2.data();

            if circle_circle_collision(&player1_data.collider, &player2_data.collider) {
                collisions.push(Collision::PlayerToPlayer {
                    player1_id: player1_id.clone(),
                    player2_id: player2_id.clone(),
                });
            }
        }
    }
    
    collisions
}
