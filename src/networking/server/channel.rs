use std::collections::HashMap;

use turbo::*;
use turbo::os::server::channel::ChannelSettings;

use crate::{
    common::ClientTimer, config, create_batches, generate_entity_events, generate_server_events, names::random_username, spawn_effect_from_player, spawn_player, systems::apply_player_movement_scaled, update_game_simulation, ClientControlled, GameState, ItemState, ServerEvent, ServerTimer, UserMessage
};

#[turbo::os::channel(program = "burningWoodVersus", name = "MainChannel")]
pub struct MainChannel {
    game_state: GameState,
    time_scale: f32,
    game_round_timer: ClientTimer,
    round_over_timer: ClientTimer,
    is_round_active: bool,

    // Network Update Timers
    simulation_timer: ServerTimer,
    player_network_timer: ServerTimer,
    projectile_network_timer: ServerTimer,
    effect_network_timer: ServerTimer,
}

impl ChannelHandler for MainChannel {
    type Send = ServerEvent;
    type Recv = UserMessage;

    fn new() -> Self {
        let mut instance = Self {
            game_state: GameState::new(),
            time_scale: 1.0,
            game_round_timer: ClientTimer::new(config::SERVER_ROUND_TIME_MS as u64),
            round_over_timer: ClientTimer::new(config::SERVER_ROUND_OVER_DURATION_MS as u64),
            is_round_active: true,

            // Network Update Timers
            simulation_timer: ServerTimer::new(config::SERVER_SIMULATION_RATE_MS as u64),
            player_network_timer: ServerTimer::new(config::SERVER_PLAYER_NETWORK_RATE_MS as u64),
            projectile_network_timer: ServerTimer::new(config::SERVER_PROJECTILE_NETWORK_RATE_MS as u64),
            effect_network_timer: ServerTimer::new(config::SERVER_EFFECT_NETWORK_RATE_MS as u64),
        };

        instance.start_new_round();
        instance
    }

    fn on_open(&mut self, settings: &mut ChannelSettings) -> Result<(), std::io::Error> {
        settings.set_interval(config::SERVER_INTERVAL_RATE_MS);
        Ok(())
    }

    fn on_connect(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        self.send_state_sync_events(user_id);
        Ok(())
    }

    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        // Message Expired
        let (seq, should_process) = match &data {
            UserMessage::PlayerJoin { seq, .. } => (*seq, true),
            UserMessage::Move { seq, .. } => {
                let should_process = if let Some(player) = self.game_state.players.get(user_id) {
                    player.should_process_sequence(*seq)
                } else { true };
                (*seq, should_process)
            },
            UserMessage::UseItem { seq, .. } => {
                let should_process = if let Some(player) = self.game_state.players.get(user_id) {
                    player.should_process_sequence(*seq)
                } else { true };
                (*seq, should_process)
            },
            UserMessage::UseAnyItem { seq, .. } => {
                let should_process = if let Some(player) = self.game_state.players.get(user_id) {
                    player.should_process_sequence(*seq)
                } else { true };
                (*seq, should_process)
            },

            // Entity Recovery
            UserMessage::RequestPlayerLookup { seq, .. } => (*seq, true),
            UserMessage::RequestProjectileLookup { seq, .. } => (*seq, true),
            UserMessage::RequestEffectLookup { seq, .. } => (*seq, true),

            // Debug
            #[cfg(feature = "debug")]
            UserMessage::DebugGetItem { seq, .. } => (*seq, true),
        };

        // Process Message
        if should_process {
            match data {
                UserMessage::PlayerJoin { username, .. } => {
                    let spawn_event = spawn_player(&mut self.game_state, user_id, username);
                    
                    let is_new_player = !self.game_state.players.contains_key(user_id);
                    if is_new_player {
                        os::server::channel::broadcast(spawn_event);
                    } else {
                        os::server::channel::send(user_id, &spawn_event);
                    }
                    
                    if let Some(player) = self.game_state.players.get_mut(user_id) {
                        player.set_last_processed_sequence(seq);
                    }
                }
                UserMessage::Move { direction, .. } => {
                    let direction = direction.clamp_normalized();
                    if let Some(player) = self.game_state.players.get_mut(user_id) {
                        apply_player_movement_scaled(player, direction, self.time_scale);
                        player.set_last_processed_sequence(seq);
                    }
                }
                UserMessage::UseItem { item_index, .. } => {
                    spawn_effect_from_player(self.simulation_timer.elapsed_ticks(), &mut self.game_state, user_id, item_index);
                    if let Some(player) = self.game_state.players.get_mut(user_id) {
                        player.set_last_processed_sequence(seq);
                    }
                }
                UserMessage::UseAnyItem { .. } => {
                    let mut item_used = false;
                    if let Some(player) = self.game_state.players.get(user_id) {
                        for item_index in 0..config::PLAYER_NUM_HELD_ITEMS {
                            item_used = spawn_effect_from_player(self.simulation_timer.elapsed_ticks(), &mut self.game_state, user_id, item_index);
                            if item_used {
                                break;
                            }
                        }
                    }

                    if item_used {
                        if let Some(player) = self.game_state.players.get_mut(user_id) {
                            player.set_last_processed_sequence(seq);
                        }
                    }
                }

                // Entity Recovery
                UserMessage::RequestPlayerLookup { user_id: lookup_user_id, .. } => {
                    let player_state = self.game_state.players.get(&lookup_user_id).cloned();
                    let response = ServerEvent::PlayerLookupResponse {
                        user_id: lookup_user_id.clone(),
                        player_state,
                    };
                    os::server::channel::send(user_id, response);
                }
                UserMessage::RequestProjectileLookup { projectile_id, .. } => {
                    let projectile_state = self.game_state.projectiles.get(&projectile_id).cloned();
                    let response = ServerEvent::ProjectileLookupResponse {
                        projectile_id: projectile_id.clone(),
                        projectile_state,
                    };
                    os::server::channel::send(user_id, response);
                }
                UserMessage::RequestEffectLookup { effect_id, .. } => {
                    let effect_state = self.game_state.active_effects.get(&effect_id).cloned();
                    let response = ServerEvent::EffectLookupResponse {
                        effect_id: effect_id.clone(),
                        effect_state,
                    };
                    os::server::channel::send(user_id, response);
                }

                // Debug
                #[cfg(feature = "debug")]
                UserMessage::DebugGetItem { item_type, .. } => {
                    if let Some(player) = self.game_state.players.get_mut(user_id) {
                        let item_state = ItemState::new(item_type.clone());
                        player.set_held_item(0, Some(item_state.clone()));

                        let event = ServerEvent::PlayerHeldItemChanged {
                            user_id: user_id.to_string(),
                            item_index: 0,
                            item_state: Some(item_state.clone()),
                        };
                        os::server::channel::broadcast(event);
                    }
                }
            }
        }

        Ok(())
    }

    fn on_disconnect(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        self.game_state.remove_player(user_id);
        Ok(())
    }

    fn on_interval(&mut self) -> Result<(), std::io::Error> {
        // Round Management
        self.update_round_state();

        // Simulation Step
        if self.is_round_active {
            self.run_simulation_step();
        }

        // Network Broadcast
        self.send_staggered_network_updates();

        Ok(())
    }
}

impl MainChannel {
    fn run_simulation_step(&mut self) {
        self.simulation_timer.tick();
        update_game_simulation(&mut self.game_state, self.simulation_timer.elapsed_ticks(), self.time_scale);
    }

    fn send_staggered_network_updates(&mut self) {
        // Tick Timers
        self.player_network_timer.tick();
        self.projectile_network_timer.tick();
        self.effect_network_timer.tick();
        
        // All Events
        let mut all_events = Vec::new();

        // Player Events
        let sent_player_events = if self.player_network_timer.check_and_reset() {
            let player_events = generate_entity_events(&self.game_state.players);
            let has_events = !player_events.is_empty();
            if has_events {
                all_events.extend(player_events);
            }
            has_events
        } else {
            false
        };
        
        // Projectile Events
        let sent_projectile_events = if self.projectile_network_timer.check_and_reset() {
            let projectile_events = generate_entity_events(&self.game_state.projectiles);
            let has_events = !projectile_events.is_empty();
            if has_events {
                all_events.extend(projectile_events);
            }
            has_events
        } else {
            false
        };
        
        // Effect Events
        let sent_effect_events = if self.effect_network_timer.check_and_reset() {
            let effect_events = generate_entity_events(&self.game_state.active_effects);
            let has_events = !effect_events.is_empty();
            if has_events {
                all_events.extend(effect_events);
            }
            has_events
        } else {
            false
        };
        
        // Batch and Send Events
        if !all_events.is_empty() {
            let batched_events = create_batches(all_events);
            for batch in batched_events {
                os::server::channel::broadcast(batch);
            }
        }
        
        // Clear Dirty Flags for Sent Events
        if sent_player_events || sent_projectile_events || sent_effect_events {
            self.clear_selective_dirty_flags(sent_player_events, sent_projectile_events, sent_effect_events);
        }
    }

    fn clear_selective_dirty_flags(&mut self, sent_player_events: bool, sent_projectile_events: bool, sent_effect_events: bool) {
        if sent_player_events {
            self.game_state.cleanup_destroyed_players(false);
            self.game_state.clear_player_dirty_flags();
        }
        if sent_projectile_events {
            self.game_state.cleanup_destroyed_projectiles(false);
            self.game_state.clear_projectile_dirty_flags();
        }
        if sent_effect_events {
            self.game_state.cleanup_destroyed_effects(false);
            self.game_state.clear_effect_dirty_flags();
        }
    }

    // TODO: Batch
    fn send_state_sync_events(&self, user_id: &str) {
        // Send Round State
        if let Some(round_end_time) = self.game_state.round_end_time_ms {
            let event = ServerEvent::RoundStarted {
                round_end_time_ms: round_end_time,
            };
            os::server::channel::send(user_id, &event);
        } else if !self.is_round_active {
            // Player is joining during round over period
            if let Some(round_over_end_time) = self.game_state.round_over_end_time_ms {
                let event = ServerEvent::RoundEnded {
                    round_over_end_time_ms: round_over_end_time,
                };
                os::server::channel::send(user_id, &event);
            }
        }

        // Send Existing Players
        for (player_id, player) in &self.game_state.players {
            let event = ServerEvent::PlayerSpawned {
                user_id: player_id.clone(),
                player_state: player.clone(),
            };
            os::server::channel::send(user_id, &event);
        }
        
        // Send Existing Projectiles individually
        // for (projectile_id, projectile) in &self.game_state.projectiles {
        //     let event = ServerEvent::ProjectileSpawned {
        //         projectile_id: projectile_id.clone(),
        //         projectile_state: projectile.clone(),
        //     };
        //     os::server::channel::send(user_id, &event);
        // }
        
        // Send Existing Effects
        for (effect_id, effect) in &self.game_state.active_effects {
            let event = ServerEvent::EffectSpawned {
                effect_id: effect_id.clone(),
                effect_state: effect.clone(),
            };
            os::server::channel::send(user_id, &event);
        }
    }

    fn update_round_state(&mut self) {
        if self.is_round_active {
            if self.game_round_timer.ready() {
                self.end_round();
            }
        } else {
            if self.round_over_timer.ready() {
                self.start_new_round();
            }
        }
    }

    fn start_new_round(&mut self) {
        let was_round_over = !self.is_round_active;

        self.is_round_active = true;
        self.game_round_timer.reset();

        let round_end_time = time::now() + config::SERVER_ROUND_TIME_MS;
        self.game_state.round_end_time_ms = Some(round_end_time);
        self.game_state.round_over_end_time_ms = None;

        if was_round_over {
            let event = ServerEvent::RoundOverEnded;
            os::server::channel::broadcast(event);
        }

        let event = ServerEvent::RoundStarted {
            round_end_time_ms: round_end_time,
        };
        os::server::channel::broadcast(event);

        // Reset Player
        for (user_id, player) in self.game_state.players.iter_mut() {
            player.reset_for_new_round();
        }
    }

    fn end_round(&mut self) {
        self.is_round_active = false;
        self.round_over_timer.reset();
        self.game_state.round_end_time_ms = None;

        let round_over_end_time = time::now() + config::SERVER_ROUND_OVER_DURATION_MS;
        self.game_state.round_over_end_time_ms = Some(round_over_end_time);

        let event = ServerEvent::RoundEnded {
            round_over_end_time_ms: round_over_end_time,
        };
        os::server::channel::broadcast(event);
    }
}
