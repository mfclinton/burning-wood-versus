use turbo::*;

// --- ADJECTIVES ---

pub const ADJ_BURNING:   &str = "Burning";
pub const ADJ_BURNT:     &str = "Burnt";
pub const ADJ_BLAZING:   &str = "Blazing";
pub const ADJ_ANCIENT:   &str = "Ancient";
pub const ADJ_OLD:       &str = "Old";
pub const ADJ_CLASSIC:   &str = "Classic";
pub const ADJ_POWERFUL:  &str = "Powerful";
pub const ADJ_ALMIGHTY:  &str = "Almighty";
pub const ADJ_IMPRESSIVE:&str = "Impressive";
pub const ADJ_MIGHTY:    &str = "Mighty";
pub const ADJ_LITTLE:    &str = "Little";
pub const ADJ_TINY:      &str = "Tiny";
pub const ADJ_POCKET:    &str = "Pocket";
pub const ADJ_LOST:      &str = "Lost";
pub const ADJ_CONFUSED:  &str = "Confused";
pub const ADJ_MOIST:     &str = "Moist";
pub const ADJ_DAMP:      &str = "Damp";
pub const ADJ_SUPER:     &str = "Super";
pub const ADJ_ULTRA:     &str = "Ultra";
pub const ADJ_MEGA:      &str = "Mega";
pub const ADJ_GARGANT:   &str = "Gargantuan";
pub const ADJ_FIERY:     &str = "Fiery";
pub const ADJ_MID:       &str = "Mid";
pub const ADJ_CHAOTIC:   &str = "Chaotic";
pub const ADJ_TEMPERMENTAL:   &str = "Tempermental";
pub const ADJ_WOODEN:    &str = "Wooden";
pub const ADJ_TRICKY:    &str = "Tricky";

pub const ADJECTIVES: [&str; 27] = [
    ADJ_BURNING,
    ADJ_BURNT,
    ADJ_BLAZING,
    ADJ_ANCIENT,
    ADJ_OLD,
    ADJ_CLASSIC,
    ADJ_POWERFUL,
    ADJ_ALMIGHTY,
    ADJ_IMPRESSIVE,
    ADJ_MIGHTY,
    ADJ_LITTLE,
    ADJ_TINY,
    ADJ_POCKET,
    ADJ_LOST,
    ADJ_CONFUSED,
    ADJ_MOIST,
    ADJ_DAMP,
    ADJ_SUPER,
    ADJ_ULTRA,
    ADJ_MEGA,
    ADJ_GARGANT,
    ADJ_FIERY,
    ADJ_MID,
    ADJ_CHAOTIC,
    ADJ_TEMPERMENTAL,
    ADJ_WOODEN,
    ADJ_TRICKY
];

// --- Nouns ---

pub const NOUN_WOOD:       &str = "Wood";
pub const NOUN_STICK:      &str = "Stick";
pub const NOUN_BLAZE:      &str = "Blaze";
pub const NOUN_FIRE:       &str = "Fire";
pub const NOUN_INFERNO:    &str = "Inferno";
pub const NOUN_ADVENTURER: &str = "Adventurer";
pub const NOUN_HUNTER:     &str = "Hunter";
pub const NOUN_VOYAGER:    &str = "Voyager";
pub const NOUN_OPPONENT:   &str = "Opponent";
pub const NOUN_TIMBER:     &str = "Timber";
pub const NOUN_LOG:        &str = "Log";
pub const NOUN_EMBER:      &str = "Ember";
pub const NOUN_ASH:        &str = "Ash";
pub const NOUN_CINDER:     &str = "Cinder";
pub const NOUN_COAL:       &str = "Coal";
pub const NOUN_SPARK:      &str = "Spark";
pub const NOUN_FLAME:      &str = "Flame";
pub const NOUN_TORCH:      &str = "Torch";
pub const NOUN_PYRE:       &str = "Pyre";
pub const NOUN_SMOKE:      &str = "Smoke";
pub const NOUN_CHAR:       &str = "Char";
pub const NOUN_FURNACE:    &str = "Furnace";
pub const NOUN_CAMPFIRE:   &str = "Campfire";
pub const NOUN_BONFIRE:    &str = "Bonfire";
pub const NOUN_FOREST:     &str = "Forest";
pub const NOUN_GROVE:      &str = "Grove";
pub const NOUN_OAK:        &str = "Oak";
pub const NOUN_PINE:       &str = "Pine";
pub const NOUN_BIRCH:      &str = "Birch";
pub const NOUN_MAPLE:      &str = "Maple";
pub const NOUN_CEDAR:      &str = "Cedar";
pub const NOUN_REDWOOD:    &str = "Redwood";
pub const NOUN_BARK:       &str = "Bark";
pub const NOUN_TWIG:       &str = "Twig";
pub const NOUN_BRANCH:     &str = "Branch";
pub const NOUN_SPROUT:     &str = "Sprout";
pub const NOUN_STUMP:      &str = "Stump";
pub const NOUN_SAPLING:    &str = "Sapling";
pub const NOUN_RANGER:     &str = "Ranger";
pub const NOUN_SCOUT:      &str = "Scout";

pub const NOUNS: [&str; 40] = [
    NOUN_WOOD,
    NOUN_STICK,
    NOUN_BLAZE,
    NOUN_FIRE,
    NOUN_INFERNO,
    NOUN_ADVENTURER,
    NOUN_HUNTER,
    NOUN_VOYAGER,
    NOUN_OPPONENT,
    NOUN_TIMBER,
    NOUN_LOG,
    NOUN_EMBER,
    NOUN_ASH,
    NOUN_CINDER,
    NOUN_COAL,
    NOUN_SPARK,
    NOUN_FLAME,
    NOUN_TORCH,
    NOUN_PYRE,
    NOUN_SMOKE,
    NOUN_CHAR,
    NOUN_FURNACE,
    NOUN_CAMPFIRE,
    NOUN_BONFIRE,
    NOUN_FOREST,
    NOUN_GROVE,
    NOUN_OAK,
    NOUN_PINE,
    NOUN_BIRCH,
    NOUN_MAPLE,
    NOUN_CEDAR,
    NOUN_REDWOOD,
    NOUN_BARK,
    NOUN_TWIG,
    NOUN_BRANCH,
    NOUN_SPROUT,
    NOUN_STUMP,
    NOUN_SAPLING,
    NOUN_RANGER,
    NOUN_SCOUT,
];

// --- Functions ---

pub fn random_username() -> String {
    format!("{} {}", 
        random::pick(&ADJECTIVES).unwrap(), 
        random::pick(&NOUNS).unwrap()
    )
}
