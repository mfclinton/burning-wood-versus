use turbo::*;
use std::collections::HashMap;

use crate::{
    common::circle_circle_collision, create_uuid, entities::{EffectState, EntityState, PlayerData, PlayerState, ProjectilePayload, ProjectileType}, CircleCollider, EffectData, EffectSource, EffectType, GameState, ProjectileState, Vec2
};

// --- Effect Context ---

pub struct EffectContext<'a> {
    effect_id: String,
    cached_effect_data: EffectData,
    players: &'a mut HashMap<String, PlayerState>,
    projectiles: &'a mut HashMap<String, ProjectileState>,
    active_effects: &'a mut HashMap<String, EffectState>,
}

impl EffectContext<'_> {
    pub fn from_game_state<'a>(
        game_state: &'a mut GameState,
        effect_id: String,
    ) -> Option<EffectContext<'a>> {
        let effect_data = {
            let effect = game_state.active_effects.get(&effect_id)?;
            effect.data().clone()
        };
        Some(EffectContext {
            effect_id,
            cached_effect_data: effect_data,
            players: &mut game_state.players,
            projectiles: &mut game_state.projectiles,
            active_effects: &mut game_state.active_effects,
        })
    }

    // Modifiers
    pub fn refresh_effect_data(&mut self) {
        if let Some(effect) = self.get_source_effect_mut() {
            self.cached_effect_data = effect.data().clone();
        }
    }

    // Accessors
    pub fn get_source_effect(&self) -> Option<&EffectState> {
        self.active_effects.get(&self.effect_id)
    }

    pub fn get_source_effect_mut(&mut self) -> Option<&mut EffectState> {
        self.active_effects.get_mut(&self.effect_id)
    }

    pub fn get_source_player(&self) -> Option<&PlayerState> {
        match &self.cached_effect_data.source {
            EffectSource::Player { player_id, .. } => self.players.get(player_id),
            _ => None,
        }
    }

    pub fn get_source_player_mut(&mut self) -> Option<&mut PlayerState> {
        match &self.cached_effect_data.source {
            EffectSource::Player { player_id, .. } => self.players.get_mut(player_id),
            _ => None,
        }
    }

    // Helpers
    pub fn should_follow_player(&self) -> bool {
        match &self.cached_effect_data.source {
            EffectSource::Player { follows_player, .. } => *follows_player,
            _ => false,
        }
    }
}

// --- Simulation Functions ---

pub fn apply_effects(game_state: &mut GameState, tick: u64) {
    let effect_ids: Vec<String> = game_state.active_effects.keys().cloned().collect();
    
    for effect_id in effect_ids {
        apply_effect(game_state, tick, &effect_id);
    }

    clear_expired_effects(game_state, tick);
}

pub fn clear_expired_effects(game_state: &mut GameState, tick: u64) {
    let mut expired_effects = Vec::new();
    for (effect_id, effect) in game_state.active_effects.iter() {
        let effect_data = effect.data();
        let elapsed_ticks = tick - effect_data.start_tick;
        
        if effect_data.effect_type.is_expired(elapsed_ticks) {
            expired_effects.push(effect_id.clone());
        }
    }
    
    for effect_id in expired_effects {
        game_state.remove_effect(&effect_id);
    }
}

// --- Effect Functions ---

fn apply_effect(game_state: &mut GameState, tick: u64, effect_id: &str) -> Option<()> {
    let mut ctx = EffectContext::from_game_state(game_state, effect_id.to_string())?;

    let effect = ctx.cached_effect_data.effect_type.clone();
    
    update_effect_collider(&mut ctx);
    match &effect {
        EffectType::HealthModifier { health_modifier } => {
            apply_health_modifier(&mut ctx, *health_modifier);
        },
        EffectType::ProjectileSpawner { projectile_type, damage, speed_multiplier, radius_multiplier, auto_target, .. } => {
            apply_projectile_spawner(&mut ctx, *projectile_type, *damage, *speed_multiplier, *radius_multiplier, *auto_target);
        },
        EffectType::MagnetField { target_projectile_types, pull_force, affects_players, .. } => {
            apply_magnet_field(&mut ctx, target_projectile_types.clone(), *pull_force, *affects_players);
        },
        EffectType::DamageField { target_projectile_types, .. } => {
            apply_damage_field(&mut ctx, target_projectile_types.clone());
        },
        EffectType::HealthField { health_modifier, .. } => {
            apply_health_field(&mut ctx, *health_modifier);
        },
        EffectType::LifeDrainField { health_transfer, .. } => {
            apply_life_drain_field(&mut ctx, *health_transfer);
        },
        EffectType::DelayedMultiProjectileSpawner { projectile_type, damage, speed_multiplier, projectile_count, delay_ticks, .. } => {
            apply_delayed_multi_projectile_spawner(&mut ctx, tick, *projectile_type, *damage, *speed_multiplier, *projectile_count, *delay_ticks);
        },
    }

    Some(())
}

fn apply_health_modifier(ctx: &mut EffectContext, health_modifier: i32) {
    if let Some(player) = ctx.get_source_player_mut() {
        player.modify_health(health_modifier);
    }
}

// TODO
fn apply_projectile_spawner(ctx: &mut EffectContext, projectile_type: ProjectileType, damage: i32, speed_multiplier: f32, radius_multiplier: f32, auto_target: bool) {
    let Some(player) = ctx.get_source_player() else { return };
    
    let player_data = player.data();
    let start_tick = ctx.cached_effect_data.start_tick;
    
    // Calculate Direction and Position
    let direction = if auto_target {
        calculate_auto_target_direction(ctx, player_data, radius_multiplier)
    } else {
        let velocity_dir = player_data.collider.velocity.normalized();
        if velocity_dir == Vec2::zero() { Vec2::random() } else { velocity_dir }
    };

    // Create Projectile
    let projectile_id = create_uuid(start_tick);
    let mut projectile = ProjectileState::new(
        projectile_id.clone(),
        Vec2::default(),
        projectile_type,
        ProjectilePayload::Health(-damage),
        start_tick,
        100,
    );
    
    // Set Position and Speed
    let position = player_data.collider.position + direction * (player_data.collider.radius + projectile.data().collider.radius * 1.25 * radius_multiplier);
    projectile.set_position(position);

    // TODO: Min speed
    let projectile_velocity = direction * player_data.collider.velocity.length().max(4.0) * speed_multiplier;
    projectile.set_velocity(projectile_velocity);

    ctx.projectiles.insert(projectile_id, projectile);
}


fn apply_magnet_field(ctx: &mut EffectContext, target_projectile_types: Vec<ProjectileType>, pull_force: f32, affects_players: bool) {
    if target_projectile_types.is_empty() && !affects_players {
        return;
    }

    let effect_collider = &ctx.cached_effect_data.collider;
    let source_player_id = match &ctx.cached_effect_data.source {
        EffectSource::Player { player_id, .. } => player_id.clone(),
        _ => return,
    };

    for projectile in ctx.projectiles.values_mut() {
        if projectile.just_destroyed() { continue; }
        let projectile_data = projectile.data();

        if !target_projectile_types.contains(&projectile_data.projectile_type) {
            continue;
        }

        if let Some(force_vector) = calculate_force_vector(effect_collider, &projectile_data.collider, pull_force) {
            projectile.modify_velocity(force_vector);
        }
    }

    if affects_players {
        for player in ctx.players.values_mut() {
            if player.just_destroyed() { continue; }
            let player_data = player.data();
            if source_player_id == player_data.id {
                continue;
            }

            let distance = player_data.collider.position.distance_to(&effect_collider.position);
            if distance <= effect_collider.radius && distance > 0.1 {
                if let Some(force_vector) = calculate_force_vector(effect_collider, &player_data.collider, pull_force) {
                    player.modify_velocity(force_vector);
                }
            }
        }
    }
}


fn apply_damage_field(ctx: &mut EffectContext, target_projectile_types: Vec<ProjectileType>) {
    let effect_collider = &ctx.cached_effect_data.collider;

    for projectile in ctx.projectiles.values_mut() {
        if projectile.just_destroyed() { continue; }
        let projectile_data = projectile.data();

        if !target_projectile_types.contains(&projectile_data.projectile_type) {
            continue;
        }
        
        if circle_circle_collision(effect_collider, &projectile_data.collider) {
            projectile.mark_destroyed();
        }
    }
}

fn apply_health_field(ctx: &mut EffectContext, health_modifier: i32) {
    let effect_collider = &ctx.cached_effect_data.collider;

    for player in ctx.players.values_mut() {
        if player.just_destroyed() { continue; }
        let player_data = player.data();

        if circle_circle_collision(effect_collider, &player_data.collider) {
            player.modify_health(health_modifier);
        }
    }
}


fn apply_life_drain_field(ctx: &mut EffectContext, health_transfer: i32) {
    let effect_collider = &ctx.cached_effect_data.collider;
    let source_player_id = match &ctx.cached_effect_data.source {
        EffectSource::Player { player_id, .. } => player_id.clone(),
        _ => return,
    };

    let mut total_drained = 0;
    for (player_id, player) in ctx.players.iter_mut() {
        if player.just_destroyed() { continue; }
        if player_id == &source_player_id {
            continue;
        }
        
        let player_data = player.data();
        if circle_circle_collision(effect_collider, &player_data.collider) {
            player.modify_health(-health_transfer);
            total_drained += health_transfer;
        }
    }
    
    if total_drained > 0 {
        if let Some(source_player) = ctx.players.get_mut(&source_player_id) {
            source_player.modify_health(total_drained);
        }
    }
}

fn apply_delayed_multi_projectile_spawner(ctx: &mut EffectContext, current_tick: u64, projectile_type: ProjectileType, damage: i32, speed_multiplier: f32, projectile_count: u32, delay_ticks: u64) {
    let start_tick = ctx.cached_effect_data.start_tick;
    
    let elapsed_ticks = current_tick - start_tick;
    if elapsed_ticks != delay_ticks {
        return;
    }

    let effect_position = ctx.cached_effect_data.collider.position;
    
    let angle_step = 2.0 * std::f32::consts::PI / projectile_count as f32;
    let random_offset = random::f32() * 2.0 * std::f32::consts::PI;
    for i in 0..projectile_count {
        let angle = i as f32 * angle_step + random_offset;
        let direction = Vec2::new(angle.cos(), angle.sin());
        
        let projectile_id = create_uuid(start_tick + i as u64);
        let projectile_payload = ProjectilePayload::Health(-damage);
        let projectile_position = effect_position;
        let projectile_lifetime = 120;

        let mut projectile = ProjectileState::new(
            projectile_id.clone(),
            projectile_position,
            projectile_type,
            projectile_payload,
            current_tick,
            projectile_lifetime,
        );
        projectile.set_velocity(direction * speed_multiplier);

        ctx.projectiles.insert(projectile_id.clone(), projectile);
    }
}

// --- Auto-Targeting Helper ---

fn calculate_auto_target_direction(ctx: &EffectContext, source_player_data: &PlayerData, radius_multiplier: f32) -> Vec2 {
    let mut target_direction = Vec2::random().normalized();
    
    let nearest_player_opt = get_nearest_player(ctx, source_player_data.collider.position, f32::INFINITY, Some(&source_player_data.id));
    if let Some(target_player_data) = nearest_player_opt {
        target_direction = (target_player_data.collider.position - source_player_data.collider.position).normalized();
    }
    
    target_direction
}

// --- Force Calculation Helper ---

fn calculate_force_vector(effect_collider: &CircleCollider, target_collider: &CircleCollider, pull_force: f32) -> Option<Vec2> {
    let distance = effect_collider.position.distance_to(&target_collider.position);
    if distance > effect_collider.radius {
        return None;
    }

    let direction = if pull_force > 0.0 {
        (effect_collider.position - target_collider.position).normalized()
    } else {
        (target_collider.position - effect_collider.position).normalized()
    };
    
    let force_strength = (1.0 - distance / effect_collider.radius) * pull_force.abs();
    Some(direction * force_strength)
}

// --- Helpers ---

fn get_effect_radius(effect_type: &EffectType, player_radius: f32, radius_multiplier: f32) -> f32 {
    let max_player_size = 20.0; // Assuming max player radius
    
    match effect_type {
        EffectType::MagnetField { .. } => {
            let base_size = 60.0;
            base_size + (player_radius / max_player_size) * radius_multiplier * 20.0
        },
        EffectType::LifeDrainField { .. } => {
            let base_size = 40.0;
            base_size + (player_radius / max_player_size) * radius_multiplier * 15.0
        },
        EffectType::HealthField { .. } => {
            let base_size = 35.0;
            base_size + (player_radius / max_player_size) * radius_multiplier * 15.0
        },
        EffectType::DamageField { .. } => {
            let base_size = 30.0;
            base_size + (player_radius / max_player_size) * radius_multiplier * 20.0
        },
        _ => player_radius * radius_multiplier, // Keep relative for other effects
    }
}

fn get_nearest_player(ctx: &EffectContext, center_position: Vec2, scan_radius: f32, exclude_player_id: Option<&str>) -> Option<PlayerData> {
    let mut closest_player_data: Option<PlayerData> = None;
    let mut closest_distance = scan_radius;
    for (player_id, player) in ctx.players.iter() {
        if player.just_destroyed() { continue; }
        // Excluded
        if let Some(exclude_id) = exclude_player_id {
            if player_id == exclude_id {
                continue;
            }
        }
        
        // Is Closer
        let player_data = player.data();
        let distance = center_position.distance_to(&player_data.collider.position);
        if distance < closest_distance {
            closest_distance = distance;
            closest_player_data = Some(player_data.clone());
        }
    }
    
    closest_player_data
}

fn update_effect_collider(ctx: &mut EffectContext) {
    if let Some(player) = ctx.get_source_player() {
        // Data
        let player_data = player.data();

        let new_radius = ctx.cached_effect_data.effect_type.get_effect_radius(player_data.collider.radius);
        let new_position = if ctx.should_follow_player() {
            player_data.collider.position
        } else {
            ctx.cached_effect_data.collider.position
        };

        if let Some(effect) = ctx.get_source_effect_mut() {
            effect.set_radius(new_radius);
            effect.set_position(new_position);

            ctx.refresh_effect_data();
        }
    }
}
