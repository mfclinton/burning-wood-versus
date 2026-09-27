use crate::common::Sprite;

// --- UI Controls ---

pub const UI_CIRCULAR_PAD_BASE: Sprite = Sprite::new("ui/controls/circular_pad_base", 56, 56);
pub const UI_CIRCULAR_PAD_KNOB: Sprite = Sprite::new("ui/controls/circular_pad_knob", 32, 32);

// --- Logo ---

pub const UI_LOGO_BURNINGWOOD: Sprite = Sprite::new("ui/title_screen/Logo_BurningWood", 592, 244);
pub const UI_LOGO_VS: Sprite = Sprite::new("ui/title_screen/Logo_VS", 444, 363);

// --- General UI ---

pub const UI_BUTTON_BACK_NORMAL: Sprite = Sprite::new("ui/buttons/Button_Back_Passive", 102, 35);
pub const UI_BUTTON_BACK_SELECTED: Sprite = Sprite::new("ui/buttons/Button_Back_Active", 102, 30);

pub const UI_TOGGLE_INACTIVE: Sprite = Sprite::new("ui/buttons/Toggle_Inactive", 192, 106);
pub const UI_TOGGLE_INACTIVE_SELECTED: Sprite = Sprite::new("ui/buttons/Toggle_Inactive_Selected", 192, 106);
pub const UI_TOGGLE_ACTIVE: Sprite = Sprite::new("ui/buttons/Toggle_Active", 192, 106);
pub const UI_TOGGLE_ACTIVE_SELECTED: Sprite = Sprite::new("ui/buttons/Toggle_Active_Selected", 192, 106);

// --- Title Screen UI ---

pub const UI_TITLESCREEN_BG: Sprite = Sprite::new("ui/title_screen/TitleScreen", 960, 540);
pub const UI_TITLESCREEN_POST: Sprite = Sprite::new("ui/title_screen/TitleScreen_Post", 38, 239);

pub const UI_TITLESCREEN_SETTINGSBUTTON_NORMAL: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_SettingsButton", 178, 56);
pub const UI_TITLESCREEN_SETTINGSBUTTON_SELECTED: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_SettingsButton_Selected", 178, 56);

pub const UI_TITLESCREEN_CREDITSBUTTON_NORMAL: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_CreditsButton", 180, 56);
pub const UI_TITLESCREEN_CREDITSBUTTON_SELECTED: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_CreditsButton_Selected", 180, 56);

pub const UI_TITLESCREEN_ENTERBUTTON_NORMAL: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_EnterButton", 182, 58);
pub const UI_TITLESCREEN_ENTERBUTTON_SELECTED: Sprite = Sprite::new("ui/title_screen/buttons/TitleScreen_EnterButton_Selected", 182, 58);

pub const UI_TITLESCREEN_HIGHSCORE_PANEL: Sprite = Sprite::new("ui/title_screen/Highscore_Board", 126, 120);

pub const UI_TITLESCREEN_EYES: Sprite = Sprite::new("ui/title_screen/TitleScreen_Eyes", 64, 26);
pub const UI_TITLESCREEN_FLAME: Sprite = Sprite::new("ui/title_screen/TitleScreen_Flame", 64, 64);
pub const UI_TITLESCREEN_LIGHTS: Sprite = Sprite::new("ui/title_screen/TitleScreen_Lights", 232, 354);

// --- Settings Screen UI ---

pub const UI_SETTINGS_SCREEN_PANEL: Sprite = Sprite::new("ui/settings_screen/SettingsPage_BG", 591, 505);

// --- Credits Screen UI ---

pub const UI_CREDITS_SCREEN_PANEL: Sprite = Sprite::new("ui/title_screen/CreditsScreen", 598, 430);

// --- Player Config Screen UI ---

pub const UI_PLAYERCONFIG_NAMEPLATE: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_NamePlate", 332, 75);

pub const UI_PLAYERCONFIG_RANDOMIZE_BUTTON_NORMAL: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_RandomizeButton_Passive", 71, 71);
pub const UI_PLAYERCONFIG_RANDOMIZE_BUTTON_SELECTED: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_RandomizeButton_Active", 71, 71);

pub const UI_PLAYERCONFIG_MULTIP_BUTTON_NORMAL: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_MultiP_Button_Passive", 172, 163);
pub const UI_PLAYERCONFIG_MULTIP_BUTTON_SELECTED: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_MultiP_Button_Active", 172, 163);

pub const UI_PLAYERCONFIG_SINGLEP_BUTTON_NORMAL: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_SingleP_Button_Passive", 172, 163);
pub const UI_PLAYERCONFIG_SINGLEP_BUTTON_SELECTED: Sprite = Sprite::new("ui/player_config_screen/Name_Generator_Screen_SingleP_Button_Active", 172, 163);

// --- Game Screen UI ---

pub const UI_GAMESCREEN_ITEMSLOT: Sprite = Sprite::new("ui/game_screen/ItemSlot", 42, 42);
pub const UI_GAMESCREEN_KEYQ: Sprite = Sprite::new("ui/game_screen/Key_Q", 18, 24);
pub const UI_GAMESCREEN_KEYE: Sprite = Sprite::new("ui/game_screen/Key_E", 18, 20);

// --- Game Over Screen UI ---

pub const UI_BUTTON_RETRY_NORMAL: Sprite = Sprite::new("ui/buttons/Button_Retry_Passive", 137, 35);
pub const UI_BUTTON_RETRY_SELECTED: Sprite = Sprite::new("ui/buttons/Button_Retry_Active", 137, 30);

pub const UI_GAMEOVER_PANEL: Sprite = Sprite::new("ui/game_over/GameOver_Panel", 560, 538);
pub const UI_GAMEOVER_HEADER: Sprite = Sprite::new("ui/game_over/GameOver_Header", 327, 40);
pub const UI_GAMEOVER_POSTER: Sprite = Sprite::new("ui/game_over/GameOver_Poster", 298, 334);

pub const UI_WIN_HEADER: Sprite = Sprite::new("ui/game_over/Win_Header", 100, 40);

// --- Fire ---

pub const FIRE_1: Sprite = Sprite::new("fire/fire_1", 16, 32);
pub const FIRE_2: Sprite = Sprite::new("fire/fire_2", 16, 32);
pub const FIRE_3: Sprite = Sprite::new("fire/fire_3", 16, 32);
pub const FIRE_4: Sprite = Sprite::new("fire/fire_4", 16, 32);
pub const FIRE_5: Sprite = Sprite::new("fire/fire_5", 20, 32);
pub const FIRE_6: Sprite = Sprite::new("fire/fire_6", 46, 48);

pub const FIRE_FIRST_PLACE_CROWN: Sprite = Sprite::new("fire/First_Place_Crown", 37, 34);

// --- Projectiles ---

pub const PROJECTILES_COAL_IDLE: Sprite = Sprite::new("projectiles/coal_idle", 33, 33);
pub const PROJECTILES_COAL_SUMMON: Sprite = Sprite::new("projectiles/coal_summon", 33, 33);
pub const PROJECTILES_COAL_DESTROY: Sprite = Sprite::new("projectiles/coal_destroy", 33, 33);

pub const PROJECTILES_WATER_IDLE: Sprite = Sprite::new("projectiles/water_idle", 32, 32);
pub const PROJECTILES_WATER_SUMMON: Sprite = Sprite::new("projectiles/water_summon", 32, 32);
pub const PROJECTILES_WATER_DESTROY: Sprite = Sprite::new("projectiles/water_destroy", 32, 32);

pub const PROJECTILES_WOOD_IDLE: Sprite = Sprite::new("projectiles/wood_idle", 32, 32);
pub const PROJECTILES_WOOD_SUMMON: Sprite = Sprite::new("projectiles/wood_summon", 32, 32);
pub const PROJECTILES_WOOD_DESTROY: Sprite = Sprite::new("projectiles/wood_destroy", 32, 32);

pub const PROJECTILES_CHEST_IDLE: Sprite = Sprite::new("projectiles/chest_idle", 32, 32);
pub const PROJECTILES_CHEST_SUMMON: Sprite = Sprite::new("projectiles/chest_summon", 32, 32);
pub const PROJECTILES_CHEST_DESTROY: Sprite = Sprite::new("projectiles/chest_destroy", 32, 32);

// --- Events ---

pub const EVENTS_TOTEM_PURPLE: Sprite = Sprite::new("events/Totem_Purple", 50, 48);
pub const EVENTS_EVENT_PURPLE: Sprite = Sprite::new("events/Event_Purple", 444, 142);

// --- Items ---

pub const ITEM_SPRBAT_ICON: Sprite = Sprite::new("items/icons/sprBat", 34, 32);
pub const ITEM_SPRGHOST_ICON: Sprite = Sprite::new("items/icons/sprGhost", 32, 32);
pub const ITEM_SPRGRAVITYBALL_ICON: Sprite = Sprite::new("items/icons/sprGravityBall", 32, 32);
pub const ITEM_SPRLIGHTNING_ICON: Sprite = Sprite::new("items/icons/sprGoldenIdol", 32, 32);
pub const ITEM_SPRMINE_ICON: Sprite = Sprite::new("items/icons/sprMine", 32, 32);
pub const ITEM_SPROILBARREL_ICON: Sprite = Sprite::new("items/icons/sprOilBarrel", 32, 32);
pub const ITEM_SPRTREE_ICON: Sprite = Sprite::new("items/icons/sprTree", 32, 32);
pub const ITEM_SPRWATERGUN_ICON: Sprite = Sprite::new("items/icons/sprWaterGun", 32, 32);
pub const ITEM_SPRMETALWATERGUN_ICON: Sprite = Sprite::new("items/icons/sprMetalWaterGun", 32, 32);
pub const ITEM_SPRWATERBOMB_ICON: Sprite = Sprite::new("items/icons/sprWaterBomb", 32, 32);
pub const ITEM_SPRWIND_ICON: Sprite = Sprite::new("items/icons/sprWind", 32, 32);
pub const ITEM_SPEEDSHOES_32X32_ICON: Sprite = Sprite::new("items/icons/SpeedShoes_32x32", 36, 32);
pub const ITEM_SPRBLACKHOLE_ICON: Sprite = Sprite::new("items/icons/sprBlackhole", 32, 32);
pub const ITEM_SPRGAS_SMALL_ICON: Sprite = Sprite::new("items/icons/sprSmallGasCan", 32, 32);
pub const ITEM_SPRGAS_MEDIUM_ICON: Sprite = Sprite::new("items/icons/sprMediumGasCan", 32, 32);
pub const ITEM_SPRGAS_LARGE_ICON: Sprite = Sprite::new("items/icons/sprLargeGasCan", 32, 32);

// --- Effects ---

pub const EFFECTS_PROJECTILE_SPAWNER: Sprite = Sprite::new("effects/projectile_spawner", 64, 64);
pub const EFFECTS_MAGNET_FIELD: Sprite = Sprite::new("effects/magnet_field", 64, 64);
pub const EFFECTS_CAMP_FIRE: Sprite = Sprite::new("effects/sprCampFire", 64, 64);
pub const EFFECTS_LIFE_DRAIN_FIELD: Sprite = Sprite::new("effects/lifeDrain_field", 75, 64);
pub const EFFECTS_GOLDENIDOL_ACTIVE: Sprite = Sprite::new("effects/sprGoldenIdol_active", 48, 48);
pub const EFFECTS_GASCAN_USE: Sprite = Sprite::new("effects/sprGasCan_use", 80, 73);
pub const EFFECTS_WATERBOMB_USE: Sprite = Sprite::new("effects/sprWaterBomb_use", 80, 64);

// --- FX ---

pub const FX_PLAYERHEALTH_LOSS: Sprite = Sprite::new("fx/sprPlayerHealthLoss_FX", 16, 32);
pub const FX_PLAYERHEALTH_GAIN: Sprite = Sprite::new("fx/sprPlayerHealthGain_FX", 16, 32);

// --- World ---

pub const WORLD_BRICK: Sprite = Sprite::new("world/brick", 128, 128);
pub const WORLD_WALL: Sprite = Sprite::new("world/wall", 128, 128);
