use core::net;
use std::collections::HashMap;

use turbo::os::server;
use turbo::*;

use crate::{common::Span, config, draw_fade_to_black, entities::{EntityState, PlayerState}, movement_vector_with_pad, screen_management, send_message, set_lighting_shader, update_and_render_game_state, transition_t, update_ambient_light, App, ItemType, ScreenType, UserMessage, play_game_sound_if_enabled, game_sounds};

// Animation Timing Constants
const FADE_TO_BLACK_SPAN: Span = Span::new(0.1, 0.95);

// --- Screen Management Functions ---

pub fn enter(app: &mut App) {
    if app.screen_state.previous_screen == Some(ScreenType::GameOptions) {
        return;
    }

    // Initialize Shader
    set_lighting_shader();
    update_ambient_light(0.0);

    // Set Intro Transition
    app.screen_state.transition_started_ms = time::now();
    app.screen_state.transition_duration_ms = 10000;
}

pub fn update(app: &mut App) {
    // Clear Screen
    clear(0x333333ff);
    
    // Process Player
    process_player(app);
    transition_to_options_menu_keyboard(app);
    
    // Update and Render Core Game State
    update_and_render_game_state(app);

    // Screen-specific Rendering
    let t = transition_t(app);

    // Fade to Black
    let fade_to_black_exempt = [Some(ScreenType::GameOptions)];
    if !fade_to_black_exempt.contains(&app.screen_state.next_screen) &&
        !fade_to_black_exempt.contains(&Some(app.screen_state.current_screen.clone())) &&
        !fade_to_black_exempt.contains(&app.screen_state.previous_screen) {
        draw_fade_to_black(t, FADE_TO_BLACK_SPAN);
    }

    // Circular Pad
    app.circular_pad.render();
}

pub fn exit(app: &mut App) {
    if app.screen_state.next_screen == Some(ScreenType::GameOptions) {
        return;
    }

    // Disable Shader
    shaders::reset();
}

// --- Player Logic ---

fn process_player(app: &mut App) {
    // Game Over
    if app.screen_state.next_screen == Some(ScreenType::GameOver) {
        return;
    }

    // Player
    if let Some(user_id) = turbo::os::client::user_id() {
        if let Some(player) = app.game_state_manager.local_state.players.get_mut(&user_id) {
            // Movement
            process_movement_input(app);

            // Use Item
           process_item_input(app);
        }
        else {
            // New Player
            if app.join_request_timer.check_and_reset() {
                let message = UserMessage::PlayerJoin { 
                    seq: app.input_state.next_sequence(),
                    username: app.player_config_state.username.clone() 
                };
                app.input_state.buffer_input(message.clone(), time::now() as f32);
                send_message(&message);
            }
        }
    }

    // Debug
    #[cfg(feature = "debug")]
    {
        debug_get_item(app);
    }

}

fn process_movement_input(app: &mut App) {
    // Process Input
    app.circular_pad.update();
    let step = movement_vector_with_pad(&app.circular_pad);
    if step.length() < 0.01 {
        return;
    }

    // Local Prediction
    let seq = app.input_state.next_sequence();
    let input_message = UserMessage::Move {
        seq,
        direction: step,
    };
    
    app.game_state_manager.apply_client_player_prediction(&input_message);
    app.input_state.buffer_input(input_message.clone(), time::now() as f32);
    
    if app.input_batch_timer.check_and_reset() {        
        send_message(&input_message);
    }
}

fn process_item_input(app: &mut App) {
    // Use Any Item
    if PlayerState::get_auto_use_item_just_pressed() {
        // Server
        let message = UserMessage::UseAnyItem { 
            seq: app.input_state.next_sequence(),
        };
        send_message(&message);
    }

    // Specific Slot
    let mut item_keys_just_pressed = PlayerState::get_item_keyboard_key_just_pressed();

    for (item_index, just_pressed) in item_keys_just_pressed.iter().enumerate() {
        if *just_pressed {
            // Server
            let message = UserMessage::UseItem {
                seq: app.input_state.next_sequence(),
                item_index
            };
            send_message(&message);

            // Play sound effect
            play_game_sound_if_enabled(app, game_sounds::POWERUP_USE);

            // Local Prediction
            app.game_state_manager.apply_client_player_prediction(&message);
            app.input_state.buffer_input(message.clone(), time::now() as f32);
            break;
        }
    }
}

// --- Options Menu ---

fn transition_to_options_menu_keyboard(app: &mut App) {
    if keyboard::get().escape().just_pressed() && app.screen_state.next_screen.is_none() {
        screen_management::trigger_transition_to_screen(app, ScreenType::GameOptions, 0);
    }
}

// --- Debug ---

#[cfg(feature = "debug")]
fn debug_get_item(app: &mut App) {
    let keyboard = keyboard::get();
    if let Some(ch) = keyboard.text().chars().next() {
        if let Some(item_type) = char_to_item_type(ch) {
            let message = UserMessage::DebugGetItem {
                seq: app.input_state.next_sequence(),
                item_type,
            };
            send_message(&message);
        }
    }
}

#[cfg(feature = "debug")]
fn char_to_item_type(ch: char) -> Option<ItemType> {    
    match ch {
        '1' => Some(ItemType::WaterGun),
        '2' => Some(ItemType::Blackhole),
        '3' => Some(ItemType::Lightning),
        '4' => Some(ItemType::SmallGas),
        '5' => Some(ItemType::MediumGas),
        '6' => Some(ItemType::LargeGas),
        '7' => Some(ItemType::Bat),
        '8' => Some(ItemType::Mine),
        '9' => Some(ItemType::MetalWaterGun),
        '0' => Some(ItemType::WaterBomb),
        'p' => Some(ItemType::Wind),
        _ => None,
    }
}
