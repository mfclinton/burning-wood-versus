use std::collections::HashMap;
use crate::{entities::EntityState, GameState, ServerEvent};

use turbo::*;

pub fn generate_server_events(game_state: &GameState) -> Vec<ServerEvent> {
    let mut events = vec![];

    // Player Events
    for (_, player) in &game_state.players {
        player.generate_events(&mut events);
    }

    // Projectile Events
    for (_, projectile) in &game_state.projectiles {
        projectile.generate_events(&mut events);
    }

    // Effect Events
    for (_, effect) in &game_state.active_effects {
        effect.generate_events(&mut events);
    }

    events
}

pub fn generate_entity_events<T: EntityState>(entities: &HashMap<String, T>) -> Vec<ServerEvent> {
    let mut events = vec![];
    for (_, entity) in entities {
        entity.generate_events(&mut events);
    }
    events
}
