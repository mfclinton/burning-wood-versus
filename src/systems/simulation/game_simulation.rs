use std::collections::HashMap;

use turbo::*;

use crate::{
    apply_effects, check_cull_projectile_early, check_player_player_collisions, check_player_projectile_collisions, config, is_projectile_expired, process_collisions, spawn_effect_from_event, spawn_random_projectile, systems::update_player_physics_scaled, update_projectile_physics, GameState
};

// --- Simulation Update Function ---

// TODO: improve
pub fn update_game_simulation(game_state: &mut GameState, tick: u64, time_scale: f32) {
    // Update Players
    for (_, player) in game_state.players.iter_mut() {
        // Movement
        update_player_physics_scaled(player, &game_state.map_bounds, time_scale);

        // Item State
        player.update_item_states();
    }

    // Spawn Effect from Event
    if tick % 5000 == 0 {
        let effect = spawn_effect_from_event(tick, &game_state.map_bounds);
        game_state.add_effect(effect);
    }

    // Update Effects
    apply_effects(game_state, tick);

    // Update Projectiles
    for (_, projectile) in game_state.projectiles.iter_mut() {
        update_projectile_physics(projectile);
    }
    
    // Spawn Projectile
    if tick % 4 == 0 {
        let projectile_state = spawn_random_projectile(&game_state.map_bounds, tick);
        game_state.add_projectile(projectile_state);
    }

    // Remove Expired and Out-of-Bounds Projectiles
    let mut projectiles_to_remove = Vec::new();
    for (projectile_id, projectile) in &mut game_state.projectiles {
        check_cull_projectile_early(projectile, &game_state.map_bounds);
        if is_projectile_expired(projectile, tick) {
            projectiles_to_remove.push(projectile_id.clone());
        }
    }
    
    for projectile_id in projectiles_to_remove {
        game_state.remove_projectile(&projectile_id);
    }
    
    // Check Collisions
    let mut collisions = check_player_projectile_collisions(&mut game_state.players, &game_state.projectiles);
    collisions.extend(check_player_player_collisions(&game_state.players));

    process_collisions(game_state, collisions);
    
    // Damage Players Over Time
    let num_ticks_per_second: u64 = (1000 / config::SERVER_SIMULATION_RATE_MS) as u64;
    if (tick % num_ticks_per_second == 0)
    {
        for player in game_state.players.values_mut() {
            player.modify_health(-1);
        }
    }

    // Check Player Died
    let mut dead_player_ids = Vec::new();
    for (player_id, player) in &game_state.players {
        if player.is_dead() {
            dead_player_ids.push(player_id.clone());
        }
    }

    for player_id in dead_player_ids {
        game_state.remove_player(&player_id);
    }
}
