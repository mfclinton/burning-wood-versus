use turbo::*;
use crate::{common::{inset_bounds_by_fraction, Anchor, Renderable, ScaleMode, Text, Vec2Int}, entities::{EntityState, ItemPhase, ItemState}, sprites, CircleCollider, ItemType, PlayerState, Sprite, Vec2, JERSEY_20};

impl ItemState {
    // Main Draw Functions
    pub fn draw_item_in_equipment_slot(&self, slot_bounds: Bounds) {
        let item_bounds = inset_bounds_by_fraction(&slot_bounds, 0.13);
        self.draw_item_with_bounds(item_bounds, true, true);
    }

    // Helpers
    pub fn get_sprite(&self) -> Sprite {
        match self.data().item_type {
            ItemType::Lightning => sprites::ITEM_SPRLIGHTNING_ICON,
            ItemType::WaterGun => sprites::ITEM_SPRWATERGUN_ICON,
            ItemType::Blackhole => sprites::ITEM_SPRBLACKHOLE_ICON,
            ItemType::SmallGas => sprites::ITEM_SPRGAS_SMALL_ICON,
            ItemType::MediumGas => sprites::ITEM_SPRGAS_MEDIUM_ICON,
            ItemType::LargeGas => sprites::ITEM_SPRGAS_LARGE_ICON,
            ItemType::Bat => sprites::ITEM_SPRBAT_ICON,
            ItemType::Mine => sprites::ITEM_SPRMINE_ICON,
            ItemType::MetalWaterGun => sprites::ITEM_SPRMETALWATERGUN_ICON,
            ItemType::WaterBomb => sprites::ITEM_SPRWATERBOMB_ICON,
            ItemType::Wind => sprites::ITEM_SPRWIND_ICON,
        }
    }

    pub fn get_color(&self) -> u32 {
        match self.data().phase {
            ItemPhase::Preparing { .. } => 0xFFFF00FF,
            ItemPhase::Ready { .. } => 0xFFFFFFFF,
            ItemPhase::Active { .. } => 0x00FF00FF,
            ItemPhase::Cooldown { .. } => 0xFF0000FF,
        }
    }

    pub fn draw_item_with_bounds(&self, sprite_bounds: Bounds, fixed: bool, show_charges: bool) {
        let item_data = self.data();
        let item_charges = &item_data.charges;

        let sprite = self.get_sprite();
        let color = self.get_color();

        // Draw Item Sprite
        sprite!(
            sprite.name,
            bounds = sprite_bounds,
            color = color,
            fixed = fixed
        );

        // Draw Charges
        if *item_charges > 1 && show_charges {
            // Charge String
            let charge_str = format!("{}", item_charges);
            
            // Charge Text
            let mut charge_text = Text::new(&charge_str, &JERSEY_20);
            charge_text = charge_text.with_scale(0.018, ScaleMode::HeightRelative);
            
            let text_pos = sprite_bounds.translate_by_fraction(-0.05, -0.05);
            let charge_text_bounds = charge_text.get_bounds_anchored_at(text_pos.bottom_right().into(), Anchor::BottomRight);
            
            charge_text.render(charge_text_bounds.xy().into(), 0xFFFFFFFF, fixed);
        }
    }
}
