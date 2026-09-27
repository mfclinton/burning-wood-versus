use turbo::*;

use crate::{config, entities::{ItemState, BaseEntityDirtyState, EntityDirtyState, EntityState, ClientControlled}, lerp, CircleCollider, Vec2, sprites, ServerEvent};

// --- Data ---

#[turbo::serialize]
pub struct PlayerData {
    pub id: String,
    pub username: String,
    pub collider: CircleCollider,
    pub health: i32,
    pub held_items: [Option<ItemState>; config::PLAYER_NUM_HELD_ITEMS],
    pub last_processed_sequence: u64,
}

// --- Dirty ---

#[derive(Default)]
#[turbo::serialize]
pub struct PlayerDirty {
    pub base: BaseEntityDirtyState,
    pub health: bool,
    pub held_item: [bool; 2],
}

impl EntityDirtyState for PlayerDirty {
    fn any(&self) -> bool {
        self.base.any() || self.health || self.held_item.iter().any(|&x| x)
    }

    fn clear(&mut self) {
        self.base.clear();
        self.health = false;
        self.held_item = [false; 2];
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

impl PlayerDirty {
    pub fn new() -> Self {
        Self {
            base: BaseEntityDirtyState::new(),
            ..Default::default()
        }
    }
}

// --- State ---

#[turbo::serialize]
pub struct PlayerState {
    data: PlayerData,
    dirty: PlayerDirty,
}

impl EntityState for PlayerState {
    type Data = PlayerData;
    type Dirty = PlayerDirty;

    // Overrides
    fn clear_all(&mut self) {
        self.dirty_mut().clear();
        for item in self.data_mut().held_items.iter_mut() {
            if let Some(item_state) = item {
                item_state.dirty_mut().clear();
            }
        }
    }

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
        
        // Lifetime
        if self.just_created() {
            events.push(ServerEvent::PlayerSpawned {
                user_id: id.clone(),
                player_state: self.clone(),
            });
        }

        if self.just_destroyed() {
            events.push(ServerEvent::PlayerDeleted {
                user_id: id.clone(),
            });
        }

        // Player Core
        if self.dirty.base.position || self.dirty.base.velocity {
            events.push(ServerEvent::PlayerMoveStateChanged {
                user_id: id.clone(),
                position: self.data.collider.position,
                velocity: self.data.collider.velocity,
                last_processed_seq: self.get_last_processed_sequence(),
            });
        }

        if self.dirty.health {
            events.push(ServerEvent::PlayerHealthChanged {
                user_id: id.clone(),
                health: self.data.health,
            });
        }

        // Held Item
        for (i, &dirty) in self.dirty.held_item.iter().enumerate() {
            if dirty {
                events.push(ServerEvent::PlayerHeldItemChanged {
                    user_id: id.clone(),
                    item_index: i,
                    item_state: self.data.held_items[i].clone(),
                });
            }
        }

        for i in 0..self.data.held_items.len() {
            if let Some(item_state) = &self.data.held_items[i] {
                item_state.generate_events(&id, i, events);
            }
        }
    }

    // Dirty Helper
    fn _copy_if_dirty_custom(&mut self, source: &Self) {
        let source_dirty = source.dirty();

        if source_dirty.health {
            self.set_health(source.data.health);
        }

        for i in 0..self.data.held_items.len() {
            self.set_held_item(i, source.data.held_items[i].clone());
        }
    }
}

impl PlayerState {
    pub fn new(id: String, username: String, health: i32, position: Vec2) -> Self {
        let mut player = Self {
            data: PlayerData {
                id,
                username,
                collider: CircleCollider {
                    position,
                    ..Default::default()
                },
                health,
                held_items: [const { None }; config::PLAYER_NUM_HELD_ITEMS],
                last_processed_sequence: 0,
            },
            dirty: PlayerDirty::new(),
        };

        player.update_radius();
        player
    }

    // Health
    pub fn set_health(&mut self, health: i32) {
        if self.data.health != health {
            self.data.health = health.clamp(0, config::PLAYER_MAX_HEALTH);
            self.dirty.health = true;

            self.update_radius();
        }
    }

    pub fn modify_health(&mut self, amount: i32) {
        self.set_health(self.data.health + amount);
    }

    // Held Item
    pub fn try_add_held_item(&mut self, item: ItemState) -> bool {
        for i in 0..self.data.held_items.len() {
            if self.data.held_items[i].is_none() {
                self.set_held_item(i, Some(item));
                return true;
            }
        }

        false
    }

    pub fn set_held_item(&mut self, item_index: usize, item: Option<ItemState>) {
        if self.data.held_items[item_index] != item {
            self.data.held_items[item_index] = item;
            self.dirty.held_item[item_index] = true;
        }
    }

    pub fn update_item_states(&mut self) {
        for i in 0..self.data.held_items.len() {
            if let Some(item_state) = &mut self.data.held_items[i] {
                // Update Item State
                item_state.update();
                self.dirty.held_item[i] = item_state.dirty().any();
    
                // Item Exhausted
                if item_state.is_exhausted() {
                    self.set_held_item(i, None);
                }
            }
        }
    }

    // Health-Driven Traits
    pub fn health_t(&self) -> f32 {
        self.data.health as f32 / config::PLAYER_MAX_HEALTH as f32
    }

    pub fn scaled_radius(&self) -> f32 {
        let health_t = self.health_t();
        lerp(config::PLAYER_MIN_RADIUS, config::PLAYER_MAX_RADIUS, health_t)
    }

    pub fn scaled_acceleration(&self) -> f32 {
        let health_t = self.health_t();
        lerp(config::PLAYER_MAX_ACCELERATION, config::PLAYER_MIN_ACCELERATION, health_t)
    }

    pub fn scaled_damping(&self) -> f32 {
        let health_t = self.health_t();
        lerp(config::PLAYER_MIN_DAMPING, config::PLAYER_MAX_DAMPING, health_t)
    }

    // Helpers
    pub fn is_dead(&self) -> bool {
        self.data.health <= 0
    }

    pub fn can_pickup_item(&self) -> bool {
        self.data.held_items.iter().any(|item| item.is_none())
    }

    pub fn can_use_item(&self) -> bool {
        self.data.held_items.iter().any(|item| item.as_ref().map_or(false, |item| item.can_use()))
    }

    fn update_radius(&mut self) {
        self.data.collider.radius = self.scaled_radius();
    }
}

impl PlayerState {
    pub fn reset_for_new_round(&mut self) {
        self.set_health(config::PLAYER_INITIAL_HEALTH);
        self.data.last_processed_sequence = 0;
        for i in 0..self.data.held_items.len() {
            self.set_held_item(i, None);
        }
    }

    pub fn get_item_keyboard_key_just_pressed() -> [bool; config::PLAYER_NUM_HELD_ITEMS] {
        let keyboard = keyboard::get();
        [
            keyboard.key_q().just_pressed(),
            keyboard.key_e().just_pressed(),
        ]
    }

    pub fn get_item_keyboard_key_pressed() -> [bool; config::PLAYER_NUM_HELD_ITEMS] {
        let keyboard = keyboard::get();
        [
            keyboard.key_q().pressed(),
            keyboard.key_e().pressed(),
        ]
    }

    pub fn get_auto_use_item_just_pressed() -> bool {
        let keyboard = keyboard::get();
        keyboard.space().just_pressed()
    }
}

// --- Client Controlled ---

impl ClientControlled for PlayerState {
    fn get_last_processed_sequence(&self) -> u64 {
        self.data.last_processed_sequence
    }
    
    fn set_last_processed_sequence(&mut self, seq: u64) {
        self.data.last_processed_sequence = seq;
    }
}

// --- Helpers ---

impl PlayerState {
    pub fn lerp_player_to_source(&mut self, other: &PlayerState, lerp_speed: f32) {
        let self_data = self.data();
        let other_data = other.data();
        
        let current_position = self_data.collider.position;
        let number_of_held_items = self_data.held_items.len();

        // Lerp Position
        let new_position = current_position.lerp(&other_data.collider.position, lerp_speed);
        self.set_position(new_position);
        
        // Sync Health
        self.set_health(other_data.health);
        
        // Sync Held Items
        for i in 0..number_of_held_items {
            self.set_held_item(i, other_data.held_items[i].clone());
        }
    }
}
