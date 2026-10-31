use std::collections::{HashMap, HashSet};
use bevy::{
    prelude::*,
    sprite::Anchor,
};

// ---------------------------------------------------------------------------
// CONFIG  (loaded from config.toml at startup)
// ---------------------------------------------------------------------------

#[derive(serde::Deserialize, Debug, Clone, Resource)]
pub struct WorldConfig {
    pub tile_size: f32,
    pub chunk_tiles: i32,
    pub load_radius: i32,
    pub unload_dist: f32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub base_speed: f32,
    pub bankai_speed_mult: f32,
    pub hollow_speed_mult: f32,
    pub hp_normal: f32,
    pub hp_bankai: f32,
    pub hp_hollow: f32,
    pub rei_normal: f32,
    pub rei_bankai: f32,
    pub rei_hollow: f32,
    pub rei_regen_rate: f32,
    pub shunpo_speed: f32,
    pub shunpo_duration: f32,
    pub shunpo_cooldown: f32,
    pub shunpo_rei_cost: f32,
    pub attack_duration: f32,
    pub getsuga_speed: f32,
    pub getsuga_lifetime: f32,
    pub getsuga_rei_cost: f32,
    pub getsuga_dmg_normal: f32,
    pub getsuga_dmg_bankai: f32,
    pub getsuga_dmg_hollow: f32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MenosConfig {
    pub health: f32,
    pub wander_speed: f32,
    pub chase_speed: f32,
    pub detect_radius: f32,
    pub chase_radius: f32,
    pub attack_radius: f32,
    pub cero_cooldown: f32,
    pub cero_damage: f32,
    pub spawn_chunk_pct: u32,
    pub min_per_chunk: u32,
    pub max_per_chunk: u32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct GrimmjowConfig {
    pub health: f32,
    pub speed: f32,
    pub pantera_speed: f32,
    pub pantera_hp_threshold: f32,
    pub cero_cooldown: f32,
    pub cero_damage: f32,
    pub charge_cooldown: f32,
    pub charge_speed: f32,
    pub charge_duration: f32,
    pub charge_damage: f32,
    pub spawn_chunk_pct_x1000: u32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UlquiorraConfig {
    pub health: f32,
    pub speed: f32,
    pub cero_cooldown: f32,
    pub cero_damage: f32,
    pub teleport_cooldown: f32,
    pub teleport_distance: f32,
    pub spawn_chunk_pct_x1000: u32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TosenConfig {
    pub speed: f32,
    pub spawn_chunk_pct_x1000: u32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CeroConfig {
    pub speed: f32,
    pub lifetime: f32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CombatConfig {
    pub invincibility_secs: f32,
    pub player_hit_trauma: f32,
    pub getsuga_fire_trauma: f32,
    pub shunpo_trauma: f32,
    pub shake_max_offset_px: f32,
    pub shake_max_rot_rad: f32,
    pub shake_decay_rate: f32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct NelConfig {
    pub offset_x: f32,
    pub offset_y: f32,
    pub bounce_amplitude: f32,
    pub bounce_freq: f32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DeathConfig {
    pub fade_duration: f32,
    pub respawn_duration: f32,
}

#[derive(serde::Deserialize, Debug, Clone, Resource)]
pub struct GameConfig {
    pub world: WorldConfig,
    pub player: PlayerConfig,
    pub menos: MenosConfig,
    pub grimmjow: GrimmjowConfig,
    pub ulquiorra: UlquiorraConfig,
    pub tosen: TosenConfig,
    pub cero: CeroConfig,
    pub combat: CombatConfig,
    pub nel: NelConfig,
    pub death: DeathConfig,
}


pub static GAME_CONFIG: std::sync::OnceLock<GameConfig> = std::sync::OnceLock::new();

pub fn cfg() -> &'static GameConfig {
    GAME_CONFIG.get_or_init(GameConfig::load)
}

impl GameConfig {
    pub fn load() -> Self {
        let path = "config.toml";
        match std::fs::read_to_string(path) {
            Ok(text) => match toml::from_str::<GameConfig>(&text) {
                Ok(cfg) => { println!("[config] Loaded {path}"); cfg }
                Err(e) => {
                    eprintln!("[config] Parse error in {path}: {e}\n  -> Using built-in defaults.");
                    GameConfig::default()
                }
            },
            Err(e) => {
                eprintln!("[config] Could not read {path}: {e}\n  -> Using built-in defaults.");
                GameConfig::default()
            }
        }
    }

    /// Chunk size in pixels.
    pub fn chunk_size(&self) -> f32 {
        self.world.chunk_tiles as f32 * self.world.tile_size
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            world: WorldConfig { tile_size: 32.0, chunk_tiles: 8, load_radius: 2, unload_dist: 3.6 },
            player: PlayerConfig {
                base_speed: 175.0, bankai_speed_mult: 1.25, hollow_speed_mult: 1.35,
                hp_normal: 100.0, hp_bankai: 150.0, hp_hollow: 200.0,
                rei_normal: 100.0, rei_bankai: 120.0, rei_hollow: 80.0,
                rei_regen_rate: 18.0,
                shunpo_speed: 520.0, shunpo_duration: 0.14, shunpo_cooldown: 0.65, shunpo_rei_cost: 14.0,
                attack_duration: 0.32, getsuga_speed: 440.0, getsuga_lifetime: 0.52, getsuga_rei_cost: 16.0,
                getsuga_dmg_normal: 20.0, getsuga_dmg_bankai: 35.0, getsuga_dmg_hollow: 45.0,
            },
            menos: MenosConfig {
                health: 120.0, wander_speed: 45.0, chase_speed: 80.0,
                detect_radius: 280.0, chase_radius: 400.0, attack_radius: 300.0,
                cero_cooldown: 3.5, cero_damage: 18.0,
                spawn_chunk_pct: 45, min_per_chunk: 2, max_per_chunk: 6,
            },
            grimmjow: GrimmjowConfig {
                health: 350.0, speed: 120.0, pantera_speed: 180.0, pantera_hp_threshold: 0.5,
                cero_cooldown: 2.0, cero_damage: 22.0,
                charge_cooldown: 4.0, charge_speed: 400.0, charge_duration: 0.3, charge_damage: 25.0,
                spawn_chunk_pct_x1000: 30,
            },
            ulquiorra: UlquiorraConfig {
                health: 400.0, speed: 90.0, cero_cooldown: 2.5, cero_damage: 25.0,
                teleport_cooldown: 2.0, teleport_distance: 150.0, spawn_chunk_pct_x1000: 10,
            },
            tosen: TosenConfig { speed: 60.0, spawn_chunk_pct_x1000: 20 },
            cero: CeroConfig { speed: 320.0, lifetime: 0.9 },
            combat: CombatConfig {
                invincibility_secs: 0.8,
                player_hit_trauma: 0.4, getsuga_fire_trauma: 0.1, shunpo_trauma: 0.05,
                shake_max_offset_px: 12.0, shake_max_rot_rad: 0.04, shake_decay_rate: 1.2,
            },
            nel: NelConfig { offset_x: -48.0, offset_y: -32.0, bounce_amplitude: 4.0, bounce_freq: 4.0 },
            death: DeathConfig { fade_duration: 1.5, respawn_duration: 0.5 },
        }
    }
}


// ---------------------------------------------------------------------------
// COMPONENTS & ENUMS
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct NelCompanion;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum FacingDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum IchigoForm {
    #[default]
    Bankai,
    Hollowfied,
    Normal,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum PlayerActionState {
    #[default]
    Idle,
    Walking,
    Attacking,
    Shunpo,
    Dying,
    Respawning,
}

#[derive(Component)]
pub struct CharacterController {
    pub direction: FacingDirection,
    pub form: IchigoForm,
    pub state: PlayerActionState,
    pub action_timer: Timer,
    pub walk_timer: Timer,
    pub walk_frame: bool,
    pub shunpo_cooldown: Timer,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            direction: FacingDirection::Down,
            form: IchigoForm::Bankai,
            state: PlayerActionState::Idle,
            action_timer: Timer::from_seconds(cfg().player.attack_duration, TimerMode::Once),
            walk_timer: Timer::from_seconds(0.18, TimerMode::Repeating),
            walk_frame: false,
            shunpo_cooldown: Timer::from_seconds(cfg().player.shunpo_cooldown, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Reiatsu {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

#[derive(Component)]
pub struct YSort {
    pub base_offset: f32,
}

// Player Getsuga projectile
#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub damage: f32,
    pub form: IchigoForm,
}

// Enemy Cero projectile
#[derive(Component)]
pub struct EnemyProjectile {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub damage: f32,
}

#[derive(Component)]
pub struct FadeEffect {
    pub timer: Timer,
    pub initial_alpha: f32,
}

#[derive(Component)]
pub struct ChunkEntity(pub IVec2);

// --- Enemy ---

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyKind {
    Menos,
    Grimmjow,
    Ulquiorra,
    Tosen,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyState {
    Wander,
    Alert,
    Chase,
    Attack,
    Cooldown,
    Dead,
}

#[derive(Component)]
pub struct Enemy {
    pub kind: EnemyKind,
    pub health: f32,
    pub max_health: f32,
    pub state: EnemyState,
    pub direction: FacingDirection,
    pub wander_timer: f32,
    pub wander_dir: Vec2,
    pub cero_cooldown: f32,
    pub charge_cooldown: f32,
    pub charging: bool,
    pub charge_timer: f32,
    pub charge_dir: Vec2,
    pub teleport_cooldown: f32,
    pub walk_anim_timer: f32,
    pub walk_anim_frame: bool,
    pub pantera_phase: bool,
    pub death_timer: f32,
    pub dying: bool,
}

impl Enemy {
    fn new(kind: EnemyKind) -> Self {
        let (health, cero_cd) = match kind {
            EnemyKind::Menos   => (cfg().menos.health,    cfg().menos.cero_cooldown),
            EnemyKind::Grimmjow => (cfg().grimmjow.health, cfg().grimmjow.cero_cooldown),
            EnemyKind::Ulquiorra => (cfg().ulquiorra.health, cfg().ulquiorra.cero_cooldown),
            EnemyKind::Tosen   => (80.0,             999.0),
        };
        Self {
            kind,
            health,
            max_health: health,
            state: EnemyState::Wander,
            direction: FacingDirection::Down,
            wander_timer: 0.0,
            wander_dir: Vec2::new(1.0, 0.0),
            cero_cooldown: cero_cd * 0.5, // small initial delay
            charge_cooldown: cfg().grimmjow.charge_cooldown,
            charging: false,
            charge_timer: 0.0,
            charge_dir: Vec2::ZERO,
            teleport_cooldown: cfg().ulquiorra.teleport_cooldown,
            walk_anim_timer: 0.0,
            walk_anim_frame: false,
            pantera_phase: false,
            death_timer: 0.0,
            dying: false,
        }
    }
}

// Floating health bar tracks its enemy entity
#[derive(Component)]
pub struct EnemyHealthBar {
    pub enemy: Entity,
    pub bar_width: f32,
}

#[derive(Component)]
pub struct EnemyHealthBarFill;

// Invincibility frames on player after being hit
#[derive(Component)]
pub struct InvincibilityTimer {
    pub timer: f32,
}

// Death / respawn fade overlay
#[derive(Component)]
pub struct DeathFadeOverlay {
    pub phase: DeathPhase,
    pub timer: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DeathPhase {
    FadingOut,
    FadingIn,
}

// UI Markers
#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct ReiatsuBarFill;

#[derive(Component)]
pub struct FormLabel;

#[derive(Component)]
pub struct StatusNumbersText;

#[derive(Component)]
pub struct FadeOverlayNode;

// ---------------------------------------------------------------------------
// RESOURCES
// ---------------------------------------------------------------------------

#[derive(Resource, Default)]
pub struct WorldGenState {
    pub loaded_chunks: HashSet<IVec2>,
    pub chunk_entities: HashMap<IVec2, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub trauma: f32,
    pub offset: Vec2,
    pub rotation: f32,
    pub time_acc: f32,
}

impl ScreenShake {
    pub fn add_trauma(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).min(1.0);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    // Environment
    pub sand: Handle<Image>,
    pub quartz_tree: Handle<Image>,
    pub pillar: Handle<Image>,
    pub bone: Handle<Image>,
    pub bush: Handle<Image>,
    pub moon: Handle<Image>,

    // FX
    pub getsuga_bankai: Handle<Image>,
    pub getsuga_shikai: Handle<Image>,
    pub shunpo: Handle<Image>,
    pub hit_spark: Handle<Image>,
    pub cero_green: Handle<Image>,
    pub cero_oscuras: Handle<Image>,

    // UI
    pub hud: Handle<Image>,

    // Ichigo Bankai
    pub ichigo_bankai_down: Handle<Image>,
    pub ichigo_bankai_up: Handle<Image>,
    pub ichigo_bankai_left: Handle<Image>,
    pub ichigo_bankai_down_walk: Handle<Image>,
    pub ichigo_bankai_up_walk: Handle<Image>,
    pub ichigo_bankai_right_walk: Handle<Image>,
    pub ichigo_bankai_right_stance: Handle<Image>,

    // Ichigo Hollowfied
    pub ichigo_hollow_down: Handle<Image>,
    pub ichigo_hollow_up: Handle<Image>,
    pub ichigo_hollow_right: Handle<Image>,
    pub ichigo_hollow_down_walk: Handle<Image>,
    pub ichigo_hollow_up_walk: Handle<Image>,
    pub ichigo_hollow_right_stance: Handle<Image>,

    // Ichigo Normal (Shikai)
    pub ichigo_normal_down: Handle<Image>,
    pub ichigo_normal_up: Handle<Image>,
    pub ichigo_normal_right_attack: Handle<Image>,
    pub ichigo_normal_right_dash: Handle<Image>,
    pub ichigo_normal_left_stance: Handle<Image>,
    pub ichigo_normal_left_walk: Handle<Image>,

    // Menos
    pub menos_down: Handle<Image>,
    pub menos_up: Handle<Image>,
    pub menos_front: Handle<Image>,
    pub menos_back: Handle<Image>,
    pub menos_right: Handle<Image>,

    // Grimmjow
    pub grimmjow_down: Handle<Image>,
    pub grimmjow_up: Handle<Image>,
    pub grimmjow_right: Handle<Image>,
    pub grimmjow_down_walk: Handle<Image>,
    pub grimmjow_up_walk: Handle<Image>,
    pub grimmjow_pantera_right: Handle<Image>,

    // Ulquiorra
    pub ulquiorra_down: Handle<Image>,
    pub ulquiorra_up: Handle<Image>,
    pub ulquiorra_right: Handle<Image>,
    pub ulquiorra_right_walk: Handle<Image>,
    pub ulquiorra_up_walk: Handle<Image>,

    // Tosen
    pub tosen_down: Handle<Image>,
    pub tosen_right: Handle<Image>,
    pub tosen_up: Handle<Image>,

    // Nel
    pub nel_kid_down: Handle<Image>,
}

// ---------------------------------------------------------------------------
// ENTRY POINT
// ---------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BLEACH: Hollowfied - Hueco Mundo".into(),
                        resolution: (1280u32, 720u32).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.04, 0.05, 0.09)))
        .init_resource::<WorldGenState>()
        .init_resource::<ScreenShake>()
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                player_input_system,
                player_state_system,
                player_animation_system,
                nel_companion_system,
                enemy_ai_system,
                enemy_animation_system,
                projectile_system,
                enemy_projectile_system,
                combat_hit_system,
                invincibility_system,
                enemy_health_bar_system,
                screen_shake_system,
                fade_effect_system,
                player_death_system,
                death_fade_overlay_system,
                world_generation_system,
                y_sort_system,
                camera_follow_system,
                hud_update_system,
            ),
        )
        .run();
}

// ---------------------------------------------------------------------------
// GAME SETUP SYSTEM
// ---------------------------------------------------------------------------

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        sand:         asset_server.load("environment/hueco_sand_32x32.png"),
        quartz_tree:  asset_server.load("environment/quartz_tree_64x128.png"),
        pillar:       asset_server.load("environment/las_noches_pillar_32x96.png"),
        bone:         asset_server.load("environment/bone_formation_32x32.png"),
        bush:         asset_server.load("environment/dead_bush_32x32.png"),
        moon:         asset_server.load("environment/hueco_moon_64x64.png"),

        getsuga_bankai: asset_server.load("fx/getsuga_tensho_bankai_64x48.png"),
        getsuga_shikai: asset_server.load("fx/getsuga_tensho_shikai_64x48.png"),
        shunpo:         asset_server.load("fx/shunpo_dash_48x48.png"),
        hit_spark:      asset_server.load("fx/hit_spark_32x32.png"),
        cero_green:     asset_server.load("fx/cero_green_64x32.png"),
        cero_oscuras:   asset_server.load("fx/cero_oscuras_64x32.png"),

        hud: asset_server.load("ui/hud_full_combat_200x56.png"),

        ichigo_bankai_down:        asset_server.load("Ichigo/bankai/ichigo_down_standing_64x64.png"),
        ichigo_bankai_up:          asset_server.load("Ichigo/bankai/ichigo_up_standing_64x64.png"),
        ichigo_bankai_left:        asset_server.load("Ichigo/bankai/ichigo_left_standing_64x64.png"),
        ichigo_bankai_down_walk:   asset_server.load("Ichigo/bankai/ichigo_down_walking_64x64.png"),
        ichigo_bankai_up_walk:     asset_server.load("Ichigo/bankai/ichigo_up_walking_64x64.png"),
        ichigo_bankai_right_walk:  asset_server.load("Ichigo/bankai/ichigo_right_walking_64x64.png"),
        ichigo_bankai_right_stance:asset_server.load("Ichigo/bankai/ichigo_right_stance_64x64.png"),

        ichigo_hollow_down:        asset_server.load("Ichigo/hollowfied/ichigo_down_standing_64x64.png"),
        ichigo_hollow_up:          asset_server.load("Ichigo/hollowfied/ichigo_up_standing_64x64.png"),
        ichigo_hollow_right:       asset_server.load("Ichigo/hollowfied/ichigo_right_standing_64x64.png"),
        ichigo_hollow_down_walk:   asset_server.load("Ichigo/hollowfied/ichigo_down_walking_64x64.png"),
        ichigo_hollow_up_walk:     asset_server.load("Ichigo/hollowfied/ichigo_up_walking_64x64.png"),
        ichigo_hollow_right_stance:asset_server.load("Ichigo/hollowfied/ichigo_right_stance_64x64.png"),

        ichigo_normal_down:         asset_server.load("Ichigo/normal/ichigo_down_standing_64x64.png"),
        ichigo_normal_up:           asset_server.load("Ichigo/normal/ichigo_up_standing_64x64.png"),
        ichigo_normal_right_attack: asset_server.load("Ichigo/normal/ichigo_right_attack_64x64.png"),
        ichigo_normal_right_dash:   asset_server.load("Ichigo/normal/ichigo_right_dash_64x64.png"),
        ichigo_normal_left_stance:  asset_server.load("Ichigo/normal/ichigo_left_stance_64x64.png"),
        ichigo_normal_left_walk:    asset_server.load("Ichigo/normal/ichigo_left_walking_64x64.png"),

        menos_down:  asset_server.load("Menos/menos_down_standing_128x128.png"),
        menos_up:    asset_server.load("Menos/menos_up_standing_128x128.png"),
        menos_front: asset_server.load("Menos/menos_front_standing_128x128.png"),
        menos_back:  asset_server.load("Menos/menos_back_standing_128x128.png"),
        menos_right: asset_server.load("Menos/menos_right_standing_128x128.png"),

        grimmjow_down:       asset_server.load("Grimmjow/grimmjow_down_stance_64x64.png"),
        grimmjow_up:         asset_server.load("Grimmjow/grimmjow_up_standing_64x64.png"),
        grimmjow_right:      asset_server.load("Grimmjow/grimmjow_right_stance_64x64.png"),
        grimmjow_down_walk:  asset_server.load("Grimmjow/grimmjow_down_walking_64x64.png"),
        grimmjow_up_walk:    asset_server.load("Grimmjow/grimmjow_up_walking_64x64.png"),
        grimmjow_pantera_right: asset_server.load("Grimmjow/grimmjow_pantera_right_stance_64x64.png"),

        ulquiorra_down:       asset_server.load("Ulquiorra/ulquiorra_down_standing_64x64.png"),
        ulquiorra_up:         asset_server.load("Ulquiorra/ulquiorra_up_standing_64x64.png"),
        ulquiorra_right:      asset_server.load("Ulquiorra/ulquiorra_right_standing_64x64.png"),
        ulquiorra_right_walk: asset_server.load("Ulquiorra/ulquiorra_right_walking_64x64.png"),
        ulquiorra_up_walk:    asset_server.load("Ulquiorra/ulquiorra_up_walking_64x64.png"),

        tosen_down:  asset_server.load("Tosen/tosen_down_standing_64x64.png"),
        tosen_right: asset_server.load("Tosen/tosen_right_standing_64x64.png"),
        tosen_up:    asset_server.load("Tosen/tosen_up_standing_64x64.png"),

        nel_kid_down: asset_server.load("nel/nel_kid_down_standing_32x32.png"),
    };

    // 2D Camera
    commands
        .spawn((
            Camera2d,
            MainCamera,
            Transform::from_xyz(0.0, 0.0, 999.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: assets.moon.clone(),
                    custom_size: Some(Vec2::new(88.0, 88.0)),
                    color: Color::srgba(0.92, 0.95, 1.0, 0.8),
                    ..default()
                },
                Transform::from_xyz(460.0, 240.0, -100.0),
            ));
        });

    // Spawn Player
    commands.spawn((
        Player,
        CharacterController::default(),
        Health { current: cfg().player.hp_bankai, max: cfg().player.hp_bankai },
        Reiatsu { current: cfg().player.rei_bankai, max: cfg().player.rei_bankai, regen_rate: cfg().player.rei_regen_rate },
        Sprite {
            image: assets.ichigo_bankai_down.clone(),
            ..default()
        },
        Anchor::BOTTOM_CENTER,
        Transform::from_xyz(0.0, 0.0, 10.0),
        YSort { base_offset: 0.0 },
    ));

    // Spawn Nel Companion
    commands.spawn((
        NelCompanion,
        Sprite {
            image: assets.nel_kid_down.clone(),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(cfg().nel.offset_x, cfg().nel.offset_y, 9.0),
    ));

    // HUD
    setup_hud(&mut commands, &assets);

    // Death fade overlay (starts invisible)
    commands.spawn((
        FadeOverlayNode,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ZIndex(100),
    ));

    commands.insert_resource(assets);
}

// ---------------------------------------------------------------------------
// HUD UI SETUP
// ---------------------------------------------------------------------------

fn setup_hud(commands: &mut Commands, assets: &GameAssets) {
    // Upper-Left Combat HUD Group
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            ..default()
        })
        .with_children(|root| {
            root.spawn(Node {
                width: Val::Px(240.0),
                height: Val::Px(67.0),
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|frame| {
                frame.spawn((
                    ImageNode::new(assets.hud.clone()),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                ));

                // Health Bar
                frame
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(62.0),
                        top: Val::Px(12.0),
                        width: Val::Px(162.0),
                        height: Val::Px(10.0),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|slot| {
                        slot.spawn((
                            HealthBarFill,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.88, 0.18, 0.18)),
                        ));
                    });

                // Reiatsu Bar
                frame
                    .spawn(Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(62.0),
                        top: Val::Px(26.0),
                        width: Val::Px(146.0),
                        height: Val::Px(10.0),
                        overflow: Overflow::clip(),
                        ..default()
                    })
                    .with_children(|slot| {
                        slot.spawn((
                            ReiatsuBarFill,
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.18, 0.65, 0.95)),
                        ));
                    });
            });

            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(14.0),
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|info| {
                info.spawn((
                    FormLabel,
                    Text::new("FORM: TENSA ZANGETSU (BANKAI)"),
                    TextFont {
                        font_size: FontSize::Px(12.5),
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.45, 0.1)),
                ));

                info.spawn((
                    StatusNumbersText,
                    Text::new("HP: 150/150  |  REI: 120/120"),
                    TextFont {
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.85, 0.9, 0.95)),
                ));
            });
        });

    // Bottom Controls Banner
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(14.0),
            left: Val::Px(16.0),
            right: Val::Px(16.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            row.spawn((
                Node {
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(7.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.06, 0.08, 0.12, 0.88)),
                BorderColor::all(Color::srgba(0.32, 0.42, 0.58, 0.5)),
            ))
            .with_children(|box_node| {
                box_node.spawn((
                    Text::new("[WASD] Move  •  [SPACE/LMB] Getsuga Tensho  •  [SHIFT] Shunpo (dodge Cero!)  •  [1/2/3/TAB] Transform  •  [H] Take Dmg  •  [R] Heal"),
                    TextFont {
                        font_size: FontSize::Px(11.5),
                        ..default()
                    },
                    TextColor(Color::srgb(0.88, 0.92, 0.98)),
                ));
            });
        });
}

// ---------------------------------------------------------------------------
// PLAYER INPUT & MOVEMENT SYSTEM
// ---------------------------------------------------------------------------

fn player_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut shake: ResMut<ScreenShake>,
    mut commands: Commands,
    mut player_query: Query<(
        &mut Transform,
        &mut CharacterController,
        &mut Health,
        &mut Reiatsu,
    ), With<Player>>,
) {
    let Some((mut transform, mut controller, mut health, mut reiatsu)) =
        player_query.iter_mut().next()
    else {
        return;
    };

    // Block input while dying/respawning
    if controller.state == PlayerActionState::Dying
        || controller.state == PlayerActionState::Respawning
    {
        return;
    }

    let dt = time.delta_secs();

    // 1. Passive Reiatsu Regeneration
    reiatsu.current = (reiatsu.current + reiatsu.regen_rate * dt).min(reiatsu.max);

    // 2. Debug Health Testing Keys
    if keys.just_pressed(KeyCode::KeyH) {
        health.current = (health.current - 15.0).max(0.0);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        health.current = (health.current + 25.0).min(health.max);
    }

    // 3. Form Switching
    let prev_form = controller.form;
    if keys.just_pressed(KeyCode::Digit1) {
        controller.form = IchigoForm::Normal;
    } else if keys.just_pressed(KeyCode::Digit2) {
        controller.form = IchigoForm::Bankai;
    } else if keys.just_pressed(KeyCode::Digit3) {
        controller.form = IchigoForm::Hollowfied;
    } else if keys.just_pressed(KeyCode::Tab) {
        controller.form = match controller.form {
            IchigoForm::Normal     => IchigoForm::Bankai,
            IchigoForm::Bankai     => IchigoForm::Hollowfied,
            IchigoForm::Hollowfied => IchigoForm::Normal,
        };
    }

    if controller.form != prev_form {
        // Adjust max stats by form
        let (new_hp_max, new_rei_max) = form_stats(controller.form);
        let hp_ratio = health.current / health.max;
        health.max = new_hp_max;
        health.current = (hp_ratio * new_hp_max).min(new_hp_max);
        reiatsu.max = new_rei_max;
        reiatsu.current = reiatsu.current.min(new_rei_max);

        spawn_shunpo_burst(&mut commands, &assets, transform.translation);
        shake.add_trauma(0.08);
    }

    controller.shunpo_cooldown.tick(time.delta());

    // 4. Directional Movement Input
    let mut move_dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp)    { move_dir.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown)  { move_dir.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft)  { move_dir.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { move_dir.x += 1.0; }

    if move_dir.length_squared() > 0.0 {
        move_dir = move_dir.normalize();
        if move_dir.y.abs() > move_dir.x.abs() {
            controller.direction = if move_dir.y > 0.0 { FacingDirection::Up } else { FacingDirection::Down };
        } else {
            controller.direction = if move_dir.x > 0.0 { FacingDirection::Right } else { FacingDirection::Left };
        }
    }

    // 5. Shunpo
    if (keys.just_pressed(KeyCode::ShiftLeft) || keys.just_pressed(KeyCode::ShiftRight))
        && controller.shunpo_cooldown.is_finished()
        && reiatsu.current >= cfg().player.shunpo_rei_cost
        && move_dir != Vec2::ZERO
    {
        reiatsu.current -= cfg().player.shunpo_rei_cost;
        controller.shunpo_cooldown.reset();
        controller.state = PlayerActionState::Shunpo;
        controller.action_timer = Timer::from_seconds(cfg().player.shunpo_duration, TimerMode::Once);
        spawn_shunpo_burst(&mut commands, &assets, transform.translation);
        shake.add_trauma(cfg().combat.shunpo_trauma);
    }

    // 6. Getsuga Tensho
    let attack_pressed = keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::KeyJ)
        || mouse.just_pressed(MouseButton::Left);

    let getsuga_cost = if controller.form == IchigoForm::Hollowfied { 0.0 } else { cfg().player.getsuga_rei_cost };

    if attack_pressed
        && controller.state != PlayerActionState::Attacking
        && controller.state != PlayerActionState::Shunpo
        && reiatsu.current >= getsuga_cost
    {
        reiatsu.current -= getsuga_cost;
        controller.state = PlayerActionState::Attacking;
        controller.action_timer = Timer::from_seconds(cfg().player.attack_duration, TimerMode::Once);

        spawn_getsuga(
            &mut commands,
            &assets,
            transform.translation,
            controller.direction,
            controller.form,
        );
        shake.add_trauma(cfg().combat.getsuga_fire_trauma);
    }

    // 7. Movement Execution
    match controller.state {
        PlayerActionState::Shunpo => {
            transform.translation.x += move_dir.x * cfg().player.shunpo_speed * dt;
            transform.translation.y += move_dir.y * cfg().player.shunpo_speed * dt;
        }
        PlayerActionState::Attacking => {
            let facing_vec = facing_to_vec2(controller.direction);
            transform.translation.x += facing_vec.x * 45.0 * dt;
            transform.translation.y += facing_vec.y * 45.0 * dt;
        }
        _ => {
            if move_dir != Vec2::ZERO {
                let speed = form_speed(controller.form);
                transform.translation.x += move_dir.x * speed * dt;
                transform.translation.y += move_dir.y * speed * dt;
                controller.state = PlayerActionState::Walking;
                controller.walk_timer.tick(time.delta());
                if controller.walk_timer.just_finished() {
                    controller.walk_frame = !controller.walk_frame;
                }
            } else {
                controller.state = PlayerActionState::Idle;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ACTION TIMERS SYSTEM
// ---------------------------------------------------------------------------

fn player_state_system(
    time: Res<Time>,
    mut query: Query<&mut CharacterController, With<Player>>,
) {
    for mut controller in query.iter_mut() {
        if controller.state == PlayerActionState::Attacking
            || controller.state == PlayerActionState::Shunpo
        {
            controller.action_timer.tick(time.delta());
            if controller.action_timer.is_finished() {
                controller.state = PlayerActionState::Idle;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ANIMATION & SPRITE SELECTION SYSTEM
// ---------------------------------------------------------------------------

fn player_animation_system(
    assets: Res<GameAssets>,
    mut query: Query<(&CharacterController, &mut Sprite), With<Player>>,
) {
    for (controller, mut sprite) in query.iter_mut() {
        let (image_handle, flip_x) = match controller.form {
            IchigoForm::Bankai => match controller.state {
                PlayerActionState::Attacking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_bankai_right_stance.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_bankai_right_stance.clone(), true),
                    FacingDirection::Up    => (assets.ichigo_bankai_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_bankai_down.clone(), false),
                },
                PlayerActionState::Walking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_bankai_right_walk.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_bankai_right_walk.clone(), true),
                    FacingDirection::Up    => if controller.walk_frame {
                        (assets.ichigo_bankai_up_walk.clone(), false)
                    } else {
                        (assets.ichigo_bankai_up.clone(), false)
                    },
                    FacingDirection::Down  => if controller.walk_frame {
                        (assets.ichigo_bankai_down_walk.clone(), false)
                    } else {
                        (assets.ichigo_bankai_down.clone(), false)
                    },
                },
                _ => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_bankai_right_walk.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_bankai_left.clone(), false),
                    FacingDirection::Up    => (assets.ichigo_bankai_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_bankai_down.clone(), false),
                },
            },

            IchigoForm::Hollowfied => match controller.state {
                PlayerActionState::Attacking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_hollow_right_stance.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_hollow_right_stance.clone(), true),
                    FacingDirection::Up    => (assets.ichigo_hollow_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_hollow_down.clone(), false),
                },
                PlayerActionState::Walking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_hollow_right.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_hollow_right.clone(), true),
                    FacingDirection::Up    => if controller.walk_frame {
                        (assets.ichigo_hollow_up_walk.clone(), false)
                    } else {
                        (assets.ichigo_hollow_up.clone(), false)
                    },
                    FacingDirection::Down  => if controller.walk_frame {
                        (assets.ichigo_hollow_down_walk.clone(), false)
                    } else {
                        (assets.ichigo_hollow_down.clone(), false)
                    },
                },
                _ => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_hollow_right.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_hollow_right.clone(), true),
                    FacingDirection::Up    => (assets.ichigo_hollow_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_hollow_down.clone(), false),
                },
            },

            IchigoForm::Normal => match controller.state {
                PlayerActionState::Attacking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_normal_right_attack.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_normal_right_attack.clone(), true),
                    FacingDirection::Up    => (assets.ichigo_normal_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_normal_down.clone(), false),
                },
                PlayerActionState::Walking => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_normal_right_dash.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_normal_left_walk.clone(), false),
                    FacingDirection::Up    => (assets.ichigo_normal_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_normal_down.clone(), false),
                },
                _ => match controller.direction {
                    FacingDirection::Right => (assets.ichigo_normal_right_dash.clone(), false),
                    FacingDirection::Left  => (assets.ichigo_normal_left_stance.clone(), false),
                    FacingDirection::Up    => (assets.ichigo_normal_up.clone(), false),
                    FacingDirection::Down  => (assets.ichigo_normal_down.clone(), false),
                },
            },
        };

        sprite.image = image_handle;
        sprite.flip_x = flip_x;
    }
}

// ---------------------------------------------------------------------------
// NEL COMPANION SYSTEM
// ---------------------------------------------------------------------------

fn nel_companion_system(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<NelCompanion>)>,
    mut nel_query: Query<&mut Transform, (With<NelCompanion>, Without<Player>)>,
) {
    let Some(player_t) = player_query.iter().next() else { return; };
    let Some(mut nel_t) = nel_query.iter_mut().next() else { return; };

    let bounce = (time.elapsed_secs() * cfg().nel.bounce_freq).sin() * cfg().nel.bounce_amplitude;

    nel_t.translation.x = player_t.translation.x + cfg().nel.offset_x;
    nel_t.translation.y = player_t.translation.y + cfg().nel.offset_y + bounce;
    nel_t.translation.z = player_t.translation.z - 0.5;
}

// ---------------------------------------------------------------------------
// ENEMY AI SYSTEM
// ---------------------------------------------------------------------------

fn enemy_ai_system(
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    mut shake: ResMut<ScreenShake>,
    player_query: Query<(&Transform, Entity), (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy, &mut Sprite), Without<Player>>,
    mut player_health_query: Query<&mut Health, With<Player>>,
    mut player_inv_query: Query<Option<&InvincibilityTimer>, With<Player>>,
) {
    let Some((player_transform, _player_entity)) = player_query.iter().next() else { return; };
    let player_pos = player_transform.translation.truncate();

    let dt = time.delta_secs();

    // We need player invincibility status and health for collision
    let player_inv = player_inv_query.iter_mut().next();
    let player_is_inv = player_inv.map(|o| o.is_some()).unwrap_or(false);

    for (entity, mut transform, mut enemy, mut sprite) in enemy_query.iter_mut() {
        if enemy.dying {
            enemy.death_timer += dt;
            let alpha = 1.0 - (enemy.death_timer / 0.5).min(1.0);
            sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);
            if enemy.death_timer >= 0.5 {
                commands.entity(entity).despawn();
            }
            continue;
        }

        let enemy_pos = transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let dist = to_player.length();

        // Walk animation
        enemy.walk_anim_timer += dt;
        if enemy.walk_anim_timer >= 0.4 {
            enemy.walk_anim_timer = 0.0;
            enemy.walk_anim_frame = !enemy.walk_anim_frame;
        }

        match enemy.kind {
            // ---------------------------------------------------------------
            EnemyKind::Menos => {
                // Tick Cero cooldown
                enemy.cero_cooldown -= dt;

                // State machine
                match enemy.state {
                    EnemyState::Wander => {
                        enemy.wander_timer -= dt;
                        if enemy.wander_timer <= 0.0 {
                            let h = hash_coord(
                                enemy_pos.x as i32,
                                (enemy_pos.y as i32).wrapping_add(time.elapsed_secs() as i32),
                                0xdeadbeef,
                            );
                            let angle = (h as f32 / u32::MAX as f32) * std::f32::consts::TAU;
                            enemy.wander_dir = Vec2::new(angle.cos(), angle.sin());
                            enemy.wander_timer = 1.5 + (h % 200) as f32 / 100.0;
                        }

                        let move_v = enemy.wander_dir * cfg().menos.wander_speed * dt;
                        transform.translation.x += move_v.x;
                        transform.translation.y += move_v.y;

                        if dist < cfg().menos.detect_radius {
                            enemy.state = EnemyState::Alert;
                        }
                    }
                    EnemyState::Alert => {
                        if dist < cfg().menos.chase_radius {
                            enemy.state = EnemyState::Chase;
                        } else {
                            enemy.state = EnemyState::Wander;
                        }
                    }
                    EnemyState::Chase => {
                        if dist > cfg().menos.chase_radius {
                            enemy.state = EnemyState::Wander;
                        } else if dist < cfg().menos.attack_radius && enemy.cero_cooldown <= 0.0 {
                            enemy.state = EnemyState::Attack;
                        } else {
                            let dir = if dist > 0.0 { to_player / dist } else { Vec2::Y };
                            let mv = dir * cfg().menos.chase_speed * dt;
                            transform.translation.x += mv.x;
                            transform.translation.y += mv.y;
                            update_facing(&mut enemy, dir);
                        }
                    }
                    EnemyState::Attack => {
                        // Fire Cero
                        if enemy.cero_cooldown <= 0.0 && dist < cfg().menos.attack_radius {
                            enemy.cero_cooldown = cfg().menos.cero_cooldown;
                            let dir = if dist > 0.0 { to_player / dist } else { Vec2::Y };
                            spawn_enemy_cero(
                                &mut commands,
                                &assets,
                                transform.translation,
                                dir,
                                cfg().menos.cero_damage,
                                false,
                            );
                            enemy.state = EnemyState::Cooldown;
                        } else {
                            enemy.state = EnemyState::Chase;
                        }
                    }
                    EnemyState::Cooldown => {
                        if enemy.cero_cooldown <= 0.0 {
                            enemy.state = EnemyState::Chase;
                        }
                    }
                    EnemyState::Dead => {}
                }

                // Update facing direction sprite
                update_facing_from_dir(&mut enemy, to_player);
            }

            // ---------------------------------------------------------------
            EnemyKind::Grimmjow => {
                // Pantera phase check
                if !enemy.pantera_phase && enemy.health < cfg().grimmjow.health * cfg().grimmjow.pantera_hp_threshold {
                    enemy.pantera_phase = true;
                    spawn_shunpo_burst(&mut commands, &assets, transform.translation);
                    shake.add_trauma(0.3);
                }

                let speed = if enemy.pantera_phase { cfg().grimmjow.pantera_speed } else { cfg().grimmjow.speed };
                let cero_cd = if enemy.pantera_phase { 1.2 } else { cfg().grimmjow.cero_cooldown };

                enemy.cero_cooldown -= dt;
                enemy.charge_cooldown -= dt;

                if enemy.charging {
                    enemy.charge_timer -= dt;
                    let mv = enemy.charge_dir * cfg().grimmjow.charge_speed * dt;
                    transform.translation.x += mv.x;
                    transform.translation.y += mv.y;

                    // Check collision during charge
                    let new_pos = transform.translation.truncate();
                    if !player_is_inv && new_pos.distance(player_pos) < 40.0 {
                        if let Some(mut hp) = player_health_query.iter_mut().next() {
                            hp.current = (hp.current - cfg().grimmjow.charge_damage).max(0.0);
                            shake.add_trauma(cfg().combat.player_hit_trauma);
                        }
                    }

                    if enemy.charge_timer <= 0.0 {
                        enemy.charging = false;
                    }
                } else if dist < 500.0 {
                    // Charge attack
                    if enemy.charge_cooldown <= 0.0 && dist < 350.0 {
                        enemy.charging = true;
                        enemy.charge_timer = cfg().grimmjow.charge_duration;
                        enemy.charge_cooldown = cfg().grimmjow.charge_cooldown;
                        let dir = if dist > 0.0 { to_player / dist } else { Vec2::Y };
                        enemy.charge_dir = dir;
                        spawn_shunpo_burst(&mut commands, &assets, transform.translation);
                    } else {
                        // Chase
                        let dir = if dist > 0.0 { to_player / dist } else { Vec2::Y };
                        let mv = dir * speed * dt;
                        transform.translation.x += mv.x;
                        transform.translation.y += mv.y;
                        update_facing(&mut enemy, dir);

                        // Fire Cero
                        if enemy.cero_cooldown <= 0.0 && dist < 400.0 {
                            enemy.cero_cooldown = cero_cd;
                            spawn_enemy_cero(
                                &mut commands,
                                &assets,
                                transform.translation,
                                dir,
                                cfg().grimmjow.cero_damage,
                                false, // blue tinted done via color in spawn fn
                            );
                        }
                    }
                } else {
                    // Wander
                    let mv = enemy.wander_dir * cfg().menos.wander_speed * dt;
                    transform.translation.x += mv.x;
                    transform.translation.y += mv.y;
                    enemy.wander_timer -= dt;
                    if enemy.wander_timer <= 0.0 {
                        let h = hash_coord(enemy_pos.x as i32, enemy_pos.y as i32, 0xcafe);
                        let angle = (h as f32 / u32::MAX as f32) * std::f32::consts::TAU;
                        enemy.wander_dir = Vec2::new(angle.cos(), angle.sin());
                        enemy.wander_timer = 2.0;
                    }
                }

                update_facing_from_dir(&mut enemy, to_player);
            }

            // ---------------------------------------------------------------
            EnemyKind::Ulquiorra => {
                enemy.cero_cooldown -= dt;
                enemy.teleport_cooldown -= dt;

                // Teleport instead of walking
                if enemy.teleport_cooldown <= 0.0 && dist > 100.0 {
                    enemy.teleport_cooldown = cfg().ulquiorra.teleport_cooldown;
                    // Leave fade at current pos
                    spawn_shunpo_burst(&mut commands, &assets, transform.translation);
                    // Teleport to 150px from player
                    let h = hash_coord(
                        enemy_pos.x as i32,
                        time.elapsed_secs() as i32,
                        0xabcdef,
                    );
                    let angle = (h as f32 / u32::MAX as f32) * std::f32::consts::TAU;
                    let offset = Vec2::new(angle.cos(), angle.sin()) * cfg().ulquiorra.teleport_distance;
                    let new_pos = player_pos + offset;
                    transform.translation.x = new_pos.x;
                    transform.translation.y = new_pos.y;
                    // Fade at new pos
                    spawn_shunpo_burst(&mut commands, &assets, transform.translation);
                }

                // Fire Cero Oscuras
                if enemy.cero_cooldown <= 0.0 && dist < 400.0 {
                    enemy.cero_cooldown = cfg().ulquiorra.cero_cooldown;
                    let dir = if dist > 0.0 { to_player / dist } else { Vec2::Y };
                    spawn_enemy_cero(
                        &mut commands,
                        &assets,
                        transform.translation,
                        dir,
                        cfg().ulquiorra.cero_damage,
                        true, // oscuras
                    );
                }

                update_facing_from_dir(&mut enemy, to_player);
            }

            // ---------------------------------------------------------------
            EnemyKind::Tosen => {
                // Tosen just patrols, doesn't attack
                enemy.wander_timer -= dt;
                if enemy.wander_timer <= 0.0 {
                    let h = hash_coord(
                        enemy_pos.x as i32,
                        (enemy_pos.y + time.elapsed_secs() * 10.0) as i32,
                        0x70_5e,
                    );
                    let angle = (h as f32 / u32::MAX as f32) * std::f32::consts::TAU;
                    enemy.wander_dir = Vec2::new(angle.cos(), angle.sin());
                    enemy.wander_timer = 2.0 + (h % 300) as f32 / 100.0;
                }
                let mv = enemy.wander_dir * cfg().tosen.speed * dt;
                transform.translation.x += mv.x;
                transform.translation.y += mv.y;
                let wd = enemy.wander_dir;
                update_facing_from_dir(&mut enemy, wd * 100.0);            }
        }
    }
}

// ---------------------------------------------------------------------------
// ENEMY ANIMATION SYSTEM
// ---------------------------------------------------------------------------

fn enemy_animation_system(
    assets: Res<GameAssets>,
    mut enemy_query: Query<(&Enemy, &mut Sprite)>,
) {
    for (enemy, mut sprite) in enemy_query.iter_mut() {
        if enemy.dying { continue; }

        let (image, flip_x) = match enemy.kind {
            EnemyKind::Menos => {
                let img = match enemy.direction {
                    FacingDirection::Up => assets.menos_back.clone(),
                    FacingDirection::Down => {
                        if enemy.walk_anim_frame {
                            assets.menos_front.clone()
                        } else {
                            assets.menos_down.clone()
                        }
                    },
                    FacingDirection::Right => assets.menos_right.clone(),
                    FacingDirection::Left  => assets.menos_right.clone(),
                };
                let fx = matches!(enemy.direction, FacingDirection::Left);
                (img, fx)
            }
            EnemyKind::Grimmjow => {
                if enemy.pantera_phase {
                    let img = match enemy.direction {
                        FacingDirection::Right => assets.grimmjow_pantera_right.clone(),
                        FacingDirection::Left  => assets.grimmjow_pantera_right.clone(),
                        FacingDirection::Up    => assets.grimmjow_up.clone(),
                        FacingDirection::Down  => assets.grimmjow_down.clone(),
                    };
                    let fx = matches!(enemy.direction, FacingDirection::Left);
                    (img, fx)
                } else {
                    let img = match enemy.direction {
                        FacingDirection::Right => assets.grimmjow_right.clone(),
                        FacingDirection::Left  => assets.grimmjow_right.clone(),
                        FacingDirection::Up    => if enemy.walk_anim_frame {
                            assets.grimmjow_up_walk.clone()
                        } else {
                            assets.grimmjow_up.clone()
                        },
                        FacingDirection::Down  => if enemy.walk_anim_frame {
                            assets.grimmjow_down_walk.clone()
                        } else {
                            assets.grimmjow_down.clone()
                        },
                    };
                    let fx = matches!(enemy.direction, FacingDirection::Left);
                    (img, fx)
                }
            }
            EnemyKind::Ulquiorra => {
                let img = match enemy.direction {
                    FacingDirection::Right => assets.ulquiorra_right.clone(),
                    FacingDirection::Left  => assets.ulquiorra_right.clone(),
                    FacingDirection::Up    => if enemy.walk_anim_frame {
                        assets.ulquiorra_up_walk.clone()
                    } else {
                        assets.ulquiorra_up.clone()
                    },
                    FacingDirection::Down  => assets.ulquiorra_down.clone(),
                };
                let fx = matches!(enemy.direction, FacingDirection::Left);
                (img, fx)
            }
            EnemyKind::Tosen => {
                let img = match enemy.direction {
                    FacingDirection::Right => assets.tosen_right.clone(),
                    FacingDirection::Left  => assets.tosen_right.clone(),
                    FacingDirection::Up    => assets.tosen_up.clone(),
                    FacingDirection::Down  => assets.tosen_down.clone(),
                };
                let fx = matches!(enemy.direction, FacingDirection::Left);
                (img, fx)
            }
        };

        sprite.image = image;
        sprite.flip_x = flip_x;
    }
}

// ---------------------------------------------------------------------------
// PROJECTILE & FX SPAWNING
// ---------------------------------------------------------------------------

fn spawn_getsuga(
    commands: &mut Commands,
    assets: &GameAssets,
    player_pos: Vec3,
    direction: FacingDirection,
    form: IchigoForm,
) {
    let (image, color, damage) = match form {
        IchigoForm::Normal => (
            assets.getsuga_shikai.clone(),
            Color::srgba(0.35, 0.75, 1.0, 0.95),
            cfg().player.getsuga_dmg_normal,
        ),
        IchigoForm::Bankai => (
            assets.getsuga_bankai.clone(),
            Color::srgba(1.0, 0.18, 0.18, 1.0),
            cfg().player.getsuga_dmg_bankai,
        ),
        IchigoForm::Hollowfied => (
            assets.getsuga_bankai.clone(),
            Color::srgba(1.0, 0.05, 0.1, 1.0),
            cfg().player.getsuga_dmg_hollow,
        ),
    };

    let (offset, velocity, angle) = match direction {
        FacingDirection::Right => (Vec2::new(34.0, 26.0), Vec2::new(cfg().player.getsuga_speed, 0.0), -std::f32::consts::FRAC_PI_2),
        FacingDirection::Left  => (Vec2::new(-34.0, 26.0), Vec2::new(-cfg().player.getsuga_speed, 0.0), std::f32::consts::FRAC_PI_2),
        FacingDirection::Up    => (Vec2::new(0.0, 48.0), Vec2::new(0.0, cfg().player.getsuga_speed), std::f32::consts::PI),
        FacingDirection::Down  => (Vec2::new(0.0, -10.0), Vec2::new(0.0, -cfg().player.getsuga_speed), 0.0),
    };

    commands.spawn((
        Projectile {
            velocity,
            lifetime: Timer::from_seconds(cfg().player.getsuga_lifetime, TimerMode::Once),
            damage,
            form,
        },
        Sprite { image, color, ..default() },
        Transform {
            translation: Vec3::new(
                player_pos.x + offset.x,
                player_pos.y + offset.y,
                player_pos.z + 5.0,
            ),
            rotation: Quat::from_rotation_z(angle),
            ..default()
        },
    ));
}

fn spawn_enemy_cero(
    commands: &mut Commands,
    assets: &GameAssets,
    from_pos: Vec3,
    dir: Vec2,
    damage: f32,
    oscuras: bool,
) {
    let (image, color) = if oscuras {
        (
            assets.cero_oscuras.clone(),
            Color::srgba(0.1, 0.6, 0.25, 0.95),
        )
    } else {
        (
            assets.cero_green.clone(),
            Color::srgba(0.15, 0.9, 0.3, 0.95),
        )
    };

    let angle = dir.y.atan2(dir.x) - std::f32::consts::FRAC_PI_2;

    commands.spawn((
        EnemyProjectile {
            velocity: dir * cfg().cero.speed,
            lifetime: Timer::from_seconds(cfg().cero.lifetime, TimerMode::Once),
            damage,
        },
        Sprite { image, color, ..default() },
        Transform {
            translation: Vec3::new(from_pos.x, from_pos.y + 32.0, from_pos.z + 3.0),
            rotation: Quat::from_rotation_z(angle),
            ..default()
        },
    ));
}

fn spawn_shunpo_burst(commands: &mut Commands, assets: &GameAssets, position: Vec3) {
    commands.spawn((
        Sprite {
            image: assets.shunpo.clone(),
            color: Color::srgba(0.85, 0.9, 1.0, 0.75),
            ..default()
        },
        Transform::from_xyz(position.x, position.y + 24.0, position.z - 0.5),
        FadeEffect {
            timer: Timer::from_seconds(0.24, TimerMode::Once),
            initial_alpha: 0.75,
        },
    ));
}

fn spawn_hit_spark(commands: &mut Commands, assets: &GameAssets, position: Vec3) {
    commands.spawn((
        Sprite {
            image: assets.hit_spark.clone(),
            color: Color::srgba(1.0, 0.8, 0.3, 1.0),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, position.z + 10.0),
        FadeEffect {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            initial_alpha: 1.0,
        },
    ));
}

// ---------------------------------------------------------------------------
// PROJECTILE SYSTEM (player projectiles)
// ---------------------------------------------------------------------------

fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Projectile, &mut Sprite)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut projectile, mut sprite) in query.iter_mut() {
        transform.translation.x += projectile.velocity.x * dt;
        transform.translation.y += projectile.velocity.y * dt;
        projectile.lifetime.tick(time.delta());

        let remaining = projectile.lifetime.fraction_remaining();
        if remaining < 0.35 {
            let alpha = remaining / 0.35;
            let c = sprite.color.to_srgba();
            sprite.color = Color::srgba(c.red, c.green, c.blue, alpha);
        }

        if projectile.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ---------------------------------------------------------------------------
// ENEMY PROJECTILE SYSTEM
// ---------------------------------------------------------------------------

fn enemy_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut EnemyProjectile, &mut Sprite)>,
) {
    let dt = time.delta_secs();
    for (entity, mut transform, mut proj, mut sprite) in query.iter_mut() {
        transform.translation.x += proj.velocity.x * dt;
        transform.translation.y += proj.velocity.y * dt;
        proj.lifetime.tick(time.delta());

        let remaining = proj.lifetime.fraction_remaining();
        if remaining < 0.3 {
            let alpha = remaining / 0.3;
            let c = sprite.color.to_srgba();
            sprite.color = Color::srgba(c.red, c.green, c.blue, alpha);
        }

        if proj.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ---------------------------------------------------------------------------
// COMBAT HIT DETECTION SYSTEM
// ---------------------------------------------------------------------------

fn combat_hit_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut shake: ResMut<ScreenShake>,
    // Player projectiles vs enemies
    projectile_query: Query<(Entity, &Transform, &Projectile)>,
    // Enemy projectiles vs player
    enemy_proj_query: Query<(Entity, &Transform, &EnemyProjectile)>,
    // Enemies
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    // Player
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
    mut player_health_query: Query<&mut Health, With<Player>>,
    player_inv_query: Query<Option<&InvincibilityTimer>, With<Player>>,
) {
    // 1. Getsuga projectiles vs enemies
    for (proj_entity, proj_transform, projectile) in projectile_query.iter() {
        let proj_pos = proj_transform.translation.truncate();

        for (_enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if enemy.dying { continue; }

            let enemy_pos = enemy_transform.translation.truncate();
            let hit_radius = match enemy.kind {
                EnemyKind::Menos    => 48.0,
                EnemyKind::Grimmjow => 32.0,
                EnemyKind::Ulquiorra => 32.0,
                EnemyKind::Tosen    => 28.0,
            };

            if proj_pos.distance(enemy_pos) < hit_radius {
                // Deal damage
                enemy.health -= projectile.damage;
                spawn_hit_spark(&mut commands, &assets, proj_transform.translation);
                shake.add_trauma(0.15);
                commands.entity(proj_entity).despawn();

                if enemy.health <= 0.0 {
                    enemy.dying = true;
                    enemy.death_timer = 0.0;
                    spawn_hit_spark(&mut commands, &assets, enemy_transform.translation);
                }
                break;
            }
        }
    }

    // 2. Enemy Cero vs player
    let player_is_inv = player_inv_query.iter().next().map(|o| o.is_some()).unwrap_or(false);
    if player_is_inv { return; }

    let Some((player_entity, player_transform)) = player_query.iter().next() else { return; };
    let player_pos = player_transform.translation.truncate();

    for (proj_entity, proj_transform, enemy_proj) in enemy_proj_query.iter() {
        let proj_pos = proj_transform.translation.truncate();
        if proj_pos.distance(player_pos) < 28.0 {
            // Deal damage to player
            if let Some(mut hp) = player_health_query.iter_mut().next() {
                hp.current = (hp.current - enemy_proj.damage).max(0.0);
            }
            spawn_hit_spark(&mut commands, &assets, proj_transform.translation);
            shake.add_trauma(cfg().combat.player_hit_trauma);
            commands.entity(proj_entity).despawn();

            // Add invincibility frames
            commands.entity(player_entity).insert(InvincibilityTimer { timer: cfg().combat.invincibility_secs });
            break;
        }
    }
}

// ---------------------------------------------------------------------------
// INVINCIBILITY BLINK SYSTEM
// ---------------------------------------------------------------------------

fn invincibility_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut InvincibilityTimer, &mut Sprite), With<Player>>,
) {
    let dt = time.delta_secs();
    for (entity, mut inv, mut sprite) in query.iter_mut() {
        inv.timer -= dt;
        // Blink effect: alternate alpha
        let blink = ((inv.timer / cfg().combat.invincibility_secs) * 12.0 * std::f32::consts::PI).sin();
        let alpha = if blink > 0.0 { 1.0 } else { 0.3 };
        sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);

        if inv.timer <= 0.0 {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            commands.entity(entity).remove::<InvincibilityTimer>();
        }
    }
}

// ---------------------------------------------------------------------------
// ENEMY FLOATING HEALTH BARS SYSTEM
// ---------------------------------------------------------------------------

fn enemy_health_bar_system(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Enemy)>,
    mut bar_query: Query<(Entity, &EnemyHealthBar, &mut Node, &Children)>,
    mut fill_query: Query<&mut Node, (With<EnemyHealthBarFill>, Without<EnemyHealthBar>)>,
) {
    // Collect which enemies currently have bars
    let mut enemies_with_bars: HashSet<Entity> = HashSet::new();
    let mut bars_to_remove: Vec<Entity> = Vec::new();

    for (bar_entity, bar, mut bar_node, children) in bar_query.iter_mut() {
        // Check if enemy still exists and is alive
        if let Ok((_e, enemy_transform, enemy)) = enemy_query.get(bar.enemy) {
            if enemy.dying {
                bars_to_remove.push(bar_entity);
                continue;
            }
            enemies_with_bars.insert(bar.enemy);

            // Position bar above enemy
            let enemy_pos = enemy_transform.translation;
            let bar_y_offset = match enemy.kind {
                EnemyKind::Menos    => 80.0,
                _                   => 50.0,
            };

            // We use a world-space sprite approach — update position in world coords
            // Actually these are UI bars in world space: we move them via transforms
            // (We'll use separate sprite-based bars instead of UI nodes)
            // Update fill width
            let pct = (enemy.health / enemy.max_health * 100.0).clamp(0.0, 100.0);
            bar_node.left = Val::Px(enemy_pos.x - bar.bar_width * 0.5);
            bar_node.top  = Val::Px(-(enemy_pos.y + bar_y_offset));

            for child in children.iter() {
                if let Ok(mut fill_node) = fill_query.get_mut(child) {
                    fill_node.width = Val::Percent(pct);
                }
            }
        } else {
            bars_to_remove.push(bar_entity);
        }
    }

    for e in bars_to_remove {
        commands.entity(e).despawn();
    }

    // Spawn bars for new enemies
    for (enemy_entity, _transform, enemy) in enemy_query.iter() {
        if enemy.dying { continue; }
        if enemies_with_bars.contains(&enemy_entity) { continue; }

        let bar_width = match enemy.kind {
            EnemyKind::Menos    => 64.0,
            _                   => 80.0,
        };

        // Spawn a world-space health bar as a sprite-based overlay
        // We'll use simple colored sprites tracked per-enemy
        commands.spawn((
            EnemyHealthBar { enemy: enemy_entity, bar_width },
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(bar_width),
                height: Val::Px(7.0),
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                EnemyHealthBarFill,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.85, 0.1, 0.1)),
            ));
        });
    }
}

// ---------------------------------------------------------------------------
// SCREEN SHAKE SYSTEM
// ---------------------------------------------------------------------------

fn screen_shake_system(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
) {
    let dt = time.delta_secs();
    shake.trauma = (shake.trauma - cfg().combat.shake_decay_rate * dt).max(0.0);
    shake.time_acc += dt;

    let intensity = shake.trauma * shake.trauma;
    // Pseudo-noise using sin at different frequencies
    shake.offset = Vec2::new(
        (shake.time_acc * 37.1).sin() * cfg().combat.shake_max_offset_px * intensity,
        (shake.time_acc * 43.7).sin() * cfg().combat.shake_max_offset_px * intensity,
    );
    shake.rotation = (shake.time_acc * 29.3).sin() * cfg().combat.shake_max_rot_rad * intensity;
}

// ---------------------------------------------------------------------------
// FADE EFFECT SYSTEM
// ---------------------------------------------------------------------------

fn fade_effect_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FadeEffect, &mut Sprite)>,
) {
    for (entity, mut fade, mut sprite) in query.iter_mut() {
        fade.timer.tick(time.delta());
        let alpha = fade.initial_alpha * fade.timer.fraction_remaining();
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green, c.blue, alpha);
        if fade.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ---------------------------------------------------------------------------
// PLAYER DEATH & RESPAWN SYSTEM
// ---------------------------------------------------------------------------

fn player_death_system(
    mut commands: Commands,
    mut player_query: Query<(&Health, &mut CharacterController), With<Player>>,
) {
    let Some((health, mut controller)) = player_query.iter_mut().next() else {
        return;
    };

    if health.current <= 0.0
        && controller.state != PlayerActionState::Dying
        && controller.state != PlayerActionState::Respawning
    {
        controller.state = PlayerActionState::Dying;
        // Spawn the death fade overlay component
        commands.spawn(DeathFadeOverlay {
            phase: DeathPhase::FadingOut,
            timer: 0.0,
        });
    }
}

fn death_fade_overlay_system(
    mut commands: Commands,
    time: Res<Time>,
    mut overlay_query: Query<(Entity, &mut DeathFadeOverlay)>,
    mut ui_overlay_query: Query<&mut BackgroundColor, With<FadeOverlayNode>>,
    mut player_query: Query<(&mut Health, &mut Reiatsu, &mut CharacterController, &mut Transform), With<Player>>,
) {
    let Some((overlay_entity_id, mut overlay)) = overlay_query.iter_mut().next().map(|(e, o)| (e, o)) else {
        // No active death/respawn sequence — ensure overlay is clear
        if let Some(mut bg) = ui_overlay_query.iter_mut().next() {
            *bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
        }
        return;
    };

    let dt = time.delta_secs();
    overlay.timer += dt;

    let Some(mut ui_bg) = ui_overlay_query.iter_mut().next() else { return; };
    let Some((mut health, mut reiatsu, mut controller, mut transform)) = player_query.iter_mut().next() else { return; };

    match overlay.phase {
        DeathPhase::FadingOut => {
            let t = (overlay.timer / cfg().death.fade_duration).min(1.0);
            *ui_bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, t));

            if overlay.timer >= cfg().death.fade_duration {
                // Respawn: reset stats and position
                let form = controller.form;
                let (hp_max, rei_max) = form_stats(form);
                health.max = hp_max;
                health.current = hp_max;
                reiatsu.max = rei_max;
                reiatsu.current = rei_max;
                transform.translation = Vec3::new(0.0, 0.0, 10.0);
                controller.state = PlayerActionState::Respawning;

                overlay.phase = DeathPhase::FadingIn;
                overlay.timer = 0.0;
            }
        }
        DeathPhase::FadingIn => {
            let t = 1.0 - (overlay.timer / cfg().death.respawn_duration).min(1.0);
            *ui_bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, t));

            if overlay.timer >= cfg().death.respawn_duration {
                *ui_bg = BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0));
                controller.state = PlayerActionState::Idle;
                commands.entity(overlay_entity_id).despawn();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// PROCEDURAL HUECO MUNDO WORLD GENERATION
// ---------------------------------------------------------------------------

fn hash_coord(x: i32, y: i32, seed: u32) -> u32 {
    let mut h = (x as u32).wrapping_mul(0x85ebca6b)
        ^ (y as u32).wrapping_mul(0xc2b2ae35)
        ^ seed;
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846ca68b);
    h ^= h >> 16;
    h
}

fn world_generation_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut world_gen: ResMut<WorldGenState>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Some(player_transform) = player_query.iter().next() else { return; };
    let px = player_transform.translation.x;
    let py = player_transform.translation.y;

    let center_chunk_x = (px / cfg().chunk_size()).floor() as i32;
    let center_chunk_y = (py / cfg().chunk_size()).floor() as i32;
    let center_chunk = IVec2::new(center_chunk_x, center_chunk_y);

    // Generate surrounding chunks
    for dx in -cfg().world.load_radius..=cfg().world.load_radius {
        for dy in -cfg().world.load_radius..=cfg().world.load_radius {
            let chunk_coord = IVec2::new(center_chunk_x + dx, center_chunk_y + dy);
            if world_gen.loaded_chunks.contains(&chunk_coord) { continue; }
            world_gen.loaded_chunks.insert(chunk_coord);
            let mut chunk_spawned = Vec::new();

            let origin_x = chunk_coord.x as f32 * cfg().chunk_size();
            let origin_y = chunk_coord.y as f32 * cfg().chunk_size();
            let chunk_hash = hash_coord(chunk_coord.x, chunk_coord.y, 4242);

            // Sand tiles
            for tx in 0..cfg().world.chunk_tiles {
                for ty in 0..cfg().world.chunk_tiles {
                    let tile_x = origin_x + tx as f32 * cfg().world.tile_size + cfg().world.tile_size * 0.5;
                    let tile_y = origin_y + ty as f32 * cfg().world.tile_size + cfg().world.tile_size * 0.5;
                    let thash = hash_coord(
                        chunk_coord.x * cfg().world.chunk_tiles + tx,
                        chunk_coord.y * cfg().world.chunk_tiles + ty,
                        1337,
                    );
                    let tint = match thash % 5 {
                        0 => Color::srgb(0.92, 0.94, 0.98),
                        1 => Color::srgb(0.95, 0.96, 0.99),
                        _ => Color::srgb(0.98, 0.99, 1.0),
                    };
                    let id = commands.spawn((
                        Sprite {
                            image: assets.sand.clone(),
                            color: tint,
                            flip_x: (thash & 1) != 0,
                            flip_y: (thash & 2) != 0,
                            ..default()
                        },
                        Transform::from_xyz(tile_x, tile_y, 0.0),
                        ChunkEntity(chunk_coord),
                    )).id();
                    chunk_spawned.push(id);
                }
            }

            // A. Quartz Trees
            if chunk_hash % 100 < 38 {
                let tx = (chunk_hash % (cfg().chunk_size() as u32 - 40)) as f32 + 20.0;
                let ty = ((chunk_hash / 100) % (cfg().chunk_size() as u32 - 40)) as f32 + 20.0;
                let id = commands.spawn((
                    Sprite { image: assets.quartz_tree.clone(), ..default() },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(origin_x + tx, origin_y + ty, 10.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // B. Pillars
            if (chunk_hash / 7) % 100 < 22 {
                let px2 = origin_x + (chunk_hash.wrapping_mul(3) % (cfg().chunk_size() as u32 - 40)) as f32 + 20.0;
                let py2 = origin_y + (chunk_hash.wrapping_mul(5) % (cfg().chunk_size() as u32 - 40)) as f32 + 20.0;
                let id = commands.spawn((
                    Sprite { image: assets.pillar.clone(), ..default() },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(px2, py2, 10.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // C. Bone Formations
            if (chunk_hash / 13) % 100 < 30 {
                let bx = origin_x + (chunk_hash.wrapping_mul(11) % (cfg().chunk_size() as u32 - 30)) as f32 + 15.0;
                let by_ = origin_y + (chunk_hash.wrapping_mul(17) % (cfg().chunk_size() as u32 - 30)) as f32 + 15.0;
                let id = commands.spawn((
                    Sprite { image: assets.bone.clone(), ..default() },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(bx, by_, 5.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // D. Dead Bushes
            if (chunk_hash / 19) % 100 < 25 {
                let bx2 = origin_x + (chunk_hash.wrapping_mul(7) % (cfg().chunk_size() as u32 - 30)) as f32 + 15.0;
                let by2 = origin_y + (chunk_hash.wrapping_mul(13) % (cfg().chunk_size() as u32 - 30)) as f32 + 15.0;
                let id = commands.spawn((
                    Sprite { image: assets.bush.clone(), ..default() },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(bx2, by2, 5.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // E. Menos Grande (2-6 per chunk, 45% chunk chance)
            if chunk_hash % 100 < cfg().menos.spawn_chunk_pct {
                let min_m = cfg().menos.min_per_chunk;
                let max_m = cfg().menos.max_per_chunk.max(min_m);
                let span = (max_m - min_m + 1).max(1);
                let menos_count = min_m + (chunk_hash / 97) % span; // 2-6
                for i in 0..menos_count {
                    let mh = hash_coord(chunk_coord.x.wrapping_mul(100).wrapping_add(i as i32), chunk_coord.y, 0xbeef);
                    let mx = origin_x + (mh % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                    let my = origin_y + ((mh / 7) % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                    let id = commands.spawn((
                        Enemy::new(EnemyKind::Menos),
                        Sprite {
                            image: assets.menos_down.clone(),
                            custom_size: Some(Vec2::new(128.0, 128.0)),
                            ..default()
                        },
                        Anchor::BOTTOM_CENTER,
                        Transform::from_xyz(mx, my, 10.0),
                        YSort { base_offset: 0.0 },
                        ChunkEntity(chunk_coord),
                    )).id();
                    chunk_spawned.push(id);
                }
            }

            // F. Grimmjow (3% chance per chunk)
            if chunk_hash % 10000 < cfg().grimmjow.spawn_chunk_pct_x1000 {
                let gh = hash_coord(chunk_coord.x, chunk_coord.y, 0x9a1d);
                let gx = origin_x + (gh % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let gy = origin_y + ((gh / 11) % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let id = commands.spawn((
                    Enemy::new(EnemyKind::Grimmjow),
                    Sprite {
                        image: assets.grimmjow_down.clone(),
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(gx, gy, 10.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // G. Ulquiorra (1% chance per chunk)
            if chunk_hash % 10000 < cfg().ulquiorra.spawn_chunk_pct_x1000 {
                let uh = hash_coord(chunk_coord.x, chunk_coord.y, 0xa1b2);
                let ux = origin_x + (uh % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let uy = origin_y + ((uh / 13) % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let id = commands.spawn((
                    Enemy::new(EnemyKind::Ulquiorra),
                    Sprite {
                        image: assets.ulquiorra_down.clone(),
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(ux, uy, 10.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            // H. Tosen (2% chance per chunk)
            if (chunk_hash / 3) % 10000 < 200 {
                let th = hash_coord(chunk_coord.x, chunk_coord.y, 0x5e11);
                let tx2 = origin_x + (th % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let ty2 = origin_y + ((th / 17) % (cfg().chunk_size() as u32 - 60)) as f32 + 30.0;
                let id = commands.spawn((
                    Enemy::new(EnemyKind::Tosen),
                    Sprite {
                        image: assets.tosen_down.clone(),
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    Anchor::BOTTOM_CENTER,
                    Transform::from_xyz(tx2, ty2, 10.0),
                    YSort { base_offset: 0.0 },
                    ChunkEntity(chunk_coord),
                )).id();
                chunk_spawned.push(id);
            }

            world_gen.chunk_entities.insert(chunk_coord, chunk_spawned);
        }
    }

    // Unload far chunks
    let mut to_remove = Vec::new();
    for &chunk_coord in &world_gen.loaded_chunks {
        let dist = ((chunk_coord.x - center_chunk.x).pow(2)
            + (chunk_coord.y - center_chunk.y).pow(2)) as f32;
        if dist.sqrt() > cfg().world.unload_dist {
            to_remove.push(chunk_coord);
        }
    }
    for coord in to_remove {
        world_gen.loaded_chunks.remove(&coord);
        if let Some(entities) = world_gen.chunk_entities.remove(&coord) {
            for entity in entities {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Y-SORTING SYSTEM
// ---------------------------------------------------------------------------

fn y_sort_system(mut query: Query<(&mut Transform, &YSort)>) {
    for (mut transform, ysort) in query.iter_mut() {
        transform.translation.z = 10.0 - (transform.translation.y + ysort.base_offset) * 0.001;
    }
}

// ---------------------------------------------------------------------------
// SMOOTH CAMERA FOLLOW WITH SCREEN SHAKE
// ---------------------------------------------------------------------------

fn camera_follow_system(
    time: Res<Time>,
    shake: Res<ScreenShake>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Some(player_transform) = player_query.iter().next() else { return; };
    let Some(mut camera_transform) = camera_query.iter_mut().next() else { return; };

    let target = Vec3::new(
        player_transform.translation.x + shake.offset.x,
        player_transform.translation.y + shake.offset.y,
        camera_transform.translation.z,
    );

    let factor = (9.0 * time.delta_secs()).clamp(0.0, 1.0);
    camera_transform.translation = camera_transform.translation.lerp(target, factor);
    camera_transform.rotation = Quat::from_rotation_z(shake.rotation);
}

// ---------------------------------------------------------------------------
// HUD UPDATE SYSTEM
// ---------------------------------------------------------------------------

fn hud_update_system(
    player_query: Query<(&Health, &Reiatsu, &CharacterController), With<Player>>,
    mut hp_bar_query: Query<&mut Node, (With<HealthBarFill>, Without<ReiatsuBarFill>)>,
    mut rei_bar_query: Query<&mut Node, (With<ReiatsuBarFill>, Without<HealthBarFill>)>,
    mut form_text_query: Query<(&mut Text, &mut TextColor), (With<FormLabel>, Without<StatusNumbersText>)>,
    mut status_text_query: Query<&mut Text, (With<StatusNumbersText>, Without<FormLabel>)>,
) {
    let Some((health, reiatsu, controller)) = player_query.iter().next() else { return; };

    if let Some(mut bar_node) = hp_bar_query.iter_mut().next() {
        let hp_pct = (health.current / health.max * 100.0).clamp(0.0, 100.0);
        bar_node.width = Val::Percent(hp_pct);
    }

    if let Some(mut bar_node) = rei_bar_query.iter_mut().next() {
        let rei_pct = (reiatsu.current / reiatsu.max * 100.0).clamp(0.0, 100.0);
        bar_node.width = Val::Percent(rei_pct);
    }

    if let Some((mut text, mut color)) = form_text_query.iter_mut().next() {
        let (label, col) = match controller.form {
            IchigoForm::Bankai     => ("FORM: TENSA ZANGETSU (BANKAI)",   Color::srgb(0.95, 0.45, 0.1)),
            IchigoForm::Hollowfied => ("FORM: HOLLOWFIED (VAZARD)",       Color::srgb(0.95, 0.15, 0.25)),
            IchigoForm::Normal     => ("FORM: SHIKAI (ZANGETSU)",         Color::srgb(0.35, 0.75, 1.0)),
        };
        text.0 = label.to_string();
        *color = TextColor(col);
    }

    if let Some(mut text) = status_text_query.iter_mut().next() {
        text.0 = format!(
            "HP: {:.0}/{:.0}  |  REI: {:.0}/{:.0}",
            health.current, health.max, reiatsu.current, reiatsu.max
        );
    }
}

// ---------------------------------------------------------------------------
// HELPER FUNCTIONS
// ---------------------------------------------------------------------------

fn form_stats(form: IchigoForm) -> (f32, f32) {
    let c = cfg();
    match form {
        IchigoForm::Normal     => (c.player.hp_normal, c.player.rei_normal),
        IchigoForm::Bankai     => (c.player.hp_bankai, c.player.rei_bankai),
        IchigoForm::Hollowfied => (c.player.hp_hollow, c.player.rei_hollow),
    }
}

fn form_speed(form: IchigoForm) -> f32 {
    let c = cfg();
    match form {
        IchigoForm::Normal     => c.player.base_speed,
        IchigoForm::Bankai     => c.player.base_speed * c.player.bankai_speed_mult,
        IchigoForm::Hollowfied => c.player.base_speed * c.player.hollow_speed_mult,
    }
}

fn facing_to_vec2(dir: FacingDirection) -> Vec2 {
    match dir {
        FacingDirection::Up    => Vec2::new(0.0, 1.0),
        FacingDirection::Down  => Vec2::new(0.0, -1.0),
        FacingDirection::Left  => Vec2::new(-1.0, 0.0),
        FacingDirection::Right => Vec2::new(1.0, 0.0),
    }
}

fn update_facing(enemy: &mut Enemy, dir: Vec2) {
    if dir.length_squared() < 0.001 { return; }
    if dir.y.abs() > dir.x.abs() {
        enemy.direction = if dir.y > 0.0 { FacingDirection::Up } else { FacingDirection::Down };
    } else {
        enemy.direction = if dir.x > 0.0 { FacingDirection::Right } else { FacingDirection::Left };
    }
}

fn update_facing_from_dir(enemy: &mut Enemy, to_player: Vec2) {
    update_facing(enemy, to_player);
}
