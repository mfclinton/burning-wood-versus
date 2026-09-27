use crate::{common::Sprite, sprites};

// --- Server Configuration ---
pub const SERVER_ROUND_TIME_MS: u64 = 5 * 60 * 1000; // 5 minutes
pub const SERVER_ROUND_OVER_DURATION_MS: u64 = 20 * 1000; // 20 seconds

// Network Rates
pub const SERVER_INTERVAL_RATE_MS: u32 = 50;

pub const SERVER_SIMULATION_RATE_MS: u32 = 50;
pub const SERVER_PLAYER_NETWORK_RATE_MS: u32 = 50;
pub const SERVER_PROJECTILE_NETWORK_RATE_MS: u32 = 150;
pub const SERVER_EFFECT_NETWORK_RATE_MS: u32 = 100;

// --- Client Configuration ---
pub const CLIENT_INPUT_BATCH_RATE_MS: u64 = 50;
pub const PLAYER_JOIN_REQUEST_INTERVAL_MS: u64 = 500;

pub const CLIENT_PREDICTION_LERP_SPEED: f32 = 0.15;

// --- Player Configuration ---
pub const PLAYER_NUM_HELD_ITEMS: usize = 2;

pub const PLAYER_INITIAL_HEALTH: i32 = 80;
pub const PLAYER_MAX_HEALTH: i32 = 2000;

pub const PLAYER_MIN_RADIUS: f32 = 8.0;
pub const PLAYER_MAX_RADIUS: f32 = 50.0;

pub const PLAYER_MIN_ACCELERATION: f32 = 1.0;
pub const PLAYER_MAX_ACCELERATION: f32 = 1.5;

pub const PLAYER_MIN_DAMPING: f32 = 0.80;
pub const PLAYER_MAX_DAMPING: f32 = 0.70;

pub const PLAYER_MIN_VELOCITY_THRESHOLD: f32 = 0.1;

// --- Projectile Configuration ---
pub const PROJECTILE_MIN_RADIUS: f32 = 5.0;
pub const PROJECTILE_MAX_RADIUS: f32 = 15.0;

// --- Map Configuration ---
pub const MAP_WIDTH: i32 = 128 * 10;
pub const MAP_HEIGHT: i32 = 128 * 10;

// --- Audio Configuration ---
pub const AUDIO_MAX_DISTANCE_RADIUS_MULTIPLIER: f32 = 3.0; // Audio max distance = player radius * this multiplier
pub const AUDIO_MIN_VOLUME: f32 = 0.0;
pub const AUDIO_MAX_VOLUME: f32 = 0.0001;

// --- Game Screen Configuration ---
pub const ITEM_SLOT_SPRITES: [Sprite; PLAYER_NUM_HELD_ITEMS] = [
    sprites::UI_GAMESCREEN_KEYQ,
    sprites::UI_GAMESCREEN_KEYE,
];
