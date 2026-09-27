use std::collections::HashMap;
use turbo::*;

use crate::{config, entities::{EntityRenderer, EntityState, EventState}, CircleCollider, EffectState, PlayerState, ProjectileState, Vec2};

// --- Game State ---

#[turbo::serialize]
pub struct GameState {
    pub players: HashMap<String, PlayerState>,
    pub projectiles: HashMap<String, ProjectileState>,
    pub active_effects: HashMap<String, EffectState>,
    pub events: HashMap<String, EventState>,
    pub map_bounds: Bounds,
    pub round_end_time_ms: Option<u64>,
    pub round_over_end_time_ms: Option<u64>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            projectiles: HashMap::new(),
            active_effects: HashMap::new(),
            events: HashMap::new(),
            map_bounds: Bounds::new(0 - config::MAP_WIDTH / 2, 0 - config::MAP_HEIGHT / 2, config::MAP_WIDTH, config::MAP_HEIGHT),
            round_end_time_ms: None,
            round_over_end_time_ms: None,
        }
    }

    // --- Player Management ---

    pub fn add_player(&mut self, mut player: PlayerState) {
        let player_id = player.data().id.clone();
        player.mark_created(true);

        self.players.insert(player_id, player);
    }

    pub fn remove_player(&mut self, id: &str) {
        if let Some(player) = self.players.get_mut(id) {
            player.mark_destroyed();
        }
    }

    // --- Projectile Management ---

    pub fn add_projectile(&mut self, mut projectile: ProjectileState) {
        let projectile_id = projectile.data().id.clone();
        projectile.mark_created(true);

        self.projectiles.insert(projectile_id, projectile);
    }

    pub fn remove_projectile(&mut self, id: &str) {
        if let Some(projectile) = self.projectiles.get_mut(id) {
            projectile.mark_destroyed();
        }
    }

    // --- Effect Management ---

    pub fn add_effect(&mut self, mut effect: EffectState) {
        let effect_id = effect.data().id.clone();
        effect.mark_created(true);

        self.active_effects.insert(effect_id, effect);
    }

    pub fn remove_effect(&mut self, id: &str) {
        if let Some(effect) = self.active_effects.get_mut(id) {
            effect.mark_destroyed();
        }
    }

    // --- Event Management ---

    pub fn add_event(&mut self, event: EventState) {
        let event_id = event.data().id.clone();
        self.events.insert(event_id, event);
    }

    pub fn remove_event(&mut self, id: &str) {
        if let Some(event) = self.events.get_mut(id) {
            event.mark_destroyed();
        }
    }

    // --- Entity Tracking ---

    pub fn cleanup_destroyed_entities(&mut self, wait_for_death_animation: bool) {
        self.cleanup_destroyed_players(wait_for_death_animation);
        self.cleanup_destroyed_projectiles(wait_for_death_animation);
        self.cleanup_destroyed_effects(wait_for_death_animation);
    }

    pub fn cleanup_destroyed_players(&mut self, wait_for_death_animation: bool) {
        self.players.retain(|_, player| {
            let destroyed = player.just_destroyed();
            let death_anim_done = player.is_death_animation_complete();

            let can_remove = destroyed && (!wait_for_death_animation || death_anim_done);
            !can_remove
        });
    }

    pub fn cleanup_destroyed_projectiles(&mut self, wait_for_death_animation: bool) {
        self.projectiles.retain(|_, projectile| {
            let destroyed = projectile.just_destroyed();
            let death_anim_done = projectile.is_death_animation_complete();

            let can_remove = destroyed && (!wait_for_death_animation || death_anim_done);
            !can_remove
        });
    }

    pub fn cleanup_destroyed_effects(&mut self, wait_for_death_animation: bool) {
        self.active_effects.retain(|_, effect| {
            let destroyed = effect.just_destroyed();
            let death_anim_done = effect.is_death_animation_complete();

            let can_remove = destroyed && (!wait_for_death_animation || death_anim_done);
            !can_remove
        });
    }

    pub fn clear_entity_dirty_flags(&mut self) {
        self.clear_player_dirty_flags();
        self.clear_projectile_dirty_flags();
        self.clear_effect_dirty_flags();
    }

    pub fn clear_player_dirty_flags(&mut self) {
        for player in self.players.values_mut() {
            player.clear_all();
        }
    }

    pub fn clear_projectile_dirty_flags(&mut self) {
        for projectile in self.projectiles.values_mut() {
            projectile.clear_all();
        }
    }

    pub fn clear_effect_dirty_flags(&mut self) {
        for effect in self.active_effects.values_mut() {
            effect.clear_all();
        }
    }
}
