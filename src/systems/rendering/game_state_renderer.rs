use turbo::*;

use crate::{
    draw_leaderboard, draw_round_timer, process_server_events, render_game, App
};

pub fn update_and_render_game_state(app: &mut App) {
    // Process Server Events
    let has_events = process_server_events(app);
    if has_events {
        app.game_state_manager.sync_predicted_and_check_rollback(
            &mut app.input_state,
        );
    }

    app.game_state_manager.sync_local();
    app.game_state_manager.cleanup_destroyed_states();

    // Render Game
    render_game(app);

    // Update Camera
    app.camera_controller.update_with_game(&app.game_state_manager.local_state);

    // Draw Core Game UI
    draw_leaderboard(&app.game_state_manager.local_state.players);
    draw_round_timer(app.game_state_manager.server_state.round_end_time_ms);

    // Reset Game State
    app.game_state_manager.clear_dirty_states();
}
