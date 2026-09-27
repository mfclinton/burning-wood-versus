use turbo::*;

use crate::{entities::{BaseEntityDirtyState, EntityDirtyState, EntityState}, CircleCollider, EventType, ServerEvent};

// --- Data ---

#[turbo::serialize]
pub struct EventData {
    pub id: String,
    pub event_type: EventType,
    pub collider: CircleCollider,
    pub start_tick: u64,
    pub duration: u64,
}

// --- Dirty ---

#[derive(Default)]
#[turbo::serialize]
pub struct EventDirty {
    pub base: BaseEntityDirtyState,
}

impl EntityDirtyState for EventDirty {
    fn any(&self) -> bool {
        self.base.any()
    }

    fn clear(&mut self) {
        self.base.clear();
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

impl EventDirty {
    pub fn new() -> Self {
        Self::default()
    }
}

// --- State ---

#[turbo::serialize]
pub struct EventState {
    data: EventData,
    dirty: EventDirty,
}

impl EntityState for EventState {
    type Data = EventData;
    type Dirty = EventDirty;

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
            events.push(ServerEvent::EventSpawned {
                event_id: id.clone(),
                event_data: self.clone(),
            });
        }

        if self.just_destroyed() {
            events.push(ServerEvent::EventDeleted {
                event_id: id.clone(),
            });
        }
    }

    // Dirty Helper
    fn _copy_if_dirty_custom(&mut self, source: &Self) {
        let source_dirty = source.dirty();
    }
}

impl EventState {
    pub fn new(
        id: String,
        event_type: EventType,
        start_tick: u64,
        duration: u64,
    ) -> Self {
        Self {
            data: EventData {
                id,
                event_type,
                collider: CircleCollider::default(),
                start_tick,
                duration,
            },
            dirty: EventDirty::new(),
        }
    }
}



