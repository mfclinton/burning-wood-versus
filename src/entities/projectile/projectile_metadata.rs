use crate::{PlayerState, ItemType, ItemState, ItemDefinition, weighted_random_sample};

// --- Projectile Types ---

#[derive(PartialEq, Copy)]
#[turbo::serialize]
pub enum ProjectileType {
    Water,
    Wood,
    Coal,
    Chest
}

impl ProjectileType {
    pub fn random() -> Self {
        let r = turbo::random::f32();
        if r > 0.80 {
            ProjectileType::Chest
        }
        else if r > 0.60 {
            ProjectileType::Coal
        }
        else if r > 0.33 {
            ProjectileType::Water
        }
        else {
            ProjectileType::Wood
        }
    }

    pub fn random_payload(&self) -> ProjectilePayload {
        match self {
            ProjectileType::Wood => {
                let value = 10;
                ProjectilePayload::Health(value)
            }
            ProjectileType::Coal => {
                let value = 20;
                ProjectilePayload::Health(value)
            }
            ProjectileType::Water => {
                let health = -10;
                ProjectilePayload::Health(health)
            }
            ProjectileType::Chest => {
                let items = [ItemType::WaterGun, ItemType::Blackhole, ItemType::Lightning, ItemType::SmallGas,
                                            ItemType::MediumGas, ItemType::LargeGas, ItemType::Bat, ItemType::Mine,
                                            ItemType::MetalWaterGun, ItemType::WaterBomb, ItemType::Wind];
                // Weighted random sample based on rarity
                let item = ItemDefinition::weighted_random_item(&items);
                ProjectilePayload::Item(item)
            }
        }
    }
}

// --- Projectile Payload ---

#[derive(PartialEq)]
#[turbo::serialize]
pub enum ProjectilePayload {
    Health(i32),
    Item(ItemType),
}

impl ProjectilePayload {
    pub fn apply_to_player(&self, player: &mut PlayerState) -> bool {
        match self {
            ProjectilePayload::Health(amount) => {
                player.modify_health(*amount);
            }
            ProjectilePayload::Item(item) => {
                player.try_add_held_item(ItemState::new(item.clone()));
            }
        }

        true
    }

    pub fn get_radius_t(&self) -> f32 {
        match self {
            ProjectilePayload::Health(value) => value.abs() as f32 / 20.0,
            ProjectilePayload::Item(_) => 0.5,
        }
    }
}
