use turbo::*;

use crate::{
    create_uuid, entities::{EntityState, EventState, PlayerState}, sample_inside_bounds, EffectSource, EffectState, EffectType, GameState
};

pub fn spawn_effect_from_player(
    current_tick: u64,
    game_state: &mut GameState,
    player_id: &str,
    item_index: usize,
) -> bool {
    // Validate Player
    let player = match game_state.players.get_mut(player_id) {
        Some(player) => player,
        None => return false,
    };

    if player.is_dead() || !player.can_use_item() {
        return false;
    }

    // Create Effect
    if let Some(ref mut item) = player.data_mut().held_items[item_index] {
        if let Some(effect_type) = item.try_use() {
            let effect = create_player_effect(current_tick, player, effect_type);
            game_state.add_effect(effect);
        }
    }

    true
}

pub fn spawn_effect_from_event(current_tick: u64, map_bounds: &Bounds) -> EffectState {
    // Positions
    let inside_pos = sample_inside_bounds(map_bounds);

    // Random Effect
    // TODO: flesh out events
    let effect_type = EffectType::HealthField { health_modifier: 2, radius_multiplier: 1.0, duration: 200, follows_player: false };
    let mut effect = EffectState::new(
        create_uuid(current_tick),
        EffectSource::Event { event_id: create_uuid(current_tick) },
        effect_type,
        current_tick,
    );
    effect.set_position(inside_pos);
    effect.set_radius(100.0);
    
    effect
}

// --- Helpers ---

fn create_player_effect(
    current_tick: u64,
    player: &PlayerState,
    effect_type: EffectType,
) -> EffectState {
    let mut effect = EffectState::new(
        create_uuid(current_tick),
        EffectSource::Player { 
            player_id: player.get_id().to_string(),
            follows_player: effect_type.follows_player(),
        },
        effect_type,
        current_tick,
    );
    effect.set_position(player.get_position());
    effect
}
