use turbo::*;

use crate::{
    GameState, PlayerState, config, sample_inside_bounds, ServerEvent,
};

pub fn spawn_player(game_state: &mut GameState, user_id: &str, username: String) -> ServerEvent {
    // Check Player Already Exists
    if let Some(existing_player) = game_state.players.get(user_id) {
        return ServerEvent::PlayerSpawned {
            user_id: user_id.to_string(),
            player_state: existing_player.clone(),
        };
    }

    // Random Position
    let random_position = sample_inside_bounds(&game_state.map_bounds);

    // Create New Player
    let new_player = PlayerState::new(user_id.to_string(), username.clone(), config::PLAYER_INITIAL_HEALTH, random_position);
    game_state.add_player(new_player.clone());

    ServerEvent::PlayerSpawned {
        user_id: user_id.to_string(),
        player_state: new_player,
    }
}
