use crate::{EffectType, weighted_random_sample, ProjectileType};

// --- Item Types ---

#[derive(PartialEq)]
#[turbo::serialize]
pub enum ItemType {
    WaterGun,
    Blackhole,
    Lightning,
    SmallGas,
    MediumGas,
    LargeGas,
    Bat,
    Mine,
    MetalWaterGun,
    WaterBomb,
    Wind
}

// --- Item Phases ---

#[derive(PartialEq)]
#[turbo::serialize]
pub enum ItemPhase {
    Preparing,
    Ready,
    Active, 
    Cooldown,
}

// --- Item Rarity ---

#[derive(PartialEq)]
#[turbo::serialize]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic
}

impl ItemRarity {
    pub fn weight(&self) -> f32 {
        match self {
            ItemRarity::Common => 50.0,
            ItemRarity::Uncommon => 30.0,
            ItemRarity::Rare => 15.0,
            ItemRarity::Epic => 5.0,
        }
    }
}

// --- Item Definition ---

pub struct ItemDefinition {
    pub effect_type: EffectType,
    pub max_charges: u32,
    pub preparation_time: Option<u32>,
    pub cooldown_time: Option<u32>,
    pub rarity: ItemRarity,
    pub bound_to_effect_lifetime: bool,
}

impl ItemDefinition {    
    // Helpers
    pub fn active_duration(&self) -> Option<u64> {
        if !self.bound_to_effect_lifetime || self.effect_type.is_instant() {
            None
        }
        else {
            Some(self.effect_type.duration())
        }
    }

    pub fn weighted_random_item(items: &[ItemType]) -> ItemType {
        weighted_random_sample(items, |item| ItemDefinition::get_item_definition(item).rarity.weight())
            .unwrap_or(items[0].clone())
    }
}
