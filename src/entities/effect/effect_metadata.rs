use turbo::*;

use crate::{config, create_uuid, EffectState, ProjectileType};

// --- Effect Source ---

#[turbo::serialize]
pub enum EffectSource {
    Player {
        player_id: String,
        follows_player: bool,
    },
    Event {
        event_id: String,
    },
}

// --- Effect Types ---

#[derive(PartialEq)]
#[turbo::serialize]
pub enum EffectType {
    HealthModifier {
        health_modifier: i32,
    },
    ProjectileSpawner {
        projectile_type: ProjectileType,
        damage: i32,
        speed_multiplier: f32,
        radius_multiplier: f32,
        follows_player: bool,
        auto_target: bool,
    },
    MagnetField {
        target_projectile_types: Vec<ProjectileType>,
        pull_force: f32,
        radius_multiplier: f32,
        duration: u64,
        follows_player: bool,
        affects_players: bool,
    },
    DamageField {
        target_projectile_types: Vec<ProjectileType>,
        radius_multiplier: f32,
        duration: u64,
        follows_player: bool,
    },
    HealthField {
        health_modifier: i32,
        radius_multiplier: f32,
        duration: u64,
        follows_player: bool,
    },
    LifeDrainField {
        health_transfer: i32,
        radius_multiplier: f32,
        duration: u64,
        follows_player: bool,
    },
    DelayedMultiProjectileSpawner {
        projectile_type: ProjectileType,
        damage: i32,
        speed_multiplier: f32,
        projectile_count: u32,
        delay_ticks: u64,
        follows_player: bool,
    },
}

impl EffectType {
    pub fn duration(&self) -> u64 {
        match self {
            EffectType::HealthModifier { .. } => 0,
            EffectType::ProjectileSpawner { .. } => 0,
            EffectType::MagnetField { duration, .. } => *duration,
            EffectType::DamageField { duration, .. } => *duration,
            EffectType::HealthField { duration, .. } => *duration,
            EffectType::LifeDrainField { duration, .. } => *duration,
            EffectType::DelayedMultiProjectileSpawner { delay_ticks, .. } => *delay_ticks,
        }
    }

    pub fn follows_player(&self) -> bool {
        match self {
            EffectType::HealthModifier { .. } => false,
            EffectType::ProjectileSpawner { follows_player, .. } => *follows_player,
            EffectType::MagnetField { follows_player, .. } => *follows_player,
            EffectType::DamageField { follows_player, .. } => *follows_player,
            EffectType::HealthField { follows_player, .. } => *follows_player,
            EffectType::LifeDrainField { follows_player, .. } => *follows_player,
            EffectType::DelayedMultiProjectileSpawner { follows_player, .. } => *follows_player,
        }
    }

    pub fn get_effect_radius(&self, player_radius: f32) -> f32 {        
        match self {
            EffectType::MagnetField { radius_multiplier, .. } => {
                let base_size = 60.0;
                base_size + (player_radius / config::PLAYER_MAX_RADIUS) * radius_multiplier * 20.0
            },
            EffectType::LifeDrainField { radius_multiplier, .. } => {
                let base_size = 40.0;
                base_size + (player_radius / config::PLAYER_MAX_RADIUS) * radius_multiplier * 15.0
            },
            EffectType::HealthField { radius_multiplier, .. } => {
                let base_size = 35.0;
                base_size + (player_radius / config::PLAYER_MAX_RADIUS) * radius_multiplier * 15.0
            },
            EffectType::DamageField { radius_multiplier, .. } => {
                let base_size = 30.0;
                base_size + (player_radius / config::PLAYER_MAX_RADIUS) * radius_multiplier * 20.0
            },
            _ => player_radius,
        }
    }

    // Helpers
    pub fn is_instant(&self) -> bool {
        self.duration() == 0
    }

    pub fn is_expired(&self, elapsed_ticks: u64) -> bool {
        self.duration() <= elapsed_ticks
    }
}
