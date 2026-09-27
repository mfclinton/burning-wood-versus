use crate::{entities::{BaseEntityDirtyState, EntityDirtyState, EntityState}, CircleCollider, EffectType, ItemType, ItemPhase, ItemRarity, ItemDefinition, ServerEvent};

// --- Data ---

#[derive(PartialEq)]
#[turbo::serialize]
pub struct ItemData {
    pub item_type: ItemType,
    pub phase: ItemPhase,
    pub charges: u32,
    pub timer: u32,
}

// --- Dirty ---

#[derive(Default, PartialEq)]
#[turbo::serialize]
pub struct ItemDirty {
    pub phase: bool,
    pub charges: bool,
    pub timer: bool,
}

impl ItemDirty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn any(&self) -> bool {
        self.phase || self.charges || self.timer
    }

    pub fn clear(&mut self) {
        self.phase = false;
        self.charges = false;
        self.timer = false;
    }
}

// --- State ---

#[derive(PartialEq)]
#[turbo::serialize]
pub struct ItemState {
    data: ItemData,
    dirty: ItemDirty,
}

impl ItemState {
    pub fn new(item_type: ItemType) -> Self {
        let definition = ItemDefinition::get_item_definition(&item_type);

        let (phase, timer) = if let Some(prep_time) = definition.preparation_time {
            (ItemPhase::Preparing, prep_time)
        } else {
            (ItemPhase::Ready, 0)
        };
        
        Self {
            data: ItemData {
                item_type,
                phase,
                charges: definition.max_charges,
                timer,
            },
            dirty: ItemDirty::new(),
        }
    }

    // Core
    pub fn data(&self) -> &ItemData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut ItemData {
        &mut self.data
    }

    pub fn dirty(&self) -> &ItemDirty {
        &self.dirty
    }

    pub fn dirty_mut(&mut self) -> &mut ItemDirty {
        &mut self.dirty
    }

    // Phase
    pub fn set_phase(&mut self, phase: ItemPhase) {
        if self.data.phase != phase {
            self.data.phase = phase;
            self.dirty.phase = true;
        }
    }

    // Charges
    pub fn set_charges(&mut self, charges: u32) {
        let charges = charges.max(0);
        if self.data.charges != charges {
            self.data.charges = charges;
            self.dirty.charges = true;
        }
    }

    pub fn modify_charges(&mut self, amount: i32) {
        let new_charges = (self.data.charges as i32 + amount).max(0) as u32;
        self.set_charges(new_charges);
    }

    // Timer
    pub fn set_timer(&mut self, timer: u32) {
        let timer = timer.max(0);
        if self.data.timer != timer {
            self.data.timer = timer;
            self.dirty.timer = true;
        }
    }

    pub fn modify_timer(&mut self, amount: i32) {
        let new_timer = (self.data.timer as i32 + amount).max(0) as u32;
        self.set_timer(new_timer);
    }

    // Main Functions
    pub fn update(&mut self) {
        // Timer Inactive
        if self.data.timer == 0 {
            return;
        }

        // Timer Countdown
        self.set_timer(self.data.timer - 1);
        if self.data.timer > 0 {
            return;
        }

        // Timer Finished
        let definition = ItemDefinition::get_item_definition(&self.data.item_type);
        match self.data.phase {
            ItemPhase::Preparing => {
                self.set_phase(ItemPhase::Ready);
            },
            ItemPhase::Active => {
                self.start_cooldown_or_ready(&definition);
            },
            ItemPhase::Cooldown => {
                self.set_phase(ItemPhase::Ready);
            },
            ItemPhase::Ready => {},
        }
    }

    pub fn try_use(&mut self) -> Option<EffectType> {
        // Check Item Ready
        if !self.can_use() {
            return None;
        }

        // Use Item
        self.modify_charges(-1);

        let definition = ItemDefinition::get_item_definition(&self.data.item_type);
        if let Some(duration) = definition.active_duration() {
            self.set_phase(ItemPhase::Active);
            self.set_timer(duration as u32);
            Some(definition.effect_type)
        }
        else {
            self.start_cooldown_or_ready(&definition);
            Some(definition.effect_type)
        }
    }

    // Networking
    pub fn generate_events(&self, user_id: &str, item_index: usize, events: &mut Vec<ServerEvent>) {
        let id = user_id.to_string();

        // Item Core
        if self.dirty.phase {
            events.push(ServerEvent::PlayerHeldItemPhaseChanged {
                user_id: id.clone(),
                item_index,
                phase: self.data.phase.clone(),
            });
        }

        if self.dirty.charges {
            events.push(ServerEvent::PlayerHeldItemChargesChanged {
                user_id: id.clone(),
                item_index,
                charges: self.data.charges,
            });
        }

        if self.dirty.timer {
            events.push(ServerEvent::PlayerHeldItemTimerChanged {
                user_id: id.clone(),
                item_index,
                timer: self.data.timer,
            });
        }
    }

    // Helpers
    pub fn can_use(&self) -> bool {
        matches!(self.data.phase, ItemPhase::Ready) && self.data.charges > 0
    }

    pub fn is_exhausted(&self) -> bool {
        self.data.charges == 0 && !matches!(self.data.phase, ItemPhase::Active)
    }

    fn start_cooldown_or_ready(&mut self, definition: &ItemDefinition) {
        if let Some(cooldown) = definition.cooldown_time {
            self.set_phase(ItemPhase::Cooldown);
            self.set_timer(cooldown);
        }
        else {
            self.set_phase(ItemPhase::Ready);
            self.set_timer(0);
        }
    }
}