use turbo::*;
use crate::{config, GameState, Vec2, entities::EntityState};

const BACKGROUND_MUSIC: &str = "Music/The Burning Wood Temple";

pub struct AudioManager;

impl AudioManager {
    /// Play background music (looped)
    pub fn play_background_music() {
        if !audio::is_playing(BACKGROUND_MUSIC) {
            audio::play(BACKGROUND_MUSIC);
            audio::set_volume(BACKGROUND_MUSIC, config::AUDIO_MAX_VOLUME);
        }
    }

    /// Stop background music
    pub fn stop_background_music() {
        audio::stop(BACKGROUND_MUSIC);
    }

    /// Play UI sound effect
    pub fn play_ui_sound(sound_name: &str) {
        audio::play(sound_name);
        audio::set_volume(sound_name, config::AUDIO_MAX_VOLUME);
    }

    /// Play game sound effect
    pub fn play_game_sound(sound_name: &str) {
        audio::play(sound_name);
        audio::set_volume(sound_name, config::AUDIO_MAX_VOLUME);
    }
}

// UI Sound Constants
pub mod ui_sounds {
    pub const BUTTON_HOVER: &str = "SFX/Button_Hover";
    pub const BUTTON_SELECT: &str = "SFX/Button_Select_Generic";
    pub const ENTER_BUTTON: &str = "SFX/Enter_Button";
    pub const TOGGLE_BUTTON: &str = "SFX/Toggle_Buttons";
}

// Game Sound Constants
pub mod game_sounds {
    pub const PLAYER_DEATH: &str = "SFX/PC_Death";
    pub const FLAME_UPGRADE: &str = "SFX/Flame_Upgrade";
    pub const FLAME_DEGRADE: &str = "SFX/Flame_Degrade";
    pub const POWERUP_PICKUP: &str = "SFX/Powerup_Pickup";
    pub const POWERUP_USE: &str = "SFX/Powerup_Use_Generic";
    pub const POWERUP_BLACKHOLE: &str = "SFX/Powerup_BlackHole";
    pub const POWERUP_GASCAN: &str = "SFX/Powerup_GasCan";
    pub const POWERUP_GOLDEN_IDOL: &str = "SFX/Powerup_Golden Idol";
    pub const POWERUP_GRAVITY_BALL: &str = "SFX/Powerup_Gravity Ball";
    pub const POWERUP_WATERGUN: &str = "SFX/Powerup_Watergun";
}

// Convenience functions that check settings
pub fn play_ui_sound_if_enabled(app: &crate::App, sound_name: &str) {
    if app.settings_state.sfx_enabled {
        AudioManager::play_ui_sound(sound_name);
    }
}

pub fn play_game_sound_if_enabled(app: &crate::App, sound_name: &str) {
    if app.settings_state.sfx_enabled {
        AudioManager::play_game_sound(sound_name);
    }
}

pub fn update_background_music(app: &crate::App) {
    if app.settings_state.music_enabled {
        AudioManager::play_background_music();
    } else {
        AudioManager::stop_background_music();
    }
}

// Distance-based audio system
pub fn calculate_audio_volume(source_position: Vec2, game_state: &GameState) -> f32 {
    // Get client player position and radius
    if let Some(client_user_id) = turbo::os::client::user_id() {
        if let Some(client_player) = game_state.players.get(&client_user_id) {
            let player_collider = client_player.collider();
            let player_position = player_collider.position;
            let player_radius = player_collider.radius;
            let distance = (source_position - player_position).length();

            // Calculate max hearing distance based on player radius
            let max_hearing_distance = player_radius * config::AUDIO_MAX_DISTANCE_RADIUS_MULTIPLIER;

            // Calculate volume based on distance (1.0 at close range, 0.0 at max distance)
            let volume = (1.0 - (distance / max_hearing_distance)).clamp(config::AUDIO_MIN_VOLUME, config::AUDIO_MAX_VOLUME);
            return volume;
        }
    }

    // Default to full volume if no client player found
    config::AUDIO_MAX_VOLUME
}

// Position-aware sound functions
pub fn play_positioned_game_sound_if_enabled(app: &crate::App, sound_name: &str, source_position: Vec2) {
    if app.settings_state.sfx_enabled {
        let volume = calculate_audio_volume(source_position, &app.game_state_manager.local_state);

        if volume > config::AUDIO_MIN_VOLUME {
            AudioManager::play_game_sound(sound_name);
            audio::set_volume(sound_name, volume);
        }
    }
}

pub fn play_positioned_ui_sound_if_enabled(app: &crate::App, sound_name: &str, source_position: Vec2) {
    if app.settings_state.sfx_enabled {
        let volume = calculate_audio_volume(source_position, &app.game_state_manager.local_state);

        if volume > config::AUDIO_MIN_VOLUME {
            AudioManager::play_ui_sound(sound_name);
            audio::set_volume(sound_name, volume);
        }
    }
}