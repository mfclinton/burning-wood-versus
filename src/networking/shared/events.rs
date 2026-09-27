use std::collections::HashMap;

use turbo::*;

use crate::{EventState, EffectType, EffectState, ItemType, PlayerState, ProjectilePayload, ProjectileState, Vec2, ItemState, ItemPhase};

#[turbo::serialize]
pub enum ServerEvent {
    // Misc
    EventBatch { events: Vec<ServerEvent> },

    // Players
    PlayerSpawned { user_id: String, player_state: PlayerState },
    PlayerDeleted { user_id: String },
    
    PlayerMoveStateChanged { user_id: String, position: Vec2, velocity: Vec2, last_processed_seq: u64 },
    PlayerHealthChanged { user_id: String, health: i32 },

    PlayerHeldItemChanged { user_id: String, item_index: usize, item_state: Option<ItemState> },
    PlayerHeldItemPhaseChanged { user_id: String, item_index: usize, phase: ItemPhase },
    PlayerHeldItemChargesChanged { user_id: String, item_index: usize, charges: u32 },
    PlayerHeldItemTimerChanged { user_id: String, item_index: usize, timer: u32 },

    // Projectiles
    ProjectileSpawned { projectile_id: String, projectile_state: ProjectileState },
    ProjectileDeleted { projectile_id: String },
    
    ProjectileMoved { projectile_id: String, position: Vec2 },
    ProjectileVelocityChanged { projectile_id: String, velocity: Vec2 },
    ProjectilePayloadChanged { projectile_id: String, payload: ProjectilePayload },

    // Effects
    EffectSpawned { effect_id: String, effect_state: EffectState },
    EffectDeleted { effect_id: String },

    EffectMoved { effect_id: String, position: Vec2 },
    EffectRadiusChanged { effect_id: String, radius: f32 },
    EffectTypeChanged { effect_id: String, effect_type: EffectType },

    // Events
    EventSpawned { event_id: String, event_data: EventState },
    EventDeleted { event_id: String },

    // Round Timing
    RoundStarted { round_end_time_ms: u64 },
    RoundEnded { round_over_end_time_ms: u64 },
    RoundOverEnded,

    // Entity Recovery Responses
    PlayerLookupResponse { user_id: String, player_state: Option<PlayerState> },
    ProjectileLookupResponse { projectile_id: String, projectile_state: Option<ProjectileState> },
    EffectLookupResponse { effect_id: String, effect_state: Option<EffectState> },
}
