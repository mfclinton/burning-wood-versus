use turbo::*;

use crate::{Vec2, ItemType};

#[turbo::serialize]
pub enum UserMessage {
    PlayerJoin { seq: u64, username: String },
    Move { seq: u64, direction: Vec2 },
    UseItem { seq: u64, item_index: usize },
    UseAnyItem { seq: u64 },

    // Entity Recovery
    RequestPlayerLookup { seq: u64, user_id: String },
    RequestProjectileLookup { seq: u64, projectile_id: String },
    RequestEffectLookup { seq: u64, effect_id: String },

    // Debug
    #[cfg(feature = "debug")]
    DebugGetItem { seq: u64, item_type: ItemType }
}
