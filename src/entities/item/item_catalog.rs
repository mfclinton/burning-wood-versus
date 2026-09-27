use crate::{
    ItemDefinition, ItemType, ItemRarity, EffectType, ProjectileType,
};

impl ItemDefinition {    
    pub fn get_item_definition(item_type: &ItemType) -> Self {
        match item_type {
            ItemType::Blackhole => Self {
                effect_type: EffectType::MagnetField { 
                    target_projectile_types: vec![ProjectileType::Wood, ProjectileType::Coal],
                    pull_force: 40.0,
                    radius_multiplier: 5.0, 
                    duration: 100,
                    follows_player: true,
                    affects_players: false,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: true,
            },
            ItemType::Lightning => Self {
                effect_type: EffectType::DamageField { 
                    target_projectile_types: vec![ProjectileType::Water],
                    radius_multiplier: 4.0, 
                    duration: 50,
                    follows_player: true,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: true,
            },
            ItemType::SmallGas => Self {
                effect_type: EffectType::HealthModifier { 
                    health_modifier: 25,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::MediumGas => Self {
                effect_type: EffectType::HealthModifier { 
                    health_modifier: 50,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::LargeGas => Self {
                effect_type: EffectType::HealthModifier { 
                    health_modifier: 100,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::WaterGun => Self {
                effect_type: EffectType::ProjectileSpawner { 
                    projectile_type: ProjectileType::Water,
                    damage: 40,
                    speed_multiplier: 2.0,
                    radius_multiplier: 2.0,
                    follows_player: false,
                    auto_target: false,
                },
                max_charges: 8,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::Bat => Self {
                effect_type: EffectType::LifeDrainField { 
                    health_transfer: 2,
                    radius_multiplier: 3.0,
                    duration: 120,
                    follows_player: true,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: true,
            },
            ItemType::Mine => Self {
                effect_type: EffectType::HealthField { 
                    health_modifier: 1,
                    radius_multiplier: 2.0,
                    duration: 200,
                    follows_player: false,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::MetalWaterGun => Self {
                effect_type: EffectType::ProjectileSpawner { 
                    projectile_type: ProjectileType::Water,
                    damage: 40,
                    speed_multiplier: 2.5,
                    radius_multiplier: 2.0,
                    follows_player: false,
                    auto_target: true,
                },
                max_charges: 5,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::WaterBomb => Self {
                effect_type: EffectType::DelayedMultiProjectileSpawner { 
                    projectile_type: ProjectileType::Water,
                    damage: 40,
                    speed_multiplier: 1.8,
                    projectile_count: 3,
                    delay_ticks: 30,
                    follows_player: false,
                },
                max_charges: 3,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: false,
            },
            ItemType::Wind => Self {
                effect_type: EffectType::MagnetField { 
                    target_projectile_types: vec![ProjectileType::Water],
                    pull_force: -50.0,
                    radius_multiplier: 4.0,
                    duration: 120,
                    follows_player: true,
                    affects_players: true,
                },
                max_charges: 1,
                preparation_time: None,
                cooldown_time: None,
                rarity: ItemRarity::Common,
                bound_to_effect_lifetime: true,
            },
        }
    }
}
