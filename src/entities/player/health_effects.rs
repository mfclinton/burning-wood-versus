use turbo::*;
use std::{collections::HashMap, sync::{LazyLock, Mutex, atomic::{AtomicU32, Ordering}}};

use crate::{common::{Anchor, Renderable, Vec2}, sprites, Sprite};

static EFFECT_COUNTER: AtomicU32 = AtomicU32::new(0);
static HEALTH_EFFECTS: LazyLock<Mutex<HashMap<String, Vec<HealthEffect>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static PREVIOUS_HEALTH: LazyLock<Mutex<HashMap<String, i32>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy)]
enum HealthEffectType {
    Loss,
    Gain,
}

impl HealthEffectType {
    fn sprite(self) -> Sprite {
        match self {
            Self::Loss => sprites::FX_PLAYERHEALTH_LOSS,
            Self::Gain => sprites::FX_PLAYERHEALTH_GAIN,
        }
    }
}

pub struct HealthEffect {
    position: Vec2,
    sprite: Sprite,
    animation_key: String,
}

impl HealthEffect {
    pub fn new(effect_type: HealthEffectType, player_position: Vec2, player_id: &str) -> Self {
        let counter = EFFECT_COUNTER.fetch_add(1, Ordering::Relaxed);
        let type_name = match effect_type {
            HealthEffectType::Loss => "health_loss",
            HealthEffectType::Gain => "health_gain",
        };
        let animation_key = format!("{}_{}_{}", player_id, type_name, counter);
        let sprite = effect_type.sprite();

        let anim = animation::get(&animation_key);
        anim.use_sprite(sprite.name);
        anim.set_repeat(1);
        anim.set_speed(1.0);

        Self {
            position: player_position,
            sprite,
            animation_key,
        }
    }

    pub fn render(&self, player_radius: f32) {
        let mut sprite = self.sprite * 0.1;
        sprite *= player_radius;
        let sprite_bounds = sprite.get_bounds_anchored_at(self.position.into(), Anchor::Center);

        sprite!(
            animation_key = self.animation_key.as_str(),
            default_sprite = self.sprite.name,
            bounds = sprite_bounds
        );
    }
}

pub fn check_and_spawn_health_effects(player_id: &str, current_health: i32, player_position: Vec2) {
    let mut previous_health_map = PREVIOUS_HEALTH.lock().unwrap();
    let mut effects_map = HEALTH_EFFECTS.lock().unwrap();

    if let Some(&previous_health) = previous_health_map.get(player_id) {
        let health_delta = current_health - previous_health;

        if health_delta < -1 {
            let effects = effects_map.entry(player_id.to_string()).or_insert_with(Vec::new);
            effects.push(HealthEffect::new(HealthEffectType::Loss, player_position, player_id));
        } else if health_delta > 0 {
            let effects = effects_map.entry(player_id.to_string()).or_insert_with(Vec::new);
            effects.push(HealthEffect::new(HealthEffectType::Gain, player_position, player_id));
        }
    }

    previous_health_map.insert(player_id.to_string(), current_health);
}

pub fn render_health_effects(player_id: &str, player_radius: f32) {
    let mut effects_map = HEALTH_EFFECTS.lock().unwrap();

    if let Some(effects) = effects_map.get_mut(player_id) {
        effects.retain(|effect| {
            let anim = animation::get(&effect.animation_key);
            if anim.done() {
                false
            } else {
                effect.render(player_radius);
                true
            }
        });
    }
}