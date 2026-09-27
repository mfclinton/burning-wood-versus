use turbo::*;

use crate::{Vec2, EffectSource, EffectType, CircleCollider, entities::{BaseEntityDirtyState, EntityDirtyState, EntityState}, ServerEvent};

// --- Data ---

#[turbo::serialize]
pub struct EffectData {
    pub id: String,
    pub source: EffectSource,
    pub effect_type: EffectType,
    pub collider: CircleCollider,
    pub start_tick: u64,
}

// --- Dirty ---

#[derive(Default)]
#[turbo::serialize]
pub struct EffectDirty {
    pub base: BaseEntityDirtyState,
    pub effect_type: bool,
}

impl EntityDirtyState for EffectDirty {
    fn any(&self) -> bool {
        self.base.any() || self.effect_type
    }

    fn clear(&mut self) {
        self.base.clear();
        self.effect_type = false;
    }

    // Radius
    fn radius_dirty(&self) -> bool {
        self.base.radius_dirty()
    }

    fn set_radius_dirty(&mut self, value: bool) {
        self.base.set_radius_dirty(value);
    }

    // Position
    fn position_dirty(&self) -> bool {
        self.base.position_dirty()
    }

    fn set_position_dirty(&mut self, value: bool) {
        self.base.set_position_dirty(value);
    }

    // Velocity
    fn velocity_dirty(&self) -> bool {
        self.base.velocity_dirty()
    }

    fn set_velocity_dirty(&mut self, value: bool) {
        self.base.set_velocity_dirty(value);
    }

    // Lifecycle
    fn just_created(&self) -> bool {
        self.base.just_created()
    }
    
    fn just_destroyed(&self) -> bool {
        self.base.just_destroyed()
    }
    
    fn set_just_created(&mut self, value: bool) {
        self.base.set_just_created(value);
    }
    
    fn set_just_destroyed(&mut self, value: bool) {
        self.base.set_just_destroyed(value);
    }
}

impl EffectDirty {
    pub fn new() -> Self {
        Self {
            base: BaseEntityDirtyState::new(),
            ..Default::default()
        }
    }
}

// --- State ---

#[turbo::serialize]
pub struct EffectState {
    data: EffectData,
    dirty: EffectDirty,
}

impl EntityState for EffectState {
    type Data = EffectData;
    type Dirty = EffectDirty;

    // Core
    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }

    fn dirty(&self) -> &Self::Dirty {
        &self.dirty
    }

    fn dirty_mut(&mut self) -> &mut Self::Dirty {
        &mut self.dirty
    }

    // Common
    fn get_id(&self) -> String {
        self.data.id.clone()
    }

    // Collider
    fn collider(&self) -> &CircleCollider {
        &self.data.collider
    }

    fn collider_mut(&mut self) -> &mut CircleCollider {
        &mut self.data.collider
    }
    
    // Networking
    fn generate_events(&self, events: &mut Vec<ServerEvent>) {
        let id = self.get_id();
        
        if self.just_created() {
            events.push(ServerEvent::EffectSpawned {
                effect_id: id.clone(),
                effect_state: self.clone(),
            });
        }

        if self.just_destroyed() {
            events.push(ServerEvent::EffectDeleted {
                effect_id: id.clone(),
            });
        }

        if self.dirty.base.position {
            events.push(ServerEvent::EffectMoved {
                effect_id: id.clone(),
                position: self.data.collider.position,
            });
        }

        if self.dirty.base.radius {
            events.push(ServerEvent::EffectRadiusChanged {
                effect_id: id.clone(),
                radius: self.data.collider.radius,
            });
        }

        if self.dirty.effect_type {
            events.push(ServerEvent::EffectTypeChanged {
                effect_id: id.clone(),
                effect_type: self.data.effect_type.clone(),
            });
        }
    }

    // Dirty Helper
    fn _copy_if_dirty_custom(&mut self, source: &Self) {
        let source_dirty = source.dirty();

        if source_dirty.effect_type {
            self.set_type(source.data().effect_type.clone());
        }
    }
}

impl EffectState {
    pub fn new(
        id: String,
        source: EffectSource,
        effect_type: EffectType,
        start_tick: u64,
    ) -> Self {
        Self {
            data: EffectData {
                id,
                source,
                effect_type,
                collider: CircleCollider {
                    radius: 10.0,
                    ..Default::default()
                },
                start_tick,
            },
            dirty: EffectDirty::new(),
        }
    }

    // Type
    pub fn set_type(&mut self, new_type: EffectType) {
        if self.data.effect_type != new_type {
            self.data.effect_type = new_type;
            self.dirty.effect_type = true;
        }
    }

    pub fn lerp_effect_to_source(&mut self, other: &EffectState, lerp_speed: f32) {
        let self_data = self.data();
        let other_data = other.data();
        
        let current_position = self_data.collider.position;

        // Lerp Position
        let new_position = current_position.lerp(&other_data.collider.position, lerp_speed);
        self.set_position(new_position);
    }
}
