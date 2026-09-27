use crate::{
    Vec2, 
    ItemType
};

pub enum Collision {
    PlayerToPlayer {
        player1_id: String,
        player2_id: String,
    },
    PlayerToProjectile {
        player_id: String,
        projectile_id: String,
    },
}
