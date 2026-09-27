use std::collections::HashMap;

use turbo::*;

use crate::{common::{Anchor, Renderable, ScaleMode, Text, Vec2}, PlayerData, entities::EntityState, networking::is_client_player, PlayerState, JERSEY_20};

const NUM_LEADERBOARD_ENTRIES: usize = 5;

pub fn draw_leaderboard(players: &HashMap<String, PlayerState>) {
    let canvas_bounds = bounds::screen();
    let ordered_players = get_ordered_players(players);
    
    let mut top_right_anchor = canvas_bounds.top_right();
    for (i, player) in ordered_players.iter().enumerate() {
        let entry_str = format!("{}. {} - {}", (i+1), player.username, player.health);
        let mut entry_text = Text::new(&entry_str, &JERSEY_20);
        entry_text = entry_text.with_scale(0.05, ScaleMode::HeightRelative);

        let entry_text_bounds = entry_text.get_bounds_anchored_at(top_right_anchor.into(), Anchor::TopRight);

        // Color
        let color = if is_client_player(&player.id) {
            0xff00ffff // Green
        } else {
            0xffffffff // White
        };
        
        entry_text.render(entry_text_bounds.xy().into(), color, true);

        top_right_anchor = entry_text_bounds.bottom_right();
    }
}

// --- Helper Functions ---

fn get_ordered_players(players: &HashMap<String, PlayerState>) -> Vec<PlayerData> {
    let mut leaderboard: Vec<PlayerData> = players
        .iter()
        .map(|(_user_id, player)| {
            player.data().clone()
        })
        .collect();

    leaderboard.sort_by(|a, b| b.health.cmp(&a.health));

    leaderboard.into_iter().take(NUM_LEADERBOARD_ENTRIES).collect()
}
