use turbo::*;
use crate::{Vec2, ServerEvent, CircleCollider};

// --- Base Dirty State ---

#[derive(Default)]
#[turbo::serialize]
pub struct BaseEntityDirtyState {
    pub radius: bool,
    pub position: bool,
    pub velocity: bool,
    pub just_created: bool,
    pub just_destroyed: bool,
}

impl BaseEntityDirtyState {
    pub fn new() -> Self {
        Self {
            just_created: true,
            ..Default::default()
        }
    }
}

impl EntityDirtyState for BaseEntityDirtyState {
    fn any(&self) -> bool {
        self.radius || self.position || self.velocity
    }

    fn clear(&mut self) {
        let new_dirty = Self {
            just_created: false,
            just_destroyed: self.just_destroyed,
            ..Default::default()
        };
        *self = new_dirty;
    }

    // Radius
    fn radius_dirty(&self) -> bool {
        self.radius
    }

    fn set_radius_dirty(&mut self, value: bool) {
        self.radius = value;
    }

    // Position
    fn position_dirty(&self) -> bool {
        self.position
    }

    fn set_position_dirty(&mut self, value: bool) {
        self.position = value;
    }

    // Velocity
    fn velocity_dirty(&self) -> bool {
        self.velocity
    }

    fn set_velocity_dirty(&mut self, value: bool) {
        self.velocity = value;
    }

    // Lifecycle
    fn just_created(&self) -> bool {
        self.just_created
    }
    
    fn just_destroyed(&self) -> bool {
        self.just_destroyed
    }
    
    fn set_just_created(&mut self, value: bool) {
        self.just_created = value;
    }
    
    fn set_just_destroyed(&mut self, value: bool) {
        self.just_destroyed = value;
    }
}

// --- Entity Dirty State Trait ---

pub trait EntityDirtyState {
    // Core
    fn any(&self) -> bool;
    fn clear(&mut self);
    
    // Collider
    fn radius_dirty(&self) -> bool;
    fn position_dirty(&self) -> bool;
    fn velocity_dirty(&self) -> bool;

    fn set_radius_dirty(&mut self, value: bool);
    fn set_position_dirty(&mut self, value: bool);
    fn set_velocity_dirty(&mut self, value: bool);

    // Lifecycle
    fn just_created(&self) -> bool;
    fn just_destroyed(&self) -> bool;

    fn set_just_created(&mut self, value: bool);
    fn set_just_destroyed(&mut self, value: bool);
    
    fn has_lifecycle_changes(&self) -> bool {
        self.just_created() || self.just_destroyed()
    }
}

// --- Entity State Trait ---

pub trait EntityState {
    type Data;
    type Dirty: EntityDirtyState;

    // Core
    fn data(&self) -> &Self::Data;
    fn data_mut(&mut self) -> &mut Self::Data;

    fn dirty(&self) -> &Self::Dirty;
    fn dirty_mut(&mut self) -> &mut Self::Dirty;

    // Common
    fn get_id(&self) -> String;

    // Collider
    fn collider(&self) -> &CircleCollider;
    fn collider_mut(&mut self) -> &mut CircleCollider;

    // Radius
    fn get_radius(&self) -> f32 {
        self.collider().radius
    }

    fn get_diameter(&self) -> f32 {
        self.collider().diameter()
    }

    fn set_radius(&mut self, radius: f32) {
        let collider = self.collider_mut();
        if collider.radius != radius {
            collider.radius = radius;
            self.dirty_mut().set_radius_dirty(true);
        }
    }

    fn modify_radius(&mut self, delta: f32) {
        self.set_radius(self.get_radius() + delta);
    }

    // Position
    fn get_position(&self) -> Vec2 {
        self.collider().position
    }

    fn set_position(&mut self, position: Vec2) {
        let collider = self.collider_mut();
        if collider.position != position {
            collider.position = position;
            self.dirty_mut().set_position_dirty(true);
        }
    }

    fn modify_position(&mut self, delta: Vec2) {
        self.set_position(self.get_position() + delta);
    }

    // Velocity
    fn get_velocity(&self) -> Vec2 {
        self.collider().velocity
    }

    fn set_velocity(&mut self, velocity: Vec2) {
        let collider = self.collider_mut();
        if collider.velocity != velocity {
            collider.velocity = velocity;
            self.dirty_mut().set_velocity_dirty(true);
        }
    }

    fn modify_velocity(&mut self, delta: Vec2) {
        self.set_velocity(self.get_velocity() + delta);
    }

    // Lifecycle
    fn just_created(&self) -> bool {
        self.dirty().just_created()
    }

    fn just_destroyed(&self) -> bool {
        self.dirty().just_destroyed()
    }

    fn mark_created(&mut self, value: bool) {
        self.dirty_mut().set_just_created(value);
    }

    fn mark_destroyed(&mut self) {
        self.dirty_mut().set_just_destroyed(true);
    }

    fn clear_all(&mut self) {
        self.dirty_mut().clear();
    }

    fn has_changes(&self) -> bool {
        self.dirty().any() || self.dirty().has_lifecycle_changes()
    }

    // Networking
    fn generate_events(&self, events: &mut Vec<ServerEvent>);

    // Dirty Helper
    fn copy_if_dirty(&mut self, source: &Self) {
        self._copy_if_dirty_lifecycle(source);
        self._copy_if_dirty_collider(source);
        self._copy_if_dirty_custom(source);
    }

    fn _copy_if_dirty_lifecycle(&mut self, source: &Self) {
        let source_dirty = source.dirty();

        // Lifecycle
        if source_dirty.just_created() {
            self.dirty_mut().set_just_created(true);
        }
        if source_dirty.just_destroyed() {
            self.dirty_mut().set_just_destroyed(true);
        }
    }

    fn _copy_if_dirty_collider(&mut self, source: &Self) {
        let source_dirty = source.dirty();

        // Properties
        if source_dirty.radius_dirty() {
            self.set_radius(source.get_radius());
        }
        if source_dirty.position_dirty() {
            self.set_position(source.get_position());
        }
        if source_dirty.velocity_dirty() {
            self.set_velocity(source.get_velocity());
        }
    }

    fn _copy_if_dirty_custom(&mut self, source: &Self);
}
