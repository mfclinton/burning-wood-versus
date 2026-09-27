use std::collections::{HashMap, HashSet};

use turbo::*;

use crate::{
    entities::{EntityDirtyState, EntityState}, screens::screen_management, state::ScreenType, App, GameState, MainChannel, ServerEvent, UserMessage, InputState, ClientControlled, play_game_sound_if_enabled, game_sounds, EffectType, play_positioned_game_sound_if_enabled
};

// --- Entity Recovery Context ---

pub struct EntityRecoveryContext<'a> {
    pending_lookups: &'a mut HashSet<String>,
    input_state: &'a mut InputState,
}

pub fn connect() {
    let connection = match MainChannel::subscribe("*") {
        Some(conn) => conn,
        None => return,
    };
}

pub fn send_message(message: &UserMessage) {
    let connection = match MainChannel::subscribe("*") {
        Some(conn) => conn,
        None => return,
    };
    connection.send(message);
}


pub fn process_server_events(app: &mut App) -> bool {
    let connection = match MainChannel::subscribe("*") {
        Some(conn) => conn,
        None => return false,
    };

    // Process Server Events
    while let Ok(server_event) = connection.recv() {        
        let mut recovery_ctx = EntityRecoveryContext {
            pending_lookups: &mut app.pending_entity_lookups,
            input_state: &mut app.input_state,
        };
        
        game_state_process_server_event_with_recovery(&server_event, &mut app.game_state_manager.server_state, &mut Some(&mut recovery_ctx));
        app_process_server_event(&server_event, app);

        return true;
    }

    false
}


// --- Event Processing ---

fn game_state_process_server_event_with_recovery(event: &ServerEvent, game_state: &mut GameState, recovery_ctx: &mut Option<&mut EntityRecoveryContext>) {
    match event {
        // Misc
        ServerEvent::EventBatch { events } => {
            for event in events {
                game_state_process_server_event_with_recovery(event, game_state, recovery_ctx);
            }
        },

        // Players
        ServerEvent::PlayerSpawned { user_id, player_state } => {
            game_state.add_player(player_state.clone());
        },
        ServerEvent::PlayerDeleted { user_id } => {
            game_state.remove_player(user_id);
        },
        ServerEvent::PlayerMoveStateChanged { user_id, position, velocity, last_processed_seq } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                player.set_position(*position);
                player.set_velocity(*velocity);
                player.set_last_processed_sequence(*last_processed_seq);
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },
        ServerEvent::PlayerHealthChanged { user_id, health } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                player.set_health(*health);
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },

        // Player Items
        ServerEvent::PlayerHeldItemChanged { user_id, item_index, item_state } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                player.set_held_item(*item_index, item_state.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },
        ServerEvent::PlayerHeldItemPhaseChanged { user_id, item_index, phase } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                if let Some(ref mut item) = player.data_mut().held_items[*item_index] {
                    item.set_phase(phase.clone());
                }
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },
        ServerEvent::PlayerHeldItemChargesChanged { user_id, item_index, charges } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                if let Some(ref mut item) = player.data_mut().held_items[*item_index] {
                    item.set_charges(*charges);
                }
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },
        ServerEvent::PlayerHeldItemTimerChanged { user_id, item_index, timer } => {
            if let Some(player) = game_state.players.get_mut(user_id) {
                if let Some(ref mut item) = player.data_mut().held_items[*item_index] {
                    item.set_timer(*timer);
                }
            } else if let Some(ctx) = recovery_ctx {
                request_missing_player(ctx, user_id);
            }
        },

        // Projectiles
        ServerEvent::ProjectileSpawned { projectile_id, projectile_state } => {
            game_state.add_projectile(projectile_state.clone());
        },
        ServerEvent::ProjectileDeleted { projectile_id } => {
            game_state.remove_projectile(projectile_id);
        },
        ServerEvent::ProjectileMoved { projectile_id, position } => {
            if let Some(projectile) = game_state.projectiles.get_mut(projectile_id) {
                projectile.set_position(position.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_projectile(ctx, projectile_id);
            }
        },
        ServerEvent::ProjectileVelocityChanged { projectile_id, velocity } => {
            if let Some(projectile) = game_state.projectiles.get_mut(projectile_id) {
                projectile.set_velocity(velocity.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_projectile(ctx, projectile_id);
            }
        },
        ServerEvent::ProjectilePayloadChanged { projectile_id, payload } => {
            if let Some(projectile) = game_state.projectiles.get_mut(projectile_id) {
                projectile.set_payload(payload.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_projectile(ctx, projectile_id);
            }
        },

        // Effects
        ServerEvent::EffectSpawned { effect_id, effect_state } => {
            game_state.add_effect(effect_state.clone());
        },
        ServerEvent::EffectDeleted { effect_id } => {
            game_state.remove_effect(effect_id);
        },
        ServerEvent::EffectMoved { effect_id, position } => {
            if let Some(effect) = game_state.active_effects.get_mut(effect_id) {
                effect.set_position(position.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_effect(ctx, effect_id);
            }
        },
        ServerEvent::EffectRadiusChanged { effect_id, radius } => {
            if let Some(effect) = game_state.active_effects.get_mut(effect_id) {
                effect.set_radius(*radius);
            } else if let Some(ctx) = recovery_ctx {
                request_missing_effect(ctx, effect_id);
            }
        },
        ServerEvent::EffectTypeChanged { effect_id, effect_type } => {
            if let Some(effect) = game_state.active_effects.get_mut(effect_id) {
                effect.set_type(effect_type.clone());
            } else if let Some(ctx) = recovery_ctx {
                request_missing_effect(ctx, effect_id);
            }
        },

        // Events
        ServerEvent::EventSpawned { event_id, event_data } => {
            game_state.add_event(event_data.clone());
        }
        ServerEvent::EventDeleted { event_id } => {
            game_state.remove_event(event_id);
        },

        // Round Timing
        ServerEvent::RoundStarted { round_end_time_ms } => {
            game_state.round_end_time_ms = Some(*round_end_time_ms);
            game_state.round_over_end_time_ms = None;
        },
        ServerEvent::RoundEnded { round_over_end_time_ms } => {
            game_state.round_over_end_time_ms = Some(*round_over_end_time_ms);
        },
        ServerEvent::RoundOverEnded => {
            
        },

        // Entity Recovery Responses
        ServerEvent::PlayerLookupResponse { user_id, player_state } => {
            if let Some(ctx) = recovery_ctx {
                let lookup_key = format!("player:{}", user_id);
                ctx.pending_lookups.remove(&lookup_key);
                
                if let Some(player_state) = player_state {
                    game_state.add_player(player_state.clone());
                }
            }
        },
        ServerEvent::ProjectileLookupResponse { projectile_id, projectile_state } => {
            if let Some(ctx) = recovery_ctx {
                let lookup_key = format!("projectile:{}", projectile_id);
                ctx.pending_lookups.remove(&lookup_key);
                
                if let Some(projectile_state) = projectile_state {
                    game_state.projectiles.insert(projectile_id.clone(), projectile_state.clone());
                }
            }
        },
        ServerEvent::EffectLookupResponse { effect_id, effect_state } => {
            if let Some(ctx) = recovery_ctx {
                let lookup_key = format!("effect:{}", effect_id);
                ctx.pending_lookups.remove(&lookup_key);
                
                if let Some(effect_state) = effect_state {
                    game_state.active_effects.insert(effect_id.clone(), effect_state.clone());
                }
            }
        },
    }
}


fn app_process_server_event(event: &ServerEvent, app: &mut App) {
    match event {
        // Misc
        ServerEvent::EventBatch { events } => {
            for event in events {
                app_process_server_event(event, app);
            }
        }

        // Player
        ServerEvent::PlayerDeleted { user_id } => {
            if turbo::os::client::user_id() == Some(user_id.clone()) && app.screen_state.current_screen != ScreenType::GameOver {
                // Get player position before deletion for positioned audio
                if let Some(player) = app.game_state_manager.server_state.players.get(user_id) {
                    let player_position = player.collider().position;
                    play_positioned_game_sound_if_enabled(app, game_sounds::PLAYER_DEATH, player_position);
                } else {
                    // Fallback to non-positioned audio
                    play_game_sound_if_enabled(app, game_sounds::PLAYER_DEATH);
                }
                screen_management::trigger_transition_to_screen(app, ScreenType::GameOver, 500);
            }
        },

        // Player Health
        ServerEvent::PlayerHealthChanged { user_id, health } => {
            if turbo::os::client::user_id() == Some(user_id.clone()) {
                if let Some(player) = app.game_state_manager.server_state.players.get(user_id) {
                    let old_health = player.data().health;
                    let new_health = *health;
                    let player_position = player.collider().position;

                    if new_health > old_health {
                        log!("Playing positioned FLAME_UPGRADE sound");
                        play_positioned_game_sound_if_enabled(app, game_sounds::FLAME_UPGRADE, player_position);
                    } else if new_health < old_health {
                        log!("Playing positioned FLAME_DEGRADE sound");
                        play_positioned_game_sound_if_enabled(app, game_sounds::FLAME_DEGRADE, player_position);
                    }
                }
            }
        },

        // Player Items
        ServerEvent::PlayerHeldItemChanged { user_id, item_state, .. } => {
            if turbo::os::client::user_id() == Some(user_id.clone()) {
                if item_state.is_some() {
                    // Get player position for positioned audio
                    if let Some(player) = app.game_state_manager.server_state.players.get(user_id) {
                        log!("Playing positioned POWERUP_PICKUP sound");
                        let player_position = player.collider().position;
                        play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_PICKUP, player_position);
                    } else {
                        // Fallback to non-positioned audio
                        log!("Playing non-positioned POWERUP_PICKUP sound");
                        play_game_sound_if_enabled(app, game_sounds::POWERUP_PICKUP);
                    }
                }
            }
        },

        // Effect Spawned
        ServerEvent::EffectSpawned { effect_state, .. } => {
            let source = &effect_state.data().source;
            let effect_position = effect_state.collider().position;

            if let crate::EffectSource::Player { player_id, .. } = source {
                if turbo::os::client::user_id() == Some(player_id.clone()) {
                    match &effect_state.data().effect_type {
                        EffectType::MagnetField { .. } => {
                            log!("MagnetField sound not implemented");
                            play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_GRAVITY_BALL, effect_position);
                        },
                        EffectType::DamageField { .. } => {
                            log!("DamageField sound not implemented");
                            play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_BLACKHOLE, effect_position);
                        },
                        EffectType::HealthField { .. } => {
                            log!("HealthField sound not implemented");
                            play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_GASCAN, effect_position);
                        },
                        EffectType::ProjectileSpawner { .. } => {
                            log!("ProjectileSpawner sound not implemented");
                            play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_WATERGUN, effect_position);
                        },
                        _ => {
                            log!("PowerUp sound not implemented");
                            play_positioned_game_sound_if_enabled(app, game_sounds::POWERUP_USE, effect_position);
                        }
                    }
                }
            }
        },

        // Round Timing
        ServerEvent::RoundEnded { .. } => {
            if app.screen_state.current_screen == ScreenType::Game {
                screen_management::trigger_transition_to_screen(app, ScreenType::RoundOver, 0);
            }
        },
        ServerEvent::RoundOverEnded => {
            if app.screen_state.current_screen == ScreenType::RoundOver {
                screen_management::trigger_transition_to_screen(app, ScreenType::Game, 1000);
            }
        },

        _ => {}
    }
}

// --- Entity Recovery ---

fn request_missing_player(recovery_ctx: &mut EntityRecoveryContext, user_id: &str) {
    let lookup_key = format!("player:{}", user_id);
    if recovery_ctx.pending_lookups.contains(&lookup_key) {
        return;
    }
    
    recovery_ctx.pending_lookups.insert(lookup_key);
    let message = UserMessage::RequestPlayerLookup {
        seq: recovery_ctx.input_state.next_sequence(),
        user_id: user_id.to_string(),
    };
    send_message(&message);
}

fn request_missing_projectile(recovery_ctx: &mut EntityRecoveryContext, projectile_id: &str) {
    let lookup_key = format!("projectile:{}", projectile_id);
    if recovery_ctx.pending_lookups.contains(&lookup_key) {
        return;
    }
    
    recovery_ctx.pending_lookups.insert(lookup_key);
    let message = UserMessage::RequestProjectileLookup {
        seq: recovery_ctx.input_state.next_sequence(),
        projectile_id: projectile_id.to_string(),
    };
    send_message(&message);
}

fn request_missing_effect(recovery_ctx: &mut EntityRecoveryContext, effect_id: &str) {
    let lookup_key = format!("effect:{}", effect_id);
    if recovery_ctx.pending_lookups.contains(&lookup_key) {
        return;
    }
    
    recovery_ctx.pending_lookups.insert(lookup_key);
    let message = UserMessage::RequestEffectLookup {
        seq: recovery_ctx.input_state.next_sequence(),
        effect_id: effect_id.to_string(),
    };
    send_message(&message);
}
