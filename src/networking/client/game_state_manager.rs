use turbo::{os::server, *};

use crate::{common::Vec2, config, entities::{EntityDirtyState, EntityState}, is_client_player, networking::UserMessage, systems::apply_player_movement_scaled, update_player_physics_scaled, update_projectile_physics_scaled, ClientControlled, GameState, InputState};

// --- Client Game State Manager ---

#[turbo::serialize]
pub struct GameStateManager {
    pub server_state: GameState,
    pub predicted_state: GameState,
    pub local_state: GameState,
    pub lerp_speed: f32,
    pub time_scale: f32,
}

impl GameStateManager {
    pub fn new() -> Self {
        Self {
            server_state: GameState::new(),
            predicted_state: GameState::new(),
            local_state: GameState::new(),
            lerp_speed: config::CLIENT_PREDICTION_LERP_SPEED,
            time_scale: 1.0,
        }
    }
    
    // --- Predicted Client Player Input ---

    pub fn apply_client_player_prediction(&mut self, user_message: &UserMessage) {
        if let Some(client_player_id) = turbo::os::client::user_id() {
            match user_message {
                UserMessage::Move { direction, .. } => {
                    if let Some(client_player) = self.predicted_state.players.get_mut(&client_player_id) {
                        apply_player_movement_scaled(client_player, *direction, self.time_scale);
                    }
                },
                UserMessage::UseItem { item_index, .. } => {
                    
                },
                _ => {}
            }
        }
    }
    
    pub fn check_and_rollback_player(&mut self, input_state: &mut InputState) {
        if let Some(client_player_id) = turbo::os::client::user_id() {
            let server_player = self.server_state.players.get(&client_player_id);
            
            if let Some(server) = server_player {
                let server_seq = server.get_last_processed_sequence();
                input_state.acknowledge_sequence(server_seq);
                
                self.rollback_and_replay_player(&client_player_id, input_state);
            }
        }
    }
    
    fn rollback_and_replay_player(&mut self, client_player_id: &str, input_state: &InputState) {
        // Rollback To Server State
        if let Some(server_player) = self.server_state.players.get(client_player_id) {
            self.predicted_state.players.insert(client_player_id.to_string(), server_player.clone());
        }

        // Replay Unacknowledged Inputs
        let last_ack = input_state.last_acknowledged_sequence;

        let unacked_inputs = input_state.get_inputs_since(last_ack);
        for buffered_input in unacked_inputs {
            self.apply_client_player_prediction(&buffered_input.message);
        }
    }

    // --- Predicted State Synchronization ---
    
    pub fn sync_predicted_selective(&mut self) {
        self.sync_predicted_players();
        self.sync_predicted_projectiles();
        self.sync_predicted_effects();
        self.sync_predicted_events();
    }

    fn sync_predicted_players(&mut self) {
        for (user_id, server_player) in &self.server_state.players {
            if is_client_player(&user_id) {
                // Sync Client
                if let Some(predicted_player) = self.predicted_state.players.get_mut(user_id) {
                    // Existing Client
                    predicted_player.copy_if_dirty(server_player);
                } else {
                    // New Client
                    self.predicted_state.players.insert(user_id.clone(), server_player.clone());
                }
            } else {
                // Sync Foreign
                self.predicted_state.players.insert(user_id.clone(), server_player.clone());
            }
        }
    }

    fn sync_predicted_projectiles(&mut self) {
        for (projectile_id, server_projectile) in &self.server_state.projectiles {
            if let Some(predicted_projectile) = self.predicted_state.projectiles.get_mut(projectile_id) {
                // Existing Projectile
                predicted_projectile.copy_if_dirty(server_projectile);
            } else {
                // New Projectile
                self.predicted_state.projectiles.insert(projectile_id.clone(), server_projectile.clone());
            }
        }
    }

    fn sync_predicted_effects(&mut self) {
        for (effect_id, server_effect) in &self.server_state.active_effects {
            if let Some(predicted_effect) = self.predicted_state.active_effects.get_mut(effect_id) {
                // Existing Effect
                predicted_effect.copy_if_dirty(server_effect);
            } else {
                // New Effect
                self.predicted_state.active_effects.insert(effect_id.clone(), server_effect.clone());
            }
        }
    }

    fn sync_predicted_events(&mut self) {
        for (event_id, server_event) in &self.server_state.events {
            if let Some(predicted_event) = self.predicted_state.events.get_mut(event_id) {
                // Existing Event
                predicted_event.copy_if_dirty(server_event);
            } else {
                // New Event
                self.predicted_state.events.insert(event_id.clone(), server_event.clone());
            }
        }
    }

    // --- Predicted Physics Simulation ---
    
    fn update_predicted_physics(&mut self) {        
        self.update_predicted_player_physics();
        self.update_predicted_projectile_physics();
    }
    
    fn update_predicted_player_physics(&mut self) {        
        // Client Player
        if let Some(client_player_id) = turbo::os::client::user_id() {
            if let Some(client_player) = self.predicted_state.players.get_mut(&client_player_id) {
                if client_player.just_destroyed() {
                    return;
                }

                update_player_physics_scaled(client_player, &self.predicted_state.map_bounds, self.time_scale);
            }
        }
    }
    
    fn update_predicted_projectile_physics(&mut self) {        
        // Projectile
        for (_, projectile) in self.predicted_state.projectiles.iter_mut() {
            if projectile.just_destroyed() {
                continue;
            }

            update_projectile_physics_scaled(projectile, self.time_scale);
        }
    }

    pub fn sync_predicted_and_check_rollback(&mut self, input_state: &mut InputState) {
        self.sync_predicted_selective();
        self.check_and_rollback_player(input_state);
    }

    // --- Local Display State Synchronization ---

    pub fn sync_local(&mut self) {
        // Update Predicted State
        self.update_predicted_physics();

        // Interpolate Predicted State
        self.sync_local_players();
        self.sync_local_projectiles();
        self.sync_local_effects();
        self.sync_local_events();
    }

    fn sync_local_players(&mut self) {
        for (user_id, predicted_player) in self.predicted_state.players.iter() {
            // Lerp Player State
            if let Some(local_player) = self.local_state.players.get_mut(user_id) {
                // Existing
                local_player._copy_if_dirty_lifecycle(predicted_player);
                local_player.lerp_player_to_source(predicted_player, config::CLIENT_PREDICTION_LERP_SPEED);
            } else {
                // New
                self.local_state.add_player(predicted_player.clone());
            }
        }
    }
    
    fn sync_local_projectiles(&mut self) {
        // Lerp Projectiles
        for (projectile_id, predicted_projectile) in self.predicted_state.projectiles.iter() {
            if let Some(local_projectile) = self.local_state.projectiles.get_mut(projectile_id) {
                // Existing
                local_projectile._copy_if_dirty_lifecycle(predicted_projectile);
                local_projectile.lerp_projectile_to_source(predicted_projectile, config::CLIENT_PREDICTION_LERP_SPEED);
            } else {
                // New
                self.local_state.projectiles.insert(projectile_id.clone(), predicted_projectile.clone());
            }
        }
    }

    fn sync_local_effects(&mut self) {
        // Lerp Effects
        for (effect_id, server_effect) in self.server_state.active_effects.iter() {
            if let Some(local_effect) = self.local_state.active_effects.get_mut(effect_id) {
                // Existing
                local_effect._copy_if_dirty_lifecycle(server_effect);
                local_effect.lerp_effect_to_source(server_effect, config::CLIENT_PREDICTION_LERP_SPEED);
            } else {
                // New
                self.local_state.active_effects.insert(effect_id.clone(), server_effect.clone());
            }
        }
    }

    fn sync_local_events(&mut self) {
        // Sync Events
        for (event_id, server_event) in self.server_state.events.iter() {
            if let Some(local_event) = self.local_state.events.get_mut(event_id) {
                // Existing
            } else {
                // New
                self.local_state.events.insert(event_id.clone(), server_event.clone());
            }
        }
    }

    // --- Helper ---

    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.time_scale = time_scale;
    }

    pub fn cleanup_destroyed_states(&mut self) {
        self.server_state.cleanup_destroyed_entities(true);
        self.predicted_state.cleanup_destroyed_entities(true);
        self.local_state.cleanup_destroyed_entities(true);
    }

    pub fn clear_dirty_states(&mut self) {
        self.server_state.clear_entity_dirty_flags();
        self.predicted_state.clear_entity_dirty_flags();
        self.local_state.clear_entity_dirty_flags();
    }

    // --- Debug ---

    pub fn debug_client_player(&self) {
        if let Some(client_player_id) = turbo::os::client::user_id() {
            // Server
            if let Some(server_player) = self.server_state.players.get(&client_player_id) {
                server_player.render_debug_color(0x0000FF99); // Blue
            }
            
            // Predicted
            if let Some(predicted_player) = self.predicted_state.players.get(&client_player_id) {
                predicted_player.render_debug_color(0x00FF0099); // Green
            }
        }
    }

    pub fn debug_projectiles(&self) {
        // Server
        for (projectile_id, server_projectile) in &self.server_state.projectiles {
            server_projectile.render_debug_color(0x0000FF99); // Blue
        }
        
        // Predicted
        for (projectile_id, predicted_projectile) in &self.predicted_state.projectiles {
            predicted_projectile.render_debug_color(0x00FF0099); // Green
        }
    }

}
