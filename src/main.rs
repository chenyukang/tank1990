#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::RenderAssetUsages;
use bevy::audio::{
    AddAudioSource, AudioPlayer, AudioSource, Decodable, PlaybackSettings, Source, Volume,
};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PresentMode;
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

const ASSET_MANIFEST_PATH: &str = "assets/manifest.ron";
const PERSONAL_ASSET_MANIFEST_PATH: &str = "assets/personal/manifest.ron";
const ASSET_ROOT_DIR: &str = "assets";
const PERSONAL_TANK_ATLAS_PATH: &str = "personal/tanks.png";
const PERSONAL_TERRAIN_ATLAS_PATH: &str = "personal/terrain.png";
const PERSONAL_BULLET_ATLAS_PATH: &str = "personal/bullets.png";
const PERSONAL_EFFECT_ATLAS_PATH: &str = "personal/effects.png";
const PERSONAL_POWERUP_ATLAS_PATH: &str = "personal/powerups.png";
const PERSONAL_BASE_INTACT_PATH: &str = "personal/base_intact.png";
const PERSONAL_BASE_DESTROYED_PATH: &str = "personal/base_destroyed.png";
const PERSONAL_SCORE_BADGE_PATH: &str = "personal/score_badge.png";
const PERSONAL_STAGE_FLAG_PATH: &str = "personal/stage_flag.png";
const PERSONAL_GLYPH_ATLAS_PATH: &str = "personal/glyphs.png";
const PERSONAL_FIRE_SOUND_PATH: &str = "personal/sounds/fire.ogg";
const PERSONAL_BRICK_HIT_SOUND_PATH: &str = "personal/sounds/brick_hit.ogg";
const PERSONAL_STEEL_HIT_SOUND_PATH: &str = "personal/sounds/steel_hit.ogg";
const PERSONAL_TANK_EXPLOSION_SOUND_PATH: &str = "personal/sounds/tank_explosion.ogg";
const PERSONAL_BASE_DESTROYED_SOUND_PATH: &str = "personal/sounds/base_destroyed.ogg";
const PERSONAL_POWERUP_PICKUP_SOUND_PATH: &str = "personal/sounds/powerup_pickup.ogg";
const PERSONAL_STAGE_START_SOUND_PATH: &str = "personal/sounds/stage_start.ogg";
const PERSONAL_LEVEL_CLEAR_SOUND_PATH: &str = "personal/sounds/level_clear.ogg";
const PERSONAL_GAME_OVER_SOUND_PATH: &str = "personal/sounds/game_over.ogg";
const PERSONAL_BACKGROUND_MUSIC_SOUND_PATH: &str = "personal/sounds/background.ogg";
#[cfg(test)]
const PERSONAL_SPRITE_OVERRIDE_PATHS: [&str; 10] = [
    PERSONAL_TANK_ATLAS_PATH,
    PERSONAL_TERRAIN_ATLAS_PATH,
    PERSONAL_BULLET_ATLAS_PATH,
    PERSONAL_EFFECT_ATLAS_PATH,
    PERSONAL_POWERUP_ATLAS_PATH,
    PERSONAL_BASE_INTACT_PATH,
    PERSONAL_BASE_DESTROYED_PATH,
    PERSONAL_SCORE_BADGE_PATH,
    PERSONAL_STAGE_FLAG_PATH,
    PERSONAL_GLYPH_ATLAS_PATH,
];
#[cfg(test)]
const PERSONAL_SOUND_OVERRIDE_PATHS: [&str; 10] = [
    PERSONAL_FIRE_SOUND_PATH,
    PERSONAL_BRICK_HIT_SOUND_PATH,
    PERSONAL_STEEL_HIT_SOUND_PATH,
    PERSONAL_TANK_EXPLOSION_SOUND_PATH,
    PERSONAL_BASE_DESTROYED_SOUND_PATH,
    PERSONAL_POWERUP_PICKUP_SOUND_PATH,
    PERSONAL_STAGE_START_SOUND_PATH,
    PERSONAL_LEVEL_CLEAR_SOUND_PATH,
    PERSONAL_GAME_OVER_SOUND_PATH,
    PERSONAL_BACKGROUND_MUSIC_SOUND_PATH,
];
const LEVEL_COUNT: usize = 45;
const LEVEL_CLEAR_DELAY_SECONDS: f32 = 2.0;
const LEVEL_CLEAR_SCORECARD_SECONDS: f32 = 4.0;
const STAGE_INTRO_SECONDS: f32 = 1.2;
const ARENA_COUNT: usize = 6;
const DEFAULT_VERSUS_ARENA: usize = 1;
const TANK_ATLAS_TILES: usize = 48;
const TANK_ANIMATION_FRAMES: usize = 2;
const TANK_ATLAS_TILE_SIZE: usize = 16;
const BULLET_ATLAS_TILES: usize = 4;
const BULLET_ATLAS_TILE_SIZE: usize = 4;
const TERRAIN_ATLAS_TILES: usize = 6;
const TERRAIN_ATLAS_TILE_SIZE: usize = 8;
const EFFECT_ATLAS_TILES: usize = 20;
const EFFECT_ATLAS_TILE_SIZE: usize = 16;
const POWERUP_ATLAS_TILES: usize = 6;
const POWERUP_ATLAS_TILE_SIZE: usize = 16;
const GENERATED_BASE_SIZE: usize = 16;
const GENERATED_UI_ICON_SIZE: usize = 8;

const VIRTUAL_WIDTH: f32 = 256.0;
const VIRTUAL_HEIGHT: f32 = 240.0;
const DEFAULT_WINDOW_SCALE: u32 = 3;
const MIN_WINDOW_SCALE: u32 = 2;
const MAX_WINDOW_SCALE: u32 = 4;
const WINDOW_SCALE_ENV: &str = "TANK_WINDOW_SCALE";
static WINDOW_SCALE_CACHE: OnceLock<f32> = OnceLock::new();

const BOARD_ORIGIN_X: f32 = 0.0;
const BOARD_ORIGIN_Y: f32 = 16.0;
const BOARD_TILES: usize = 26;
const TILE_SIZE: f32 = 8.0;
const TANK_SIZE: f32 = 16.0;
const BULLET_SIZE: f32 = 4.0;

const PLAYER_SPEED: f32 = 60.0;
const BULLET_SPEED: f32 = 240.0;
const PLAYER_FAST_BULLET_SPEED: f32 = 300.0;
const POWER_ENEMY_BULLET_SPEED: f32 = 300.0;
const ENEMY_BULLET_LIMIT: usize = 4;
const ENEMY_BULLET_LIMIT_PER_TANK: usize = 1;
const CLASSIC_MAX_ACTIVE_ENEMIES: usize = 4;
const CLASSIC_BASE_X: usize = 12;
const CLASSIC_BASE_Y: usize = 24;
const ENEMY_MARKER_COUNT: usize = 20;
const ENEMY_MARKER_COLUMNS: usize = 4;
const ENEMY_MARKER_SIZE: f32 = 8.0;
const PLAYER_LIFE_ICON_SIZE: f32 = 8.0;
const ENEMY_MARKER_LEFT: f32 = 216.0;
const ENEMY_MARKER_TOP: f32 = 159.0;
const ENEMY_MARKER_CELL_X: f32 = 9.0;
const ENEMY_MARKER_CELL_Y: f32 = 9.0;
const SNAP_DISTANCE: f32 = 2.0;
const REQUIRED_GLYPHS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const GENERATED_GLYPH_WIDTH: usize = 5;
const GENERATED_GLYPH_HEIGHT: usize = 7;
const GLYPH_ADVANCE: f32 = 6.0;
static PAUSED_BANNER_LINES: [&str; 4] = ["PAUSED", "ESC RESUME", "R RESTART", "M MENU"];
static GAME_OVER_BANNER_LINES: [&str; 2] = ["GAME OVER", "PRESS R OR M"];
static LEVEL_CLEAR_BANNER_LINES: [&str; 1] = ["LEVEL CLEAR"];
static P1_WIN_BANNER_LINES: [&str; 2] = ["P1 WIN", "PRESS R OR M"];
static P2_WIN_BANNER_LINES: [&str; 2] = ["P2 WIN", "PRESS R OR M"];
static VICTORY_BANNER_LINES: [&str; 3] = ["VICTORY", "ALL STAGES CLEAR", "PRESS R OR M"];
static MODE_SELECT_HINT_LINES: [&str; 3] =
    ["WS ARROWS SELECT", "AD ARROWS CHANGE", "SPACE ENTER START"];
const DEFAULT_RESPAWN_INVULNERABILITY_SECONDS: f32 = 2.0;
const HELMET_SECONDS: f32 = 6.0;
const CLOCK_SECONDS: f32 = 6.0;
const SHOVEL_SECONDS: f32 = 10.0;
const SHOVEL_WARNING_SECONDS: f32 = 2.0;
const EXPLOSION_FRAME_SECONDS: f32 = 0.07;
const BULLET_IMPACT_FRAME_SECONDS: f32 = 0.05;
const STAGE_CLEAR_LIFE_BONUS: u32 = 1000;
const MAX_PLAYER_LIVES: i32 = 9;
const ENEMY_ALIGNMENT_FIRE_FRACTION: f32 = 0.45;
const VERSUS_POWERUP_INTERVAL_SECONDS: f32 = 8.0;
const SOUND_SAMPLE_RATE: u32 = 22_050;
const MAX_RETRO_SOUND_SECONDS: f32 = 1.0;
const MAX_RETRO_SOUND_FREQUENCY: f32 = 4_000.0;
const MAX_RETRO_SOUND_VOLUME: f32 = 1.0;
const BACKGROUND_MUSIC_VOLUME: f32 = 0.18;
const BACKGROUND_MUSIC_STEP_SECONDS: f32 = 0.12;
const ICE_SPEED_MULTIPLIER: f32 = 1.18;
const SPAWN_SHIMMER_FRAME_SECONDS: f32 = 0.08;

fn window_scale() -> f32 {
    *WINDOW_SCALE_CACHE.get_or_init(configured_window_scale)
}

fn configured_window_scale() -> f32 {
    let requested = std::env::var(WINDOW_SCALE_ENV).ok();
    parse_window_scale(requested.as_deref()) as f32
}

fn parse_window_scale(value: Option<&str>) -> u32 {
    value
        .and_then(|raw| {
            let trimmed = raw.trim();
            let numeric = trimmed
                .strip_suffix('x')
                .or_else(|| trimmed.strip_suffix('X'))
                .unwrap_or(trimmed);
            numeric.parse::<u32>().ok()
        })
        .filter(|scale| (MIN_WINDOW_SCALE..=MAX_WINDOW_SCALE).contains(scale))
        .unwrap_or(DEFAULT_WINDOW_SCALE)
}

fn virtual_window_size(scale: f32) -> (u32, u32) {
    (
        (VIRTUAL_WIDTH * scale).round() as u32,
        (VIRTUAL_HEIGHT * scale).round() as u32,
    )
}

fn main() {
    let window_size = virtual_window_size(window_scale());

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerControl::default())
        .insert_resource(GameMode::Campaign)
        .insert_resource(ModeSelect::default())
        .insert_resource(GameStatus::default())
        .insert_resource(EnemyFreeze::default())
        .insert_resource(VersusPlayerFreeze::default())
        .insert_resource(BaseReinforcement::default())
        .insert_resource(VersusPowerUpDirector::inactive())
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tank 1990 Bevy Remake".into(),
                        resolution: window_size.into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_audio_source::<RetroSound>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            spawn_versus_powerups
                .after(cancel_colliding_bullets)
                .before(pickup_powerups),
        )
        .add_systems(
            FixedUpdate,
            advance_after_stage_intro
                .after(update_player_control)
                .before(spawn_enemies),
        )
        .add_systems(
            FixedUpdate,
            update_versus_frozen_player_visuals
                .after(tick_shields)
                .before(update_enemy_visual_feedback),
        )
        .add_systems(
            FixedUpdate,
            update_base_reinforcement_visuals
                .after(tick_powerup_effects)
                .before(update_powerup_visuals),
        )
        .add_systems(
            FixedUpdate,
            sync_background_music.after(update_status_panel),
        )
        .add_systems(
            FixedUpdate,
            clear_terminal_transients
                .after(check_game_phase)
                .before(advance_after_level_clear),
        )
        .add_systems(FixedUpdate, tick_destroyed_tanks.after(animate_sprites))
        .add_systems(
            FixedUpdate,
            (
                handle_shared_controls,
                update_player_control,
                spawn_enemies,
                move_player_tank,
                move_enemy_tanks,
                fire_player_bullet,
                fire_enemy_bullets,
                move_bullets,
                cancel_colliding_bullets,
                pickup_powerups,
                tick_powerup_effects,
                update_powerup_visuals,
                animate_sprites,
                tick_spawn_protections,
                tick_player_respawns,
                tick_shields,
                update_enemy_visual_feedback,
                check_game_phase,
                advance_after_level_clear,
                update_status_panel,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct SpriteAssets {
    manifest: AssetManifest,
    terrain_image: Handle<Image>,
    terrain_layout: Handle<TextureAtlasLayout>,
    tank_image: Handle<Image>,
    tank_layout: Handle<TextureAtlasLayout>,
    bullet_image: Handle<Image>,
    bullet_layout: Handle<TextureAtlasLayout>,
    effect_image: Handle<Image>,
    effect_layout: Handle<TextureAtlasLayout>,
    powerup_image: Handle<Image>,
    powerup_layout: Handle<TextureAtlasLayout>,
    glyph_image: Handle<Image>,
    glyph_layout: Handle<TextureAtlasLayout>,
    base_intact: Handle<Image>,
    base_destroyed: Handle<Image>,
    score_badge_icon: Handle<Image>,
    stage_flag_icon: Handle<Image>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct AssetManifest {
    tanks: TankSpriteManifest,
    bullets: DirectionalSpriteManifest,
    atlases: GeneratedAtlasesManifest,
    terrain: TerrainSpriteManifest,
    effects: EffectSpriteManifest,
    powerups: PowerUpSpriteManifest,
    base: BaseSpriteManifest,
    ui: UiSpriteManifest,
    glyphs: GlyphManifest,
    sounds: SoundManifest,
}

impl AssetManifest {
    fn tank_index(&self, set: TankSpriteSet, direction: Direction, frame: usize) -> usize {
        self.tanks.frames_for(set)[frame.min(TANK_ANIMATION_FRAMES - 1)].index(direction)
    }

    fn bullet_index(&self, direction: Direction) -> usize {
        self.bullets.index(direction)
    }

    fn terrain_index(&self, tile: TileKind) -> Option<usize> {
        match tile {
            TileKind::Brick => Some(self.terrain.brick),
            TileKind::Steel => Some(self.terrain.steel),
            TileKind::Water => Some(self.terrain.water.first),
            TileKind::Forest => Some(self.terrain.forest),
            TileKind::Ice => Some(self.terrain.ice),
            TileKind::Empty | TileKind::Base => None,
        }
    }

    fn terrain_animation_frames(&self, tile: TileKind) -> Option<SpriteFrameRange> {
        match tile {
            TileKind::Water => Some(self.terrain.water),
            _ => None,
        }
    }

    fn powerup_index(&self, kind: PowerUpKind) -> usize {
        match kind {
            PowerUpKind::Star => self.powerups.star,
            PowerUpKind::Helmet => self.powerups.helmet,
            PowerUpKind::Clock => self.powerups.clock,
            PowerUpKind::Grenade => self.powerups.grenade,
            PowerUpKind::Shovel => self.powerups.shovel,
            PowerUpKind::Tank => self.powerups.tank,
        }
    }

    fn explosion_frames(&self) -> SpriteFrameRange {
        self.effects.explosion
    }

    fn spawn_shimmer_frames(&self) -> SpriteFrameRange {
        self.effects.spawn_shimmer
    }

    fn base_destruction_frames(&self) -> SpriteFrameRange {
        self.effects.base_destruction
    }

    fn powerup_sparkle_frames(&self) -> SpriteFrameRange {
        self.effects.powerup_sparkle
    }

    fn bullet_impact_frames(&self) -> SpriteFrameRange {
        self.effects.bullet_impact
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct TankSpriteManifest {
    player1: Vec<DirectionalSpriteManifest>,
    player2: Vec<DirectionalSpriteManifest>,
    enemies: EnemyTankSpriteManifest,
}

impl TankSpriteManifest {
    fn frames_for(&self, set: TankSpriteSet) -> &[DirectionalSpriteManifest] {
        match set {
            TankSpriteSet::Player1 => &self.player1,
            TankSpriteSet::Player2 => &self.player2,
            TankSpriteSet::EnemyBasic => &self.enemies.basic,
            TankSpriteSet::EnemyFast => &self.enemies.fast,
            TankSpriteSet::EnemyPower => &self.enemies.power,
            TankSpriteSet::EnemyArmor => &self.enemies.armor,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct EnemyTankSpriteManifest {
    basic: Vec<DirectionalSpriteManifest>,
    fast: Vec<DirectionalSpriteManifest>,
    power: Vec<DirectionalSpriteManifest>,
    armor: Vec<DirectionalSpriteManifest>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct DirectionalSpriteManifest {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl DirectionalSpriteManifest {
    fn index(self, direction: Direction) -> usize {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct TerrainSpriteManifest {
    brick: usize,
    steel: usize,
    water: SpriteFrameRange,
    forest: usize,
    ice: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct EffectSpriteManifest {
    explosion: SpriteFrameRange,
    spawn_shimmer: SpriteFrameRange,
    base_destruction: SpriteFrameRange,
    powerup_sparkle: SpriteFrameRange,
    bullet_impact: SpriteFrameRange,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct SpriteFrameRange {
    first: usize,
    last: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PowerUpSpriteManifest {
    star: usize,
    helmet: usize,
    clock: usize,
    grenade: usize,
    shovel: usize,
    tank: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct GeneratedAtlasManifest {
    tile_width: usize,
    tile_height: usize,
    tiles: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct GeneratedAtlasesManifest {
    tanks: GeneratedAtlasManifest,
    terrain: GeneratedAtlasManifest,
    bullets: GeneratedAtlasManifest,
    effects: GeneratedAtlasManifest,
    powerups: GeneratedAtlasManifest,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct GeneratedSpriteManifest {
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct BaseSpriteManifest {
    intact: GeneratedSpriteManifest,
    destroyed: GeneratedSpriteManifest,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct UiSpriteManifest {
    score_badge: GeneratedSpriteManifest,
    stage_flag: GeneratedSpriteManifest,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct GlyphManifest {
    characters: String,
    tile_width: usize,
    tile_height: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct SoundManifest {
    fire: RetroSoundSpec,
    brick_hit: RetroSoundSpec,
    steel_hit: RetroSoundSpec,
    tank_explosion: RetroSoundSpec,
    base_destroyed: RetroSoundSpec,
    powerup_pickup: RetroSoundSpec,
    stage_start: RetroSoundSpec,
    level_clear: RetroSoundSpec,
    game_over: RetroSoundSpec,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
enum RetroSoundSpec {
    Sweep {
        duration_secs: f32,
        start_frequency: f32,
        end_frequency: f32,
        volume: f32,
    },
    Noise {
        duration_secs: f32,
        volume: f32,
        seed: u32,
    },
    Layered {
        notes: Vec<SoundNote>,
    },
}

#[derive(Resource)]
struct SoundAssets {
    sound_enabled: bool,
    fire: SoundHandle,
    brick_hit: SoundHandle,
    steel_hit: SoundHandle,
    tank_explosion: SoundHandle,
    base_destroyed: SoundHandle,
    powerup_pickup: SoundHandle,
    stage_start: SoundHandle,
    level_clear: SoundHandle,
    game_over: SoundHandle,
    background_music: SoundHandle,
}

#[derive(Clone)]
enum SoundHandle {
    Retro(Handle<RetroSound>),
    File(Handle<AudioSource>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SoundKind {
    Fire,
    BrickHit,
    SteelHit,
    TankExplosion,
    BaseDestroyed,
    PowerupPickup,
    StageStart,
    LevelClear,
    GameOver,
}

const CAMPAIGN_BASE_DESTROYED_SOUNDS: [SoundKind; 2] =
    [SoundKind::BaseDestroyed, SoundKind::GameOver];
const VERSUS_BASE_DESTROYED_SOUNDS: [SoundKind; 2] =
    [SoundKind::BaseDestroyed, SoundKind::LevelClear];
const NO_BASE_DESTROYED_SOUNDS: [SoundKind; 0] = [];

#[derive(Asset, TypePath)]
struct RetroSound {
    samples: Arc<[f32]>,
    sample_rate: u32,
}

struct RetroSoundDecoder {
    samples: Arc<[f32]>,
    sample_rate: u32,
    cursor: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AudioMode {
    Bgm,
    Classic,
}

fn next_audio_mode(mode: AudioMode) -> AudioMode {
    match mode {
        AudioMode::Bgm => AudioMode::Classic,
        AudioMode::Classic => AudioMode::Bgm,
    }
}

fn audio_mode_label(mode: AudioMode) -> &'static str {
    match mode {
        AudioMode::Bgm => "BGM",
        AudioMode::Classic => "CLASSIC",
    }
}

fn toggle_sound_enabled(enabled: bool) -> bool {
    !enabled
}

fn sound_enabled_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

#[derive(Component)]
struct BackgroundMusic;

impl Iterator for RetroSoundDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.samples.get(self.cursor).copied();
        self.cursor += usize::from(sample.is_some());
        sample
    }
}

impl Source for RetroSoundDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len().saturating_sub(self.cursor))
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32,
        ))
    }
}

impl Decodable for RetroSound {
    type DecoderItem = f32;
    type Decoder = RetroSoundDecoder;

    fn decoder(&self) -> Self::Decoder {
        RetroSoundDecoder {
            samples: self.samples.clone(),
            sample_rate: self.sample_rate,
            cursor: 0,
        }
    }
}

#[derive(Resource)]
struct PlayerControl {
    p1_last_direction: Direction,
    p2_last_direction: Direction,
    p1_direction_priority: Vec<Direction>,
    p2_direction_priority: Vec<Direction>,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            p1_last_direction: Direction::Up,
            p2_last_direction: Direction::Down,
            p1_direction_priority: Vec::new(),
            p2_direction_priority: Vec::new(),
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
enum GameMode {
    Campaign,
    VersusDeathmatch,
    VersusBaseBattle,
}

impl GameMode {
    fn is_versus(self) -> bool {
        matches!(self, Self::VersusDeathmatch | Self::VersusBaseBattle)
    }

    fn mode_select_option(self) -> ModeSelectOption {
        match self {
            Self::Campaign => ModeSelectOption::Campaign,
            Self::VersusDeathmatch | Self::VersusBaseBattle => ModeSelectOption::Battle,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ModeSelectOption {
    Campaign,
    Battle,
    Music,
    Sound,
}

#[derive(Resource)]
struct ModeSelect {
    selected: ModeSelectOption,
    stage: usize,
    arena: usize,
    audio_mode: AudioMode,
    sound_enabled: bool,
}

impl Default for ModeSelect {
    fn default() -> Self {
        Self {
            selected: ModeSelectOption::Campaign,
            stage: 1,
            arena: DEFAULT_VERSUS_ARENA,
            audio_mode: AudioMode::Bgm,
            sound_enabled: true,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlayerId {
    One,
    Two,
}

impl PlayerId {
    fn team(self) -> Team {
        match self {
            Self::One => Team::Player1,
            Self::Two => Team::Player2,
        }
    }

    fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

#[derive(Resource)]
struct GameStatus {
    phase: GamePhase,
    stage: usize,
    arena: usize,
    winner: Option<PlayerId>,
    transition_timer: Timer,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self {
            phase: GamePhase::ModeSelect,
            stage: 1,
            arena: DEFAULT_VERSUS_ARENA,
            winner: None,
            transition_timer: Timer::from_seconds(LEVEL_CLEAR_DELAY_SECONDS, TimerMode::Once),
        }
    }
}

impl GameStatus {
    fn is_playing(&self) -> bool {
        self.phase == GamePhase::Playing
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GamePhase {
    ModeSelect,
    StageIntro,
    Playing,
    Paused,
    GameOver,
    LevelClear,
    RoundOver,
    Victory,
}

#[derive(Resource)]
struct ScoreBoard {
    score: u32,
    lives: i32,
    enemies_destroyed: usize,
    total_enemies: usize,
    enemy_kills: EnemyKillCounts,
    p1_score: u32,
    p2_score: u32,
    p1_lives: i32,
    p2_lives: i32,
    target_score: u32,
    respawn_invulnerability_secs: f32,
}

impl ScoreBoard {
    fn campaign(total_enemies: usize) -> Self {
        Self {
            score: 0,
            lives: 3,
            enemies_destroyed: 0,
            total_enemies,
            enemy_kills: EnemyKillCounts::default(),
            p1_score: 0,
            p2_score: 0,
            p1_lives: 3,
            p2_lives: 0,
            target_score: 0,
            respawn_invulnerability_secs: DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
        }
    }

    fn versus(lives: i32, target_score: u32, respawn_invulnerability_secs: f32) -> Self {
        Self {
            score: 0,
            lives,
            enemies_destroyed: 0,
            total_enemies: 0,
            enemy_kills: EnemyKillCounts::default(),
            p1_score: 0,
            p2_score: 0,
            p1_lives: lives,
            p2_lives: lives,
            target_score,
            respawn_invulnerability_secs,
        }
    }

    fn player_score(&self, player: PlayerId) -> u32 {
        match player {
            PlayerId::One => self.p1_score,
            PlayerId::Two => self.p2_score,
        }
    }

    fn add_player_score(&mut self, player: PlayerId) {
        match player {
            PlayerId::One => self.p1_score += 1,
            PlayerId::Two => self.p2_score += 1,
        }
    }

    fn record_enemy_destroyed(&mut self, kind: EnemyKind) {
        self.score += enemy_score(kind);
        self.enemies_destroyed += 1;
        self.enemy_kills.add(kind);
    }

    fn set_player_lives(&mut self, player: PlayerId, lives: i32) {
        match player {
            PlayerId::One => self.p1_lives = lives,
            PlayerId::Two => self.p2_lives = lives,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct EnemyKillCounts {
    basic: usize,
    fast: usize,
    power: usize,
    armor: usize,
}

impl EnemyKillCounts {
    fn add(&mut self, kind: EnemyKind) {
        *self.count_mut(kind) += 1;
    }

    fn count(self, kind: EnemyKind) -> usize {
        match kind {
            EnemyKind::Basic => self.basic,
            EnemyKind::Fast => self.fast,
            EnemyKind::Power => self.power,
            EnemyKind::Armor => self.armor,
        }
    }

    fn total(self) -> usize {
        self.basic + self.fast + self.power + self.armor
    }

    fn count_mut(&mut self, kind: EnemyKind) -> &mut usize {
        match kind {
            EnemyKind::Basic => &mut self.basic,
            EnemyKind::Fast => &mut self.fast,
            EnemyKind::Power => &mut self.power,
            EnemyKind::Armor => &mut self.armor,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Resource)]
struct StageRules {
    player_steel_destruction: bool,
}

impl StageRules {
    fn from_level(level: &LevelDefinition) -> Self {
        Self {
            player_steel_destruction: level.player_steel_destruction,
        }
    }
}

#[derive(Resource, Default)]
struct EnemyFreeze {
    timer: Option<Timer>,
}

impl EnemyFreeze {
    fn start(&mut self) {
        self.timer = Some(Timer::from_seconds(CLOCK_SECONDS, TimerMode::Once));
    }

    fn reset(&mut self) {
        self.timer = None;
    }

    fn is_active(&self) -> bool {
        self.timer
            .as_ref()
            .is_some_and(|timer| !timer.is_finished())
    }

    fn tick(&mut self, delta: Duration) {
        let Some(timer) = &mut self.timer else {
            return;
        };
        timer.tick(delta);
        if timer.is_finished() {
            self.timer = None;
        }
    }
}

#[derive(Resource, Default)]
struct VersusPlayerFreeze {
    frozen_player: Option<PlayerId>,
    timer: Option<Timer>,
}

impl VersusPlayerFreeze {
    fn start(&mut self, player: PlayerId) {
        self.frozen_player = Some(player);
        self.timer = Some(Timer::from_seconds(CLOCK_SECONDS, TimerMode::Once));
    }

    fn reset(&mut self) {
        self.frozen_player = None;
        self.timer = None;
    }

    fn is_player_frozen(&self, player: PlayerId) -> bool {
        self.frozen_player == Some(player)
            && self
                .timer
                .as_ref()
                .is_some_and(|timer| !timer.is_finished())
    }

    fn tick(&mut self, delta: Duration) {
        let Some(timer) = &mut self.timer else {
            return;
        };
        timer.tick(delta);
        if timer.is_finished() {
            self.reset();
        }
    }
}

#[derive(Resource, Default)]
struct BaseReinforcement {
    timer: Option<Timer>,
    saved_tiles: Vec<(usize, usize, TileKind)>,
}

impl BaseReinforcement {
    fn start(&mut self) {
        self.timer = Some(Timer::from_seconds(SHOVEL_SECONDS, TimerMode::Once));
    }

    fn reset(&mut self) {
        self.timer = None;
        self.saved_tiles.clear();
    }

    fn tick(&mut self, delta: Duration) -> bool {
        let Some(timer) = &mut self.timer else {
            return false;
        };
        timer.tick(delta);
        timer.is_finished()
    }

    fn warning_elapsed_secs(&self) -> Option<f32> {
        let timer = self.timer.as_ref()?;
        (timer.remaining_secs() <= SHOVEL_WARNING_SECONDS).then_some(timer.elapsed_secs())
    }

    fn contains_position(&self, x: usize, y: usize) -> bool {
        self.saved_tiles
            .iter()
            .any(|(tile_x, tile_y, _)| *tile_x == x && *tile_y == y)
    }
}

#[derive(Resource)]
struct EnemyDirector {
    roster: VecDeque<EnemyRosterEntry>,
    spawns: Vec<SpawnPoint>,
    spawn_timer: Timer,
    max_active: usize,
    spawn_cursor: usize,
    spawned_count: usize,
}

impl EnemyDirector {
    fn from_level(level: &LevelDefinition) -> Self {
        Self {
            roster: level
                .enemies
                .iter()
                .enumerate()
                .map(|(index, kind)| EnemyRosterEntry {
                    kind: *kind,
                    carried_powerup: carrier_powerup_for_spawn(index + 1, &level.powerup_carriers),
                })
                .collect(),
            spawns: level.enemy_spawns.clone(),
            spawn_timer: Timer::from_seconds(level.spawn_interval_secs, TimerMode::Repeating),
            max_active: level.max_enemies_on_screen,
            spawn_cursor: 0,
            spawned_count: 0,
        }
    }

    fn inactive() -> Self {
        Self {
            roster: VecDeque::new(),
            spawns: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_active: 0,
            spawn_cursor: 0,
            spawned_count: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct EnemyRosterEntry {
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
}

#[derive(Resource)]
struct VersusPowerUpDirector {
    spawn_points: Vec<Vec2>,
    spawn_timer: Timer,
    spawn_cursor: usize,
    kind_cursor: usize,
    spawn_immediately: bool,
}

impl VersusPowerUpDirector {
    fn from_arena(arena: &ArenaDefinition) -> Self {
        Self {
            spawn_points: arena
                .powerup_spawns
                .iter()
                .map(|point| Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE))
                .collect(),
            spawn_timer: Timer::from_seconds(VERSUS_POWERUP_INTERVAL_SECONDS, TimerMode::Repeating),
            spawn_cursor: 0,
            kind_cursor: 0,
            spawn_immediately: true,
        }
    }

    fn inactive() -> Self {
        Self {
            spawn_points: Vec::new(),
            spawn_timer: Timer::from_seconds(VERSUS_POWERUP_INTERVAL_SECONDS, TimerMode::Repeating),
            spawn_cursor: 0,
            kind_cursor: 0,
            spawn_immediately: false,
        }
    }

    fn next_spawn(&mut self) -> Option<(Vec2, PowerUpKind)> {
        if self.spawn_points.is_empty() {
            return None;
        }

        let top_left = self.spawn_points[self.spawn_cursor];
        self.spawn_cursor = (self.spawn_cursor + 1) % self.spawn_points.len();
        let kind = powerup_for_cycle(self.kind_cursor);
        self.kind_cursor += 1;
        self.spawn_immediately = false;
        self.spawn_timer.reset();
        Some((top_left, kind))
    }
}

#[derive(Resource, Clone)]
struct TileGrid {
    tiles: Vec<TileKind>,
}

impl TileGrid {
    fn empty() -> Self {
        Self {
            tiles: vec![TileKind::Empty; BOARD_TILES * BOARD_TILES],
        }
    }

    fn from_level(level: &LevelDefinition) -> Result<Self, String> {
        Self::from_map(&level.map)
    }

    fn from_arena(arena: &ArenaDefinition) -> Result<Self, String> {
        Self::from_map(&arena.map)
    }

    fn from_map(map: &[String]) -> Result<Self, String> {
        if map.len() != BOARD_TILES {
            return Err(format!(
                "expected {BOARD_TILES} map rows, got {}",
                map.len()
            ));
        }

        let mut tiles = Vec::with_capacity(BOARD_TILES * BOARD_TILES);
        for (y, row) in map.iter().enumerate() {
            let chars: Vec<char> = row.chars().collect();
            if chars.len() != BOARD_TILES {
                return Err(format!(
                    "expected row {y} to have {BOARD_TILES} columns, got {}",
                    chars.len()
                ));
            }
            for ch in chars {
                tiles.push(TileKind::from_symbol(ch)?);
            }
        }

        Ok(Self { tiles })
    }

    fn get(&self, x: i32, y: i32) -> Option<TileKind> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= BOARD_TILES || y >= BOARD_TILES {
            return None;
        }
        Some(self.tiles[y * BOARD_TILES + x])
    }

    fn set(&mut self, x: usize, y: usize, tile: TileKind) {
        self.tiles[y * BOARD_TILES + x] = tile;
    }

    fn can_tank_occupy(&self, top_left: Vec2) -> bool {
        if top_left.x < 0.0
            || top_left.y < 0.0
            || top_left.x + TANK_SIZE > board_size()
            || top_left.y + TANK_SIZE > board_size()
        {
            return false;
        }

        let left = (top_left.x / TILE_SIZE).floor() as i32;
        let right = ((top_left.x + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;
        let top = (top_left.y / TILE_SIZE).floor() as i32;
        let bottom = ((top_left.y + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;

        for y in top..=bottom {
            for x in left..=right {
                if !self.get(x, y).is_some_and(TileKind::tank_passable) {
                    return false;
                }
            }
        }

        true
    }

    fn tank_overlaps_tile(&self, top_left: Vec2, tile: TileKind) -> bool {
        let left = (top_left.x / TILE_SIZE).floor() as i32;
        let right = ((top_left.x + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;
        let top = (top_left.y / TILE_SIZE).floor() as i32;
        let bottom = ((top_left.y + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;

        for y in top..=bottom {
            for x in left..=right {
                if self.get(x, y) == Some(tile) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TileKind {
    Empty,
    Brick,
    Steel,
    Water,
    Forest,
    Ice,
    Base,
}

impl TileKind {
    fn from_symbol(symbol: char) -> Result<Self, String> {
        match symbol {
            '.' => Ok(Self::Empty),
            'B' => Ok(Self::Brick),
            'S' => Ok(Self::Steel),
            'W' => Ok(Self::Water),
            'F' => Ok(Self::Forest),
            'I' => Ok(Self::Ice),
            'E' => Ok(Self::Base),
            other => Err(format!("unknown tile symbol {other:?}")),
        }
    }

    fn tank_passable(self) -> bool {
        matches!(self, Self::Empty | Self::Forest | Self::Ice)
    }

    fn bullet_blocks(self) -> bool {
        matches!(self, Self::Brick | Self::Steel | Self::Base)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn movement(self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0.0, -1.0),
            Self::Down => Vec2::new(0.0, 1.0),
            Self::Left => Vec2::new(-1.0, 0.0),
            Self::Right => Vec2::new(1.0, 0.0),
        }
    }
}

#[derive(Deserialize)]
struct LevelDefinition {
    name: String,
    map: Vec<String>,
    player_spawn: SpawnPoint,
    base_position: GridPoint,
    enemy_spawns: Vec<SpawnPoint>,
    enemies: Vec<EnemyKind>,
    powerup_carriers: Vec<PowerUpCarrier>,
    spawn_interval_secs: f32,
    max_enemies_on_screen: usize,
    #[serde(default)]
    player_steel_destruction: bool,
}

#[derive(Deserialize)]
struct ArenaDefinition {
    name: String,
    map: Vec<String>,
    p1_spawn: SpawnPoint,
    p2_spawn: SpawnPoint,
    battle_rules: BattleRules,
    powerup_spawns: Vec<GridPoint>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
enum BattleRules {
    Deathmatch {
        target_score: u32,
        lives: i32,
        respawn_invulnerability_secs: f32,
    },
    BaseBattle {
        p1_base: GridPoint,
        p2_base: GridPoint,
        lives: i32,
        respawn_invulnerability_secs: f32,
    },
}

#[derive(Clone, Deserialize)]
struct SpawnPoint {
    x: usize,
    y: usize,
    facing: Direction,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
struct GridPoint {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
struct PowerUpCarrier {
    enemy: usize,
    kind: PowerUpKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
enum EnemyKind {
    Basic,
    Fast,
    Power,
    Armor,
}

#[derive(Component)]
struct Player {
    id: PlayerId,
}

#[derive(Component)]
struct GameEntity;

#[derive(Component)]
struct EnemyTank {
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
}

#[derive(Component)]
struct EnemyAi {
    turn_timer: Timer,
    fire_timer: Timer,
}

#[derive(Component)]
struct SpawnProtection {
    timer: Timer,
}

impl SpawnProtection {
    fn for_spawn_shimmer(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(spawn_shimmer_duration_secs(frames), TimerMode::Once),
        }
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
struct PlayerRespawnDelay {
    timer: Timer,
}

impl PlayerRespawnDelay {
    fn for_spawn_shimmer(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(spawn_shimmer_duration_secs(frames), TimerMode::Once),
        }
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
struct PlayerRespawnPending {
    timer: Timer,
}

impl PlayerRespawnPending {
    fn for_explosion(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(explosion_duration_secs(frames), TimerMode::Once),
        }
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
struct Tank {
    top_left: Vec2,
    facing: Direction,
    speed: f32,
}

#[derive(Component)]
struct TankSpriteState {
    set: TankSpriteSet,
    frame: usize,
    timer: Timer,
}

impl TankSpriteState {
    fn new(set: TankSpriteSet) -> Self {
        Self {
            set,
            frame: 0,
            timer: Timer::from_seconds(0.14, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct Bullet {
    previous_top_left: Vec2,
    top_left: Vec2,
    facing: Direction,
    owner: Team,
    speed: f32,
    breaks_steel: bool,
    resolved: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BulletTileHit {
    x: usize,
    y: usize,
    tile: TileKind,
    impact_top_left: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BulletPathClash {
    impact_top_left: Vec2,
    time: f32,
}

#[derive(Component)]
struct EnemyBulletSource {
    shooter: Entity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Team {
    Player1,
    Player2,
    Enemy,
}

impl Team {
    fn player_id(self) -> Option<PlayerId> {
        match self {
            Self::Player1 => Some(PlayerId::One),
            Self::Player2 => Some(PlayerId::Two),
            Self::Enemy => None,
        }
    }

    fn is_player(self) -> bool {
        self.player_id().is_some()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TankSpriteSet {
    Player1,
    Player2,
    EnemyBasic,
    EnemyFast,
    EnemyPower,
    EnemyArmor,
}

impl TankSpriteSet {
    fn player(player: PlayerId) -> Self {
        match player {
            PlayerId::One => Self::Player1,
            PlayerId::Two => Self::Player2,
        }
    }

    fn enemy(kind: EnemyKind) -> Self {
        match kind {
            EnemyKind::Basic => Self::EnemyBasic,
            EnemyKind::Fast => Self::EnemyFast,
            EnemyKind::Power => Self::EnemyPower,
            EnemyKind::Armor => Self::EnemyArmor,
        }
    }
}

#[derive(Component)]
struct Health {
    current: i32,
}

#[derive(Component)]
struct RespawnPoint {
    top_left: Vec2,
    facing: Direction,
}

#[derive(Component)]
struct PlayerLives {
    current: i32,
}

#[derive(Component)]
struct PlayerUpgrade {
    level: u8,
}

#[derive(Component)]
struct Shield {
    timer: Timer,
}

#[derive(Component)]
struct GridTile {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct BaseSprite {
    owner: Option<PlayerId>,
    top_left: Vec2,
}

#[derive(Component)]
struct StatusGlyph {
    kind: StatusValue,
    digit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StatusValue {
    Score,
    Lives,
    Stage,
    P2Score,
    P2Lives,
    Arena,
    Target,
}

#[derive(Component)]
struct EnemyMarker {
    index: usize,
}

#[derive(Component)]
struct PhaseBanner;

#[derive(Component)]
struct ModeSelectCursor;

#[derive(Component)]
struct ModeSelectStageGlyph {
    digit: usize,
}

#[derive(Component)]
struct ModeSelectArenaGlyph {
    digit: usize,
}

#[derive(Component)]
struct ModeSelectBattleKindGlyph {
    digit: usize,
}

#[derive(Component)]
struct ModeSelectMusicGlyph {
    digit: usize,
}

#[derive(Component)]
struct ModeSelectSoundGlyph {
    digit: usize,
}

#[derive(Component)]
struct SpriteAnimation {
    first: usize,
    last: usize,
    timer: Timer,
    despawn_on_finish: bool,
}

#[derive(Component)]
struct DestroyedTank {
    timer: Timer,
}

impl DestroyedTank {
    fn for_explosion(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(explosion_duration_secs(frames), TimerMode::Once),
        }
    }

    fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
struct PowerUp {
    kind: PowerUpKind,
}

#[derive(Component)]
struct PowerUpSparkle;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
enum PowerUpKind {
    Star,
    Helmet,
    Clock,
    Grenade,
    Shovel,
    Tank,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mode_select: Res<ModeSelect>,
    mut images: ResMut<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut retro_sounds: ResMut<Assets<RetroSound>>,
) {
    commands.spawn(Camera2d);

    let sprite_assets = create_sprite_assets(&asset_server, &mut images, &mut atlas_layouts);
    let sound_assets =
        create_sound_assets(&asset_server, &mut retro_sounds, &sprite_assets.manifest);
    spawn_mode_select_screen(
        &mut commands,
        &sprite_assets,
        ModeSelectOption::Campaign,
        1,
        DEFAULT_VERSUS_ARENA,
        mode_select.audio_mode,
        mode_select.sound_enabled,
    );

    commands.insert_resource(sprite_assets);
    commands.insert_resource(sound_assets);
    commands.insert_resource(TileGrid::empty());
    commands.insert_resource(StageRules::default());
    commands.insert_resource(EnemyDirector::inactive());
    commands.insert_resource(ScoreBoard::campaign(0));
}

fn handle_shared_controls(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    mut sounds: ResMut<SoundAssets>,
    mut game_mode: ResMut<GameMode>,
    mut game_status: ResMut<GameStatus>,
    mut tile_grid: ResMut<TileGrid>,
    mut director: ResMut<EnemyDirector>,
    mut score_board: ResMut<ScoreBoard>,
    mut stage_rules: ResMut<StageRules>,
    mut versus_powerups: ResMut<VersusPowerUpDirector>,
    mut mode_select: ResMut<ModeSelect>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    mut menu_queries: ParamSet<(
        Query<Entity, With<GameEntity>>,
        Query<&mut Transform, With<ModeSelectCursor>>,
        Query<(&ModeSelectStageGlyph, &mut Sprite)>,
        Query<(&ModeSelectArenaGlyph, &mut Sprite)>,
        Query<(&ModeSelectBattleKindGlyph, &mut Sprite)>,
        Query<(&ModeSelectMusicGlyph, &mut Sprite)>,
        Query<(&ModeSelectSoundGlyph, &mut Sprite)>,
    )>,
) {
    if game_status.phase == GamePhase::ModeSelect {
        if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
            mode_select.selected = previous_mode_select_option(mode_select.selected);
            update_mode_select_cursor(&mut menu_queries.p1(), mode_select.selected);
        }

        if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
            mode_select.selected = next_mode_select_option(mode_select.selected);
            update_mode_select_cursor(&mut menu_queries.p1(), mode_select.selected);
        }

        if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
            match mode_select.selected {
                ModeSelectOption::Campaign => {
                    mode_select.stage = previous_stage(mode_select.stage);
                    update_mode_select_stage_digits(
                        &mut menu_queries.p2(),
                        &assets.manifest.glyphs,
                        mode_select.stage,
                    );
                }
                ModeSelectOption::Battle => {
                    mode_select.arena = previous_arena(mode_select.arena);
                    update_mode_select_arena_digits(
                        &mut menu_queries.p3(),
                        &assets.manifest.glyphs,
                        mode_select.arena,
                    );
                    update_mode_select_battle_kind(
                        &mut menu_queries.p4(),
                        &assets.manifest.glyphs,
                        mode_select.arena,
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode = next_audio_mode(mode_select.audio_mode);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                }
                ModeSelectOption::Sound => {
                    mode_select.sound_enabled = toggle_sound_enabled(mode_select.sound_enabled);
                    sounds.sound_enabled = mode_select.sound_enabled;
                    update_mode_select_sound_value(
                        &mut menu_queries.p6(),
                        &assets.manifest.glyphs,
                        mode_select.sound_enabled,
                    );
                }
            }
        }

        if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
            match mode_select.selected {
                ModeSelectOption::Campaign => {
                    mode_select.stage = next_stage(mode_select.stage);
                    update_mode_select_stage_digits(
                        &mut menu_queries.p2(),
                        &assets.manifest.glyphs,
                        mode_select.stage,
                    );
                }
                ModeSelectOption::Battle => {
                    mode_select.arena = next_arena(mode_select.arena);
                    update_mode_select_arena_digits(
                        &mut menu_queries.p3(),
                        &assets.manifest.glyphs,
                        mode_select.arena,
                    );
                    update_mode_select_battle_kind(
                        &mut menu_queries.p4(),
                        &assets.manifest.glyphs,
                        mode_select.arena,
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode = next_audio_mode(mode_select.audio_mode);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                }
                ModeSelectOption::Sound => {
                    mode_select.sound_enabled = toggle_sound_enabled(mode_select.sound_enabled);
                    sounds.sound_enabled = mode_select.sound_enabled;
                    update_mode_select_sound_value(
                        &mut menu_queries.p6(),
                        &assets.manifest.glyphs,
                        mode_select.sound_enabled,
                    );
                }
            }
        }

        if keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::Enter)
            || keys.just_pressed(KeyCode::ShiftRight)
        {
            match mode_select.selected {
                ModeSelectOption::Campaign => {
                    game_status.stage = selected_campaign_stage(&mode_select);
                    *game_mode = GameMode::Campaign;
                    restart_level(
                        &mut commands,
                        &assets,
                        &sounds,
                        &mut game_status,
                        &mut tile_grid,
                        &mut director,
                        &mut score_board,
                        &mut stage_rules,
                        &mut versus_powerups,
                        &mut enemy_freeze,
                        &mut versus_freeze,
                        &mut base_reinforcement,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Battle => {
                    game_status.arena = mode_select.arena;
                    start_versus_round(
                        &mut commands,
                        &assets,
                        &sounds,
                        &mut game_mode,
                        &mut game_status,
                        &mut tile_grid,
                        &mut director,
                        &mut score_board,
                        &mut stage_rules,
                        &mut versus_powerups,
                        &mut enemy_freeze,
                        &mut versus_freeze,
                        &mut base_reinforcement,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode = next_audio_mode(mode_select.audio_mode);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                }
                ModeSelectOption::Sound => {
                    mode_select.sound_enabled = toggle_sound_enabled(mode_select.sound_enabled);
                    sounds.sound_enabled = mode_select.sound_enabled;
                    update_mode_select_sound_value(
                        &mut menu_queries.p6(),
                        &assets.manifest.glyphs,
                        mode_select.sound_enabled,
                    );
                }
            }
        }
        return;
    }

    if keys.just_pressed(KeyCode::Escape) {
        game_status.phase = toggle_pause_phase(game_status.phase);
    }

    if keys.just_pressed(KeyCode::KeyM) {
        enter_mode_select(
            &mut commands,
            &assets,
            &mut game_status,
            &mut tile_grid,
            &mut director,
            &mut score_board,
            &mut stage_rules,
            &mut versus_powerups,
            &mut mode_select,
            &mut enemy_freeze,
            &mut versus_freeze,
            &mut base_reinforcement,
            *game_mode,
            &menu_queries.p0(),
        );
        return;
    }

    if keys.just_pressed(KeyCode::KeyR) {
        match *game_mode {
            GameMode::Campaign => restart_level(
                &mut commands,
                &assets,
                &sounds,
                &mut game_status,
                &mut tile_grid,
                &mut director,
                &mut score_board,
                &mut stage_rules,
                &mut versus_powerups,
                &mut enemy_freeze,
                &mut versus_freeze,
                &mut base_reinforcement,
                &menu_queries.p0(),
            ),
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => start_versus_round(
                &mut commands,
                &assets,
                &sounds,
                &mut game_mode,
                &mut game_status,
                &mut tile_grid,
                &mut director,
                &mut score_board,
                &mut stage_rules,
                &mut versus_powerups,
                &mut enemy_freeze,
                &mut versus_freeze,
                &mut base_reinforcement,
                &menu_queries.p0(),
            ),
        }
    }
}

fn enter_mode_select(
    commands: &mut Commands,
    assets: &SpriteAssets,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    score_board: &mut ScoreBoard,
    stage_rules: &mut StageRules,
    versus_powerups: &mut VersusPowerUpDirector,
    mode_select: &mut ModeSelect,
    enemy_freeze: &mut EnemyFreeze,
    versus_freeze: &mut VersusPlayerFreeze,
    base_reinforcement: &mut BaseReinforcement,
    selected_mode: GameMode,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    mode_select.selected = selected_mode.mode_select_option();
    mode_select.stage = game_status.stage.clamp(1, LEVEL_COUNT);
    mode_select.arena = game_status.arena.clamp(1, ARENA_COUNT);
    spawn_mode_select_screen(
        commands,
        assets,
        mode_select.selected,
        mode_select.stage,
        mode_select.arena,
        mode_select.audio_mode,
        mode_select.sound_enabled,
    );

    *tile_grid = TileGrid::empty();
    *director = EnemyDirector::inactive();
    *score_board = ScoreBoard::campaign(0);
    *stage_rules = StageRules::default();
    *versus_powerups = VersusPowerUpDirector::inactive();
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::ModeSelect;
    game_status.winner = None;
    game_status.transition_timer.reset();
}

fn restart_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    score_board: &mut ScoreBoard,
    stage_rules: &mut StageRules,
    versus_powerups: &mut VersusPowerUpDirector,
    enemy_freeze: &mut EnemyFreeze,
    versus_freeze: &mut VersusPlayerFreeze,
    base_reinforcement: &mut BaseReinforcement,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let (level, new_tile_grid) = load_stage_bundle_or_panic(game_status.stage);
    info!("Loaded {}", level.name);

    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(commands, assets, GameMode::Campaign);
    spawn_level(
        commands,
        assets,
        &level,
        &new_tile_grid,
        3,
        DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
    );
    play_sound(commands, sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    *director = EnemyDirector::from_level(&level);
    *score_board = ScoreBoard::campaign(level.enemies.len());
    *stage_rules = StageRules::from_level(&level);
    *versus_powerups = VersusPowerUpDirector::inactive();
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::StageIntro;
    game_status.winner = None;
    game_status.transition_timer = Timer::from_seconds(STAGE_INTRO_SECONDS, TimerMode::Once);
}

fn start_versus_round(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_mode: &mut GameMode,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    score_board: &mut ScoreBoard,
    stage_rules: &mut StageRules,
    versus_powerups: &mut VersusPowerUpDirector,
    enemy_freeze: &mut EnemyFreeze,
    versus_freeze: &mut VersusPlayerFreeze,
    base_reinforcement: &mut BaseReinforcement,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let arena_index = game_status.arena.clamp(1, ARENA_COUNT);
    let (arena, new_tile_grid) = load_arena_bundle_or_panic(arena_index);
    info!("Loaded {}", arena.name);
    let (round_mode, target_score, lives, respawn_invulnerability_secs) = match arena.battle_rules {
        BattleRules::Deathmatch {
            target_score,
            lives,
            respawn_invulnerability_secs,
        } => (
            GameMode::VersusDeathmatch,
            target_score,
            lives,
            respawn_invulnerability_secs,
        ),
        BattleRules::BaseBattle {
            lives,
            respawn_invulnerability_secs,
            ..
        } => (
            GameMode::VersusBaseBattle,
            0,
            lives,
            respawn_invulnerability_secs,
        ),
    };

    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(commands, assets, round_mode);
    spawn_arena(
        commands,
        assets,
        &arena,
        &new_tile_grid,
        lives,
        respawn_invulnerability_secs,
    );
    play_sound(commands, sounds, SoundKind::StageStart);

    *game_mode = round_mode;
    *tile_grid = new_tile_grid;
    *director = EnemyDirector::inactive();
    *score_board = ScoreBoard::versus(lives, target_score, respawn_invulnerability_secs);
    *stage_rules = StageRules::default();
    *versus_powerups = VersusPowerUpDirector::from_arena(&arena);
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::StageIntro;
    game_status.arena = arena_index;
    game_status.winner = None;
    game_status.transition_timer = Timer::from_seconds(STAGE_INTRO_SECONDS, TimerMode::Once);
}

fn stage_path(stage: usize) -> String {
    format!("assets/levels/{stage:03}.level.ron")
}

fn personal_stage_path(stage: usize) -> String {
    format!("assets/personal/levels/{stage:03}.level.ron")
}

fn runtime_stage_path(stage: usize) -> String {
    preferred_existing_path(&personal_stage_path(stage), &stage_path(stage), |path| {
        Path::new(path).is_file()
    })
}

fn preferred_existing_path(
    personal_path: &str,
    authored_path: &str,
    exists: impl Fn(&str) -> bool,
) -> String {
    if exists(personal_path) {
        personal_path.to_string()
    } else {
        authored_path.to_string()
    }
}

fn load_stage_definition(stage: usize) -> Result<LevelDefinition, String> {
    load_level(&runtime_stage_path(stage))
}

fn load_stage_bundle(stage: usize) -> Result<(LevelDefinition, TileGrid), String> {
    let path = runtime_stage_path(stage);
    let level = load_stage_definition(stage)?;
    let grid = TileGrid::from_level(&level)
        .map_err(|err| format!("failed to build level grid {path}: {err}"))?;
    Ok((level, grid))
}

fn load_stage_bundle_or_panic(stage: usize) -> (LevelDefinition, TileGrid) {
    let path = runtime_stage_path(stage);
    load_stage_bundle(stage).unwrap_or_else(|err| {
        panic!("{}", campaign_stage_load_error(stage, &path, &err));
    })
}

fn campaign_stage_load_error(stage: usize, path: &str, err: &str) -> String {
    format!("failed to load campaign stage {stage} from {path}: {err}")
}

fn arena_path(arena: usize) -> String {
    format!("assets/arenas/arena_{arena:02}.ron")
}

fn personal_arena_path(arena: usize) -> String {
    format!("assets/personal/arenas/arena_{arena:02}.ron")
}

fn runtime_arena_path(arena: usize) -> String {
    preferred_existing_path(&personal_arena_path(arena), &arena_path(arena), |path| {
        Path::new(path).is_file()
    })
}

fn load_arena_definition(arena: usize) -> Result<ArenaDefinition, String> {
    load_arena(&runtime_arena_path(arena))
}

fn load_arena_bundle(arena: usize) -> Result<(ArenaDefinition, TileGrid), String> {
    let path = runtime_arena_path(arena);
    let arena_definition = load_arena_definition(arena)?;
    let grid = TileGrid::from_arena(&arena_definition)
        .map_err(|err| format!("failed to build arena grid {path}: {err}"))?;
    Ok((arena_definition, grid))
}

fn load_arena_bundle_or_panic(arena: usize) -> (ArenaDefinition, TileGrid) {
    let path = runtime_arena_path(arena);
    load_arena_bundle(arena).unwrap_or_else(|err| {
        panic!("{}", versus_arena_load_error(arena, &path, &err));
    })
}

fn versus_arena_load_error(arena: usize, path: &str, err: &str) -> String {
    format!("failed to load versus arena {arena} from {path}: {err}")
}

fn runtime_asset_manifest_path() -> String {
    preferred_existing_path(PERSONAL_ASSET_MANIFEST_PATH, ASSET_MANIFEST_PATH, |path| {
        Path::new(path).is_file()
    })
}

fn load_level(path: &str) -> Result<LevelDefinition, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    parse_level(&contents).map_err(|err| format!("failed to load level {path}: {err}"))
}

fn load_arena(path: &str) -> Result<ArenaDefinition, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    parse_arena(&contents).map_err(|err| format!("failed to load arena {path}: {err}"))
}

fn battle_kind_label(rules: BattleRules) -> &'static str {
    match rules {
        BattleRules::Deathmatch { .. } => "DUEL",
        BattleRules::BaseBattle { .. } => "BASE",
    }
}

fn battle_kind_label_for_arena(arena: usize) -> &'static str {
    load_arena_definition(arena)
        .map(|arena| battle_kind_label(arena.battle_rules))
        .unwrap_or("DUEL")
}

fn load_asset_manifest(path: &str) -> Result<AssetManifest, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    parse_asset_manifest(&contents)
}

fn parse_asset_manifest(contents: &str) -> Result<AssetManifest, String> {
    let manifest: AssetManifest =
        ron::from_str(contents).map_err(|err| format!("failed to parse asset manifest: {err}"))?;
    validate_asset_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_asset_manifest(manifest: &AssetManifest) -> Result<(), String> {
    validate_generated_atlases(&manifest.atlases)?;
    validate_tank_frames(&manifest.tanks)?;
    validate_bullet_manifest(manifest.bullets)?;

    for (name, index) in [
        ("terrain.brick", manifest.terrain.brick),
        ("terrain.steel", manifest.terrain.steel),
        ("terrain.forest", manifest.terrain.forest),
        ("terrain.ice", manifest.terrain.ice),
    ] {
        if index >= TERRAIN_ATLAS_TILES {
            return Err(format!(
                "{name} index {index} is outside the generated terrain atlas"
            ));
        }
    }
    validate_frame_range(
        "terrain.water",
        manifest.terrain.water,
        TERRAIN_ATLAS_TILES,
        "terrain",
    )?;

    for (name, frames) in [
        ("effects.explosion", manifest.effects.explosion),
        ("effects.spawn_shimmer", manifest.effects.spawn_shimmer),
        (
            "effects.base_destruction",
            manifest.effects.base_destruction,
        ),
        ("effects.powerup_sparkle", manifest.effects.powerup_sparkle),
        ("effects.bullet_impact", manifest.effects.bullet_impact),
    ] {
        validate_frame_range(name, frames, EFFECT_ATLAS_TILES, "effect")?;
    }

    for (name, index) in [
        ("powerups.star", manifest.powerups.star),
        ("powerups.helmet", manifest.powerups.helmet),
        ("powerups.clock", manifest.powerups.clock),
        ("powerups.grenade", manifest.powerups.grenade),
        ("powerups.shovel", manifest.powerups.shovel),
        ("powerups.tank", manifest.powerups.tank),
    ] {
        if index >= POWERUP_ATLAS_TILES {
            return Err(format!(
                "{name} index {index} is outside the generated power-up atlas"
            ));
        }
    }

    validate_generated_sprite(
        "base.intact",
        manifest.base.intact,
        GENERATED_BASE_SIZE,
        GENERATED_BASE_SIZE,
    )?;
    validate_generated_sprite(
        "base.destroyed",
        manifest.base.destroyed,
        GENERATED_BASE_SIZE,
        GENERATED_BASE_SIZE,
    )?;
    validate_generated_sprite(
        "ui.score_badge",
        manifest.ui.score_badge,
        GENERATED_UI_ICON_SIZE,
        GENERATED_UI_ICON_SIZE,
    )?;
    validate_generated_sprite(
        "ui.stage_flag",
        manifest.ui.stage_flag,
        GENERATED_UI_ICON_SIZE,
        GENERATED_UI_ICON_SIZE,
    )?;
    validate_glyph_manifest(&manifest.glyphs)?;
    validate_sound_manifest(&manifest.sounds)?;

    Ok(())
}

fn validate_generated_atlases(manifest: &GeneratedAtlasesManifest) -> Result<(), String> {
    validate_generated_atlas(
        "atlases.tanks",
        manifest.tanks,
        TANK_ATLAS_TILE_SIZE,
        TANK_ATLAS_TILE_SIZE,
        TANK_ATLAS_TILES,
    )?;
    validate_generated_atlas(
        "atlases.terrain",
        manifest.terrain,
        TERRAIN_ATLAS_TILE_SIZE,
        TERRAIN_ATLAS_TILE_SIZE,
        TERRAIN_ATLAS_TILES,
    )?;
    validate_generated_atlas(
        "atlases.bullets",
        manifest.bullets,
        BULLET_ATLAS_TILE_SIZE,
        BULLET_ATLAS_TILE_SIZE,
        BULLET_ATLAS_TILES,
    )?;
    validate_generated_atlas(
        "atlases.effects",
        manifest.effects,
        EFFECT_ATLAS_TILE_SIZE,
        EFFECT_ATLAS_TILE_SIZE,
        EFFECT_ATLAS_TILES,
    )?;
    validate_generated_atlas(
        "atlases.powerups",
        manifest.powerups,
        POWERUP_ATLAS_TILE_SIZE,
        POWERUP_ATLAS_TILE_SIZE,
        POWERUP_ATLAS_TILES,
    )
}

fn validate_generated_atlas(
    name: &str,
    manifest: GeneratedAtlasManifest,
    expected_tile_width: usize,
    expected_tile_height: usize,
    expected_tiles: usize,
) -> Result<(), String> {
    if manifest.tile_width != expected_tile_width || manifest.tile_height != expected_tile_height {
        return Err(format!(
            "{name} tiles must be {expected_tile_width}x{expected_tile_height}, got {}x{}",
            manifest.tile_width, manifest.tile_height
        ));
    }
    if manifest.tiles != expected_tiles {
        return Err(format!(
            "{name} must contain {expected_tiles} tiles, got {}",
            manifest.tiles
        ));
    }
    Ok(())
}

fn validate_generated_sprite(
    name: &str,
    manifest: GeneratedSpriteManifest,
    expected_width: usize,
    expected_height: usize,
) -> Result<(), String> {
    if manifest.width != expected_width || manifest.height != expected_height {
        return Err(format!(
            "{name} must be {expected_width}x{expected_height}, got {}x{}",
            manifest.width, manifest.height
        ));
    }
    Ok(())
}

fn validate_bullet_manifest(manifest: DirectionalSpriteManifest) -> Result<(), String> {
    for (direction, index) in [
        ("up", manifest.up),
        ("down", manifest.down),
        ("left", manifest.left),
        ("right", manifest.right),
    ] {
        if index >= BULLET_ATLAS_TILES {
            return Err(format!(
                "bullets.{direction} index {index} is outside the generated bullet atlas"
            ));
        }
    }

    Ok(())
}

fn validate_glyph_manifest(manifest: &GlyphManifest) -> Result<(), String> {
    if manifest.tile_width != GENERATED_GLYPH_WIDTH {
        return Err(format!(
            "glyphs.tile_width {} must match generated glyph width {GENERATED_GLYPH_WIDTH}",
            manifest.tile_width
        ));
    }
    if manifest.tile_height != GENERATED_GLYPH_HEIGHT {
        return Err(format!(
            "glyphs.tile_height {} must match generated glyph height {GENERATED_GLYPH_HEIGHT}",
            manifest.tile_height
        ));
    }

    let mut seen = HashSet::new();
    for ch in manifest.characters.chars() {
        if !seen.insert(ch) {
            return Err(format!("glyphs.characters includes duplicate glyph '{ch}'"));
        }

        let pattern = glyph_pattern(ch);
        if pattern.len() != manifest.tile_height
            || pattern
                .iter()
                .any(|row| row.chars().count() != manifest.tile_width)
        {
            return Err(format!(
                "glyphs.characters glyph '{ch}' must use a {}x{} pattern",
                manifest.tile_width, manifest.tile_height
            ));
        }
        if !glyph_pattern_has_pixels(pattern) {
            return Err(format!(
                "glyphs.characters includes unsupported blank glyph '{ch}'"
            ));
        }
    }

    for required in REQUIRED_GLYPHS.chars() {
        if !seen.contains(&required) {
            return Err(format!(
                "glyphs.characters must include required glyph '{required}'"
            ));
        }
    }

    Ok(())
}

fn validate_sound_manifest(manifest: &SoundManifest) -> Result<(), String> {
    for (name, spec) in sound_manifest_specs(manifest) {
        validate_sound_spec(name, spec)?;
    }
    Ok(())
}

fn sound_manifest_specs(manifest: &SoundManifest) -> [(&'static str, &RetroSoundSpec); 9] {
    [
        ("sounds.fire", &manifest.fire),
        ("sounds.brick_hit", &manifest.brick_hit),
        ("sounds.steel_hit", &manifest.steel_hit),
        ("sounds.tank_explosion", &manifest.tank_explosion),
        ("sounds.base_destroyed", &manifest.base_destroyed),
        ("sounds.powerup_pickup", &manifest.powerup_pickup),
        ("sounds.stage_start", &manifest.stage_start),
        ("sounds.level_clear", &manifest.level_clear),
        ("sounds.game_over", &manifest.game_over),
    ]
}

fn validate_sound_spec(name: &str, spec: &RetroSoundSpec) -> Result<(), String> {
    match spec {
        RetroSoundSpec::Sweep {
            duration_secs,
            start_frequency,
            end_frequency,
            volume,
        } => {
            validate_sound_duration(name, *duration_secs)?;
            validate_sound_frequency(name, "start_frequency", *start_frequency)?;
            validate_sound_frequency(name, "end_frequency", *end_frequency)?;
            validate_sound_volume(name, *volume)
        }
        RetroSoundSpec::Noise {
            duration_secs,
            volume,
            seed,
        } => {
            validate_sound_duration(name, *duration_secs)?;
            validate_sound_volume(name, *volume)?;
            if *seed == 0 {
                return Err(format!("{name} noise seed must be nonzero"));
            }
            Ok(())
        }
        RetroSoundSpec::Layered { notes } => {
            if notes.is_empty() {
                return Err(format!("{name} must define at least one note"));
            }

            let total_duration: f32 = notes.iter().map(|note| note.duration_secs).sum();
            validate_sound_duration(name, total_duration)?;
            for (index, note) in notes.iter().enumerate() {
                let note_name = format!("{name}.notes[{index}]");
                validate_sound_duration(&note_name, note.duration_secs)?;
                validate_sound_frequency(&note_name, "frequency", note.frequency)?;
                validate_sound_volume(&note_name, note.volume)?;
            }
            Ok(())
        }
    }
}

fn validate_sound_duration(name: &str, duration_secs: f32) -> Result<(), String> {
    if duration_secs <= 0.0 || duration_secs > MAX_RETRO_SOUND_SECONDS {
        return Err(format!(
            "{name} duration {duration_secs} must be in 0..={MAX_RETRO_SOUND_SECONDS} seconds"
        ));
    }
    Ok(())
}

fn validate_sound_frequency(name: &str, field: &str, frequency: f32) -> Result<(), String> {
    if frequency <= 0.0 || frequency > MAX_RETRO_SOUND_FREQUENCY {
        return Err(format!(
            "{name} {field} {frequency} must be in 0..={MAX_RETRO_SOUND_FREQUENCY} Hz"
        ));
    }
    Ok(())
}

fn validate_sound_volume(name: &str, volume: f32) -> Result<(), String> {
    if volume <= 0.0 || volume > MAX_RETRO_SOUND_VOLUME {
        return Err(format!(
            "{name} volume {volume} must be in 0..={MAX_RETRO_SOUND_VOLUME}"
        ));
    }
    Ok(())
}

fn validate_tank_frames(manifest: &TankSpriteManifest) -> Result<(), String> {
    for (name, frames) in [
        ("tanks.player1", &manifest.player1),
        ("tanks.player2", &manifest.player2),
        ("tanks.enemies.basic", &manifest.enemies.basic),
        ("tanks.enemies.fast", &manifest.enemies.fast),
        ("tanks.enemies.power", &manifest.enemies.power),
        ("tanks.enemies.armor", &manifest.enemies.armor),
    ] {
        if frames.len() != TANK_ANIMATION_FRAMES {
            return Err(format!(
                "{name} must define {TANK_ANIMATION_FRAMES} animation frames, got {}",
                frames.len()
            ));
        }

        for (frame_index, frame) in frames.iter().enumerate() {
            for (direction, index) in [
                ("up", frame.up),
                ("down", frame.down),
                ("left", frame.left),
                ("right", frame.right),
            ] {
                if index >= TANK_ATLAS_TILES {
                    return Err(format!(
                        "{name}[{frame_index}].{direction} index {index} is outside the generated tank atlas"
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_frame_range(
    name: &str,
    frames: SpriteFrameRange,
    atlas_tiles: usize,
    atlas_name: &str,
) -> Result<(), String> {
    if frames.first > frames.last {
        return Err(format!(
            "{name} frame range {}..={} starts after it ends",
            frames.first, frames.last
        ));
    }

    if frames.last >= atlas_tiles {
        return Err(format!(
            "{name} frame range {}..={} is outside the generated {atlas_name} atlas",
            frames.first, frames.last
        ));
    }

    Ok(())
}

fn parse_level(contents: &str) -> Result<LevelDefinition, String> {
    let level: LevelDefinition =
        ron::from_str(contents).map_err(|err| format!("failed to parse level: {err}"))?;

    let grid = TileGrid::from_level(&level)?;
    if level.enemies.len() != 20 {
        return Err(format!(
            "expected a classic 20-enemy roster, got {}",
            level.enemies.len()
        ));
    }
    if level.enemy_spawns.len() != 3 {
        return Err(format!(
            "expected 3 enemy spawn points, got {}",
            level.enemy_spawns.len()
        ));
    }
    validate_classic_enemy_spawns(&level.enemy_spawns)?;
    if level.max_enemies_on_screen == 0 {
        return Err("max_enemies_on_screen must be greater than zero".to_string());
    }
    if level.max_enemies_on_screen > CLASSIC_MAX_ACTIVE_ENEMIES {
        return Err(format!(
            "max_enemies_on_screen must be at most {CLASSIC_MAX_ACTIVE_ENEMIES}, got {}",
            level.max_enemies_on_screen
        ));
    }
    if level.spawn_interval_secs <= 0.0 {
        return Err("spawn_interval_secs must be positive".to_string());
    }
    validate_level_positions(&level, &grid)?;
    validate_powerup_carriers(&level)?;

    Ok(level)
}

fn parse_arena(contents: &str) -> Result<ArenaDefinition, String> {
    let arena: ArenaDefinition =
        ron::from_str(contents).map_err(|err| format!("failed to parse arena: {err}"))?;

    let grid = TileGrid::from_arena(&arena)?;
    validate_arena_spawns(&grid, &arena)?;
    validate_battle_rules(&grid, arena.battle_rules)?;
    validate_powerup_spawns(&grid, &arena.powerup_spawns)?;

    Ok(arena)
}

fn validate_battle_rules(grid: &TileGrid, rules: BattleRules) -> Result<(), String> {
    match rules {
        BattleRules::Deathmatch {
            target_score,
            lives,
            respawn_invulnerability_secs,
        } => {
            if target_score == 0 {
                return Err("deathmatch target_score must be greater than zero".to_string());
            }
            if lives <= 0 {
                return Err("deathmatch lives must be greater than zero".to_string());
            }
            if respawn_invulnerability_secs <= 0.0 {
                return Err("deathmatch respawn_invulnerability_secs must be positive".to_string());
            }
        }
        BattleRules::BaseBattle {
            p1_base,
            p2_base,
            lives,
            respawn_invulnerability_secs,
        } => {
            if lives <= 0 {
                return Err("base battle lives must be greater than zero".to_string());
            }
            if respawn_invulnerability_secs <= 0.0 {
                return Err("base battle respawn_invulnerability_secs must be positive".to_string());
            }
            validate_base_position(grid, "p1 base position", &p1_base)?;
            validate_base_position(grid, "p2 base position", &p2_base)?;
            validate_base_positions_do_not_overlap(p1_base, p2_base)?;
        }
    }

    Ok(())
}

fn spawn_mode_select_screen(
    commands: &mut Commands,
    assets: &SpriteAssets,
    selected: ModeSelectOption,
    stage: usize,
    arena: usize,
    audio_mode: AudioMode,
    sound_enabled: bool,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(16, 16, 14),
            Vec2::new(208.0 * window_scale(), 208.0 * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(0.0, 16.0),
            Vec2::new(208.0, 208.0),
            0.0,
        )),
        GameEntity,
    ));

    spawn_pixel_text(commands, assets, "TANK", Vec2::new(86.0, 66.0), 0.3);
    spawn_pixel_text(commands, assets, "1990", Vec2::new(88.0, 79.0), 0.3);
    spawn_pixel_text(
        commands,
        assets,
        "1 PLAYER",
        mode_select_option_top_left(ModeSelectOption::Campaign),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "BATTLE",
        mode_select_option_top_left(ModeSelectOption::Battle),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "MUSIC",
        mode_select_option_top_left(ModeSelectOption::Music),
        0.3,
    );
    spawn_mode_select_music_value(commands, assets, audio_mode, Vec2::new(124.0, 131.0), 0.3);
    spawn_pixel_text(
        commands,
        assets,
        "SOUND",
        mode_select_option_top_left(ModeSelectOption::Sound),
        0.3,
    );
    spawn_mode_select_sound_value(
        commands,
        assets,
        sound_enabled,
        Vec2::new(124.0, 145.0),
        0.3,
    );
    spawn_pixel_text(commands, assets, "STAGE", Vec2::new(59.0, 160.0), 0.3);
    spawn_mode_select_stage_digits(commands, assets, stage, Vec2::new(95.0, 160.0), 0.3);
    spawn_pixel_text(commands, assets, "ARENA", Vec2::new(59.0, 172.0), 0.3);
    spawn_mode_select_arena_digits(commands, assets, arena, Vec2::new(95.0, 172.0), 0.3);
    spawn_mode_select_battle_kind(commands, assets, arena, Vec2::new(115.0, 172.0), 0.3);
    spawn_mode_select_hints(commands, assets);
    spawn_mode_select_cursor(commands, assets, selected);
}

fn spawn_mode_select_hints(commands: &mut Commands, assets: &SpriteAssets) {
    for (index, line) in MODE_SELECT_HINT_LINES.iter().enumerate() {
        let text_width = phase_text_width(line);
        spawn_pixel_text(
            commands,
            assets,
            line,
            Vec2::new((208.0 - text_width) / 2.0, 190.0 + index as f32 * 11.0),
            0.3,
        );
    }
}

fn spawn_mode_select_stage_digits(
    commands: &mut Commands,
    assets: &SpriteAssets,
    stage: usize,
    top_left: Vec2,
    z: f32,
) {
    let text = format!("{:02}", stage.min(99));
    for digit in 0..2 {
        let ch = text.chars().nth(digit).unwrap_or('0');
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            ModeSelectStageGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_arena_digits(
    commands: &mut Commands,
    assets: &SpriteAssets,
    arena: usize,
    top_left: Vec2,
    z: f32,
) {
    let text = format!("{:02}", arena.min(99));
    for digit in 0..2 {
        let ch = text.chars().nth(digit).unwrap_or('0');
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            ModeSelectArenaGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_battle_kind(
    commands: &mut Commands,
    assets: &SpriteAssets,
    arena: usize,
    top_left: Vec2,
    z: f32,
) {
    let text = battle_kind_label_for_arena(arena);
    for digit in 0..4 {
        let ch = text.chars().nth(digit).unwrap_or(' ');
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            ModeSelectBattleKindGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_music_value(
    commands: &mut Commands,
    assets: &SpriteAssets,
    mode: AudioMode,
    top_left: Vec2,
    z: f32,
) {
    let text = audio_mode_label(mode);
    for digit in 0..7 {
        let ch = text.chars().nth(digit).unwrap_or(' ');
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            ModeSelectMusicGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_sound_value(
    commands: &mut Commands,
    assets: &SpriteAssets,
    enabled: bool,
    top_left: Vec2,
    z: f32,
) {
    let text = sound_enabled_label(enabled);
    for digit in 0..3 {
        let ch = text.chars().nth(digit).unwrap_or(' ');
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            ModeSelectSoundGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_cursor(
    commands: &mut Commands,
    assets: &SpriteAssets,
    selected: ModeSelectOption,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: animated_tank_sprite_index(
                    &assets.manifest,
                    TankSpriteSet::Player1,
                    Direction::Right,
                    0,
                ),
            },
        ),
        Transform::from_translation(mode_select_cursor_translation(selected))
            .with_scale(Vec3::splat(window_scale())),
        ModeSelectCursor,
        GameEntity,
    ));
}

fn spawn_screen_frame(commands: &mut Commands, assets: &SpriteAssets, mode: GameMode) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(80, 80, 72),
            Vec2::new(48.0 * window_scale(), 208.0 * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(208.0, 16.0),
            Vec2::new(48.0, 208.0),
            0.0,
        )),
        GameEntity,
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(36, 36, 32),
            Vec2::new(40.0 * window_scale(), 192.0 * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(212.0, 24.0),
            Vec2::new(40.0, 192.0),
            0.1,
        )),
        GameEntity,
    ));

    match mode {
        GameMode::Campaign => spawn_campaign_status_panel(commands, assets),
        GameMode::VersusDeathmatch => spawn_versus_status_panel(commands, assets, true),
        GameMode::VersusBaseBattle => spawn_versus_status_panel(commands, assets, false),
    }
}

fn spawn_campaign_status_panel(commands: &mut Commands, assets: &SpriteAssets) {
    spawn_pixel_text(commands, assets, "P1", Vec2::new(214.0, 26.0), 0.3);
    spawn_pixel_text(commands, assets, "SCORE", Vec2::new(214.0, 38.0), 0.3);
    spawn_score_badge_icon(commands, assets);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Score,
        6,
        Vec2::new(214.0, 49.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "STAGE", Vec2::new(214.0, 76.0), 0.3);
    spawn_stage_flag_icon(commands, assets);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Stage,
        2,
        stage_number_top_left(),
        0.3,
    );

    spawn_pixel_text(commands, assets, "LIFE", Vec2::new(214.0, 112.0), 0.3);
    spawn_player_life_icon(
        commands,
        assets,
        PlayerId::One,
        campaign_life_icon_top_left(),
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Lives,
        1,
        Vec2::new(234.0, 123.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "ENEMY", Vec2::new(214.0, 148.0), 0.3);
    for index in 0..ENEMY_MARKER_COUNT {
        spawn_enemy_marker(commands, assets, index);
    }
}

fn spawn_enemy_marker(commands: &mut Commands, assets: &SpriteAssets, index: usize) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: enemy_marker_tank_index(&assets.manifest),
            },
        ),
        Transform::from_translation(virtual_center_scaled(
            enemy_marker_top_left(index),
            Vec2::splat(ENEMY_MARKER_SIZE),
            0.3,
        ))
        .with_scale(Vec3::splat(window_scale() * ENEMY_MARKER_SIZE / TANK_SIZE)),
        Visibility::Visible,
        EnemyMarker { index },
        GameEntity,
    ));
}

fn spawn_player_life_icon(
    commands: &mut Commands,
    assets: &SpriteAssets,
    player_id: PlayerId,
    top_left: Vec2,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: player_life_icon_tank_index(&assets.manifest, player_id),
            },
        ),
        Transform::from_translation(virtual_center_scaled(
            top_left,
            Vec2::splat(PLAYER_LIFE_ICON_SIZE),
            0.3,
        ))
        .with_scale(Vec3::splat(
            window_scale() * PLAYER_LIFE_ICON_SIZE / TANK_SIZE,
        )),
        GameEntity,
    ));
}

fn spawn_stage_flag_icon(commands: &mut Commands, assets: &SpriteAssets) {
    commands.spawn((
        Sprite::from_image(assets.stage_flag_icon.clone()),
        Transform::from_translation(virtual_center_scaled(
            stage_flag_icon_top_left(),
            generated_sprite_size(assets.manifest.ui.stage_flag),
            0.3,
        ))
        .with_scale(Vec3::splat(window_scale())),
        GameEntity,
    ));
}

fn spawn_score_badge_icon(commands: &mut Commands, assets: &SpriteAssets) {
    commands.spawn((
        Sprite::from_image(assets.score_badge_icon.clone()),
        Transform::from_translation(virtual_center_scaled(
            score_badge_icon_top_left(),
            generated_sprite_size(assets.manifest.ui.score_badge),
            0.3,
        ))
        .with_scale(Vec3::splat(window_scale())),
        GameEntity,
    ));
}

fn spawn_versus_status_panel(commands: &mut Commands, assets: &SpriteAssets, show_target: bool) {
    spawn_pixel_text(commands, assets, "P1", Vec2::new(214.0, 26.0), 0.3);
    spawn_pixel_text(commands, assets, "SCORE", Vec2::new(214.0, 38.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Score,
        2,
        Vec2::new(226.0, 49.0),
        0.3,
    );
    spawn_pixel_text(commands, assets, "LIFE", Vec2::new(214.0, 62.0), 0.3);
    spawn_player_life_icon(
        commands,
        assets,
        PlayerId::One,
        versus_life_icon_top_left(PlayerId::One),
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Lives,
        1,
        Vec2::new(234.0, 73.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "P2", Vec2::new(214.0, 98.0), 0.3);
    spawn_pixel_text(commands, assets, "SCORE", Vec2::new(214.0, 110.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::P2Score,
        2,
        Vec2::new(226.0, 121.0),
        0.3,
    );
    spawn_pixel_text(commands, assets, "LIFE", Vec2::new(214.0, 134.0), 0.3);
    spawn_player_life_icon(
        commands,
        assets,
        PlayerId::Two,
        versus_life_icon_top_left(PlayerId::Two),
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::P2Lives,
        1,
        Vec2::new(234.0, 145.0),
        0.3,
    );

    spawn_pixel_text(
        commands,
        assets,
        "ARENA",
        versus_arena_label_top_left(),
        0.3,
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Arena,
        2,
        versus_arena_number_top_left(),
        0.3,
    );

    if show_target {
        spawn_pixel_text(
            commands,
            assets,
            "TARGET",
            versus_target_label_top_left(),
            0.3,
        );
        spawn_status_digits(
            commands,
            assets,
            StatusValue::Target,
            2,
            versus_target_number_top_left(),
            0.3,
        );
    } else {
        spawn_pixel_text(commands, assets, "BASE", versus_base_label_top_left(), 0.3);
    }
}

fn spawn_status_digits(
    commands: &mut Commands,
    assets: &SpriteAssets,
    kind: StatusValue,
    digits: usize,
    top_left: Vec2,
    z: f32,
) {
    for digit in 0..digits {
        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index('0', &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            StatusGlyph { kind, digit },
            GameEntity,
        ));
    }
}

fn spawn_pixel_text(
    commands: &mut Commands,
    assets: &SpriteAssets,
    text: &str,
    top_left: Vec2,
    z: f32,
) {
    spawn_pixel_text_inner(commands, assets, text, top_left, z, false);
}

fn spawn_phase_text(
    commands: &mut Commands,
    assets: &SpriteAssets,
    lines: &[String],
    center_y: f32,
    z: f32,
) {
    let line_gap = 3.0;
    let line_height = 7.0;
    let text_step = line_height + line_gap;
    let text_block_height =
        lines.len() as f32 * line_height + lines.len().saturating_sub(1) as f32 * line_gap;
    let background_height = text_block_height + 10.0;
    let background_top = center_y - background_height / 2.0;
    let first_line_top = center_y - text_block_height / 2.0;

    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(48, 48, 40),
            Vec2::new(132.0 * window_scale(), background_height * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(36.0, background_top),
            Vec2::new(132.0, background_height),
            z - 0.1,
        )),
        PhaseBanner,
        GameEntity,
    ));

    for (index, line) in lines.iter().enumerate() {
        let text_width = phase_text_width(line);
        spawn_pixel_text_inner(
            commands,
            assets,
            line,
            Vec2::new(
                (208.0 - text_width) / 2.0,
                first_line_top + index as f32 * text_step,
            ),
            z,
            true,
        );
    }
}

fn spawn_pixel_text_inner(
    commands: &mut Commands,
    assets: &SpriteAssets,
    text: &str,
    top_left: Vec2,
    z: f32,
    phase_banner: bool,
) {
    for (index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            continue;
        }
        let mut entity = commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + index as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            GameEntity,
        ));

        if phase_banner {
            entity.insert(PhaseBanner);
        }
    }
}

fn spawn_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
    level: &LevelDefinition,
    tile_grid: &TileGrid,
    player_lives: i32,
    spawn_invulnerability_secs: f32,
) {
    spawn_terrain(commands, assets, tile_grid);

    spawn_base_sprite(commands, assets, &level.base_position, None);

    spawn_player_tank(
        commands,
        assets,
        &level.player_spawn,
        PlayerId::One,
        player_lives,
        spawn_invulnerability_secs,
    );
}

fn spawn_arena(
    commands: &mut Commands,
    assets: &SpriteAssets,
    arena: &ArenaDefinition,
    tile_grid: &TileGrid,
    player_lives: i32,
    spawn_invulnerability_secs: f32,
) {
    spawn_terrain(commands, assets, tile_grid);
    if let BattleRules::BaseBattle {
        p1_base, p2_base, ..
    } = arena.battle_rules
    {
        spawn_base_sprite(commands, assets, &p1_base, Some(PlayerId::One));
        spawn_base_sprite(commands, assets, &p2_base, Some(PlayerId::Two));
    }
    spawn_player_tank(
        commands,
        assets,
        &arena.p1_spawn,
        PlayerId::One,
        player_lives,
        spawn_invulnerability_secs,
    );
    spawn_player_tank(
        commands,
        assets,
        &arena.p2_spawn,
        PlayerId::Two,
        player_lives,
        spawn_invulnerability_secs,
    );
}

fn spawn_base_sprite(
    commands: &mut Commands,
    assets: &SpriteAssets,
    point: &GridPoint,
    owner: Option<PlayerId>,
) {
    let top_left = grid_point_top_left(point);
    commands.spawn((
        Sprite::from_image(assets.base_intact.clone()),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            generated_sprite_size(assets.manifest.base.intact),
            4.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        BaseSprite { owner, top_left },
        GameEntity,
    ));
}

fn spawn_terrain(commands: &mut Commands, assets: &SpriteAssets, tile_grid: &TileGrid) {
    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            let tile = tile_grid.tiles[y * BOARD_TILES + x];
            spawn_terrain_tile(commands, assets, tile, x, y);
        }
    }
}

fn spawn_terrain_tile(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile: TileKind,
    x: usize,
    y: usize,
) {
    let Some(index) = assets.manifest.terrain_index(tile) else {
        return;
    };

    let mut entity = commands.spawn((
        Sprite::from_atlas_image(
            assets.terrain_image.clone(),
            TextureAtlas {
                layout: assets.terrain_layout.clone(),
                index,
            },
        ),
        Transform::from_translation(board_tile_center(x, y, terrain_z(tile)))
            .with_scale(Vec3::splat(window_scale())),
        GridTile { x, y },
        GameEntity,
    ));

    if let Some(frames) = assets.manifest.terrain_animation_frames(tile) {
        entity.insert(SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
            despawn_on_finish: false,
        });
    }
}

fn sync_tile_sprite(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    x: usize,
    y: usize,
    tile: TileKind,
) {
    // Refresh same-kind tiles too, so temporary tints from shovel warnings cannot stick.
    tile_grid.set(x, y, tile);
    for (tile_entity, grid_tile) in tile_sprites {
        if grid_tile.x == x && grid_tile.y == y {
            commands.entity(tile_entity).despawn();
            break;
        }
    }

    spawn_terrain_tile(commands, assets, tile, x, y);
}

fn base_wall_positions(tile_grid: &TileGrid) -> Vec<(usize, usize)> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found_base = true;
            }
        }
    }

    if !found_base {
        return Vec::new();
    }

    base_wall_positions_around_rect(tile_grid, min_x, min_y, max_x, max_y)
}

fn base_wall_positions_for_top_left(tile_grid: &TileGrid, top_left: Vec2) -> Vec<(usize, usize)> {
    let min_x = (top_left.x / TILE_SIZE).floor().max(0.0) as usize;
    let min_y = (top_left.y / TILE_SIZE).floor().max(0.0) as usize;
    let max_x = (min_x + 1).min(BOARD_TILES - 1);
    let max_y = (min_y + 1).min(BOARD_TILES - 1);

    base_wall_positions_around_rect(tile_grid, min_x, min_y, max_x, max_y)
}

fn base_wall_positions_around_rect(
    tile_grid: &TileGrid,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let left = min_x.saturating_sub(2);
    let right = (max_x + 2).min(BOARD_TILES - 1);
    let top = min_y.saturating_sub(2);
    let bottom = max_y.min(BOARD_TILES - 1);
    let mut positions = Vec::new();

    for y in top..=bottom {
        for x in left..=right {
            if tile_grid.tiles[y * BOARD_TILES + x] != TileKind::Base {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn base_center_from_grid(tile_grid: &TileGrid) -> Option<Vec2> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found_base = true;
            }
        }
    }

    if !found_base {
        return None;
    }

    Some(Vec2::new(
        (min_x + max_x + 1) as f32 * TILE_SIZE / 2.0,
        (min_y + max_y + 1) as f32 * TILE_SIZE / 2.0,
    ))
}

fn base_top_left_from_grid(tile_grid: &TileGrid) -> Option<Vec2> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                found_base = true;
            }
        }
    }

    found_base.then_some(Vec2::new(
        min_x as f32 * TILE_SIZE,
        min_y as f32 * TILE_SIZE,
    ))
}

fn base_contains_tile(base_top_left: Vec2, tile_x: usize, tile_y: usize) -> bool {
    let tile_top_left = Vec2::new(tile_x as f32 * TILE_SIZE, tile_y as f32 * TILE_SIZE);
    rects_overlap(
        base_top_left,
        Vec2::splat(TANK_SIZE),
        tile_top_left,
        Vec2::splat(TILE_SIZE),
    )
}

fn grid_point_top_left(point: &GridPoint) -> Vec2 {
    Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE)
}

fn spawn_point_top_left(spawn: &SpawnPoint) -> Vec2 {
    Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE)
}

fn spawn_player_tank(
    commands: &mut Commands,
    assets: &SpriteAssets,
    spawn: &SpawnPoint,
    player_id: PlayerId,
    player_lives: i32,
    spawn_invulnerability_secs: f32,
) {
    let player_top_left = spawn_point_top_left(spawn);

    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: animated_tank_sprite_index(
                    &assets.manifest,
                    TankSpriteSet::player(player_id),
                    spawn.facing,
                    0,
                ),
            },
        ),
        Transform::from_translation(board_object_center(
            player_top_left.x,
            player_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        Tank {
            top_left: player_top_left,
            facing: spawn.facing,
            speed: PLAYER_SPEED,
        },
        TankSpriteState::new(TankSpriteSet::player(player_id)),
        Health { current: 1 },
        RespawnPoint {
            top_left: player_top_left,
            facing: spawn.facing,
        },
        PlayerLives {
            current: player_lives,
        },
        PlayerUpgrade { level: 0 },
        Shield {
            timer: Timer::from_seconds(spawn_invulnerability_secs, TimerMode::Once),
        },
        Player { id: player_id },
        GameEntity,
    ));
    spawn_spawn_effect(commands, assets, player_top_left);
}

fn update_player_control(keys: Res<ButtonInput<KeyCode>>, mut control: ResMut<PlayerControl>) {
    let PlayerControl {
        p1_last_direction,
        p2_last_direction,
        p1_direction_priority,
        p2_direction_priority,
    } = control.as_mut();

    update_direction_priority(
        &keys,
        PlayerId::One,
        p1_direction_priority,
        p1_last_direction,
    );
    update_direction_priority(
        &keys,
        PlayerId::Two,
        p2_direction_priority,
        p2_last_direction,
    );
}

fn move_player_tank(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    control: Res<PlayerControl>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    versus_freeze: Res<VersusPlayerFreeze>,
    mut tank_queries: ParamSet<(
        Query<&Tank>,
        Query<
            (
                &mut Tank,
                &mut Sprite,
                &mut Transform,
                &mut TankSpriteState,
                &Player,
            ),
            (With<Player>, Without<PlayerRespawnDelay>),
        >,
    )>,
) {
    if !game_status.is_playing() {
        return;
    }

    let occupied: Vec<Vec2> = tank_queries.p0().iter().map(|tank| tank.top_left).collect();

    for (mut tank, mut sprite, mut transform, mut tank_sprite, player) in &mut tank_queries.p1() {
        if versus_freeze.is_player_frozen(player.id) {
            update_tank_sprite(
                &mut sprite,
                &mut tank_sprite,
                tank.facing,
                false,
                time.delta(),
                &assets.manifest,
            );
            continue;
        }

        let Some(direction) =
            held_direction(&keys, player_last_direction(&control, player.id), player.id)
        else {
            update_tank_sprite(
                &mut sprite,
                &mut tank_sprite,
                tank.facing,
                false,
                time.delta(),
                &assets.manifest,
            );
            continue;
        };

        tank.facing = direction;

        let mut next = tank.top_left;
        snap_to_lane(&mut next, direction);
        next += direction.movement()
            * tank_move_speed(tank.speed, &grid, tank.top_left)
            * time.delta_secs();
        next = round_vec2(next);

        let mut moved = false;
        if grid.can_tank_occupy(next) && tank_position_free(next, tank.top_left, &occupied) {
            tank.top_left = next;
            transform.translation =
                board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
            moved = true;
        }
        update_tank_sprite(
            &mut sprite,
            &mut tank_sprite,
            tank.facing,
            moved,
            time.delta(),
            &assets.manifest,
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    mut director: ResMut<EnemyDirector>,
    active_enemies: Query<&EnemyTank>,
    tanks: Query<&Tank>,
) {
    if !game_status.is_playing()
        || enemy_freeze.is_active()
        || director.roster.is_empty()
        || active_enemies.iter().count() >= director.max_active
    {
        return;
    }

    let first_spawn = director.spawned_count == 0;
    if !first_spawn && !director.spawn_timer.tick(time.delta()).just_finished() {
        return;
    }

    for _ in 0..director.spawns.len() {
        let spawn = director.spawns[director.spawn_cursor].clone();
        director.spawn_cursor = (director.spawn_cursor + 1) % director.spawns.len();
        let top_left = Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE);
        let occupied: Vec<Vec2> = tanks.iter().map(|tank| tank.top_left).collect();

        if !grid.can_tank_occupy(top_left) || !tank_spawn_position_free(top_left, &occupied) {
            continue;
        }

        let enemy = director
            .roster
            .pop_front()
            .expect("checked non-empty roster above");
        director.spawned_count += 1;
        let kind = enemy.kind;
        let carried_powerup = enemy.carried_powerup;

        commands.spawn((
            Sprite::from_atlas_image(
                assets.tank_image.clone(),
                TextureAtlas {
                    layout: assets.tank_layout.clone(),
                    index: animated_tank_sprite_index(
                        &assets.manifest,
                        TankSpriteSet::enemy(kind),
                        spawn.facing,
                        0,
                    ),
                },
            ),
            Transform::from_translation(board_object_center(
                top_left.x,
                top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            ))
            .with_scale(Vec3::splat(window_scale())),
            Tank {
                top_left,
                facing: spawn.facing,
                speed: enemy_speed(kind),
            },
            Health {
                current: enemy_health(kind),
            },
            TankSpriteState::new(TankSpriteSet::enemy(kind)),
            EnemyTank {
                kind,
                carried_powerup,
            },
            EnemyAi {
                turn_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
                fire_timer: Timer::from_seconds(enemy_fire_interval(kind), TimerMode::Repeating),
            },
            SpawnProtection::for_spawn_shimmer(assets.manifest.spawn_shimmer_frames()),
            GameEntity,
        ));
        spawn_spawn_effect(&mut commands, &assets, top_left);
        break;
    }
}

fn move_enemy_tanks(
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    mut tank_queries: ParamSet<(
        Query<(&Tank, Option<&Player>)>,
        Query<
            (
                &mut Tank,
                &mut Sprite,
                &mut Transform,
                &mut EnemyAi,
                &mut TankSpriteState,
            ),
            (With<EnemyTank>, Without<Player>, Without<SpawnProtection>),
        >,
    )>,
) {
    if !game_status.is_playing() || enemy_freeze.is_active() {
        return;
    }

    let occupied: Vec<(Vec2, bool)> = tank_queries
        .p0()
        .iter()
        .map(|(tank, player)| (tank.top_left, player.is_some()))
        .collect();
    let player_top_lefts: Vec<Vec2> = occupied
        .iter()
        .filter_map(|(top_left, is_player)| is_player.then_some(*top_left))
        .collect();
    let occupied_positions: Vec<Vec2> = occupied.iter().map(|(top_left, _)| *top_left).collect();
    let base_center = base_center_from_grid(&grid);

    for (mut tank, mut sprite, mut transform, mut ai, mut tank_sprite) in &mut tank_queries.p1() {
        ai.turn_timer.tick(time.delta());
        if ai.turn_timer.just_finished() {
            tank.facing = preferred_enemy_direction(
                tank.top_left,
                tank.facing,
                &player_top_lefts,
                base_center,
            );
        }

        let mut next = tank.top_left;
        snap_to_lane(&mut next, tank.facing);
        next += tank.facing.movement()
            * tank_move_speed(tank.speed, &grid, tank.top_left)
            * time.delta_secs();
        next = round_vec2(next);

        let mut moved = false;
        if grid.can_tank_occupy(next)
            && tank_position_free(next, tank.top_left, &occupied_positions)
        {
            tank.top_left = next;
            transform.translation =
                board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
            moved = true;
        } else {
            tank.facing = next_direction(tank.facing);
        }

        update_tank_sprite(
            &mut sprite,
            &mut tank_sprite,
            tank.facing,
            moved,
            time.delta(),
            &assets.manifest,
        );
    }
}

fn fire_enemy_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    grid: Res<TileGrid>,
    players: Query<&Tank, With<Player>>,
    enemy_bullets: Query<&EnemyBulletSource>,
    mut enemies: Query<
        (
            Entity,
            &mut Tank,
            &EnemyTank,
            &mut EnemyAi,
            &mut Sprite,
            &TankSpriteState,
        ),
        (With<EnemyTank>, Without<Player>, Without<SpawnProtection>),
    >,
) {
    if !game_status.is_playing() || enemy_freeze.is_active() {
        return;
    }

    let player_top_lefts: Vec<Vec2> = players.iter().map(|tank| tank.top_left).collect();
    let base_center = base_center_from_grid(&grid);
    let mut active_enemy_bullet_shooters: Vec<Entity> =
        enemy_bullets.iter().map(|source| source.shooter).collect();
    if active_enemy_bullet_shooters.len() >= ENEMY_BULLET_LIMIT {
        return;
    }

    for (enemy_entity, mut tank, enemy, mut ai, mut sprite, tank_sprite) in &mut enemies {
        ai.fire_timer.tick(time.delta());
        let active_for_tank = active_enemy_bullet_shooters
            .iter()
            .filter(|shooter| **shooter == enemy_entity)
            .count();
        if !enemy_fire_slot_available(active_enemy_bullet_shooters.len(), active_for_tank) {
            continue;
        }

        let aim_direction = enemy_aim_direction(tank.top_left, &player_top_lefts, base_center);
        let snap_fire_ready = aim_direction.is_some()
            && enemy_alignment_fire_ready(enemy.kind, ai.fire_timer.elapsed_secs());
        if let Some(direction) = aim_direction {
            if !ai.fire_timer.just_finished() && !snap_fire_ready {
                continue;
            }
            tank.facing = direction;
            set_tank_sprite_direction(&mut sprite, tank_sprite, tank.facing, &assets.manifest);
        } else if !ai.fire_timer.just_finished()
            || !enemy_random_fire_ready(tank.top_left, tank.facing, enemy.kind)
        {
            continue;
        }

        let bullet_top_left = spawn_bullet_position(tank.top_left, tank.facing);
        commands.spawn((
            Sprite::from_atlas_image(
                assets.bullet_image.clone(),
                TextureAtlas {
                    layout: assets.bullet_layout.clone(),
                    index: assets.manifest.bullet_index(tank.facing),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(window_scale())),
            Bullet {
                previous_top_left: bullet_top_left,
                top_left: bullet_top_left,
                facing: tank.facing,
                owner: Team::Enemy,
                speed: enemy_bullet_speed(enemy.kind),
                breaks_steel: false,
                resolved: false,
            },
            EnemyBulletSource {
                shooter: enemy_entity,
            },
            GameEntity,
        ));
        play_sound(&mut commands, &sounds, SoundKind::Fire);
        ai.fire_timer.reset();
        active_enemy_bullet_shooters.push(enemy_entity);

        if enemy.kind == EnemyKind::Power
            || active_enemy_bullet_shooters.len() >= ENEMY_BULLET_LIMIT
        {
            break;
        }
    }
}

fn fire_player_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    stage_rules: Res<StageRules>,
    versus_freeze: Res<VersusPlayerFreeze>,
    players: Query<(&Tank, &PlayerUpgrade, &Player), (With<Player>, Without<PlayerRespawnDelay>)>,
    bullets: Query<&Bullet>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (tank, upgrade, player) in &players {
        if versus_freeze.is_player_frozen(player.id) {
            continue;
        }

        if !player_fire_pressed(&keys, player.id) {
            continue;
        }

        let owner = player.id.team();
        let active_player_bullets = bullets
            .iter()
            .filter(|bullet| bullet.owner == owner)
            .count();
        if active_player_bullets >= player_bullet_limit(upgrade.level) {
            continue;
        }

        let bullet_top_left = spawn_bullet_position(tank.top_left, tank.facing);
        commands.spawn((
            Sprite::from_atlas_image(
                assets.bullet_image.clone(),
                TextureAtlas {
                    layout: assets.bullet_layout.clone(),
                    index: assets.manifest.bullet_index(tank.facing),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(window_scale())),
            Bullet {
                previous_top_left: bullet_top_left,
                top_left: bullet_top_left,
                facing: tank.facing,
                owner,
                speed: player_bullet_speed(upgrade.level),
                breaks_steel: player_bullets_break_steel(upgrade.level, *stage_rules),
                resolved: false,
            },
            GameEntity,
        ));
        play_sound(&mut commands, &sounds, SoundKind::Fire);
    }
}

fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_mode: Res<GameMode>,
    mut grid: ResMut<TileGrid>,
    mut game_status: ResMut<GameStatus>,
    mut score_board: ResMut<ScoreBoard>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform), With<Bullet>>,
    tile_sprites: Query<(Entity, &GridTile)>,
    active_powerups: Query<Entity, With<PowerUp>>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    mut base_sprites: Query<(&BaseSprite, &mut Sprite), (Without<Player>, Without<Bullet>)>,
    mut enemy_tanks: Query<
        (
            Entity,
            &Tank,
            &mut Transform,
            &EnemyTank,
            &mut Health,
            Option<&SpawnProtection>,
        ),
        (With<EnemyTank>, Without<Player>, Without<Bullet>),
    >,
    mut player_tanks: Query<
        (
            Entity,
            &mut Tank,
            &mut Transform,
            &mut PlayerLives,
            &mut Health,
            &mut PlayerUpgrade,
            Option<&Shield>,
            &Player,
        ),
        (
            With<Player>,
            Without<BaseSprite>,
            Without<EnemyTank>,
            Without<Bullet>,
            Without<PowerUp>,
        ),
    >,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut bullet, mut transform) in &mut bullets {
        let facing = bullet.facing;
        let speed = bullet.speed;
        let previous_top_left = bullet.top_left;
        bullet.previous_top_left = previous_top_left;
        bullet.top_left += facing.movement() * speed * time.delta_secs();
        bullet.top_left = round_vec2(bullet.top_left);

        let center = bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0);
        if center.x < 0.0 || center.y < 0.0 || center.x >= board_size() || center.y >= board_size()
        {
            resolve_bullet(&mut commands, entity, &mut bullet);
            continue;
        }

        let tile_hit = bullet_blocking_tile_hit(&grid, previous_top_left, bullet.top_left);

        if *game_mode == GameMode::Campaign && bullet.owner.is_player() {
            let mut hit_enemy = false;
            for (
                enemy_entity,
                enemy_tank,
                mut enemy_transform,
                enemy,
                mut health,
                spawn_protection,
            ) in &mut enemy_tanks
            {
                if let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, enemy_tank.top_left)
                {
                    if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                        continue;
                    }

                    if spawn_protection.is_some() {
                        spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                        resolve_bullet(&mut commands, entity, &mut bullet);
                        play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                        hit_enemy = true;
                        break;
                    }

                    health.current -= 1;
                    let hit_sound = enemy_hit_sound(health.current);
                    if health.current <= 0 {
                        let enemy_top_left = enemy_tank.top_left;
                        score_board.record_enemy_destroyed(enemy.kind);
                        mark_enemy_tank_destroyed(
                            &mut commands,
                            &assets,
                            enemy_entity,
                            enemy_top_left,
                            &mut enemy_transform,
                        );
                        if let Some(powerup_kind) = enemy.carried_powerup {
                            spawn_powerup(
                                &mut commands,
                                &assets,
                                powerup_kind,
                                enemy_top_left,
                                active_powerups.iter(),
                                &active_sparkles,
                            );
                        }
                    } else {
                        spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                    }
                    play_sound(&mut commands, &sounds, hit_sound);
                    resolve_bullet(&mut commands, entity, &mut bullet);
                    hit_enemy = true;
                    break;
                }
            }
            if hit_enemy {
                continue;
            }
        }

        if game_mode.is_versus()
            && let Some(shooter) = bullet.owner.player_id()
        {
            let mut hit_player = false;
            for (
                player_entity,
                mut player_tank,
                mut player_transform,
                mut lives,
                mut player_health,
                mut upgrade,
                shield,
                player,
            ) in &mut player_tanks
            {
                if player.id == shooter {
                    continue;
                }
                let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, player_tank.top_left)
                else {
                    continue;
                };
                if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                    continue;
                }

                if shield.is_none() {
                    spawn_explosion(&mut commands, &assets, player_tank.top_left);
                    play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                    resolve_player_destroyed(
                        &mut commands,
                        &assets,
                        &sounds,
                        &mut game_status,
                        &mut score_board,
                        player_entity,
                        &mut player_tank,
                        &mut player_transform,
                        &mut lives,
                        &mut player_health,
                        &mut upgrade,
                        player.id,
                        Some(shooter),
                        *game_mode,
                    );
                } else {
                    spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                }
                resolve_bullet(&mut commands, entity, &mut bullet);
                hit_player = true;
                break;
            }
            if hit_player {
                continue;
            }
        }

        if bullet.owner == Team::Enemy {
            let mut hit_player = false;
            for (
                player_entity,
                mut player_tank,
                mut player_transform,
                mut lives,
                mut player_health,
                mut upgrade,
                shield,
                player,
            ) in &mut player_tanks
            {
                let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, player_tank.top_left)
                else {
                    continue;
                };
                if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                    continue;
                }

                if shield.is_some() {
                    spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                    resolve_bullet(&mut commands, entity, &mut bullet);
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                    hit_player = true;
                    break;
                }

                spawn_explosion(&mut commands, &assets, player_tank.top_left);
                play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                resolve_player_destroyed(
                    &mut commands,
                    &assets,
                    &sounds,
                    &mut game_status,
                    &mut score_board,
                    player_entity,
                    &mut player_tank,
                    &mut player_transform,
                    &mut lives,
                    &mut player_health,
                    &mut upgrade,
                    player.id,
                    None,
                    GameMode::Campaign,
                );
                resolve_bullet(&mut commands, entity, &mut bullet);
                hit_player = true;
                break;
            }
            if hit_player {
                continue;
            }
        }

        if let Some(tile_hit) = tile_hit {
            spawn_bullet_impact_effect(&mut commands, &assets, tile_hit.impact_top_left);
            if bullet_destroys_tile(tile_hit.tile, bullet.breaks_steel) {
                grid.set(tile_hit.x, tile_hit.y, TileKind::Empty);
                play_sound(
                    &mut commands,
                    &sounds,
                    if tile_hit.tile == TileKind::Steel {
                        SoundKind::SteelHit
                    } else {
                        SoundKind::BrickHit
                    },
                );
                for (tile_entity, grid_tile) in &tile_sprites {
                    if grid_tile.x == tile_hit.x && grid_tile.y == tile_hit.y {
                        commands.entity(tile_entity).despawn();
                        break;
                    }
                }
            }

            if tile_hit.tile == TileKind::Base && game_status.is_playing() {
                let mut hit_base = None;
                for (base, mut sprite) in &mut base_sprites {
                    if base_contains_tile(base.top_left, tile_hit.x, tile_hit.y) {
                        let can_destroy =
                            base_can_be_destroyed_by_bullet(*game_mode, bullet.owner, base.owner);
                        if can_destroy {
                            sprite.image = assets.base_destroyed.clone();
                        }
                        hit_base = Some((base.owner, base.top_left, can_destroy));
                        break;
                    }
                }

                let (base_owner, base_top_left, can_destroy_base) = hit_base.unwrap_or((
                    None,
                    base_top_left_from_grid(&grid).unwrap_or(Vec2::new(
                        tile_hit.x as f32 * TILE_SIZE,
                        tile_hit.y as f32 * TILE_SIZE,
                    )),
                    base_can_be_destroyed_by_bullet(*game_mode, bullet.owner, None),
                ));

                if can_destroy_base {
                    let sound_sequence = base_destroyed_sounds(*game_mode, base_owner);
                    match *game_mode {
                        GameMode::Campaign => {
                            game_status.phase = GamePhase::GameOver;
                        }
                        GameMode::VersusBaseBattle => {
                            if let Some(owner) = base_owner {
                                game_status.phase = GamePhase::RoundOver;
                                game_status.winner = Some(base_battle_winner_for_base(owner));
                            }
                        }
                        GameMode::VersusDeathmatch => {}
                    }

                    for sound in sound_sequence {
                        play_sound(&mut commands, &sounds, *sound);
                    }
                    spawn_base_destruction_effect(&mut commands, &assets, base_top_left);
                } else {
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                }
            } else if tile_hit.tile == TileKind::Steel && !bullet.breaks_steel {
                play_sound(&mut commands, &sounds, SoundKind::SteelHit);
            }

            resolve_bullet(&mut commands, entity, &mut bullet);
            continue;
        }

        transform.translation = board_object_center(
            bullet.top_left.x,
            bullet.top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        );
    }
}

fn resolve_bullet(commands: &mut Commands, entity: Entity, bullet: &mut Bullet) {
    bullet.resolved = true;
    commands.entity(entity).despawn();
}

fn resolve_player_destroyed(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_status: &mut GameStatus,
    score_board: &mut ScoreBoard,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
    lives: &mut PlayerLives,
    health: &mut Health,
    upgrade: &mut PlayerUpgrade,
    target: PlayerId,
    shooter: Option<PlayerId>,
    mode: GameMode,
) {
    lives.current -= 1;
    health.current = 1;

    match mode {
        GameMode::Campaign => {
            score_board.lives = lives.current;
            if lives.current <= 0 {
                game_status.phase = GamePhase::GameOver;
                play_sound(commands, sounds, SoundKind::GameOver);
                mark_player_tank_destroyed_terminal(
                    commands,
                    assets,
                    player_entity,
                    tank,
                    transform,
                );
                return;
            }
        }
        GameMode::VersusDeathmatch => {
            score_board.set_player_lives(target, lives.current);
            if let Some(shooter) = shooter {
                score_board.add_player_score(shooter);
                if let Some(winner) = deathmatch_winner_after_hit(
                    score_board.player_score(shooter),
                    lives.current,
                    score_board.target_score,
                    shooter,
                ) {
                    game_status.phase = GamePhase::RoundOver;
                    game_status.winner = Some(winner);
                    play_sound(commands, sounds, SoundKind::LevelClear);
                    mark_player_tank_destroyed_terminal(
                        commands,
                        assets,
                        player_entity,
                        tank,
                        transform,
                    );
                    return;
                }
            }
        }
        GameMode::VersusBaseBattle => {
            score_board.set_player_lives(target, lives.current);
            if let Some(shooter) = shooter {
                score_board.add_player_score(shooter);
                if lives.current <= 0 {
                    game_status.phase = GamePhase::RoundOver;
                    game_status.winner = Some(shooter);
                    play_sound(commands, sounds, SoundKind::LevelClear);
                    mark_player_tank_destroyed_terminal(
                        commands,
                        assets,
                        player_entity,
                        tank,
                        transform,
                    );
                    return;
                }
            }
        }
    }

    mark_player_tank_destroyed_for_respawn(
        commands,
        assets,
        player_entity,
        tank,
        transform,
        upgrade,
    );
}

fn reset_player_upgrade(upgrade: &mut PlayerUpgrade, sprite: &mut Sprite) {
    upgrade.level = 0;
    sprite.color = player_upgrade_visual_color(upgrade.level);
}

fn deathmatch_winner_after_hit(
    shooter_score: u32,
    target_lives: i32,
    target_score: u32,
    shooter: PlayerId,
) -> Option<PlayerId> {
    if shooter_score >= target_score || target_lives <= 0 {
        Some(shooter)
    } else {
        None
    }
}

fn base_battle_winner_for_base(base_owner: PlayerId) -> PlayerId {
    base_owner.opponent()
}

fn base_can_be_destroyed_by_bullet(
    game_mode: GameMode,
    bullet_owner: Team,
    base_owner: Option<PlayerId>,
) -> bool {
    match game_mode {
        GameMode::Campaign => true,
        GameMode::VersusBaseBattle => {
            matches!((bullet_owner.player_id(), base_owner), (Some(shooter), Some(owner)) if shooter != owner)
        }
        GameMode::VersusDeathmatch => false,
    }
}

fn base_destroyed_sounds(
    game_mode: GameMode,
    base_owner: Option<PlayerId>,
) -> &'static [SoundKind] {
    match game_mode {
        GameMode::Campaign => &CAMPAIGN_BASE_DESTROYED_SOUNDS,
        GameMode::VersusBaseBattle if base_owner.is_some() => &VERSUS_BASE_DESTROYED_SOUNDS,
        GameMode::VersusBaseBattle | GameMode::VersusDeathmatch => &NO_BASE_DESTROYED_SOUNDS,
    }
}

fn clock_freeze_target(game_mode: GameMode, collector: PlayerId) -> Option<PlayerId> {
    match game_mode {
        GameMode::Campaign => None,
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => Some(collector.opponent()),
    }
}

fn grenade_player_target(game_mode: GameMode, collector: PlayerId) -> Option<PlayerId> {
    match game_mode {
        GameMode::Campaign => None,
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => Some(collector.opponent()),
    }
}

fn grant_extra_life(
    lives: &mut PlayerLives,
    score_board: &mut ScoreBoard,
    game_mode: GameMode,
    player: PlayerId,
) {
    lives.current = lives.current.saturating_add(1).min(MAX_PLAYER_LIVES);
    match game_mode {
        GameMode::Campaign => {
            score_board.lives = lives.current;
        }
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
            score_board.set_player_lives(player, lives.current);
        }
    }
}

fn cancel_colliding_bullets(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    bullets: Query<(Entity, &Bullet)>,
) {
    if !bullet_clashes_can_resolve(game_status.phase) {
        return;
    }

    let bullets: Vec<(Entity, Vec2, Vec2)> = bullets
        .iter()
        .filter(|(_, bullet)| !bullet.resolved)
        .map(|(entity, bullet)| (entity, bullet.previous_top_left, bullet.top_left))
        .collect();
    let mut destroyed = HashSet::new();
    let mut clashes = Vec::new();

    for i in 0..bullets.len() {
        for j in (i + 1)..bullets.len() {
            if let Some(clash) =
                bullet_paths_clash(bullets[i].1, bullets[i].2, bullets[j].1, bullets[j].2)
            {
                clashes.push((
                    clash.time,
                    bullets[i].0,
                    bullets[j].0,
                    clash.impact_top_left,
                ));
            }
        }
    }

    clashes.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, first, second, impact_top_left) in clashes {
        if destroyed.contains(&first) || destroyed.contains(&second) {
            continue;
        }

        destroyed.insert(first);
        destroyed.insert(second);
        spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
        play_sound(&mut commands, &sounds, SoundKind::SteelHit);
    }

    for entity in destroyed {
        commands.entity(entity).despawn();
    }
}

fn bullet_clashes_can_resolve(phase: GamePhase) -> bool {
    phase == GamePhase::Playing
}

fn spawn_versus_powerups(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    mut director: ResMut<VersusPowerUpDirector>,
    active_powerups: Query<Entity, With<PowerUp>>,
) {
    if !game_mode.is_versus()
        || !game_status.is_playing()
        || !active_powerups.is_empty()
        || director.spawn_points.is_empty()
    {
        return;
    }

    let should_spawn =
        director.spawn_immediately || director.spawn_timer.tick(time.delta()).just_finished();
    if !should_spawn {
        return;
    }

    if let Some((top_left, kind)) = director.next_spawn() {
        spawn_powerup_entity(&mut commands, &assets, kind, top_left);
    }
}

fn pickup_powerups(
    mut commands: Commands,
    mut game_status: ResMut<GameStatus>,
    game_mode: Res<GameMode>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    mut tile_grid: ResMut<TileGrid>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    powerups: Query<(Entity, &PowerUp, &Transform), (With<PowerUp>, Without<Player>)>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    tile_sprites: Query<(Entity, &GridTile)>,
    bases: Query<&BaseSprite>,
    mut players: Query<
        (
            Entity,
            &mut Tank,
            &Player,
            &mut PlayerUpgrade,
            &mut PlayerLives,
            &mut Health,
            &mut Transform,
            &mut Sprite,
            Option<&Shield>,
        ),
        (With<Player>, Without<EnemyTank>, Without<PowerUp>),
    >,
    mut enemy_tanks: Query<
        (Entity, &Tank, &mut Transform, &EnemyTank),
        (With<EnemyTank>, Without<Player>, Without<PowerUp>),
    >,
    mut score_board: ResMut<ScoreBoard>,
) {
    if !game_status.is_playing() {
        return;
    }

    let active_powerup_entities: Vec<Entity> = powerups
        .iter()
        .map(|(powerup_entity, _, _)| powerup_entity)
        .collect();

    for (powerup_entity, powerup, transform) in &powerups {
        let powerup_top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        let mut grenade_target = None;
        for (
            player_entity,
            tank,
            player,
            mut upgrade,
            mut lives,
            _health,
            _transform,
            mut sprite,
            _shield,
        ) in &mut players
        {
            if !rects_overlap(
                tank.top_left,
                Vec2::splat(TANK_SIZE),
                powerup_top_left,
                Vec2::splat(TANK_SIZE),
            ) {
                continue;
            }

            match powerup.kind {
                PowerUpKind::Star => {
                    upgrade.level = (upgrade.level + 1).min(3);
                    sprite.color = player_upgrade_visual_color(upgrade.level);
                }
                PowerUpKind::Helmet => {
                    commands.entity(player_entity).insert(Shield {
                        timer: Timer::from_seconds(HELMET_SECONDS, TimerMode::Once),
                    });
                }
                PowerUpKind::Clock => {
                    if let Some(frozen_player) = clock_freeze_target(*game_mode, player.id) {
                        versus_freeze.start(frozen_player);
                    } else {
                        enemy_freeze.start();
                    }
                }
                PowerUpKind::Grenade => {
                    if let Some(target) = grenade_player_target(*game_mode, player.id) {
                        grenade_target = Some((player.id, target));
                    } else {
                        destroy_visible_enemies(
                            &mut commands,
                            &assets,
                            &sounds,
                            &mut score_board,
                            active_powerup_entities
                                .iter()
                                .copied()
                                .filter(|active| *active != powerup_entity),
                            &active_sparkles,
                            &mut enemy_tanks,
                        );
                    }
                }
                PowerUpKind::Shovel => {
                    let positions = shovel_reinforcement_positions(
                        *game_mode,
                        player.id,
                        &tile_grid,
                        bases.iter(),
                    );
                    reinforce_base_walls(
                        &mut commands,
                        &assets,
                        &mut tile_grid,
                        &tile_sprites,
                        &mut base_reinforcement,
                        positions,
                    );
                }
                PowerUpKind::Tank => {
                    grant_extra_life(&mut lives, &mut score_board, *game_mode, player.id);
                }
            }
            commands.entity(powerup_entity).despawn();
            despawn_powerup_sparkles(&mut commands, &active_sparkles);
            play_sound(&mut commands, &sounds, SoundKind::PowerupPickup);
            break;
        }

        if let Some((shooter, target)) = grenade_target {
            for (
                target_entity,
                mut target_tank,
                target_player,
                mut target_upgrade,
                mut target_lives,
                mut target_health,
                mut target_transform,
                _target_sprite,
                target_shield,
            ) in &mut players
            {
                if target_player.id != target {
                    continue;
                }

                if target_shield.is_some() {
                    let impact_top_left =
                        target_tank.top_left + Vec2::splat((TANK_SIZE - BULLET_SIZE) / 2.0);
                    spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                    break;
                }

                spawn_explosion(&mut commands, &assets, target_tank.top_left);
                play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                resolve_player_destroyed(
                    &mut commands,
                    &assets,
                    &sounds,
                    &mut game_status,
                    &mut score_board,
                    target_entity,
                    &mut target_tank,
                    &mut target_transform,
                    &mut target_lives,
                    &mut target_health,
                    &mut target_upgrade,
                    target,
                    Some(shooter),
                    *game_mode,
                );
                break;
            }
        }
    }
}

fn tick_powerup_effects(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    assets: Res<SpriteAssets>,
    mut tile_grid: ResMut<TileGrid>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    tile_sprites: Query<(Entity, &GridTile)>,
) {
    if !game_status.is_playing() {
        return;
    }

    enemy_freeze.tick(time.delta());
    versus_freeze.tick(time.delta());

    if base_reinforcement.tick(time.delta()) {
        restore_base_walls(
            &mut commands,
            &assets,
            &mut tile_grid,
            &tile_sprites,
            &mut base_reinforcement,
        );
    }
}

fn update_base_reinforcement_visuals(
    game_status: Res<GameStatus>,
    base_reinforcement: Res<BaseReinforcement>,
    mut tile_sprites: Query<(&GridTile, &mut Sprite)>,
) {
    if !game_status.is_playing() || base_reinforcement.saved_tiles.is_empty() {
        return;
    }

    let color = base_reinforcement
        .warning_elapsed_secs()
        .map(shovel_warning_visual_color)
        .unwrap_or(Color::WHITE);

    for (grid_tile, mut sprite) in &mut tile_sprites {
        if base_reinforcement.contains_position(grid_tile.x, grid_tile.y) {
            sprite.color = color;
        }
    }
}

fn update_powerup_visuals(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut powerups: Query<&mut Sprite, With<PowerUp>>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    let [r, g, b] = powerup_visual_rgb(time.elapsed_secs());
    for mut sprite in &mut powerups {
        sprite.color = Color::srgb_u8(r, g, b);
    }
}

fn destroy_visible_enemies<F: QueryFilter>(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    score_board: &mut ScoreBoard,
    active_powerups: impl IntoIterator<Item = Entity>,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
    enemy_tanks: &mut Query<(Entity, &Tank, &mut Transform, &EnemyTank), F>,
) {
    let mut destroyed_any = false;
    let mut powerup_drop = None;
    for (enemy_entity, enemy_tank, mut transform, enemy) in enemy_tanks {
        score_board.record_enemy_destroyed(enemy.kind);
        if powerup_drop.is_none()
            && let Some(powerup) = enemy.carried_powerup
        {
            powerup_drop = Some((enemy_tank.top_left, powerup));
        }
        mark_enemy_tank_destroyed(
            commands,
            assets,
            enemy_entity,
            enemy_tank.top_left,
            &mut transform,
        );
        destroyed_any = true;
    }

    if destroyed_any {
        play_sound(commands, sounds, SoundKind::TankExplosion);
    }
    if let Some((top_left, powerup)) = powerup_drop {
        spawn_powerup(
            commands,
            assets,
            powerup,
            top_left,
            active_powerups,
            active_sparkles,
        );
    }
}

fn shovel_reinforcement_positions<'a>(
    mode: GameMode,
    player: PlayerId,
    tile_grid: &TileGrid,
    bases: impl IntoIterator<Item = &'a BaseSprite>,
) -> Vec<(usize, usize)> {
    match mode {
        GameMode::Campaign => base_wall_positions(tile_grid),
        GameMode::VersusBaseBattle => bases
            .into_iter()
            .find(|base| base.owner == Some(player))
            .map(|base| base_wall_positions_for_top_left(tile_grid, base.top_left))
            .unwrap_or_default(),
        GameMode::VersusDeathmatch => Vec::new(),
    }
}

fn reinforce_base_walls(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    base_reinforcement: &mut BaseReinforcement,
    positions: Vec<(usize, usize)>,
) {
    if !base_reinforcement.saved_tiles.is_empty() {
        if reinforcement_matches_positions(&base_reinforcement.saved_tiles, &positions) {
            base_reinforcement.start();
            return;
        }

        restore_base_walls(
            commands,
            assets,
            tile_grid,
            tile_sprites,
            base_reinforcement,
        );
    }

    if positions.is_empty() {
        return;
    }

    base_reinforcement.saved_tiles = positions
        .iter()
        .map(|(x, y)| (*x, *y, tile_grid.tiles[y * BOARD_TILES + x]))
        .collect();

    for (x, y) in positions {
        sync_tile_sprite(
            commands,
            assets,
            tile_grid,
            tile_sprites,
            x,
            y,
            TileKind::Steel,
        );
    }
    base_reinforcement.start();
}

fn reinforcement_matches_positions(
    saved_tiles: &[(usize, usize, TileKind)],
    positions: &[(usize, usize)],
) -> bool {
    saved_tiles.len() == positions.len()
        && positions
            .iter()
            .all(|position| saved_tiles.iter().any(|(x, y, _)| (*x, *y) == *position))
}

fn restore_base_walls(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    base_reinforcement: &mut BaseReinforcement,
) {
    let saved_tiles = std::mem::take(&mut base_reinforcement.saved_tiles);
    for (x, y, tile) in saved_tiles {
        sync_tile_sprite(commands, assets, tile_grid, tile_sprites, x, y, tile);
    }
    base_reinforcement.reset();
}

fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut animations: Query<(Entity, &mut Sprite, &mut SpriteAnimation)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (entity, mut sprite, mut animation) in &mut animations {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if atlas.index >= animation.last {
            if animation.despawn_on_finish {
                commands.entity(entity).despawn();
            } else {
                atlas.index = animation.first;
            }
        } else {
            atlas.index += 1;
        }
    }
}

fn tick_destroyed_tanks(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut destroyed_tanks: Query<(Entity, &mut DestroyedTank)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (entity, mut destroyed_tank) in &mut destroyed_tanks {
        if destroyed_tank.tick(time.delta()) {
            commands.entity(entity).despawn();
        }
    }
}

fn tick_spawn_protections(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut protected_tanks: Query<(Entity, &mut SpawnProtection)>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut protection) in &mut protected_tanks {
        if protection.tick(time.delta()) {
            commands.entity(entity).remove::<SpawnProtection>();
        }
    }
}

fn tick_player_respawns(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    game_status: Res<GameStatus>,
    score_board: Res<ScoreBoard>,
    mut pending_players: Query<(
        Entity,
        &mut PlayerRespawnPending,
        &RespawnPoint,
        &mut PlayerUpgrade,
        &mut Transform,
        &mut Sprite,
        &mut TankSpriteState,
    )>,
    active_tanks: Query<&Tank>,
    mut respawning_players: Query<(Entity, &mut PlayerRespawnDelay)>,
) {
    if !game_status.is_playing() {
        return;
    }

    let mut occupied_positions: Vec<Vec2> = active_tanks.iter().map(|tank| tank.top_left).collect();

    for (
        entity,
        mut pending_respawn,
        respawn,
        mut upgrade,
        mut transform,
        mut sprite,
        mut tank_sprite,
    ) in &mut pending_players
    {
        if !pending_respawn.tick(time.delta()) {
            continue;
        }
        if !tank_spawn_position_free(respawn.top_left, &occupied_positions) {
            continue;
        }

        tank_sprite.frame = 0;
        tank_sprite.timer.reset();
        reset_player_upgrade(&mut upgrade, &mut sprite);
        set_tank_sprite_direction(&mut sprite, &tank_sprite, respawn.facing, &assets.manifest);
        transform.translation = board_object_center(
            respawn.top_left.x,
            respawn.top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0,
        );

        commands
            .entity(entity)
            .remove::<PlayerRespawnPending>()
            .insert((
                Tank {
                    top_left: respawn.top_left,
                    facing: respawn.facing,
                    speed: PLAYER_SPEED,
                },
                Health { current: 1 },
                Shield {
                    timer: Timer::from_seconds(
                        score_board.respawn_invulnerability_secs,
                        TimerMode::Once,
                    ),
                },
                PlayerRespawnDelay::for_spawn_shimmer(assets.manifest.spawn_shimmer_frames()),
            ));
        occupied_positions.push(respawn.top_left);
        spawn_spawn_effect(&mut commands, &assets, respawn.top_left);
    }

    for (entity, mut respawn_delay) in &mut respawning_players {
        if respawn_delay.tick(time.delta()) {
            commands.entity(entity).remove::<PlayerRespawnDelay>();
        }
    }
}

fn tick_shields(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut shielded: Query<
        (Entity, &mut Shield, &PlayerUpgrade, &mut Sprite),
        (
            With<Player>,
            Without<PlayerRespawnDelay>,
            Without<PlayerRespawnPending>,
            Without<DestroyedTank>,
        ),
    >,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut shield, upgrade, mut sprite) in &mut shielded {
        shield.timer.tick(time.delta());
        let [r, g, b] = player_shield_visual_rgb(shield.timer.elapsed_secs(), upgrade.level);
        sprite.color = Color::srgb_u8(r, g, b);

        if shield.timer.is_finished() {
            sprite.color = player_upgrade_visual_color(upgrade.level);
            commands.entity(entity).remove::<Shield>();
        }
    }
}

fn update_versus_frozen_player_visuals(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    versus_freeze: Res<VersusPlayerFreeze>,
    mut players: Query<
        (&Player, &PlayerUpgrade, Option<&Shield>, &mut Sprite),
        (
            With<Player>,
            Without<PlayerRespawnDelay>,
            Without<PlayerRespawnPending>,
            Without<DestroyedTank>,
        ),
    >,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (player, upgrade, shield, mut sprite) in &mut players {
        if versus_freeze.is_player_frozen(player.id) {
            let [r, g, b] = player_frozen_visual_rgb(time.elapsed_secs());
            sprite.color = Color::srgb_u8(r, g, b);
        } else if shield.is_none() {
            sprite.color = player_upgrade_visual_color(upgrade.level);
        }
    }
}

fn update_enemy_visual_feedback(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    mut enemies: Query<(&EnemyTank, &Health, Option<&SpawnProtection>, &mut Sprite)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    let frozen = enemy_freeze.is_active();
    for (enemy, health, spawn_protection, mut sprite) in &mut enemies {
        sprite.color = enemy_visual_color(
            enemy.kind,
            enemy.carried_powerup,
            health.current,
            time.elapsed_secs(),
            spawn_protection.is_some(),
            frozen,
        );
    }
}

fn check_game_phase(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    sounds: Res<SoundAssets>,
    mut game_status: ResMut<GameStatus>,
    mut score_board: ResMut<ScoreBoard>,
    director: Res<EnemyDirector>,
    active_enemies: Query<&EnemyTank>,
) {
    if *game_mode != GameMode::Campaign || !game_status.is_playing() {
        return;
    }

    let next_phase = campaign_phase(
        score_board.lives,
        score_board.total_enemies,
        score_board.enemies_destroyed,
        director.roster.len(),
        active_enemies.iter().count(),
    );
    if next_phase != GamePhase::Playing {
        game_status.phase = next_phase;
        game_status.transition_timer = Timer::from_seconds(
            campaign_phase_transition_seconds(next_phase),
            TimerMode::Once,
        );
        match next_phase {
            GamePhase::LevelClear => {
                score_board.score = score_board
                    .score
                    .saturating_add(stage_clear_bonus(score_board.lives));
                play_sound(&mut commands, &sounds, SoundKind::LevelClear);
            }
            GamePhase::GameOver => play_sound(&mut commands, &sounds, SoundKind::GameOver),
            _ => {}
        }
    }
}

fn clear_terminal_transients(
    mut commands: Commands,
    game_status: Res<GameStatus>,
    bullets: Query<Entity, With<Bullet>>,
    powerups: Query<Entity, With<PowerUp>>,
    sparkles: Query<Entity, With<PowerUpSparkle>>,
) {
    if !terminal_phase_clears_transients(game_status.phase) {
        return;
    }

    for entity in &bullets {
        commands.entity(entity).despawn();
    }
    for entity in &powerups {
        commands.entity(entity).despawn();
    }
    for entity in &sparkles {
        commands.entity(entity).despawn();
    }
}

fn advance_after_stage_intro(time: Res<Time>, mut game_status: ResMut<GameStatus>) {
    if game_status.phase != GamePhase::StageIntro {
        return;
    }

    game_status.transition_timer.tick(time.delta());
    if game_status.transition_timer.just_finished() {
        game_status.phase = GamePhase::Playing;
        game_status.transition_timer =
            Timer::from_seconds(LEVEL_CLEAR_DELAY_SECONDS, TimerMode::Once);
    }
}

fn advance_after_level_clear(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    mut game_status: ResMut<GameStatus>,
    mut tile_grid: ResMut<TileGrid>,
    mut director: ResMut<EnemyDirector>,
    mut score_board: ResMut<ScoreBoard>,
    mut stage_rules: ResMut<StageRules>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    if game_status.phase != GamePhase::LevelClear {
        return;
    }

    game_status.transition_timer.tick(time.delta());
    if !game_status.transition_timer.just_finished() {
        return;
    }

    if game_status.stage >= LEVEL_COUNT {
        enter_victory_screen(
            &mut commands,
            &mut game_status,
            &mut tile_grid,
            &mut director,
            &mut stage_rules,
            &mut enemy_freeze,
            &mut versus_freeze,
            &mut base_reinforcement,
            &game_entities,
        );
        return;
    }

    let next_stage = game_status.stage + 1;
    let (level, new_tile_grid) = load_stage_bundle_or_panic(next_stage);
    info!("Loaded {}", level.name);

    for entity in &game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(&mut commands, &assets, GameMode::Campaign);
    spawn_level(
        &mut commands,
        &assets,
        &level,
        &new_tile_grid,
        score_board.lives.max(1),
        DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
    );
    play_sound(&mut commands, &sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    *director = EnemyDirector::from_level(&level);
    *stage_rules = StageRules::from_level(&level);
    score_board.enemies_destroyed = 0;
    score_board.total_enemies = level.enemies.len();
    score_board.enemy_kills = EnemyKillCounts::default();
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.stage = next_stage;
    game_status.phase = GamePhase::StageIntro;
    game_status.transition_timer = Timer::from_seconds(STAGE_INTRO_SECONDS, TimerMode::Once);
}

fn enter_victory_screen(
    commands: &mut Commands,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    stage_rules: &mut StageRules,
    enemy_freeze: &mut EnemyFreeze,
    versus_freeze: &mut VersusPlayerFreeze,
    base_reinforcement: &mut BaseReinforcement,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    *tile_grid = TileGrid::empty();
    *director = EnemyDirector::inactive();
    *stage_rules = StageRules::default();
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::Victory;
    game_status.winner = None;
    game_status.transition_timer.reset();
}

fn update_status_panel(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    score_board: Res<ScoreBoard>,
    mut glyphs: Query<(&StatusGlyph, &mut Sprite)>,
    mut markers: Query<(&EnemyMarker, &mut Visibility)>,
    banners: Query<Entity, With<PhaseBanner>>,
) {
    for (glyph, mut sprite) in &mut glyphs {
        let text = status_value_text(glyph.kind, *game_mode, &game_status, &score_board);

        if let Some(ch) = text.chars().nth(glyph.digit)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = glyph_index(ch, &assets.manifest.glyphs);
        }
    }

    let enemies_remaining =
        enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed);
    for (marker, mut visibility) in &mut markers {
        *visibility = if marker.index < enemies_remaining {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if game_status.phase == GamePhase::Playing {
        for entity in &banners {
            commands.entity(entity).despawn();
        }
        return;
    }

    if !banners.is_empty() {
        return;
    }

    let Some(lines) = phase_banner_text(&game_status, *game_mode, &score_board) else {
        return;
    };
    spawn_phase_text(&mut commands, &assets, &lines, 114.5, 9.0);
}

fn status_value_text(
    kind: StatusValue,
    mode: GameMode,
    game_status: &GameStatus,
    score_board: &ScoreBoard,
) -> String {
    match kind {
        StatusValue::Score => match mode {
            GameMode::Campaign => format!("{:06}", score_board.score.min(999_999)),
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
                format!("{:02}", score_board.p1_score.min(99))
            }
        },
        StatusValue::Lives => match mode {
            GameMode::Campaign => format!("{}", score_board.lives.clamp(0, MAX_PLAYER_LIVES)),
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
                format!("{}", score_board.p1_lives.clamp(0, MAX_PLAYER_LIVES))
            }
        },
        StatusValue::Stage => format!("{:02}", game_status.stage.min(99)),
        StatusValue::P2Score => format!("{:02}", score_board.p2_score.min(99)),
        StatusValue::P2Lives => format!("{}", score_board.p2_lives.clamp(0, MAX_PLAYER_LIVES)),
        StatusValue::Arena => format!("{:02}", game_status.arena.min(99)),
        StatusValue::Target => format!("{:02}", score_board.target_score.min(99)),
    }
}

fn enemy_marker_top_left(index: usize) -> Vec2 {
    let col = index % ENEMY_MARKER_COLUMNS;
    let row = index / ENEMY_MARKER_COLUMNS;
    Vec2::new(
        ENEMY_MARKER_LEFT + col as f32 * ENEMY_MARKER_CELL_X,
        ENEMY_MARKER_TOP + row as f32 * ENEMY_MARKER_CELL_Y,
    )
}

fn enemy_markers_remaining(total_enemies: usize, enemies_destroyed: usize) -> usize {
    total_enemies.saturating_sub(enemies_destroyed)
}

fn enemy_marker_tank_index(manifest: &AssetManifest) -> usize {
    animated_tank_sprite_index(manifest, TankSpriteSet::EnemyBasic, Direction::Down, 0)
}

fn campaign_life_icon_top_left() -> Vec2 {
    Vec2::new(222.0, 123.0)
}

fn versus_life_icon_top_left(player: PlayerId) -> Vec2 {
    match player {
        PlayerId::One => Vec2::new(222.0, 73.0),
        PlayerId::Two => Vec2::new(222.0, 145.0),
    }
}

fn versus_arena_label_top_left() -> Vec2 {
    Vec2::new(214.0, 158.0)
}

fn versus_arena_number_top_left() -> Vec2 {
    Vec2::new(226.0, 169.0)
}

fn versus_target_label_top_left() -> Vec2 {
    Vec2::new(214.0, 184.0)
}

fn versus_target_number_top_left() -> Vec2 {
    Vec2::new(226.0, 195.0)
}

fn versus_base_label_top_left() -> Vec2 {
    Vec2::new(220.0, 190.0)
}

fn player_life_icon_tank_index(manifest: &AssetManifest, player: PlayerId) -> usize {
    animated_tank_sprite_index(manifest, TankSpriteSet::player(player), Direction::Up, 0)
}

fn score_badge_icon_top_left() -> Vec2 {
    Vec2::new(244.0, 38.0)
}

fn stage_flag_icon_top_left() -> Vec2 {
    Vec2::new(216.0, 87.0)
}

fn stage_number_top_left() -> Vec2 {
    Vec2::new(230.0, 87.0)
}

fn phase_text_width(text: &str) -> f32 {
    text.chars().count() as f32 * GLYPH_ADVANCE - 1.0
}

fn phase_banner_lines(
    phase: GamePhase,
    winner: Option<PlayerId>,
) -> Option<&'static [&'static str]> {
    match phase {
        GamePhase::ModeSelect | GamePhase::Playing => None,
        GamePhase::StageIntro => None,
        GamePhase::Paused => Some(&PAUSED_BANNER_LINES),
        GamePhase::GameOver => Some(&GAME_OVER_BANNER_LINES),
        GamePhase::LevelClear => Some(&LEVEL_CLEAR_BANNER_LINES),
        GamePhase::RoundOver => match winner {
            Some(PlayerId::One) => Some(&P1_WIN_BANNER_LINES),
            Some(PlayerId::Two) => Some(&P2_WIN_BANNER_LINES),
            None => Some(&GAME_OVER_BANNER_LINES),
        },
        GamePhase::Victory => Some(&VICTORY_BANNER_LINES),
    }
}

fn stage_intro_banner_text(stage: usize) -> Vec<String> {
    vec![format!("STAGE {:02}", stage.min(99)), "READY".to_string()]
}

fn level_clear_banner_text(stage: usize, score_board: &ScoreBoard) -> Vec<String> {
    vec![
        format!("STAGE {:02}", stage.min(99)),
        "LEVEL CLEAR".to_string(),
        stage_clear_kill_line(
            EnemyKind::Basic,
            score_board.enemy_kills.count(EnemyKind::Basic),
            EnemyKind::Fast,
            score_board.enemy_kills.count(EnemyKind::Fast),
        ),
        stage_clear_kill_line(
            EnemyKind::Power,
            score_board.enemy_kills.count(EnemyKind::Power),
            EnemyKind::Armor,
            score_board.enemy_kills.count(EnemyKind::Armor),
        ),
        format!("TOTAL {:02}", score_board.enemy_kills.total().min(99)),
        format!("BONUS {}", stage_clear_bonus(score_board.lives)),
    ]
}

fn stage_clear_kill_line(
    left: EnemyKind,
    left_count: usize,
    right: EnemyKind,
    right_count: usize,
) -> String {
    format!(
        "{}X{:02} {}X{:02}",
        enemy_score(left),
        left_count.min(99),
        enemy_score(right),
        right_count.min(99)
    )
}

fn arena_intro_banner_text(arena: usize, mode: GameMode) -> Vec<String> {
    vec![
        format!("ARENA {:02}", arena.min(99)),
        arena_intro_kind_label(mode).to_string(),
        "READY".to_string(),
    ]
}

fn arena_intro_kind_label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::VersusDeathmatch => "DUEL",
        GameMode::VersusBaseBattle => "BASE BATTLE",
        GameMode::Campaign => "READY",
    }
}

fn phase_banner_text(
    status: &GameStatus,
    mode: GameMode,
    score_board: &ScoreBoard,
) -> Option<Vec<String>> {
    if status.phase == GamePhase::StageIntro {
        return Some(match mode {
            GameMode::Campaign => stage_intro_banner_text(status.stage),
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
                arena_intro_banner_text(status.arena, mode)
            }
        });
    }
    if status.phase == GamePhase::LevelClear {
        return Some(level_clear_banner_text(status.stage, score_board));
    }

    phase_banner_lines(status.phase, status.winner)
        .map(|lines| lines.iter().map(|line| (*line).to_string()).collect())
}

fn campaign_phase(
    lives: i32,
    total_enemies: usize,
    enemies_destroyed: usize,
    roster_remaining: usize,
    active_enemies: usize,
) -> GamePhase {
    if lives <= 0 {
        return GamePhase::GameOver;
    }

    if enemies_destroyed >= total_enemies || (roster_remaining == 0 && active_enemies == 0) {
        GamePhase::LevelClear
    } else {
        GamePhase::Playing
    }
}

fn campaign_phase_transition_seconds(phase: GamePhase) -> f32 {
    if phase == GamePhase::LevelClear {
        LEVEL_CLEAR_SCORECARD_SECONDS
    } else {
        LEVEL_CLEAR_DELAY_SECONDS
    }
}

fn stage_clear_bonus(lives: i32) -> u32 {
    lives.max(0) as u32 * STAGE_CLEAR_LIFE_BONUS
}

fn toggle_pause_phase(phase: GamePhase) -> GamePhase {
    match phase {
        GamePhase::Playing => GamePhase::Paused,
        GamePhase::Paused => GamePhase::Playing,
        phase => phase,
    }
}

fn visual_effects_can_advance(phase: GamePhase) -> bool {
    phase != GamePhase::Paused
}

fn terminal_phase_clears_transients(phase: GamePhase) -> bool {
    matches!(
        phase,
        GamePhase::GameOver | GamePhase::LevelClear | GamePhase::RoundOver | GamePhase::Victory
    )
}

fn next_arena(current: usize) -> usize {
    if current >= ARENA_COUNT {
        1
    } else {
        current + 1
    }
}

fn previous_arena(current: usize) -> usize {
    if current <= 1 {
        ARENA_COUNT
    } else {
        current - 1
    }
}

fn next_stage(current: usize) -> usize {
    if current >= LEVEL_COUNT {
        1
    } else {
        current + 1
    }
}

fn previous_stage(current: usize) -> usize {
    if current <= 1 {
        LEVEL_COUNT
    } else {
        current - 1
    }
}

fn selected_campaign_stage(mode_select: &ModeSelect) -> usize {
    mode_select.stage.clamp(1, LEVEL_COUNT)
}

fn next_mode_select_option(option: ModeSelectOption) -> ModeSelectOption {
    match option {
        ModeSelectOption::Campaign => ModeSelectOption::Battle,
        ModeSelectOption::Battle => ModeSelectOption::Music,
        ModeSelectOption::Music => ModeSelectOption::Sound,
        ModeSelectOption::Sound => ModeSelectOption::Campaign,
    }
}

fn previous_mode_select_option(option: ModeSelectOption) -> ModeSelectOption {
    match option {
        ModeSelectOption::Campaign => ModeSelectOption::Sound,
        ModeSelectOption::Battle => ModeSelectOption::Campaign,
        ModeSelectOption::Music => ModeSelectOption::Battle,
        ModeSelectOption::Sound => ModeSelectOption::Music,
    }
}

fn update_mode_select_stage_digits(
    glyphs: &mut Query<(&ModeSelectStageGlyph, &mut Sprite)>,
    glyph_manifest: &GlyphManifest,
    stage: usize,
) {
    let text = format!("{:02}", stage.min(99));
    for (glyph, mut sprite) in glyphs {
        if let Some(ch) = text.chars().nth(glyph.digit)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = glyph_index(ch, glyph_manifest);
        }
    }
}

fn mode_select_option_top_left(option: ModeSelectOption) -> Vec2 {
    match option {
        ModeSelectOption::Campaign => Vec2::new(82.0, 95.0),
        ModeSelectOption::Battle => Vec2::new(88.0, 113.0),
        ModeSelectOption::Music => Vec2::new(82.0, 131.0),
        ModeSelectOption::Sound => Vec2::new(82.0, 145.0),
    }
}

fn mode_select_cursor_translation(option: ModeSelectOption) -> Vec3 {
    let option = mode_select_option_top_left(option);
    board_object_center(
        60.0,
        option.y - 4.0 - BOARD_ORIGIN_Y,
        Vec2::splat(TANK_SIZE),
        0.3,
    )
}

fn update_mode_select_cursor(
    cursors: &mut Query<&mut Transform, With<ModeSelectCursor>>,
    selected: ModeSelectOption,
) {
    let translation = mode_select_cursor_translation(selected);
    for mut transform in cursors {
        transform.translation = translation;
    }
}

fn update_mode_select_arena_digits(
    glyphs: &mut Query<(&ModeSelectArenaGlyph, &mut Sprite)>,
    glyph_manifest: &GlyphManifest,
    arena: usize,
) {
    let text = format!("{:02}", arena.min(99));
    for (glyph, mut sprite) in glyphs {
        if let Some(ch) = text.chars().nth(glyph.digit)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = glyph_index(ch, glyph_manifest);
        }
    }
}

fn update_mode_select_battle_kind(
    glyphs: &mut Query<(&ModeSelectBattleKindGlyph, &mut Sprite)>,
    glyph_manifest: &GlyphManifest,
    arena: usize,
) {
    let text = battle_kind_label_for_arena(arena);
    for (glyph, mut sprite) in glyphs {
        let ch = text.chars().nth(glyph.digit).unwrap_or(' ');
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = glyph_index(ch, glyph_manifest);
        }
    }
}

fn update_mode_select_music_value(
    glyphs: &mut Query<(&ModeSelectMusicGlyph, &mut Sprite)>,
    glyph_manifest: &GlyphManifest,
    mode: AudioMode,
) {
    let text = audio_mode_label(mode);
    for (glyph, mut sprite) in glyphs {
        let ch = text.chars().nth(glyph.digit).unwrap_or(' ');
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = glyph_index(ch, glyph_manifest);
        }
    }
}

fn update_mode_select_sound_value(
    glyphs: &mut Query<(&ModeSelectSoundGlyph, &mut Sprite)>,
    glyph_manifest: &GlyphManifest,
    enabled: bool,
) {
    let text = sound_enabled_label(enabled);
    for (glyph, mut sprite) in glyphs {
        let ch = text.chars().nth(glyph.digit).unwrap_or(' ');
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = glyph_index(ch, glyph_manifest);
        }
    }
}

fn player_last_direction(control: &PlayerControl, player: PlayerId) -> Direction {
    match player {
        PlayerId::One => control.p1_last_direction,
        PlayerId::Two => control.p2_last_direction,
    }
}

fn update_direction_priority(
    keys: &ButtonInput<KeyCode>,
    player: PlayerId,
    priority: &mut Vec<Direction>,
    last_direction: &mut Direction,
) {
    for (key, direction) in direction_key_pairs(player) {
        if keys.just_pressed(key) {
            record_direction_press(priority, direction);
        }
    }

    prune_direction_priority(priority, |direction| {
        direction_is_held(keys, direction, player)
    });
    if let Some(direction) = preferred_direction(priority) {
        *last_direction = direction;
    }
}

fn direction_key_pairs(player: PlayerId) -> [(KeyCode, Direction); 4] {
    match player {
        PlayerId::One => [
            (KeyCode::KeyW, Direction::Up),
            (KeyCode::KeyS, Direction::Down),
            (KeyCode::KeyA, Direction::Left),
            (KeyCode::KeyD, Direction::Right),
        ],
        PlayerId::Two => [
            (KeyCode::ArrowUp, Direction::Up),
            (KeyCode::ArrowDown, Direction::Down),
            (KeyCode::ArrowLeft, Direction::Left),
            (KeyCode::ArrowRight, Direction::Right),
        ],
    }
}

fn record_direction_press(priority: &mut Vec<Direction>, direction: Direction) {
    priority.retain(|held| *held != direction);
    priority.push(direction);
}

fn prune_direction_priority(priority: &mut Vec<Direction>, is_held: impl Fn(Direction) -> bool) {
    priority.retain(|direction| is_held(*direction));
}

fn preferred_direction(priority: &[Direction]) -> Option<Direction> {
    priority.last().copied()
}

fn held_direction(
    keys: &ButtonInput<KeyCode>,
    last_direction: Direction,
    player: PlayerId,
) -> Option<Direction> {
    if direction_is_held(keys, last_direction, player) {
        return Some(last_direction);
    }

    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .find(|direction| direction_is_held(keys, *direction, player))
}

fn direction_is_held(keys: &ButtonInput<KeyCode>, direction: Direction, player: PlayerId) -> bool {
    match (player, direction) {
        (PlayerId::One, Direction::Up) => keys.pressed(KeyCode::KeyW),
        (PlayerId::One, Direction::Down) => keys.pressed(KeyCode::KeyS),
        (PlayerId::One, Direction::Left) => keys.pressed(KeyCode::KeyA),
        (PlayerId::One, Direction::Right) => keys.pressed(KeyCode::KeyD),
        (PlayerId::Two, Direction::Up) => keys.pressed(KeyCode::ArrowUp),
        (PlayerId::Two, Direction::Down) => keys.pressed(KeyCode::ArrowDown),
        (PlayerId::Two, Direction::Left) => keys.pressed(KeyCode::ArrowLeft),
        (PlayerId::Two, Direction::Right) => keys.pressed(KeyCode::ArrowRight),
    }
}

fn player_fire_pressed(keys: &ButtonInput<KeyCode>, player: PlayerId) -> bool {
    match player {
        PlayerId::One => keys.pressed(KeyCode::Space),
        PlayerId::Two => keys.pressed(KeyCode::Enter) || keys.pressed(KeyCode::ShiftRight),
    }
}

fn snap_to_lane(top_left: &mut Vec2, direction: Direction) {
    match direction {
        Direction::Up | Direction::Down => {
            let snapped = (top_left.x / TILE_SIZE).round() * TILE_SIZE;
            if (top_left.x - snapped).abs() <= SNAP_DISTANCE {
                top_left.x = snapped;
            }
        }
        Direction::Left | Direction::Right => {
            let snapped = (top_left.y / TILE_SIZE).round() * TILE_SIZE;
            if (top_left.y - snapped).abs() <= SNAP_DISTANCE {
                top_left.y = snapped;
            }
        }
    }
}

fn spawn_bullet_position(tank_top_left: Vec2, direction: Direction) -> Vec2 {
    let center = tank_top_left + Vec2::splat(TANK_SIZE / 2.0);
    match direction {
        Direction::Up => Vec2::new(center.x - BULLET_SIZE / 2.0, tank_top_left.y - BULLET_SIZE),
        Direction::Down => Vec2::new(center.x - BULLET_SIZE / 2.0, tank_top_left.y + TANK_SIZE),
        Direction::Left => Vec2::new(tank_top_left.x - BULLET_SIZE, center.y - BULLET_SIZE / 2.0),
        Direction::Right => Vec2::new(tank_top_left.x + TANK_SIZE, center.y - BULLET_SIZE / 2.0),
    }
}

fn explosion_duration_secs(frames: SpriteFrameRange) -> f32 {
    (frames.last - frames.first + 1) as f32 * EXPLOSION_FRAME_SECONDS
}

fn spawn_shimmer_duration_secs(frames: SpriteFrameRange) -> f32 {
    (frames.last - frames.first + 1) as f32 * SPAWN_SHIMMER_FRAME_SECONDS
}

fn spawn_bullet_impact_effect(
    commands: &mut Commands,
    assets: &SpriteAssets,
    bullet_top_left: Vec2,
) {
    let frames = assets.manifest.bullet_impact_frames();
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            8.1,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(BULLET_IMPACT_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

fn spawn_explosion(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
    let frames = assets.manifest.explosion_frames();
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            8.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(EXPLOSION_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

fn mark_enemy_tank_destroyed(
    commands: &mut Commands,
    assets: &SpriteAssets,
    enemy_entity: Entity,
    top_left: Vec2,
    transform: &mut Transform,
) {
    let frames = assets.manifest.explosion_frames();
    spawn_explosion(commands, assets, top_left);
    park_tank_transform(transform);
    commands
        .entity(enemy_entity)
        .remove::<(Tank, Health, EnemyTank, EnemyAi, SpawnProtection)>()
        .insert(DestroyedTank::for_explosion(frames));
}

fn parked_tank_top_left() -> Vec2 {
    Vec2::new(-TANK_SIZE * 4.0, -TANK_SIZE * 4.0)
}

fn parked_tank_translation() -> Vec3 {
    let top_left = parked_tank_top_left();
    board_object_center(top_left.x, top_left.y, Vec2::splat(TANK_SIZE), 6.0)
}

fn park_tank_transform(transform: &mut Transform) {
    transform.translation = parked_tank_translation();
}

fn park_tank(tank: &mut Tank, transform: &mut Transform) {
    tank.top_left = parked_tank_top_left();
    park_tank_transform(transform);
}

fn mark_player_tank_destroyed_for_respawn(
    commands: &mut Commands,
    assets: &SpriteAssets,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
    upgrade: &mut PlayerUpgrade,
) {
    park_tank(tank, transform);
    upgrade.level = 0;
    commands
        .entity(player_entity)
        .remove::<(Tank, Health, Shield)>()
        .insert(PlayerRespawnPending::for_explosion(
            assets.manifest.explosion_frames(),
        ));
}

fn mark_player_tank_destroyed_terminal(
    commands: &mut Commands,
    assets: &SpriteAssets,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
) {
    park_tank(tank, transform);
    commands
        .entity(player_entity)
        .remove::<(
            Tank,
            Health,
            Shield,
            PlayerRespawnDelay,
            PlayerRespawnPending,
        )>()
        .insert(DestroyedTank::for_explosion(
            assets.manifest.explosion_frames(),
        ));
}

fn spawn_spawn_effect(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
    let frames = assets.manifest.spawn_shimmer_frames();
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            8.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(SPAWN_SHIMMER_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

fn spawn_base_destruction_effect(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
    let frames = assets.manifest.base_destruction_frames();
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            8.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.09, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

fn spawn_powerup(
    commands: &mut Commands,
    assets: &SpriteAssets,
    kind: PowerUpKind,
    top_left: Vec2,
    active_powerups: impl IntoIterator<Item = Entity>,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
) {
    for active_powerup in active_powerups {
        commands.entity(active_powerup).despawn();
    }
    despawn_powerup_sparkles(commands, active_sparkles);

    spawn_powerup_entity(commands, assets, kind, top_left);
}

fn spawn_powerup_entity(
    commands: &mut Commands,
    assets: &SpriteAssets,
    kind: PowerUpKind,
    top_left: Vec2,
) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.powerup_image.clone(),
            TextureAtlas {
                layout: assets.powerup_layout.clone(),
                index: assets.manifest.powerup_index(kind),
            },
        ),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            5.5,
        ))
        .with_scale(Vec3::splat(window_scale())),
        PowerUp { kind },
        GameEntity,
    ));
    spawn_powerup_sparkle_effect(commands, assets, top_left);
}

fn spawn_powerup_sparkle_effect(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
    let frames = assets.manifest.powerup_sparkle_frames();
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            5.8,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            despawn_on_finish: false,
        },
        PowerUpSparkle,
        GameEntity,
    ));
}

fn despawn_powerup_sparkles(
    commands: &mut Commands,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
) {
    for sparkle in active_sparkles {
        commands.entity(sparkle).despawn();
    }
}

fn tank_center(top_left: Vec2) -> Vec2 {
    top_left + Vec2::splat(TANK_SIZE / 2.0)
}

fn closest_player_center(from_center: Vec2, player_top_lefts: &[Vec2]) -> Option<Vec2> {
    let mut closest = None;
    let mut closest_distance = f32::MAX;

    for player_top_left in player_top_lefts {
        let player_center = tank_center(*player_top_left);
        let distance = from_center.distance_squared(player_center);
        if distance < closest_distance {
            closest = Some(player_center);
            closest_distance = distance;
        }
    }

    closest
}

fn axis_direction_toward(from_center: Vec2, target_center: Vec2) -> Direction {
    let delta = target_center - from_center;
    if delta.x.abs() > delta.y.abs() {
        if delta.x < 0.0 {
            Direction::Left
        } else {
            Direction::Right
        }
    } else if delta.y < 0.0 {
        Direction::Up
    } else {
        Direction::Down
    }
}

fn aligned_fire_direction(from_center: Vec2, target_center: Vec2) -> Option<Direction> {
    let delta = target_center - from_center;
    if delta.x.abs() <= TILE_SIZE / 2.0 && delta.y.abs() >= TILE_SIZE {
        Some(if delta.y < 0.0 {
            Direction::Up
        } else {
            Direction::Down
        })
    } else if delta.y.abs() <= TILE_SIZE / 2.0 && delta.x.abs() >= TILE_SIZE {
        Some(if delta.x < 0.0 {
            Direction::Left
        } else {
            Direction::Right
        })
    } else {
        None
    }
}

fn enemy_aim_direction(
    enemy_top_left: Vec2,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
) -> Option<Direction> {
    let enemy_center = tank_center(enemy_top_left);
    for player_top_left in player_top_lefts {
        if let Some(direction) = aligned_fire_direction(enemy_center, tank_center(*player_top_left))
        {
            return Some(direction);
        }
    }

    base_center.and_then(|base| aligned_fire_direction(enemy_center, base))
}

fn preferred_enemy_direction(
    top_left: Vec2,
    current: Direction,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
) -> Direction {
    if let Some(direction) = enemy_aim_direction(top_left, player_top_lefts, base_center) {
        return direction;
    }

    let own_center = tank_center(top_left);
    if let Some(player_center) = closest_player_center(own_center, player_top_lefts) {
        let delta = player_center - own_center;
        if delta.x.abs() > delta.y.abs() && delta.x.abs() > TANK_SIZE {
            return if delta.x < 0.0 {
                Direction::Left
            } else {
                Direction::Right
            };
        }
    }

    if let Some(base_center) = base_center {
        let delta = base_center - own_center;
        if delta.length_squared() > TANK_SIZE * TANK_SIZE {
            if enemy_should_roam(top_left, current) {
                return enemy_patrol_direction(top_left, current);
            }
            return axis_direction_toward(own_center, base_center);
        }
    }

    enemy_patrol_direction(top_left, current)
}

fn enemy_patrol_direction(top_left: Vec2, current: Direction) -> Direction {
    if top_left.y < 20.0 {
        Direction::Down
    } else {
        enemy_roam_direction(top_left, current)
    }
}

fn enemy_should_roam(top_left: Vec2, current: Direction) -> bool {
    enemy_roam_seed(top_left, current).is_multiple_of(4)
}

fn enemy_roam_direction(top_left: Vec2, current: Direction) -> Direction {
    direction_from_index((enemy_roam_seed(top_left, current) / 4) % 4)
}

fn enemy_roam_seed(top_left: Vec2, current: Direction) -> u32 {
    let tile_x = (top_left.x / TILE_SIZE).round() as u32;
    let tile_y = (top_left.y / TILE_SIZE).round() as u32;
    tile_x.wrapping_mul(31) ^ tile_y.wrapping_mul(17) ^ direction_index(current).wrapping_mul(13)
}

fn direction_index(direction: Direction) -> u32 {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

fn direction_from_index(index: u32) -> Direction {
    match index % 4 {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        _ => Direction::Left,
    }
}

fn enemy_alignment_fire_ready(kind: EnemyKind, elapsed_secs: f32) -> bool {
    elapsed_secs >= enemy_fire_interval(kind) * ENEMY_ALIGNMENT_FIRE_FRACTION
}

fn enemy_fire_slot_available(active_enemy_bullets: usize, active_for_tank: usize) -> bool {
    active_enemy_bullets < ENEMY_BULLET_LIMIT && active_for_tank < ENEMY_BULLET_LIMIT_PER_TANK
}

fn enemy_random_fire_ready(top_left: Vec2, facing: Direction, kind: EnemyKind) -> bool {
    enemy_fire_seed(top_left, facing, kind).is_multiple_of(enemy_random_fire_rate(kind))
}

fn enemy_random_fire_rate(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Power => 2,
        EnemyKind::Fast => 3,
        EnemyKind::Basic | EnemyKind::Armor => 4,
    }
}

fn enemy_fire_seed(top_left: Vec2, facing: Direction, kind: EnemyKind) -> u32 {
    let tile_x = (top_left.x / TILE_SIZE).round() as u32;
    let tile_y = (top_left.y / TILE_SIZE).round() as u32;
    tile_x.wrapping_mul(29)
        ^ tile_y.wrapping_mul(37)
        ^ direction_index(facing).wrapping_mul(11)
        ^ enemy_kind_index(kind).wrapping_mul(19)
}

fn enemy_kind_index(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Basic => 0,
        EnemyKind::Fast => 1,
        EnemyKind::Power => 2,
        EnemyKind::Armor => 3,
    }
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn tank_move_speed(base_speed: f32, grid: &TileGrid, top_left: Vec2) -> f32 {
    if grid.tank_overlaps_tile(top_left, TileKind::Ice) {
        base_speed * ICE_SPEED_MULTIPLIER
    } else {
        base_speed
    }
}

fn enemy_speed(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Fast => 72.0,
        EnemyKind::Power => 56.0,
        EnemyKind::Armor => 48.0,
        EnemyKind::Basic => 52.0,
    }
}

fn enemy_health(kind: EnemyKind) -> i32 {
    match kind {
        EnemyKind::Armor => 3,
        _ => 1,
    }
}

fn enemy_fire_interval(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Power => 1.0,
        EnemyKind::Fast => 1.5,
        EnemyKind::Armor => 1.8,
        EnemyKind::Basic => 1.6,
    }
}

fn enemy_bullet_speed(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Power => POWER_ENEMY_BULLET_SPEED,
        EnemyKind::Basic | EnemyKind::Fast | EnemyKind::Armor => BULLET_SPEED,
    }
}

fn enemy_score(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Basic => 100,
        EnemyKind::Fast => 200,
        EnemyKind::Power => 300,
        EnemyKind::Armor => 400,
    }
}

fn enemy_hit_sound(health_after_hit: i32) -> SoundKind {
    if health_after_hit <= 0 {
        SoundKind::TankExplosion
    } else {
        SoundKind::SteelHit
    }
}

fn enemy_visual_color(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
    frozen: bool,
) -> Color {
    let [r, g, b] = enemy_display_rgb(
        kind,
        carried_powerup,
        health,
        elapsed_secs,
        spawn_protected,
        frozen,
    );
    Color::srgb_u8(r, g, b)
}

fn enemy_display_rgb(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
    frozen: bool,
) -> [u8; 3] {
    if spawn_protected && elapsed_secs % 0.16 < 0.08 {
        return [160, 220, 255];
    }
    if frozen {
        return enemy_frozen_visual_rgb(elapsed_secs);
    }

    enemy_visual_rgb(kind, carried_powerup, health, elapsed_secs)
}

fn enemy_frozen_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [136, 216, 255]
    } else {
        [216, 248, 255]
    }
}

fn enemy_visual_rgb(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
) -> [u8; 3] {
    if carried_powerup.is_some() && elapsed_secs % 0.25 < 0.125 {
        return [248, 232, 96];
    }

    match (kind, health) {
        (EnemyKind::Armor, 1) => [248, 168, 88],
        (EnemyKind::Armor, 2) => [216, 96, 72],
        (EnemyKind::Armor, _) => [168, 184, 216],
        (EnemyKind::Power, _) => [248, 112, 112],
        (EnemyKind::Fast, _) => [112, 216, 128],
        (EnemyKind::Basic, _) => [255, 255, 255],
    }
}

fn player_upgrade_visual_color(upgrade_level: u8) -> Color {
    let [r, g, b] = player_upgrade_visual_rgb(upgrade_level);
    Color::srgb_u8(r, g, b)
}

fn player_upgrade_visual_rgb(upgrade_level: u8) -> [u8; 3] {
    match upgrade_level.min(3) {
        0 => [255, 255, 255],
        1 => [184, 248, 184],
        2 => [255, 232, 104],
        _ => [255, 176, 104],
    }
}

fn player_shield_visual_rgb(elapsed_secs: f32, upgrade_level: u8) -> [u8; 3] {
    if elapsed_secs % 0.25 < 0.125 {
        [160, 220, 255]
    } else {
        player_upgrade_visual_rgb(upgrade_level)
    }
}

fn player_frozen_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [136, 216, 255]
    } else {
        [216, 248, 255]
    }
}

fn shovel_warning_visual_color(elapsed_secs: f32) -> Color {
    let [r, g, b] = shovel_warning_visual_rgb(elapsed_secs);
    Color::srgb_u8(r, g, b)
}

fn shovel_warning_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [255, 255, 255]
    } else {
        [248, 232, 96]
    }
}

fn player_bullet_limit(upgrade_level: u8) -> usize {
    if upgrade_level >= 2 { 2 } else { 1 }
}

fn player_bullet_speed(upgrade_level: u8) -> f32 {
    if upgrade_level >= 1 {
        PLAYER_FAST_BULLET_SPEED
    } else {
        BULLET_SPEED
    }
}

fn player_bullets_break_steel(upgrade_level: u8, stage_rules: StageRules) -> bool {
    upgrade_level >= 3 && stage_rules.player_steel_destruction
}

fn bullet_destroys_tile(tile: TileKind, breaks_steel: bool) -> bool {
    matches!(tile, TileKind::Brick) || (breaks_steel && tile == TileKind::Steel)
}

fn bullet_tank_hit(start_top_left: Vec2, end_top_left: Vec2, tank_top_left: Vec2) -> Option<Vec2> {
    let (start, delta, steps) = bullet_sweep(start_top_left, end_top_left);

    for step in 1..=steps {
        let center = start + delta * (step as f32 / steps as f32);
        let impact_top_left = round_vec2(center - Vec2::splat(BULLET_SIZE / 2.0));
        if rects_overlap(
            impact_top_left,
            Vec2::splat(BULLET_SIZE),
            tank_top_left,
            Vec2::splat(TANK_SIZE),
        ) {
            return Some(impact_top_left);
        }
    }

    None
}

fn bullet_hit_is_before_tile(
    start_top_left: Vec2,
    impact_top_left: Vec2,
    tile_hit: Option<BulletTileHit>,
) -> bool {
    let Some(tile_hit) = tile_hit else {
        return true;
    };

    bullet_impact_distance_squared(start_top_left, impact_top_left)
        < bullet_impact_distance_squared(start_top_left, tile_hit.impact_top_left)
}

fn bullet_impact_distance_squared(start_top_left: Vec2, impact_top_left: Vec2) -> f32 {
    let start_center = start_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let impact_center = impact_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    start_center.distance_squared(impact_center)
}

fn bullet_sweep(start_top_left: Vec2, end_top_left: Vec2) -> (Vec2, Vec2, usize) {
    let start = start_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let end = end_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let delta = end - start;
    let steps = ((delta.length() / (TILE_SIZE / 2.0)).ceil() as usize).max(1);
    (start, delta, steps)
}

fn bullet_overlapped_tile_range(top_left: Vec2) -> Option<(usize, usize, usize, usize)> {
    let board_size = board_size();
    if top_left.x + BULLET_SIZE <= 0.0
        || top_left.y + BULLET_SIZE <= 0.0
        || top_left.x >= board_size
        || top_left.y >= board_size
    {
        return None;
    }

    let left = (top_left.x.max(0.0) / TILE_SIZE).floor() as usize;
    let right_edge = (top_left.x + BULLET_SIZE - 0.1)
        .max(0.0)
        .min(board_size - 0.1);
    let right = (right_edge / TILE_SIZE).floor() as usize;
    let top = (top_left.y.max(0.0) / TILE_SIZE).floor() as usize;
    let bottom_edge = (top_left.y + BULLET_SIZE - 0.1)
        .max(0.0)
        .min(board_size - 0.1);
    let bottom = (bottom_edge / TILE_SIZE).floor() as usize;

    Some((left, right, top, bottom))
}

fn bullet_blocking_tile_key(delta: Vec2, tile_x: usize, tile_y: usize) -> (i32, i32, i32) {
    if delta.x.abs() > delta.y.abs() {
        let primary = if delta.x < 0.0 {
            -(tile_x as i32)
        } else {
            tile_x as i32
        };
        (0, primary, tile_y as i32)
    } else if delta.y.abs() > delta.x.abs() {
        let primary = if delta.y < 0.0 {
            -(tile_y as i32)
        } else {
            tile_y as i32
        };
        (0, primary, tile_x as i32)
    } else {
        (1, tile_y as i32, tile_x as i32)
    }
}

fn first_blocking_tile_overlapped_by_bullet(
    grid: &TileGrid,
    top_left: Vec2,
    delta: Vec2,
) -> Option<(usize, usize, TileKind)> {
    let (left, right, top, bottom) = bullet_overlapped_tile_range(top_left)?;
    let mut best = None;

    for tile_y in top..=bottom {
        for tile_x in left..=right {
            let tile = grid.tiles[tile_y * BOARD_TILES + tile_x];
            if !tile.bullet_blocks() {
                continue;
            }

            let key = bullet_blocking_tile_key(delta, tile_x, tile_y);
            match best {
                Some((_, _, _, best_key)) if best_key <= key => {}
                _ => best = Some((tile_x, tile_y, tile, key)),
            }
        }
    }

    best.map(|(tile_x, tile_y, tile, _)| (tile_x, tile_y, tile))
}

fn bullet_blocking_tile_hit(
    grid: &TileGrid,
    start_top_left: Vec2,
    end_top_left: Vec2,
) -> Option<BulletTileHit> {
    let (start, delta, steps) = bullet_sweep(start_top_left, end_top_left);

    for step in 1..=steps {
        let center = start + delta * (step as f32 / steps as f32);
        let impact_top_left = round_vec2(center - Vec2::splat(BULLET_SIZE / 2.0));
        if let Some((tile_x, tile_y, tile)) =
            first_blocking_tile_overlapped_by_bullet(grid, impact_top_left, delta)
        {
            return Some(BulletTileHit {
                x: tile_x,
                y: tile_y,
                tile,
                impact_top_left,
            });
        }
    }

    None
}

fn validate_level_positions(level: &LevelDefinition, grid: &TileGrid) -> Result<(), String> {
    validate_tank_spawn(grid, "player spawn", &level.player_spawn)?;

    for (index, spawn) in level.enemy_spawns.iter().enumerate() {
        let label = format!("enemy spawn {}", index + 1);
        validate_tank_spawn(grid, &label, spawn)?;
    }

    validate_base_position(grid, "base position", &level.base_position)?;
    validate_classic_campaign_base_position(&level.base_position)
}

fn validate_tank_spawn(grid: &TileGrid, label: &str, spawn: &SpawnPoint) -> Result<(), String> {
    let top_left = Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE);
    if grid.can_tank_occupy(top_left) {
        Ok(())
    } else {
        Err(format!(
            "{label} ({}, {}) must fit a tank on passable tiles",
            spawn.x, spawn.y
        ))
    }
}

fn validate_arena_spawns(grid: &TileGrid, arena: &ArenaDefinition) -> Result<(), String> {
    validate_tank_spawn(grid, "p1 spawn", &arena.p1_spawn)?;
    validate_tank_spawn(grid, "p2 spawn", &arena.p2_spawn)?;

    let p1_top_left = spawn_point_top_left(&arena.p1_spawn);
    let p2_top_left = spawn_point_top_left(&arena.p2_spawn);
    if tank_rects_overlap(p1_top_left, p2_top_left) {
        return Err(format!(
            "p1 spawn ({}, {}) and p2 spawn ({}, {}) must not overlap",
            arena.p1_spawn.x, arena.p1_spawn.y, arena.p2_spawn.x, arena.p2_spawn.y
        ));
    }

    Ok(())
}

fn validate_base_position(grid: &TileGrid, label: &str, point: &GridPoint) -> Result<(), String> {
    if point.x >= BOARD_TILES - 1 || point.y >= BOARD_TILES - 1 {
        return Err(format!(
            "{label} ({}, {}) must fit a 2x2 base inside the battlefield",
            point.x, point.y
        ));
    }

    for y in point.y..=(point.y + 1) {
        for x in point.x..=(point.x + 1) {
            if grid.get(x as i32, y as i32) != Some(TileKind::Base) {
                return Err(format!(
                    "{label} ({}, {}) must cover a 2x2 base tile area",
                    point.x, point.y
                ));
            }
        }
    }

    Ok(())
}

fn validate_classic_campaign_base_position(point: &GridPoint) -> Result<(), String> {
    if point.x == CLASSIC_BASE_X && point.y == CLASSIC_BASE_Y {
        return Ok(());
    }

    Err(format!(
        "base position ({}, {}) must use classic campaign base ({CLASSIC_BASE_X}, {CLASSIC_BASE_Y})",
        point.x, point.y
    ))
}

fn validate_base_positions_do_not_overlap(
    p1_base: GridPoint,
    p2_base: GridPoint,
) -> Result<(), String> {
    if rects_overlap(
        grid_point_top_left(&p1_base),
        Vec2::splat(TANK_SIZE),
        grid_point_top_left(&p2_base),
        Vec2::splat(TANK_SIZE),
    ) {
        return Err(format!(
            "p1 base ({}, {}) and p2 base ({}, {}) must not overlap",
            p1_base.x, p1_base.y, p2_base.x, p2_base.y
        ));
    }

    Ok(())
}

fn validate_powerup_spawns(grid: &TileGrid, points: &[GridPoint]) -> Result<(), String> {
    let mut seen = HashSet::new();

    for (index, point) in points.iter().enumerate() {
        let spawn_index = index + 1;
        validate_powerup_spawn(grid, spawn_index, point)?;

        if !seen.insert((point.x, point.y)) {
            return Err(format!(
                "power-up spawn {spawn_index} ({}, {}) is configured more than once",
                point.x, point.y
            ));
        }
    }

    Ok(())
}

fn validate_powerup_spawn(grid: &TileGrid, index: usize, point: &GridPoint) -> Result<(), String> {
    let top_left = Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE);
    if grid.can_tank_occupy(top_left) {
        Ok(())
    } else {
        Err(format!(
            "power-up spawn {index} ({}, {}) must fit a 16x16 reward on passable tiles",
            point.x, point.y
        ))
    }
}

fn validate_powerup_carriers(level: &LevelDefinition) -> Result<(), String> {
    let mut seen = HashSet::new();
    for carrier in &level.powerup_carriers {
        if carrier.enemy == 0 || carrier.enemy > level.enemies.len() {
            return Err(format!(
                "powerup carrier enemy {} is outside the 1..={} roster",
                carrier.enemy,
                level.enemies.len()
            ));
        }
        if !seen.insert(carrier.enemy) {
            return Err(format!(
                "powerup carrier enemy {} is configured more than once",
                carrier.enemy
            ));
        }
    }

    Ok(())
}

fn validate_classic_enemy_spawns(spawns: &[SpawnPoint]) -> Result<(), String> {
    let expected = [
        (0, 0, Direction::Down),
        (12, 0, Direction::Down),
        (24, 0, Direction::Down),
    ];

    for (index, (spawn, (x, y, facing))) in spawns.iter().zip(expected).enumerate() {
        if spawn.x != x || spawn.y != y || spawn.facing != facing {
            return Err(format!(
                "enemy spawn {} must be classic top spawn ({x}, {y}, {facing:?}), got ({}, {}, {:?})",
                index + 1,
                spawn.x,
                spawn.y,
                spawn.facing
            ));
        }
    }

    Ok(())
}

fn carrier_powerup_for_spawn(
    spawn_number: usize,
    carriers: &[PowerUpCarrier],
) -> Option<PowerUpKind> {
    carriers
        .iter()
        .find(|carrier| carrier.enemy == spawn_number)
        .map(|carrier| carrier.kind)
}

fn powerup_for_cycle(index: usize) -> PowerUpKind {
    match index % 6 {
        0 => PowerUpKind::Star,
        1 => PowerUpKind::Helmet,
        2 => PowerUpKind::Clock,
        3 => PowerUpKind::Grenade,
        4 => PowerUpKind::Shovel,
        _ => PowerUpKind::Tank,
    }
}

fn powerup_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.30 < 0.15 {
        [255, 255, 255]
    } else {
        [255, 232, 104]
    }
}

fn animated_tank_sprite_index(
    manifest: &AssetManifest,
    set: TankSpriteSet,
    direction: Direction,
    frame: usize,
) -> usize {
    manifest.tank_index(set, direction, frame)
}

fn set_tank_sprite_direction(
    sprite: &mut Sprite,
    tank_sprite: &TankSpriteState,
    facing: Direction,
    manifest: &AssetManifest,
) {
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index =
            animated_tank_sprite_index(manifest, tank_sprite.set, facing, tank_sprite.frame);
    }
}

fn update_tank_sprite(
    sprite: &mut Sprite,
    tank_sprite: &mut TankSpriteState,
    facing: Direction,
    moving: bool,
    delta: Duration,
    manifest: &AssetManifest,
) {
    if moving {
        tank_sprite.timer.tick(delta);
        if tank_sprite.timer.just_finished() {
            tank_sprite.frame = 1 - tank_sprite.frame;
        }
    } else {
        tank_sprite.frame = 0;
        tank_sprite.timer.reset();
    }

    set_tank_sprite_direction(sprite, tank_sprite, facing, manifest);
}

fn tank_rects_overlap(a: Vec2, b: Vec2) -> bool {
    rects_overlap(a, Vec2::splat(TANK_SIZE), b, Vec2::splat(TANK_SIZE))
}

fn tank_position_free(candidate: Vec2, current: Vec2, occupied: &[Vec2]) -> bool {
    occupied
        .iter()
        .filter(|position| **position != current)
        .all(|position| !tank_rects_overlap(candidate, *position))
}

fn tank_spawn_position_free(candidate: Vec2, occupied: &[Vec2]) -> bool {
    occupied
        .iter()
        .all(|position| !tank_rects_overlap(candidate, *position))
}

fn bullet_positions_overlap(a: Vec2, b: Vec2) -> bool {
    rects_overlap(a, Vec2::splat(BULLET_SIZE), b, Vec2::splat(BULLET_SIZE))
}

fn bullet_paths_clash(
    a_start: Vec2,
    a_end: Vec2,
    b_start: Vec2,
    b_end: Vec2,
) -> Option<BulletPathClash> {
    if bullet_positions_overlap(a_start, b_start) {
        return Some(BulletPathClash {
            impact_top_left: bullet_clash_impact_top_left(a_start, b_start),
            time: 0.0,
        });
    }

    let a_delta = a_end - a_start;
    let b_delta = b_end - b_start;
    let relative_delta = b_delta - a_delta;
    let expanded_min = a_start - Vec2::splat(BULLET_SIZE);
    let expanded_max = a_start + Vec2::splat(BULLET_SIZE);

    let (x_entry, x_exit) =
        swept_axis_times(b_start.x, relative_delta.x, expanded_min.x, expanded_max.x)?;
    let (y_entry, y_exit) =
        swept_axis_times(b_start.y, relative_delta.y, expanded_min.y, expanded_max.y)?;

    let entry_time = x_entry.max(y_entry);
    let exit_time = x_exit.min(y_exit);
    if entry_time > exit_time || !(0.0..=1.0).contains(&entry_time) {
        return None;
    }

    Some(BulletPathClash {
        impact_top_left: round_vec2(bullet_clash_impact_top_left(
            a_start + a_delta * entry_time,
            b_start + b_delta * entry_time,
        )),
        time: entry_time,
    })
}

fn swept_axis_times(
    point: f32,
    delta: f32,
    expanded_min: f32,
    expanded_max: f32,
) -> Option<(f32, f32)> {
    if delta == 0.0 {
        return (point >= expanded_min && point <= expanded_max)
            .then_some((f32::NEG_INFINITY, f32::INFINITY));
    }

    let first = (expanded_min - point) / delta;
    let second = (expanded_max - point) / delta;
    Some((first.min(second), first.max(second)))
}

fn bullet_clash_impact_top_left(a: Vec2, b: Vec2) -> Vec2 {
    (a + b) / 2.0
}

fn rects_overlap(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    a.x < b.x + b_size.x && a.x + a_size.x > b.x && a.y < b.y + b_size.y && a.y + a_size.y > b.y
}

fn board_size() -> f32 {
    BOARD_TILES as f32 * TILE_SIZE
}

fn board_tile_center(x: usize, y: usize, z: f32) -> Vec3 {
    board_object_center(
        x as f32 * TILE_SIZE,
        y as f32 * TILE_SIZE,
        Vec2::splat(TILE_SIZE),
        z,
    )
}

fn board_object_center(local_x: f32, local_y: f32, size: Vec2, z: f32) -> Vec3 {
    virtual_center_scaled(
        Vec2::new(BOARD_ORIGIN_X + local_x, BOARD_ORIGIN_Y + local_y),
        size,
        z,
    )
}

fn board_top_left_from_translation(translation: Vec3, object_size: f32) -> Vec2 {
    let center_x = translation.x / window_scale() + VIRTUAL_WIDTH / 2.0;
    let center_y = VIRTUAL_HEIGHT / 2.0 - translation.y / window_scale();
    Vec2::new(
        center_x - object_size / 2.0 - BOARD_ORIGIN_X,
        center_y - object_size / 2.0 - BOARD_ORIGIN_Y,
    )
}

fn virtual_center_scaled(top_left: Vec2, size: Vec2, z: f32) -> Vec3 {
    let center = top_left + size / 2.0;
    Vec3::new(
        (center.x - VIRTUAL_WIDTH / 2.0) * window_scale(),
        (VIRTUAL_HEIGHT / 2.0 - center.y) * window_scale(),
        z,
    )
}

fn round_vec2(value: Vec2) -> Vec2 {
    Vec2::new(value.x.round(), value.y.round())
}

fn terrain_z(tile: TileKind) -> f32 {
    match tile {
        TileKind::Forest => 7.5,
        TileKind::Water => 1.0,
        _ => 2.0,
    }
}

fn atlas_tile_size(manifest: GeneratedAtlasManifest) -> UVec2 {
    UVec2::new(manifest.tile_width as u32, manifest.tile_height as u32)
}

fn personal_asset_disk_path(asset_path: &str) -> PathBuf {
    Path::new(ASSET_ROOT_DIR).join(asset_path)
}

fn personal_asset_exists(asset_path: &str) -> bool {
    personal_asset_disk_path(asset_path).is_file()
}

fn image_handle_or_generated(
    asset_server: &AssetServer,
    images: &mut Assets<Image>,
    asset_path: &'static str,
    generated: impl FnOnce() -> Image,
) -> Handle<Image> {
    if personal_asset_exists(asset_path) {
        asset_server.load(asset_path)
    } else {
        images.add(generated())
    }
}

fn create_sprite_assets(
    asset_server: &AssetServer,
    images: &mut Assets<Image>,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> SpriteAssets {
    let manifest =
        load_asset_manifest(&runtime_asset_manifest_path()).expect("asset manifest should load");
    let terrain_image =
        image_handle_or_generated(asset_server, images, PERSONAL_TERRAIN_ATLAS_PATH, || {
            create_terrain_atlas(manifest.atlases.terrain)
        });
    let terrain_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        atlas_tile_size(manifest.atlases.terrain),
        manifest.atlases.terrain.tiles as u32,
        1,
        None,
        None,
    ));

    let tank_image =
        image_handle_or_generated(asset_server, images, PERSONAL_TANK_ATLAS_PATH, || {
            create_tank_atlas(manifest.atlases.tanks)
        });
    let tank_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        atlas_tile_size(manifest.atlases.tanks),
        manifest.atlases.tanks.tiles as u32,
        1,
        None,
        None,
    ));

    let bullet_image =
        image_handle_or_generated(asset_server, images, PERSONAL_BULLET_ATLAS_PATH, || {
            create_bullet_atlas(manifest.atlases.bullets)
        });
    let bullet_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        atlas_tile_size(manifest.atlases.bullets),
        manifest.atlases.bullets.tiles as u32,
        1,
        None,
        None,
    ));

    let effect_image =
        image_handle_or_generated(asset_server, images, PERSONAL_EFFECT_ATLAS_PATH, || {
            create_effect_atlas(manifest.atlases.effects)
        });
    let effect_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        atlas_tile_size(manifest.atlases.effects),
        manifest.atlases.effects.tiles as u32,
        1,
        None,
        None,
    ));

    let powerup_image =
        image_handle_or_generated(asset_server, images, PERSONAL_POWERUP_ATLAS_PATH, || {
            create_powerup_atlas(manifest.atlases.powerups)
        });
    let powerup_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        atlas_tile_size(manifest.atlases.powerups),
        manifest.atlases.powerups.tiles as u32,
        1,
        None,
        None,
    ));

    let glyph_image =
        image_handle_or_generated(asset_server, images, PERSONAL_GLYPH_ATLAS_PATH, || {
            create_glyph_atlas(&manifest.glyphs)
        });
    let glyph_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(
            manifest.glyphs.tile_width as u32,
            manifest.glyphs.tile_height as u32,
        ),
        manifest.glyphs.characters.chars().count() as u32,
        1,
        None,
        None,
    ));

    let base_intact =
        image_handle_or_generated(asset_server, images, PERSONAL_BASE_INTACT_PATH, || {
            create_base_image(manifest.base.intact, false)
        });
    let base_destroyed =
        image_handle_or_generated(asset_server, images, PERSONAL_BASE_DESTROYED_PATH, || {
            create_base_image(manifest.base.destroyed, true)
        });
    let score_badge_icon =
        image_handle_or_generated(asset_server, images, PERSONAL_SCORE_BADGE_PATH, || {
            create_score_badge_icon(manifest.ui.score_badge)
        });
    let stage_flag_icon =
        image_handle_or_generated(asset_server, images, PERSONAL_STAGE_FLAG_PATH, || {
            create_stage_flag_icon(manifest.ui.stage_flag)
        });

    SpriteAssets {
        manifest,
        terrain_image,
        terrain_layout,
        tank_image,
        tank_layout,
        bullet_image,
        bullet_layout,
        effect_image,
        effect_layout,
        powerup_image,
        powerup_layout,
        glyph_image,
        glyph_layout,
        base_intact,
        base_destroyed,
        score_badge_icon,
        stage_flag_icon,
    }
}

fn sound_handle_or_generated(
    asset_server: &AssetServer,
    sounds: &mut Assets<RetroSound>,
    asset_path: &'static str,
    spec: &RetroSoundSpec,
) -> SoundHandle {
    if personal_asset_exists(asset_path) {
        SoundHandle::File(asset_server.load(asset_path))
    } else {
        SoundHandle::Retro(sounds.add(make_manifest_sound(spec)))
    }
}

fn create_sound_assets(
    asset_server: &AssetServer,
    sounds: &mut Assets<RetroSound>,
    manifest: &AssetManifest,
) -> SoundAssets {
    SoundAssets {
        sound_enabled: true,
        fire: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_FIRE_SOUND_PATH,
            &manifest.sounds.fire,
        ),
        brick_hit: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_BRICK_HIT_SOUND_PATH,
            &manifest.sounds.brick_hit,
        ),
        steel_hit: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_STEEL_HIT_SOUND_PATH,
            &manifest.sounds.steel_hit,
        ),
        tank_explosion: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_TANK_EXPLOSION_SOUND_PATH,
            &manifest.sounds.tank_explosion,
        ),
        base_destroyed: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_BASE_DESTROYED_SOUND_PATH,
            &manifest.sounds.base_destroyed,
        ),
        powerup_pickup: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_POWERUP_PICKUP_SOUND_PATH,
            &manifest.sounds.powerup_pickup,
        ),
        stage_start: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_STAGE_START_SOUND_PATH,
            &manifest.sounds.stage_start,
        ),
        level_clear: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_LEVEL_CLEAR_SOUND_PATH,
            &manifest.sounds.level_clear,
        ),
        game_over: sound_handle_or_generated(
            asset_server,
            sounds,
            PERSONAL_GAME_OVER_SOUND_PATH,
            &manifest.sounds.game_over,
        ),
        background_music: background_music_handle_or_generated(asset_server, sounds),
    }
}

fn background_music_handle_or_generated(
    asset_server: &AssetServer,
    sounds: &mut Assets<RetroSound>,
) -> SoundHandle {
    if personal_asset_exists(PERSONAL_BACKGROUND_MUSIC_SOUND_PATH) {
        SoundHandle::File(asset_server.load(PERSONAL_BACKGROUND_MUSIC_SOUND_PATH))
    } else {
        SoundHandle::Retro(sounds.add(make_background_music_sound()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
struct SoundNote {
    duration_secs: f32,
    frequency: f32,
    volume: f32,
}

fn play_sound(commands: &mut Commands, sounds: &SoundAssets, kind: SoundKind) {
    if !sounds.sound_enabled {
        return;
    }

    let handle = match kind {
        SoundKind::Fire => &sounds.fire,
        SoundKind::BrickHit => &sounds.brick_hit,
        SoundKind::SteelHit => &sounds.steel_hit,
        SoundKind::TankExplosion => &sounds.tank_explosion,
        SoundKind::BaseDestroyed => &sounds.base_destroyed,
        SoundKind::PowerupPickup => &sounds.powerup_pickup,
        SoundKind::StageStart => &sounds.stage_start,
        SoundKind::LevelClear => &sounds.level_clear,
        SoundKind::GameOver => &sounds.game_over,
    };
    let playback = PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.45));
    match handle {
        SoundHandle::Retro(handle) => {
            commands.spawn((AudioPlayer(handle.clone()), playback));
        }
        SoundHandle::File(handle) => {
            commands.spawn((AudioPlayer(handle.clone()), playback));
        }
    }
}

fn sync_background_music(
    mut commands: Commands,
    mode_select: Res<ModeSelect>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    music: Query<Entity, With<BackgroundMusic>>,
) {
    if background_music_should_play(mode_select.audio_mode, game_status.phase) {
        if music.is_empty() {
            play_background_music(&mut commands, &sounds);
        }
        return;
    }

    for entity in &music {
        commands.entity(entity).despawn();
    }
}

fn background_music_should_play(mode: AudioMode, phase: GamePhase) -> bool {
    mode == AudioMode::Bgm && matches!(phase, GamePhase::StageIntro | GamePhase::Playing)
}

fn play_background_music(commands: &mut Commands, sounds: &SoundAssets) {
    let playback = PlaybackSettings::LOOP.with_volume(Volume::Linear(BACKGROUND_MUSIC_VOLUME));
    match &sounds.background_music {
        SoundHandle::Retro(handle) => {
            commands.spawn((AudioPlayer(handle.clone()), playback, BackgroundMusic));
        }
        SoundHandle::File(handle) => {
            commands.spawn((AudioPlayer(handle.clone()), playback, BackgroundMusic));
        }
    }
}

fn make_manifest_sound(spec: &RetroSoundSpec) -> RetroSound {
    match spec {
        RetroSoundSpec::Sweep {
            duration_secs,
            start_frequency,
            end_frequency,
            volume,
        } => make_sweep_sound(*duration_secs, *start_frequency, *end_frequency, *volume),
        RetroSoundSpec::Noise {
            duration_secs,
            volume,
            seed,
        } => make_noise_sound(*duration_secs, *volume, *seed),
        RetroSoundSpec::Layered { notes } => make_layered_sound(notes),
    }
}

fn make_sweep_sound(
    duration_secs: f32,
    start_frequency: f32,
    end_frequency: f32,
    volume: f32,
) -> RetroSound {
    let sample_count = sample_count(duration_secs);
    let mut samples = Vec::with_capacity(sample_count);
    let mut phase = 0.0_f32;

    for index in 0..sample_count {
        let t = index as f32 / sample_count as f32;
        let frequency = start_frequency + (end_frequency - start_frequency) * t;
        phase = (phase + frequency / SOUND_SAMPLE_RATE as f32) % 1.0;
        let wave = if phase < 0.5 { 1.0 } else { -1.0 };
        samples.push(wave * volume * decay_envelope(t));
    }

    sound_from_samples(samples)
}

fn make_noise_sound(duration_secs: f32, volume: f32, seed: u32) -> RetroSound {
    let sample_count = sample_count(duration_secs);
    let mut samples = Vec::with_capacity(sample_count);
    let mut state = seed.max(1);

    for index in 0..sample_count {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let t = index as f32 / sample_count as f32;
        let bit = if state & 0x8000_0000 == 0 { -1.0 } else { 1.0 };
        samples.push(bit * volume * decay_envelope(t));
    }

    sound_from_samples(samples)
}

fn make_layered_sound(notes: &[SoundNote]) -> RetroSound {
    let mut samples = Vec::new();
    for note in notes {
        append_square_note(&mut samples, *note);
    }
    sound_from_samples(samples)
}

fn make_background_music_sound() -> RetroSound {
    let melody = [
        Some(293.66),
        None,
        Some(349.23),
        Some(293.66),
        None,
        Some(261.63),
        Some(293.66),
        None,
        Some(392.0),
        None,
        Some(349.23),
        Some(329.63),
        Some(293.66),
        None,
        Some(246.94),
        None,
        Some(293.66),
        Some(293.66),
        Some(349.23),
        None,
        Some(440.0),
        None,
        Some(392.0),
        Some(349.23),
        Some(329.63),
        None,
        Some(293.66),
        None,
        Some(246.94),
        Some(261.63),
        Some(293.66),
        None,
    ];
    let mut samples = Vec::new();

    for (index, frequency) in melody.into_iter().enumerate() {
        let bass = if index % 8 < 4 { 73.42 } else { 98.0 };
        append_background_music_step(&mut samples, frequency, bass, index as u32);
    }

    sound_from_samples(samples)
}

fn append_background_music_step(
    samples: &mut Vec<f32>,
    melody_frequency: Option<f32>,
    bass_frequency: f32,
    step: u32,
) {
    let sample_count = sample_count(BACKGROUND_MUSIC_STEP_SECONDS);
    let mut melody_phase = 0.0_f32;
    let mut bass_phase = 0.0_f32;
    let mut noise_state = 0x9e37_79b9_u32 ^ step.wrapping_mul(0x85eb_ca6b);

    for index in 0..sample_count {
        let t = index as f32 / sample_count as f32;
        bass_phase = (bass_phase + bass_frequency / SOUND_SAMPLE_RATE as f32) % 1.0;
        let bass_wave = if bass_phase < 0.5 { 1.0 } else { -1.0 };
        let melody_wave = if let Some(frequency) = melody_frequency {
            melody_phase = (melody_phase + frequency / SOUND_SAMPLE_RATE as f32) % 1.0;
            if melody_phase < 0.5 { 1.0 } else { -1.0 }
        } else {
            0.0
        };
        noise_state = noise_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        let noise_wave = if noise_state & 0x8000_0000 == 0 {
            -1.0
        } else {
            1.0
        };
        let snare = background_music_percussion(step, t, noise_wave);
        let sample = (melody_wave * 0.075 + bass_wave * 0.055) * music_gate_envelope(t) + snare;
        samples.push(sample.clamp(-1.0, 1.0));
    }
}

fn background_music_percussion(step: u32, t: f32, noise_wave: f32) -> f32 {
    let decay = (1.0 - t).clamp(0.0, 1.0).powi(4);
    if step.is_multiple_of(4) {
        noise_wave * 0.055 * decay
    } else if step % 4 == 2 {
        noise_wave * 0.035 * decay
    } else {
        0.0
    }
}

fn music_gate_envelope(t: f32) -> f32 {
    let attack = (t / 0.02).clamp(0.0, 1.0);
    let release = ((1.0 - t) / 0.10).clamp(0.0, 1.0);
    attack.min(release)
}

fn append_square_note(samples: &mut Vec<f32>, note: SoundNote) {
    let sample_count = sample_count(note.duration_secs);
    let mut phase = 0.0_f32;
    for index in 0..sample_count {
        let t = index as f32 / sample_count as f32;
        phase = (phase + note.frequency / SOUND_SAMPLE_RATE as f32) % 1.0;
        let wave = if phase < 0.5 { 1.0 } else { -1.0 };
        samples.push(wave * note.volume * decay_envelope(t));
    }
}

fn sample_count(duration_secs: f32) -> usize {
    (duration_secs * SOUND_SAMPLE_RATE as f32).round().max(1.0) as usize
}

fn decay_envelope(t: f32) -> f32 {
    let attack = (t / 0.08).clamp(0.0, 1.0);
    let release = (1.0 - t).clamp(0.0, 1.0);
    attack * release * release
}

fn sound_from_samples(samples: Vec<f32>) -> RetroSound {
    RetroSound {
        samples: samples.into(),
        sample_rate: SOUND_SAMPLE_RATE,
    }
}

fn create_terrain_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let width = manifest.tile_width * manifest.tiles;
    let mut pixels = vec![0; width * manifest.tile_height * 4];

    draw_brick(&mut pixels, width, 0);
    draw_steel(&mut pixels, width, manifest.tile_width);
    draw_water(&mut pixels, width, manifest.tile_width * 2, 0);
    draw_water(&mut pixels, width, manifest.tile_width * 3, 1);
    draw_forest(&mut pixels, width, manifest.tile_width * 4);
    draw_ice(&mut pixels, width, manifest.tile_width * 5);

    image_from_pixels(width, manifest.tile_height, pixels)
}

fn draw_brick(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [128, 56, 32, 255]);
    fill_rect(pixels, width, x_offset, 3, 8, 1, [48, 24, 16, 255]);
    fill_rect(pixels, width, x_offset + 3, 0, 1, 3, [48, 24, 16, 255]);
    fill_rect(pixels, width, x_offset + 5, 4, 1, 4, [48, 24, 16, 255]);
    fill_rect(pixels, width, x_offset, 0, 8, 1, [184, 88, 48, 255]);
}

fn draw_steel(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [112, 120, 128, 255]);
    fill_rect(pixels, width, x_offset, 0, 8, 1, [200, 208, 208, 255]);
    fill_rect(pixels, width, x_offset, 0, 1, 8, [200, 208, 208, 255]);
    fill_rect(pixels, width, x_offset + 7, 0, 1, 8, [40, 48, 56, 255]);
    fill_rect(pixels, width, x_offset, 7, 8, 1, [40, 48, 56, 255]);
    fill_rect(pixels, width, x_offset + 3, 3, 2, 2, [64, 72, 80, 255]);
}

fn draw_water(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [24, 64, 144, 255]);
    for y in [1, 4, 6] {
        for x in 0..8 {
            if !(x + y + frame).is_multiple_of(3) {
                set_pixel(pixels, width, x_offset + x, y, [80, 144, 224, 255]);
            }
        }
    }
}

fn draw_forest(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [24, 96, 40, 230]);
    for (x, y) in [(1, 1), (4, 0), (6, 2), (2, 5), (5, 6), (7, 5)] {
        fill_rect(pixels, width, x_offset + x, y, 1, 2, [80, 160, 72, 240]);
    }
}

fn draw_ice(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [128, 184, 208, 255]);
    for i in 0..8 {
        set_pixel(pixels, width, x_offset + i, i, [216, 240, 248, 255]);
        set_pixel(pixels, width, x_offset + 7 - i, i, [72, 128, 168, 255]);
    }
}

fn create_tank_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let width = manifest.tile_width * manifest.tiles;
    let group_stride = manifest.tile_width * TANK_ANIMATION_FRAMES * 4;
    let mut pixels = vec![0; width * manifest.tile_height * 4];
    let player1_palette = TankPalette {
        dark: [48, 56, 24, 255],
        body: [184, 160, 64, 255],
        light: [240, 216, 104, 255],
        tread: [88, 80, 40, 255],
    };
    let player2_palette = TankPalette {
        dark: [24, 48, 72, 255],
        body: [64, 144, 184, 255],
        light: [128, 216, 240, 255],
        tread: [32, 80, 112, 255],
    };
    let basic_enemy_palette = TankPalette {
        dark: [64, 24, 24, 255],
        body: [176, 56, 40, 255],
        light: [232, 104, 72, 255],
        tread: [88, 40, 32, 255],
    };
    let fast_enemy_palette = TankPalette {
        dark: [24, 72, 32, 255],
        body: [56, 176, 72, 255],
        light: [120, 240, 136, 255],
        tread: [32, 96, 40, 255],
    };
    let power_enemy_palette = TankPalette {
        dark: [72, 24, 56, 255],
        body: [208, 64, 104, 255],
        light: [248, 128, 152, 255],
        tread: [96, 32, 64, 255],
    };
    let armor_enemy_palette = TankPalette {
        dark: [40, 48, 64, 255],
        body: [112, 128, 160, 255],
        light: [184, 200, 224, 255],
        tread: [56, 64, 88, 255],
    };

    draw_tank_group(&mut pixels, width, 0, player1_palette);
    draw_tank_group(&mut pixels, width, group_stride, player2_palette);
    draw_tank_group(&mut pixels, width, group_stride * 2, basic_enemy_palette);
    draw_tank_group(&mut pixels, width, group_stride * 3, fast_enemy_palette);
    draw_tank_group(&mut pixels, width, group_stride * 4, power_enemy_palette);
    draw_tank_group(&mut pixels, width, group_stride * 5, armor_enemy_palette);
    image_from_pixels(width, manifest.tile_height, pixels)
}

#[derive(Clone, Copy)]
struct TankPalette {
    dark: [u8; 4],
    body: [u8; 4],
    light: [u8; 4],
    tread: [u8; 4],
}

fn draw_tank(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    direction: Direction,
    palette: TankPalette,
    frame: usize,
) {
    fill_rect(pixels, width, x_offset + 2, 2, 4, 12, palette.tread);
    fill_rect(pixels, width, x_offset + 10, 2, 4, 12, palette.tread);
    for y in [3 + frame % 2, 7 + frame % 2, 11 + frame % 2] {
        fill_rect(pixels, width, x_offset + 2, y, 4, 1, palette.dark);
        fill_rect(pixels, width, x_offset + 10, y, 4, 1, palette.dark);
    }
    fill_rect(pixels, width, x_offset + 4, 4, 8, 8, palette.body);
    fill_rect(pixels, width, x_offset + 6, 6, 4, 4, palette.light);
    fill_rect(pixels, width, x_offset + 4, 11, 8, 1, palette.dark);

    match direction {
        Direction::Up => fill_rect(pixels, width, x_offset + 7, 0, 2, 7, palette.light),
        Direction::Down => fill_rect(pixels, width, x_offset + 7, 9, 2, 7, palette.light),
        Direction::Left => fill_rect(pixels, width, x_offset, 7, 7, 2, palette.light),
        Direction::Right => fill_rect(pixels, width, x_offset + 9, 7, 7, 2, palette.light),
    }
}

fn draw_tank_group(pixels: &mut [u8], width: usize, x_offset: usize, palette: TankPalette) {
    for frame in 0..2 {
        let frame_offset = x_offset + frame * 64;
        draw_tank(pixels, width, frame_offset, Direction::Up, palette, frame);
        draw_tank(
            pixels,
            width,
            frame_offset + 16,
            Direction::Down,
            palette,
            frame,
        );
        draw_tank(
            pixels,
            width,
            frame_offset + 32,
            Direction::Left,
            palette,
            frame,
        );
        draw_tank(
            pixels,
            width,
            frame_offset + 48,
            Direction::Right,
            palette,
            frame,
        );
    }
}

fn create_bullet_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let atlas_width = manifest.tile_width * manifest.tiles;
    let mut pixels = vec![0; atlas_width * manifest.tile_height * 4];
    for index in 0..manifest.tiles {
        let x_offset = index * manifest.tile_width;
        fill_rect(
            &mut pixels,
            atlas_width,
            x_offset,
            0,
            4,
            4,
            [248, 248, 216, 255],
        );
        set_pixel(&mut pixels, atlas_width, x_offset, 0, [128, 112, 64, 255]);
        set_pixel(
            &mut pixels,
            atlas_width,
            x_offset + 3,
            3,
            [128, 112, 64, 255],
        );
    }
    image_from_pixels(atlas_width, manifest.tile_height, pixels)
}

fn create_effect_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let width = manifest.tile_width * manifest.tiles;
    let mut pixels = vec![0; width * manifest.tile_height * 4];
    for frame in 0..4 {
        draw_explosion_frame(&mut pixels, width, frame * manifest.tile_width, frame);
    }
    for frame in 0..4 {
        draw_spawn_frame(
            &mut pixels,
            width,
            manifest.tile_width * 4 + frame * manifest.tile_width,
            frame,
        );
    }
    for frame in 0..4 {
        draw_base_destruction_frame(
            &mut pixels,
            width,
            manifest.tile_width * 8 + frame * manifest.tile_width,
            frame,
        );
    }
    for frame in 0..4 {
        draw_powerup_sparkle_frame(
            &mut pixels,
            width,
            manifest.tile_width * 12 + frame * manifest.tile_width,
            frame,
        );
    }
    for frame in 0..4 {
        draw_bullet_impact_frame(
            &mut pixels,
            width,
            manifest.tile_width * 16 + frame * manifest.tile_width,
            frame,
        );
    }
    image_from_pixels(width, manifest.tile_height, pixels)
}

fn draw_explosion_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let center = 8_i32;
    let radius = [2, 4, 6, 7][frame];
    for y in 0..16_i32 {
        for x in 0..16_i32 {
            let distance = (x - center).abs() + (y - center).abs();
            if distance <= radius {
                let color = if distance <= radius / 2 {
                    [248, 232, 128, 255]
                } else if frame < 3 {
                    [232, 96, 40, 255]
                } else {
                    [96, 64, 48, 210]
                };
                set_pixel(pixels, width, x_offset + x as usize, y as usize, color);
            }
        }
    }
}

fn draw_spawn_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let color = [112, 200, 248, 230];
    let inset = frame;
    fill_rect(
        pixels,
        width,
        x_offset + inset,
        inset,
        16 - inset * 2,
        1,
        color,
    );
    fill_rect(
        pixels,
        width,
        x_offset + inset,
        15 - inset,
        16 - inset * 2,
        1,
        color,
    );
    fill_rect(
        pixels,
        width,
        x_offset + inset,
        inset,
        1,
        16 - inset * 2,
        color,
    );
    fill_rect(
        pixels,
        width,
        x_offset + 15 - inset,
        inset,
        1,
        16 - inset * 2,
        color,
    );
}

fn draw_base_destruction_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    match frame {
        0 => {
            fill_rect(pixels, width, x_offset + 4, 5, 8, 7, [224, 160, 72, 255]);
            fill_rect(pixels, width, x_offset + 6, 3, 4, 6, [248, 232, 128, 255]);
            fill_rect(pixels, width, x_offset + 3, 12, 10, 2, [80, 56, 40, 255]);
        }
        1 => {
            fill_rect(pixels, width, x_offset + 2, 6, 12, 6, [232, 96, 40, 255]);
            fill_rect(pixels, width, x_offset + 5, 2, 6, 9, [248, 200, 72, 255]);
            fill_rect(pixels, width, x_offset + 1, 12, 14, 3, [72, 56, 48, 255]);
        }
        2 => {
            fill_rect(pixels, width, x_offset + 3, 4, 10, 8, [104, 88, 80, 220]);
            fill_rect(pixels, width, x_offset + 5, 7, 7, 5, [184, 72, 40, 240]);
            fill_rect(pixels, width, x_offset + 2, 12, 12, 3, [48, 40, 32, 255]);
        }
        _ => {
            fill_rect(pixels, width, x_offset + 2, 11, 12, 4, [48, 40, 32, 255]);
            fill_rect(pixels, width, x_offset + 4, 8, 4, 4, [104, 80, 56, 230]);
            fill_rect(pixels, width, x_offset + 9, 7, 3, 5, [88, 72, 64, 210]);
            set_pixel(pixels, width, x_offset + 7, 5, [184, 72, 40, 180]);
            set_pixel(pixels, width, x_offset + 10, 4, [104, 88, 80, 160]);
        }
    }
}

fn draw_powerup_sparkle_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let color = if frame.is_multiple_of(2) {
        [255, 255, 255, 220]
    } else {
        [255, 232, 104, 220]
    };
    let inset = [1, 3, 5, 3][frame];

    for (x, y) in [
        (inset, 1),
        (1, inset),
        (15 - inset, 1),
        (14, inset),
        (inset, 14),
        (1, 15 - inset),
        (15 - inset, 14),
        (14, 15 - inset),
    ] {
        set_pixel(pixels, width, x_offset + x, y, color);
    }

    if frame == 1 || frame == 3 {
        fill_rect(pixels, width, x_offset + 7, 0, 2, 3, color);
        fill_rect(pixels, width, x_offset + 7, 13, 2, 3, color);
    }
}

fn draw_bullet_impact_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    match frame {
        0 => {
            fill_rect(pixels, width, x_offset + 7, 7, 2, 2, [255, 248, 184, 255]);
            set_pixel(pixels, width, x_offset + 8, 6, [255, 255, 255, 240]);
        }
        1 => {
            fill_rect(pixels, width, x_offset + 6, 7, 4, 2, [248, 216, 96, 255]);
            fill_rect(pixels, width, x_offset + 7, 6, 2, 4, [248, 216, 96, 255]);
            set_pixel(pixels, width, x_offset + 5, 5, [255, 255, 255, 220]);
            set_pixel(pixels, width, x_offset + 10, 10, [232, 96, 40, 230]);
        }
        2 => {
            for (x, y) in [(4, 7), (6, 5), (8, 4), (11, 7), (9, 11), (5, 10)] {
                set_pixel(pixels, width, x_offset + x, y, [248, 200, 72, 220]);
            }
            fill_rect(pixels, width, x_offset + 7, 7, 2, 2, [232, 96, 40, 210]);
        }
        _ => {
            for (x, y) in [(5, 6), (9, 5), (11, 9), (7, 11)] {
                set_pixel(pixels, width, x_offset + x, y, [104, 88, 80, 150]);
            }
            set_pixel(pixels, width, x_offset + 8, 8, [72, 56, 48, 130]);
        }
    }
}

fn create_powerup_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let width = manifest.tile_width * manifest.tiles;
    let mut pixels = vec![0; width * manifest.tile_height * 4];
    draw_star_powerup(&mut pixels, width, 0);
    draw_helmet_powerup(&mut pixels, width, manifest.tile_width);
    draw_clock_powerup(&mut pixels, width, manifest.tile_width * 2);
    draw_grenade_powerup(&mut pixels, width, manifest.tile_width * 3);
    draw_shovel_powerup(&mut pixels, width, manifest.tile_width * 4);
    draw_tank_powerup(&mut pixels, width, manifest.tile_width * 5);
    image_from_pixels(width, manifest.tile_height, pixels)
}

fn draw_star_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let gold = [248, 224, 88, 255];
    let shadow = [128, 96, 32, 255];
    for (x, y) in [
        (8, 2),
        (7, 5),
        (8, 5),
        (9, 5),
        (4, 6),
        (5, 6),
        (6, 6),
        (7, 6),
        (8, 6),
        (9, 6),
        (10, 6),
        (11, 6),
        (12, 6),
        (6, 8),
        (7, 8),
        (8, 8),
        (9, 8),
        (10, 8),
        (6, 11),
        (10, 11),
    ] {
        set_pixel(pixels, width, x_offset + x, y + 1, shadow);
        set_pixel(pixels, width, x_offset + x, y, gold);
    }
}

fn draw_helmet_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset + 4, 5, 8, 6, [80, 184, 216, 255]);
    fill_rect(pixels, width, x_offset + 5, 3, 6, 3, [144, 232, 248, 255]);
    fill_rect(pixels, width, x_offset + 3, 10, 10, 2, [40, 96, 144, 255]);
    fill_rect(pixels, width, x_offset + 6, 6, 4, 2, [216, 248, 248, 255]);
}

fn draw_clock_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset + 4, 3, 8, 1, [216, 232, 248, 255]);
    fill_rect(pixels, width, x_offset + 3, 4, 1, 8, [216, 232, 248, 255]);
    fill_rect(pixels, width, x_offset + 12, 4, 1, 8, [72, 120, 184, 255]);
    fill_rect(pixels, width, x_offset + 4, 12, 8, 1, [72, 120, 184, 255]);
    fill_rect(pixels, width, x_offset + 7, 5, 2, 5, [248, 248, 208, 255]);
    fill_rect(pixels, width, x_offset + 8, 9, 4, 2, [248, 248, 208, 255]);
}

fn draw_grenade_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset + 5, 6, 8, 7, [80, 128, 48, 255]);
    fill_rect(pixels, width, x_offset + 6, 4, 5, 3, [120, 176, 64, 255]);
    fill_rect(pixels, width, x_offset + 10, 2, 2, 3, [200, 184, 80, 255]);
    fill_rect(pixels, width, x_offset + 12, 1, 3, 1, [248, 232, 128, 255]);
    fill_rect(pixels, width, x_offset + 4, 11, 9, 2, [40, 72, 32, 255]);
}

fn draw_shovel_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset + 8, 2, 2, 9, [176, 112, 56, 255]);
    fill_rect(pixels, width, x_offset + 6, 10, 6, 2, [176, 112, 56, 255]);
    fill_rect(pixels, width, x_offset + 4, 11, 10, 3, [160, 168, 176, 255]);
    fill_rect(pixels, width, x_offset + 5, 12, 8, 1, [232, 240, 240, 255]);
}

fn draw_tank_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset + 3, 5, 4, 8, [88, 88, 88, 255]);
    fill_rect(pixels, width, x_offset + 9, 5, 4, 8, [88, 88, 88, 255]);
    fill_rect(pixels, width, x_offset + 5, 6, 6, 6, [224, 88, 72, 255]);
    fill_rect(pixels, width, x_offset + 7, 3, 2, 6, [248, 176, 96, 255]);
    fill_rect(pixels, width, x_offset + 6, 8, 4, 3, [248, 176, 96, 255]);
}

fn create_glyph_atlas(manifest: &GlyphManifest) -> Image {
    let glyph_width = manifest.tile_width;
    let glyph_height = manifest.tile_height;
    let glyph_count = manifest.characters.chars().count();
    let width = glyph_width * glyph_count;
    let mut pixels = vec![0; width * glyph_height * 4];

    for (glyph, ch) in manifest.characters.chars().enumerate() {
        let pattern = glyph_pattern(ch);
        for (y, row) in pattern.iter().enumerate() {
            for (x, pixel) in row.chars().enumerate() {
                if pixel == '#' {
                    set_pixel(
                        &mut pixels,
                        width,
                        glyph * glyph_width + x,
                        y,
                        [216, 216, 184, 255],
                    );
                }
            }
        }
    }

    image_from_pixels(width, glyph_height, pixels)
}

fn glyph_index(ch: char, manifest: &GlyphManifest) -> usize {
    manifest
        .characters
        .chars()
        .position(|glyph| glyph == ch)
        .unwrap_or(0)
}

fn glyph_size(manifest: &GlyphManifest) -> Vec2 {
    Vec2::new(manifest.tile_width as f32, manifest.tile_height as f32)
}

fn glyph_pattern_has_pixels(pattern: [&str; GENERATED_GLYPH_HEIGHT]) -> bool {
    pattern.iter().any(|row| row.contains('#'))
}

fn glyph_pattern(ch: char) -> [&'static str; 7] {
    match ch {
        '0' => [
            "#####", "#...#", "#...#", "#...#", "#...#", "#...#", "#####",
        ],
        '1' => [
            "..#..", ".##..", "..#..", "..#..", "..#..", "..#..", ".###.",
        ],
        '2' => [
            "#####", "....#", "....#", "#####", "#....", "#....", "#####",
        ],
        '3' => [
            "#####", "....#", "....#", ".####", "....#", "....#", "#####",
        ],
        '4' => [
            "#...#", "#...#", "#...#", "#####", "....#", "....#", "....#",
        ],
        '5' => [
            "#####", "#....", "#....", "#####", "....#", "....#", "#####",
        ],
        '6' => [
            "#####", "#....", "#....", "#####", "#...#", "#...#", "#####",
        ],
        '7' => [
            "#####", "....#", "...#.", "..#..", ".#...", ".#...", ".#...",
        ],
        '8' => [
            "#####", "#...#", "#...#", "#####", "#...#", "#...#", "#####",
        ],
        '9' => [
            "#####", "#...#", "#...#", "#####", "....#", "....#", "#####",
        ],
        'A' => [
            ".###.", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
        'B' => [
            "####.", "#...#", "#...#", "####.", "#...#", "#...#", "####.",
        ],
        'C' => [
            "#####", "#....", "#....", "#....", "#....", "#....", "#####",
        ],
        'D' => [
            "####.", "#...#", "#...#", "#...#", "#...#", "#...#", "####.",
        ],
        'E' => [
            "#####", "#....", "#....", "####.", "#....", "#....", "#####",
        ],
        'F' => [
            "#####", "#....", "#....", "####.", "#....", "#....", "#....",
        ],
        'G' => [
            "#####", "#....", "#....", "#.###", "#...#", "#...#", "#####",
        ],
        'H' => [
            "#...#", "#...#", "#...#", "#####", "#...#", "#...#", "#...#",
        ],
        'I' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "#####",
        ],
        'J' => [
            "#####", "...#.", "...#.", "...#.", "...#.", "#..#.", ".##..",
        ],
        'K' => [
            "#...#", "#..#.", "#.#..", "##...", "#.#..", "#..#.", "#...#",
        ],
        'L' => [
            "#....", "#....", "#....", "#....", "#....", "#....", "#####",
        ],
        'M' => [
            "#...#", "##.##", "#.#.#", "#...#", "#...#", "#...#", "#...#",
        ],
        'N' => [
            "#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#",
        ],
        'O' => [
            "#####", "#...#", "#...#", "#...#", "#...#", "#...#", "#####",
        ],
        'P' => [
            "####.", "#...#", "#...#", "####.", "#....", "#....", "#....",
        ],
        'Q' => [
            "#####", "#...#", "#...#", "#...#", "#.#.#", "#..#.", "####.",
        ],
        'R' => [
            "####.", "#...#", "#...#", "####.", "#.#..", "#..#.", "#...#",
        ],
        'S' => [
            "#####", "#....", "#....", "#####", "....#", "....#", "#####",
        ],
        'T' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "..#..",
        ],
        'U' => [
            "#...#", "#...#", "#...#", "#...#", "#...#", "#...#", "#####",
        ],
        'V' => [
            "#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#..",
        ],
        'W' => [
            "#...#", "#...#", "#...#", "#...#", "#.#.#", "##.##", "#...#",
        ],
        'X' => [
            "#...#", "#...#", ".#.#.", "..#..", ".#.#.", "#...#", "#...#",
        ],
        'Y' => [
            "#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#..",
        ],
        'Z' => [
            "#####", "....#", "...#.", "..#..", ".#...", "#....", "#####",
        ],
        _ => [
            ".....", ".....", ".....", ".....", ".....", ".....", ".....",
        ],
    }
}

fn generated_sprite_size(manifest: GeneratedSpriteManifest) -> Vec2 {
    Vec2::new(manifest.width as f32, manifest.height as f32)
}

fn create_base_image(manifest: GeneratedSpriteManifest, destroyed: bool) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    if destroyed {
        fill_rect(&mut pixels, manifest.width, 3, 9, 10, 4, [96, 72, 48, 255]);
        fill_rect(&mut pixels, manifest.width, 5, 5, 3, 4, [160, 48, 24, 255]);
        fill_rect(&mut pixels, manifest.width, 9, 4, 2, 6, [184, 88, 32, 255]);
        fill_rect(&mut pixels, manifest.width, 2, 12, 12, 2, [48, 40, 32, 255]);
    } else {
        fill_rect(&mut pixels, manifest.width, 4, 9, 8, 4, [160, 120, 72, 255]);
        fill_rect(&mut pixels, manifest.width, 5, 6, 6, 4, [192, 152, 88, 255]);
        fill_rect(
            &mut pixels,
            manifest.width,
            7,
            3,
            2,
            4,
            [224, 192, 112, 255],
        );
        fill_rect(&mut pixels, manifest.width, 3, 13, 10, 1, [72, 56, 32, 255]);
    }
    image_from_pixels(manifest.width, manifest.height, pixels)
}

fn create_score_badge_icon(manifest: GeneratedSpriteManifest) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    fill_rect(
        &mut pixels,
        manifest.width,
        2,
        1,
        4,
        1,
        [248, 232, 128, 255],
    );
    fill_rect(&mut pixels, manifest.width, 1, 2, 6, 4, [216, 160, 56, 255]);
    fill_rect(&mut pixels, manifest.width, 2, 6, 4, 1, [136, 88, 40, 255]);
    fill_rect(
        &mut pixels,
        manifest.width,
        3,
        3,
        2,
        2,
        [255, 248, 184, 255],
    );
    set_pixel(&mut pixels, manifest.width, 1, 2, [248, 216, 96, 255]);
    set_pixel(&mut pixels, manifest.width, 6, 2, [248, 216, 96, 255]);
    set_pixel(&mut pixels, manifest.width, 1, 5, [136, 88, 40, 255]);
    set_pixel(&mut pixels, manifest.width, 6, 5, [136, 88, 40, 255]);
    image_from_pixels(manifest.width, manifest.height, pixels)
}

fn create_stage_flag_icon(manifest: GeneratedSpriteManifest) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    fill_rect(
        &mut pixels,
        manifest.width,
        1,
        1,
        1,
        6,
        [232, 232, 208, 255],
    );
    fill_rect(&mut pixels, manifest.width, 2, 1, 5, 3, [248, 216, 72, 255]);
    fill_rect(&mut pixels, manifest.width, 2, 4, 3, 1, [176, 112, 40, 255]);
    fill_rect(&mut pixels, manifest.width, 0, 7, 4, 1, [120, 120, 96, 255]);
    set_pixel(&mut pixels, manifest.width, 6, 3, [248, 168, 56, 255]);
    image_from_pixels(manifest.width, manifest.height, pixels)
}

fn image_from_pixels(width: usize, height: usize, pixels: Vec<u8>) -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.data = Some(pixels);
    image
}

fn fill_rect(
    pixels: &mut [u8],
    width: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: [u8; 4],
) {
    for yy in y..(y + h) {
        for xx in x..(x + w) {
            set_pixel(pixels, width, xx, yy, color);
        }
    }
}

fn set_pixel(pixels: &mut [u8], width: usize, x: usize, y: usize, color: [u8; 4]) {
    let index = (y * width + x) * 4;
    pixels[index..index + 4].copy_from_slice(&color);
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = include_str!("../assets/manifest.ron");
    const LEVEL_1: &str = include_str!("../assets/levels/001.level.ron");
    const LEVEL_2: &str = include_str!("../assets/levels/002.level.ron");
    const LEVEL_3: &str = include_str!("../assets/levels/003.level.ron");
    const LEVEL_4: &str = include_str!("../assets/levels/004.level.ron");
    const LEVEL_5: &str = include_str!("../assets/levels/005.level.ron");
    const LEVEL_6: &str = include_str!("../assets/levels/006.level.ron");
    const LEVEL_7: &str = include_str!("../assets/levels/007.level.ron");
    const LEVEL_8: &str = include_str!("../assets/levels/008.level.ron");
    const LEVEL_9: &str = include_str!("../assets/levels/009.level.ron");
    const LEVEL_10: &str = include_str!("../assets/levels/010.level.ron");
    const LEVEL_11: &str = include_str!("../assets/levels/011.level.ron");
    const LEVEL_12: &str = include_str!("../assets/levels/012.level.ron");
    const LEVEL_13: &str = include_str!("../assets/levels/013.level.ron");
    const LEVEL_14: &str = include_str!("../assets/levels/014.level.ron");
    const LEVEL_15: &str = include_str!("../assets/levels/015.level.ron");
    const LEVEL_16: &str = include_str!("../assets/levels/016.level.ron");
    const LEVEL_17: &str = include_str!("../assets/levels/017.level.ron");
    const LEVEL_18: &str = include_str!("../assets/levels/018.level.ron");
    const LEVEL_19: &str = include_str!("../assets/levels/019.level.ron");
    const LEVEL_20: &str = include_str!("../assets/levels/020.level.ron");
    const LEVEL_21: &str = include_str!("../assets/levels/021.level.ron");
    const LEVEL_22: &str = include_str!("../assets/levels/022.level.ron");
    const LEVEL_23: &str = include_str!("../assets/levels/023.level.ron");
    const LEVEL_24: &str = include_str!("../assets/levels/024.level.ron");
    const LEVEL_25: &str = include_str!("../assets/levels/025.level.ron");
    const LEVEL_26: &str = include_str!("../assets/levels/026.level.ron");
    const LEVEL_27: &str = include_str!("../assets/levels/027.level.ron");
    const LEVEL_28: &str = include_str!("../assets/levels/028.level.ron");
    const LEVEL_29: &str = include_str!("../assets/levels/029.level.ron");
    const LEVEL_30: &str = include_str!("../assets/levels/030.level.ron");
    const LEVEL_31: &str = include_str!("../assets/levels/031.level.ron");
    const LEVEL_32: &str = include_str!("../assets/levels/032.level.ron");
    const LEVEL_33: &str = include_str!("../assets/levels/033.level.ron");
    const LEVEL_34: &str = include_str!("../assets/levels/034.level.ron");
    const LEVEL_35: &str = include_str!("../assets/levels/035.level.ron");
    const LEVEL_36: &str = include_str!("../assets/levels/036.level.ron");
    const LEVEL_37: &str = include_str!("../assets/levels/037.level.ron");
    const LEVEL_38: &str = include_str!("../assets/levels/038.level.ron");
    const LEVEL_39: &str = include_str!("../assets/levels/039.level.ron");
    const LEVEL_40: &str = include_str!("../assets/levels/040.level.ron");
    const LEVEL_41: &str = include_str!("../assets/levels/041.level.ron");
    const LEVEL_42: &str = include_str!("../assets/levels/042.level.ron");
    const LEVEL_43: &str = include_str!("../assets/levels/043.level.ron");
    const LEVEL_44: &str = include_str!("../assets/levels/044.level.ron");
    const LEVEL_45: &str = include_str!("../assets/levels/045.level.ron");
    const ARENA_1: &str = include_str!("../assets/arenas/arena_01.ron");
    const ARENA_2: &str = include_str!("../assets/arenas/arena_02.ron");
    const ARENA_3: &str = include_str!("../assets/arenas/arena_03.ron");
    const ARENA_4: &str = include_str!("../assets/arenas/arena_04.ron");
    const ARENA_5: &str = include_str!("../assets/arenas/arena_05.ron");
    const ARENA_6: &str = include_str!("../assets/arenas/arena_06.ron");
    const GITIGNORE: &str = include_str!("../.gitignore");
    const TEST_SPAWN_INVULNERABILITY_SECONDS: f32 = 3.25;

    fn authored_levels() -> [(usize, &'static str); LEVEL_COUNT] {
        [
            (1, LEVEL_1),
            (2, LEVEL_2),
            (3, LEVEL_3),
            (4, LEVEL_4),
            (5, LEVEL_5),
            (6, LEVEL_6),
            (7, LEVEL_7),
            (8, LEVEL_8),
            (9, LEVEL_9),
            (10, LEVEL_10),
            (11, LEVEL_11),
            (12, LEVEL_12),
            (13, LEVEL_13),
            (14, LEVEL_14),
            (15, LEVEL_15),
            (16, LEVEL_16),
            (17, LEVEL_17),
            (18, LEVEL_18),
            (19, LEVEL_19),
            (20, LEVEL_20),
            (21, LEVEL_21),
            (22, LEVEL_22),
            (23, LEVEL_23),
            (24, LEVEL_24),
            (25, LEVEL_25),
            (26, LEVEL_26),
            (27, LEVEL_27),
            (28, LEVEL_28),
            (29, LEVEL_29),
            (30, LEVEL_30),
            (31, LEVEL_31),
            (32, LEVEL_32),
            (33, LEVEL_33),
            (34, LEVEL_34),
            (35, LEVEL_35),
            (36, LEVEL_36),
            (37, LEVEL_37),
            (38, LEVEL_38),
            (39, LEVEL_39),
            (40, LEVEL_40),
            (41, LEVEL_41),
            (42, LEVEL_42),
            (43, LEVEL_43),
            (44, LEVEL_44),
            (45, LEVEL_45),
        ]
    }

    fn authored_arenas() -> [(usize, &'static str); ARENA_COUNT] {
        [
            (1, ARENA_1),
            (2, ARENA_2),
            (3, ARENA_3),
            (4, ARENA_4),
            (5, ARENA_5),
            (6, ARENA_6),
        ]
    }

    fn spawn_signature(spawn: &SpawnPoint) -> (usize, usize, Direction) {
        (spawn.x, spawn.y, spawn.facing)
    }

    fn base_battle_arena_text() -> String {
        let mut rows = vec![".........................."; BOARD_TILES];
        rows[0] = "EE........................";
        rows[1] = "EE........................";
        rows[24] = "........................EE";
        rows[25] = "........................EE";
        let map_rows = rows
            .iter()
            .map(|row| format!("    \"{row}\","))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"(
  name: "Base Arena",
  map: [
{map_rows}
  ],
  p1_spawn: (x: 4, y: 24, facing: Up),
  p2_spawn: (x: 20, y: 0, facing: Down),
  battle_rules: BaseBattle(
    p1_base: (x: 24, y: 24),
    p2_base: (x: 0, y: 0),
    lives: 3,
    respawn_invulnerability_secs: 2.0,
  ),
  powerup_spawns: [
    (x: 12, y: 12),
  ],
)"#
        )
    }

    fn test_sprite_assets() -> SpriteAssets {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let image = Handle::<Image>::default();
        let layout = Handle::<TextureAtlasLayout>::default();

        SpriteAssets {
            manifest,
            terrain_image: image.clone(),
            terrain_layout: layout.clone(),
            tank_image: image.clone(),
            tank_layout: layout.clone(),
            bullet_image: image.clone(),
            bullet_layout: layout.clone(),
            effect_image: image.clone(),
            effect_layout: layout.clone(),
            powerup_image: image.clone(),
            powerup_layout: layout.clone(),
            glyph_image: image.clone(),
            glyph_layout: layout,
            base_intact: image.clone(),
            base_destroyed: image.clone(),
            score_badge_icon: image.clone(),
            stage_flag_icon: image,
        }
    }

    fn test_sound_assets() -> SoundAssets {
        let sound = SoundHandle::Retro(Handle::<RetroSound>::default());

        SoundAssets {
            sound_enabled: true,
            fire: sound.clone(),
            brick_hit: sound.clone(),
            steel_hit: sound.clone(),
            tank_explosion: sound.clone(),
            base_destroyed: sound.clone(),
            powerup_pickup: sound.clone(),
            stage_start: sound.clone(),
            level_clear: sound.clone(),
            game_over: sound.clone(),
            background_music: sound,
        }
    }

    fn test_bullet(previous_top_left: Vec2, top_left: Vec2, resolved: bool) -> Bullet {
        Bullet {
            previous_top_left,
            top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved,
        }
    }

    fn spawn_test_player(world: &mut World, id: PlayerId, top_left: Vec2, lives: i32) {
        world.spawn((
            Tank {
                top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            Player { id },
            PlayerUpgrade { level: 0 },
            PlayerLives { current: lives },
            Health { current: 1 },
            Transform::from_translation(board_object_center(
                top_left.x,
                top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
            Sprite::default(),
        ));
    }

    fn spawn_test_powerup(world: &mut World, kind: PowerUpKind, top_left: Vec2) {
        world.spawn((
            PowerUp { kind },
            Transform::from_translation(board_object_center(
                top_left.x,
                top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
        ));
    }

    fn powerup_pickup_app(game_mode: GameMode, score_board: ScoreBoard) -> App {
        let mut app = App::new();
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(game_mode);
        app.insert_resource(TileGrid::empty());
        app.insert_resource(EnemyFreeze::default());
        app.insert_resource(VersusPlayerFreeze::default());
        app.insert_resource(BaseReinforcement::default());
        app.insert_resource(score_board);
        app.add_systems(Update, pickup_powerups);
        app
    }

    fn bullet_paths_clash_impact(
        a_start: Vec2,
        a_end: Vec2,
        b_start: Vec2,
        b_end: Vec2,
    ) -> Option<Vec2> {
        bullet_paths_clash(a_start, a_end, b_start, b_end).map(|clash| clash.impact_top_left)
    }

    fn spawn_player_with_initial_shield_for_test(
        mut commands: Commands,
        assets: Res<SpriteAssets>,
    ) {
        spawn_player_tank(
            &mut commands,
            &assets,
            &SpawnPoint {
                x: 8,
                y: 24,
                facing: Direction::Up,
            },
            PlayerId::One,
            3,
            TEST_SPAWN_INVULNERABILITY_SECONDS,
        );
    }

    fn refresh_same_kind_steel_tile_for_test(
        mut commands: Commands,
        assets: Res<SpriteAssets>,
        mut tile_grid: ResMut<TileGrid>,
        tile_sprites: Query<(Entity, &GridTile)>,
    ) {
        sync_tile_sprite(
            &mut commands,
            &assets,
            &mut tile_grid,
            &tile_sprites,
            10,
            24,
            TileKind::Steel,
        );
    }

    fn enter_victory_screen_for_test(
        mut commands: Commands,
        mut game_status: ResMut<GameStatus>,
        mut tile_grid: ResMut<TileGrid>,
        mut director: ResMut<EnemyDirector>,
        mut stage_rules: ResMut<StageRules>,
        mut enemy_freeze: ResMut<EnemyFreeze>,
        mut versus_freeze: ResMut<VersusPlayerFreeze>,
        mut base_reinforcement: ResMut<BaseReinforcement>,
        game_entities: Query<Entity, With<GameEntity>>,
    ) {
        enter_victory_screen(
            &mut commands,
            &mut game_status,
            &mut tile_grid,
            &mut director,
            &mut stage_rules,
            &mut enemy_freeze,
            &mut versus_freeze,
            &mut base_reinforcement,
            &game_entities,
        );
    }

    fn switch_base_reinforcement_for_test(
        mut commands: Commands,
        assets: Res<SpriteAssets>,
        mut tile_grid: ResMut<TileGrid>,
        tile_sprites: Query<(Entity, &GridTile)>,
        mut base_reinforcement: ResMut<BaseReinforcement>,
    ) {
        reinforce_base_walls(
            &mut commands,
            &assets,
            &mut tile_grid,
            &tile_sprites,
            &mut base_reinforcement,
            vec![(22, 0)],
        );
    }

    fn grenade_visible_enemies_for_test(
        mut commands: Commands,
        assets: Res<SpriteAssets>,
        sounds: Res<SoundAssets>,
        mut score_board: ResMut<ScoreBoard>,
        active_powerups: Query<Entity, With<PowerUp>>,
        active_sparkles: Query<Entity, With<PowerUpSparkle>>,
        mut enemy_tanks: Query<(Entity, &Tank, &mut Transform, &EnemyTank)>,
    ) {
        destroy_visible_enemies(
            &mut commands,
            &assets,
            &sounds,
            &mut score_board,
            active_powerups.iter(),
            &active_sparkles,
            &mut enemy_tanks,
        );
    }

    #[test]
    fn window_scale_defaults_and_accepts_integer_scales() {
        assert_eq!(parse_window_scale(None), DEFAULT_WINDOW_SCALE);
        assert_eq!(parse_window_scale(Some("2")), 2);
        assert_eq!(parse_window_scale(Some("3")), 3);
        assert_eq!(parse_window_scale(Some("4")), 4);
        assert_eq!(parse_window_scale(Some(" 4 ")), 4);
        assert_eq!(parse_window_scale(Some("2x")), 2);
        assert_eq!(parse_window_scale(Some("3X")), 3);
        assert_eq!(parse_window_scale(Some(" 4x ")), 4);
    }

    #[test]
    fn window_scale_rejects_non_crisp_or_out_of_range_values() {
        for value in ["", "1", "5", "1x", "5x", "3.5", "abc", "3xx", "4 x"] {
            assert_eq!(parse_window_scale(Some(value)), DEFAULT_WINDOW_SCALE);
        }
    }

    #[test]
    fn music_menu_mode_defaults_to_bgm_and_toggles_classic() {
        assert_eq!(ModeSelect::default().audio_mode, AudioMode::Bgm);
        assert!(ModeSelect::default().sound_enabled);
        assert_eq!(next_audio_mode(AudioMode::Bgm), AudioMode::Classic);
        assert_eq!(next_audio_mode(AudioMode::Classic), AudioMode::Bgm);
        assert_eq!(audio_mode_label(AudioMode::Bgm), "BGM");
        assert_eq!(audio_mode_label(AudioMode::Classic), "CLASSIC");
        assert!(!toggle_sound_enabled(true));
        assert!(toggle_sound_enabled(false));
        assert_eq!(sound_enabled_label(true), "ON");
        assert_eq!(sound_enabled_label(false), "OFF");
    }

    #[test]
    fn background_music_only_plays_during_active_rounds() {
        assert!(background_music_should_play(
            AudioMode::Bgm,
            GamePhase::StageIntro
        ));
        assert!(background_music_should_play(
            AudioMode::Bgm,
            GamePhase::Playing
        ));
        assert!(!background_music_should_play(
            AudioMode::Bgm,
            GamePhase::ModeSelect
        ));
        assert!(!background_music_should_play(
            AudioMode::Bgm,
            GamePhase::Paused
        ));
        assert!(!background_music_should_play(
            AudioMode::Bgm,
            GamePhase::LevelClear
        ));
        assert!(!background_music_should_play(
            AudioMode::Bgm,
            GamePhase::GameOver
        ));
        assert!(!background_music_should_play(
            AudioMode::Classic,
            GamePhase::Playing
        ));
    }

    #[test]
    fn background_music_sync_spawns_and_stops_with_game_phase() {
        let mut app = App::new();
        app.insert_resource(ModeSelect::default());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.add_systems(Update, sync_background_music);

        app.update();
        assert_eq!(background_music_entity_count(&mut app), 1);

        app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Classic;
        app.update();
        assert_eq!(background_music_entity_count(&mut app), 0);

        app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Bgm;
        app.update();
        assert_eq!(background_music_entity_count(&mut app), 1);

        app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Paused;
        app.update();
        assert_eq!(background_music_entity_count(&mut app), 0);
    }

    fn background_music_entity_count(app: &mut App) -> usize {
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<BackgroundMusic>>();
        query.iter(app.world()).count()
    }

    #[test]
    fn sound_effect_setting_controls_one_shot_audio() {
        let mut enabled_app = App::new();
        enabled_app.insert_resource(test_sound_assets());
        enabled_app.add_systems(Update, spawn_fire_sound_for_test);
        enabled_app.update();
        assert_eq!(retro_audio_player_count(&mut enabled_app), 1);

        let mut muted_sounds = test_sound_assets();
        muted_sounds.sound_enabled = false;
        let mut muted_app = App::new();
        muted_app.insert_resource(muted_sounds);
        muted_app.add_systems(Update, spawn_fire_sound_for_test);
        muted_app.update();
        assert_eq!(retro_audio_player_count(&mut muted_app), 0);
    }

    fn spawn_fire_sound_for_test(mut commands: Commands, sounds: Res<SoundAssets>) {
        play_sound(&mut commands, &sounds, SoundKind::Fire);
    }

    fn retro_audio_player_count(app: &mut App) -> usize {
        let mut query = app.world_mut().query::<&AudioPlayer<RetroSound>>();
        query.iter(app.world()).count()
    }

    #[test]
    fn virtual_window_size_uses_integer_scale() {
        assert_eq!(virtual_window_size(2.0), (512, 480));
        assert_eq!(virtual_window_size(3.0), (768, 720));
        assert_eq!(virtual_window_size(4.0), (1024, 960));
    }

    #[test]
    fn personal_sprite_override_paths_are_asset_root_relative_pngs() {
        assert_eq!(PERSONAL_SPRITE_OVERRIDE_PATHS.len(), 10);
        for asset_path in PERSONAL_SPRITE_OVERRIDE_PATHS {
            assert!(asset_path.starts_with("personal/"));
            assert!(asset_path.ends_with(".png"));
            assert!(!asset_path.starts_with('/'));
            assert!(!asset_path.contains(".."));
            assert_eq!(
                personal_asset_disk_path(asset_path),
                Path::new(ASSET_ROOT_DIR).join(asset_path)
            );
        }
    }

    #[test]
    fn personal_sound_override_paths_are_asset_root_relative_ogg_files() {
        assert_eq!(PERSONAL_SOUND_OVERRIDE_PATHS.len(), 10);
        assert!(PERSONAL_SOUND_OVERRIDE_PATHS.contains(&PERSONAL_BACKGROUND_MUSIC_SOUND_PATH));
        for asset_path in PERSONAL_SOUND_OVERRIDE_PATHS {
            assert!(asset_path.starts_with("personal/sounds/"));
            assert!(asset_path.ends_with(".ogg"));
            assert!(!asset_path.starts_with('/'));
            assert!(!asset_path.contains(".."));
            assert_eq!(
                personal_asset_disk_path(asset_path),
                Path::new(ASSET_ROOT_DIR).join(asset_path)
            );
        }
    }

    #[test]
    fn personal_manifest_override_path_is_gitignored_and_runtime_selectable() {
        assert_eq!(PERSONAL_ASSET_MANIFEST_PATH, "assets/personal/manifest.ron");
        assert!(
            GITIGNORE
                .lines()
                .any(|line| line.trim() == "assets/personal/")
        );

        let selected =
            preferred_existing_path(PERSONAL_ASSET_MANIFEST_PATH, ASSET_MANIFEST_PATH, |path| {
                path == PERSONAL_ASSET_MANIFEST_PATH
            });
        assert_eq!(selected, PERSONAL_ASSET_MANIFEST_PATH);
    }

    #[test]
    fn manifest_path_selection_falls_back_to_committed_manifest() {
        let selected =
            preferred_existing_path(PERSONAL_ASSET_MANIFEST_PATH, ASSET_MANIFEST_PATH, |_| false);

        assert_eq!(selected, ASSET_MANIFEST_PATH);
    }

    #[test]
    fn personal_sprite_override_directory_is_gitignored() {
        assert!(
            GITIGNORE
                .lines()
                .any(|line| line.trim() == "assets/personal/")
        );
    }

    fn assert_manifest_glyph_is_visible(manifest: &AssetManifest, ch: char) {
        assert!(
            manifest.glyphs.characters.contains(ch),
            "manifest should include glyph {ch}"
        );
        assert!(
            glyph_pattern_has_pixels(glyph_pattern(ch)),
            "glyph {ch} should render"
        );
    }

    #[test]
    fn stage_paths_use_three_digit_level_numbers() {
        assert_eq!(stage_path(1), "assets/levels/001.level.ron");
        assert_eq!(stage_path(12), "assets/levels/012.level.ron");
        assert_eq!(
            personal_stage_path(1),
            "assets/personal/levels/001.level.ron"
        );
        assert_eq!(
            personal_stage_path(12),
            "assets/personal/levels/012.level.ron"
        );
    }

    #[test]
    fn arena_paths_use_two_digit_arena_numbers() {
        assert_eq!(arena_path(1), "assets/arenas/arena_01.ron");
        assert_eq!(arena_path(12), "assets/arenas/arena_12.ron");
        assert_eq!(
            personal_arena_path(1),
            "assets/personal/arenas/arena_01.ron"
        );
        assert_eq!(
            personal_arena_path(12),
            "assets/personal/arenas/arena_12.ron"
        );
    }

    #[test]
    fn runtime_paths_prefer_personal_files_when_present() {
        let selected = preferred_existing_path(
            "assets/personal/levels/001.level.ron",
            "assets/levels/001.level.ron",
            |path| path.starts_with("assets/personal/"),
        );

        assert_eq!(selected, "assets/personal/levels/001.level.ron");
    }

    #[test]
    fn runtime_paths_fallback_to_authored_files_without_personal_override() {
        let selected = preferred_existing_path(
            "assets/personal/arenas/arena_01.ron",
            "assets/arenas/arena_01.ron",
            |_| false,
        );

        assert_eq!(selected, "assets/arenas/arena_01.ron");
    }

    #[test]
    fn load_level_errors_include_file_path_for_authoring_failures() {
        let path = unique_temp_asset_path("bad-level.ron");
        let path_text = path.to_string_lossy().into_owned();
        let invalid = LEVEL_1.replacen("spawn_interval_secs: 3.0", "spawn_interval_secs: -1.0", 1);
        fs::write(&path, invalid).expect("temp level should be written");

        let err = match load_level(&path_text) {
            Ok(_) => panic!("invalid level should fail"),
            Err(err) => err,
        };
        fs::remove_file(&path).ok();

        assert!(err.contains(&path_text));
        assert!(err.contains("spawn_interval_secs must be positive"));
    }

    #[test]
    fn load_arena_errors_include_file_path_for_authoring_failures() {
        let path = unique_temp_asset_path("bad-arena.ron");
        let path_text = path.to_string_lossy().into_owned();
        let invalid = ARENA_1.replacen("target_score: 5", "target_score: 0", 1);
        fs::write(&path, invalid).expect("temp arena should be written");

        let err = match load_arena(&path_text) {
            Ok(_) => panic!("invalid arena should fail"),
            Err(err) => err,
        };
        fs::remove_file(&path).ok();

        assert!(err.contains(&path_text));
        assert!(err.contains("deathmatch target_score must be greater than zero"));
    }

    #[test]
    fn stage_bundle_loads_level_and_authoritative_grid_together() {
        let (level, grid) = load_stage_bundle(1).expect("stage bundle should load");

        assert_eq!(level.name, "Stage 1");
        assert_eq!(
            grid.get(level.base_position.x as i32, level.base_position.y as i32),
            Some(TileKind::Base)
        );
    }

    #[test]
    fn arena_bundle_loads_arena_and_authoritative_grid_together() {
        let (arena, grid) = load_arena_bundle(5).expect("arena bundle should load");

        assert_eq!(arena.name, "Arena 5");
        let BattleRules::BaseBattle {
            p1_base, p2_base, ..
        } = arena.battle_rules
        else {
            panic!("arena five should be base battle");
        };
        assert_eq!(
            grid.get(p1_base.x as i32, p1_base.y as i32),
            Some(TileKind::Base)
        );
        assert_eq!(
            grid.get(p2_base.x as i32, p2_base.y as i32),
            Some(TileKind::Base)
        );
    }

    #[test]
    fn runtime_stage_load_error_names_stage_path_and_reason() {
        let err = campaign_stage_load_error(
            7,
            "assets/levels/007.level.ron",
            "spawn_interval_secs must be positive",
        );

        assert!(err.contains("campaign stage 7"));
        assert!(err.contains("assets/levels/007.level.ron"));
        assert!(err.contains("spawn_interval_secs must be positive"));
    }

    #[test]
    fn runtime_arena_load_error_names_arena_path_and_reason() {
        let err = versus_arena_load_error(
            5,
            "assets/arenas/arena_05.ron",
            "base battle lives must be greater than zero",
        );

        assert!(err.contains("versus arena 5"));
        assert!(err.contains("assets/arenas/arena_05.ron"));
        assert!(err.contains("base battle lives must be greater than zero"));
    }

    #[test]
    fn authored_asset_manifest_matches_generated_atlases() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        assert_eq!(
            manifest.atlases.tanks,
            GeneratedAtlasManifest {
                tile_width: TANK_ATLAS_TILE_SIZE,
                tile_height: TANK_ATLAS_TILE_SIZE,
                tiles: TANK_ATLAS_TILES
            }
        );
        assert_eq!(
            manifest.atlases.terrain,
            GeneratedAtlasManifest {
                tile_width: TERRAIN_ATLAS_TILE_SIZE,
                tile_height: TERRAIN_ATLAS_TILE_SIZE,
                tiles: TERRAIN_ATLAS_TILES
            }
        );
        assert_eq!(
            manifest.atlases.bullets,
            GeneratedAtlasManifest {
                tile_width: BULLET_ATLAS_TILE_SIZE,
                tile_height: BULLET_ATLAS_TILE_SIZE,
                tiles: BULLET_ATLAS_TILES
            }
        );
        assert_eq!(
            manifest.atlases.effects,
            GeneratedAtlasManifest {
                tile_width: EFFECT_ATLAS_TILE_SIZE,
                tile_height: EFFECT_ATLAS_TILE_SIZE,
                tiles: EFFECT_ATLAS_TILES
            }
        );
        assert_eq!(
            manifest.atlases.powerups,
            GeneratedAtlasManifest {
                tile_width: POWERUP_ATLAS_TILE_SIZE,
                tile_height: POWERUP_ATLAS_TILE_SIZE,
                tiles: POWERUP_ATLAS_TILES
            }
        );

        assert_eq!(
            manifest.tank_index(TankSpriteSet::Player1, Direction::Up, 0),
            0
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::Player1, Direction::Right, 1),
            7
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::Player2, Direction::Left, 0),
            10
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::EnemyBasic, Direction::Down, 1),
            21
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::EnemyFast, Direction::Down, 1),
            29
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::EnemyPower, Direction::Down, 1),
            37
        );
        assert_eq!(
            manifest.tank_index(TankSpriteSet::EnemyArmor, Direction::Down, 1),
            45
        );

        assert_eq!(manifest.bullet_index(Direction::Up), 0);
        assert_eq!(manifest.bullet_index(Direction::Down), 1);
        assert_eq!(manifest.bullet_index(Direction::Left), 2);
        assert_eq!(manifest.bullet_index(Direction::Right), 3);

        assert_eq!(manifest.terrain_index(TileKind::Brick), Some(0));
        assert_eq!(manifest.terrain_index(TileKind::Steel), Some(1));
        assert_eq!(manifest.terrain_index(TileKind::Water), Some(2));
        assert_eq!(manifest.terrain_index(TileKind::Forest), Some(4));
        assert_eq!(manifest.terrain_index(TileKind::Ice), Some(5));
        assert_eq!(manifest.terrain_index(TileKind::Empty), None);
        assert_eq!(manifest.terrain_index(TileKind::Base), None);
        assert_eq!(
            manifest.terrain_animation_frames(TileKind::Water),
            Some(SpriteFrameRange { first: 2, last: 3 })
        );
        assert_eq!(manifest.terrain_animation_frames(TileKind::Brick), None);

        assert_eq!(
            manifest.explosion_frames(),
            SpriteFrameRange { first: 0, last: 3 }
        );
        assert_eq!(
            manifest.spawn_shimmer_frames(),
            SpriteFrameRange { first: 4, last: 7 }
        );
        assert_eq!(
            manifest.base_destruction_frames(),
            SpriteFrameRange { first: 8, last: 11 }
        );
        assert_eq!(
            manifest.powerup_sparkle_frames(),
            SpriteFrameRange {
                first: 12,
                last: 15
            }
        );
        assert_eq!(
            manifest.bullet_impact_frames(),
            SpriteFrameRange {
                first: 16,
                last: 19
            }
        );

        assert_eq!(manifest.powerup_index(PowerUpKind::Star), 0);
        assert_eq!(manifest.powerup_index(PowerUpKind::Helmet), 1);
        assert_eq!(manifest.powerup_index(PowerUpKind::Clock), 2);
        assert_eq!(manifest.powerup_index(PowerUpKind::Grenade), 3);
        assert_eq!(manifest.powerup_index(PowerUpKind::Shovel), 4);
        assert_eq!(manifest.powerup_index(PowerUpKind::Tank), 5);

        assert_eq!(
            manifest.base.intact,
            GeneratedSpriteManifest {
                width: GENERATED_BASE_SIZE,
                height: GENERATED_BASE_SIZE
            }
        );
        assert_eq!(
            manifest.base.destroyed,
            GeneratedSpriteManifest {
                width: GENERATED_BASE_SIZE,
                height: GENERATED_BASE_SIZE
            }
        );
        assert_eq!(
            manifest.ui.score_badge,
            GeneratedSpriteManifest {
                width: GENERATED_UI_ICON_SIZE,
                height: GENERATED_UI_ICON_SIZE
            }
        );
        assert_eq!(
            manifest.ui.stage_flag,
            GeneratedSpriteManifest {
                width: GENERATED_UI_ICON_SIZE,
                height: GENERATED_UI_ICON_SIZE
            }
        );

        assert_eq!(manifest.glyphs.characters, REQUIRED_GLYPHS);
        assert_eq!(manifest.glyphs.tile_width, GENERATED_GLYPH_WIDTH);
        assert_eq!(manifest.glyphs.tile_height, GENERATED_GLYPH_HEIGHT);
        assert_eq!(glyph_index('0', &manifest.glyphs), 0);
        assert_eq!(glyph_index('A', &manifest.glyphs), 10);
        assert_eq!(glyph_index('Z', &manifest.glyphs), 35);

        assert!(matches!(
            manifest.sounds.fire,
            RetroSoundSpec::Sweep {
                duration_secs: 0.08,
                start_frequency: 920.0,
                end_frequency: 420.0,
                volume: 0.22,
            }
        ));
        assert!(matches!(
            manifest.sounds.brick_hit,
            RetroSoundSpec::Noise {
                duration_secs: 0.07,
                volume: 0.18,
                seed: 305419896,
            }
        ));
        assert!(matches!(
            manifest.sounds.base_destroyed,
            RetroSoundSpec::Layered { ref notes } if notes.len() == 3
        ));
        assert_eq!(sound_manifest_specs(&manifest.sounds).len(), 9);
    }

    #[test]
    fn asset_manifest_rejects_out_of_range_indices() {
        let invalid = MANIFEST.replacen("right: 47", "right: 48", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid tank index should fail")
                .contains("outside the generated tank atlas")
        );

        let invalid = MANIFEST.replacen(
            "    player2: [
      (up: 8, down: 9, left: 10, right: 11),
      (up: 12, down: 13, left: 14, right: 15),
    ],",
            "    player2: [
      (up: 8, down: 9, left: 10, right: 11),
    ],",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("missing tank animation frame should fail")
                .contains("must define 2 animation frames")
        );

        let invalid = MANIFEST.replacen(
            "bullets: (up: 0, down: 1, left: 2, right: 3)",
            "bullets: (up: 0, down: 1, left: 2, right: 4)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid bullet index should fail")
                .contains("bullets.right index 4 is outside the generated bullet atlas")
        );

        let invalid = MANIFEST.replacen("ice: 5", "ice: 6", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid terrain index should fail")
                .contains("outside the generated terrain atlas")
        );

        let invalid = MANIFEST.replacen(
            "water: (first: 2, last: 3)",
            "water: (first: 2, last: 6)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid terrain animation range should fail")
                .contains("outside the generated terrain atlas")
        );

        let invalid = MANIFEST.replacen(
            "powerup_sparkle: (first: 12, last: 15)",
            "powerup_sparkle: (first: 12, last: 20)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid effect range should fail")
                .contains("outside the generated effect atlas")
        );

        let invalid = MANIFEST.replacen(
            "explosion: (first: 0, last: 3)",
            "explosion: (first: 3, last: 0)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("reversed effect range should fail")
                .contains("starts after it ends")
        );

        let invalid = MANIFEST.replacen("tank: 5", "tank: 6", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid power-up index should fail")
                .contains("outside the generated power-up atlas")
        );
    }

    #[test]
    fn asset_manifest_rejects_invalid_generated_atlas_geometry() {
        let invalid = MANIFEST.replacen(
            "tanks: (tile_width: 16, tile_height: 16, tiles: 48)",
            "tanks: (tile_width: 15, tile_height: 16, tiles: 48)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid tank atlas tile size should fail")
                .contains("atlases.tanks tiles must be 16x16, got 15x16")
        );

        let invalid = MANIFEST.replacen(
            "effects: (tile_width: 16, tile_height: 16, tiles: 20)",
            "effects: (tile_width: 16, tile_height: 16, tiles: 19)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid effect atlas tile count should fail")
                .contains("atlases.effects must contain 20 tiles, got 19")
        );
    }

    #[test]
    fn asset_manifest_rejects_invalid_generated_sprite_sizes() {
        let invalid = MANIFEST.replacen(
            "intact: (width: 16, height: 16)",
            "intact: (width: 15, height: 16)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid base sprite size should fail")
                .contains("base.intact must be 16x16, got 15x16")
        );

        let invalid = MANIFEST.replacen(
            "score_badge: (width: 8, height: 8)",
            "score_badge: (width: 8, height: 9)",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("invalid UI icon size should fail")
                .contains("ui.score_badge must be 8x8, got 8x9")
        );
    }

    #[test]
    fn generated_atlas_images_use_manifest_geometry() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let atlases = [
            (
                create_tank_atlas(manifest.atlases.tanks),
                manifest.atlases.tanks,
            ),
            (
                create_terrain_atlas(manifest.atlases.terrain),
                manifest.atlases.terrain,
            ),
            (
                create_bullet_atlas(manifest.atlases.bullets),
                manifest.atlases.bullets,
            ),
            (
                create_effect_atlas(manifest.atlases.effects),
                manifest.atlases.effects,
            ),
            (
                create_powerup_atlas(manifest.atlases.powerups),
                manifest.atlases.powerups,
            ),
        ];

        for (image, atlas) in atlases {
            assert_eq!(
                image.texture_descriptor.size.width,
                (atlas.tile_width * atlas.tiles) as u32
            );
            assert_eq!(
                image.texture_descriptor.size.height,
                atlas.tile_height as u32
            );
            assert!(image.data.as_ref().is_some_and(|pixels| !pixels.is_empty()));
        }
    }

    #[test]
    fn asset_manifest_rejects_invalid_glyph_specs() {
        let invalid = MANIFEST.replacen("tile_width: 5", "tile_width: 6", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("wrong glyph width should fail")
                .contains("glyphs.tile_width 6 must match generated glyph width 5")
        );

        let invalid = MANIFEST.replacen(
            "characters: \"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
            "characters: \"00123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("duplicate glyph should fail")
                .contains("glyphs.characters includes duplicate glyph '0'")
        );

        let invalid = MANIFEST.replacen(
            "characters: \"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
            "characters: \"0123456789ABCDEFGHIJKLMNOPQRSTUVWXY?\"",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("unsupported glyph should fail")
                .contains("glyphs.characters includes unsupported blank glyph '?'")
        );

        let invalid = MANIFEST.replacen(
            "characters: \"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
            "characters: \"0123456789ABCDEFGHIJKLMNOPQRSTUVWXY\"",
            1,
        );
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("missing required glyph should fail")
                .contains("glyphs.characters must include required glyph 'Z'")
        );
    }

    #[test]
    fn asset_manifest_rejects_invalid_sound_specs() {
        let invalid = MANIFEST.replacen("duration_secs: 0.08", "duration_secs: 1.5", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("overlong sound should fail")
                .contains("sounds.fire duration 1.5")
        );

        let invalid = MANIFEST.replacen("seed: 305419896", "seed: 0", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("zero noise seed should fail")
                .contains("sounds.brick_hit noise seed must be nonzero")
        );

        let invalid = MANIFEST.replacen("frequency: 1320.0", "frequency: 0.0", 1);
        assert!(
            parse_asset_manifest(&invalid)
                .expect_err("zero note frequency should fail")
                .contains("sounds.powerup_pickup.notes[2] frequency 0")
        );

        assert!(
            validate_sound_spec(
                "sounds.base_destroyed",
                &RetroSoundSpec::Layered { notes: Vec::new() },
            )
            .expect_err("empty layered sound should fail")
            .contains("sounds.base_destroyed must define at least one note")
        );
    }

    #[test]
    fn authored_level_files_match_classic_shape() {
        for (stage, contents) in authored_levels() {
            let level = parse_level(contents).expect("level should parse");
            assert_eq!(level.name, format!("Stage {stage}"));
            assert_eq!(level.map.len(), BOARD_TILES);
            assert!(
                level
                    .map
                    .iter()
                    .all(|row| row.chars().count() == BOARD_TILES)
            );
            assert_eq!(level.enemies.len(), 20);
            assert_eq!(level.enemy_spawns.len(), 3);
            assert_eq!(
                level
                    .enemy_spawns
                    .iter()
                    .map(spawn_signature)
                    .collect::<Vec<_>>(),
                [
                    (0, 0, Direction::Down),
                    (12, 0, Direction::Down),
                    (24, 0, Direction::Down)
                ]
            );
            assert!(!level.powerup_carriers.is_empty());
            assert_eq!(level.max_enemies_on_screen, CLASSIC_MAX_ACTIVE_ENEMIES);
            assert_eq!(
                level.base_position,
                GridPoint {
                    x: CLASSIC_BASE_X,
                    y: CLASSIC_BASE_Y
                }
            );
        }
    }

    #[test]
    fn level_rules_default_to_normal_steel_and_later_levels_enable_upgrade_breaking() {
        let stage_1 = parse_level(LEVEL_1).expect("level should parse");
        assert_eq!(StageRules::from_level(&stage_1), StageRules::default());
        for contents in [
            LEVEL_3, LEVEL_4, LEVEL_5, LEVEL_6, LEVEL_7, LEVEL_8, LEVEL_9, LEVEL_10, LEVEL_11,
            LEVEL_12, LEVEL_13, LEVEL_14, LEVEL_15, LEVEL_16, LEVEL_17, LEVEL_18, LEVEL_19,
            LEVEL_20, LEVEL_21, LEVEL_22, LEVEL_23, LEVEL_24, LEVEL_25, LEVEL_26, LEVEL_27,
            LEVEL_28, LEVEL_29, LEVEL_30, LEVEL_31, LEVEL_32, LEVEL_33, LEVEL_34, LEVEL_35,
            LEVEL_36, LEVEL_37, LEVEL_38, LEVEL_39, LEVEL_40, LEVEL_41, LEVEL_42, LEVEL_43,
            LEVEL_44, LEVEL_45,
        ] {
            let level = parse_level(contents).expect("level should parse");
            assert_eq!(
                StageRules::from_level(&level),
                StageRules {
                    player_steel_destruction: true
                }
            );
        }
    }

    #[test]
    fn authored_arena_files_match_supported_battle_shapes() {
        for (index, contents) in authored_arenas() {
            let arena = parse_arena(contents).expect("arena should parse");
            assert_eq!(arena.name, format!("Arena {index}"));
            assert_eq!(arena.map.len(), BOARD_TILES);
            assert!(
                arena
                    .map
                    .iter()
                    .all(|row| row.chars().count() == BOARD_TILES)
            );
            assert!(!arena.powerup_spawns.is_empty());

            match arena.battle_rules {
                BattleRules::Deathmatch {
                    target_score,
                    lives,
                    respawn_invulnerability_secs,
                } => {
                    assert!(index < 5);
                    assert_eq!(target_score, 5);
                    assert_eq!(lives, 3);
                    assert_eq!(respawn_invulnerability_secs, 2.0);
                }
                BattleRules::BaseBattle {
                    p1_base,
                    p2_base,
                    lives,
                    respawn_invulnerability_secs,
                } => {
                    assert!(index >= 5);
                    assert_eq!(p1_base, GridPoint { x: 0, y: 24 });
                    assert_eq!(p2_base, GridPoint { x: 24, y: 0 });
                    assert_eq!(lives, 3);
                    assert_eq!(respawn_invulnerability_secs, 2.0);
                }
            }

            let grid = TileGrid::from_arena(&arena).expect("grid should build");
            assert!(grid.can_tank_occupy(Vec2::new(
                arena.p1_spawn.x as f32 * TILE_SIZE,
                arena.p1_spawn.y as f32 * TILE_SIZE
            )));
            assert!(grid.can_tank_occupy(Vec2::new(
                arena.p2_spawn.x as f32 * TILE_SIZE,
                arena.p2_spawn.y as f32 * TILE_SIZE
            )));
            for point in &arena.powerup_spawns {
                assert!(grid.can_tank_occupy(Vec2::new(
                    point.x as f32 * TILE_SIZE,
                    point.y as f32 * TILE_SIZE
                )));
            }
        }
    }

    #[test]
    fn arena_parses_base_battle_rules_and_validates_bases() {
        let arena = parse_arena(&base_battle_arena_text()).expect("base battle arena should parse");

        let BattleRules::BaseBattle {
            p1_base,
            p2_base,
            lives,
            respawn_invulnerability_secs,
        } = arena.battle_rules
        else {
            panic!("base battle arena should keep base battle rules");
        };

        assert_eq!(p1_base, GridPoint { x: 24, y: 24 });
        assert_eq!(p2_base, GridPoint { x: 0, y: 0 });
        assert_eq!(lives, 3);
        assert_eq!(respawn_invulnerability_secs, 2.0);
    }

    #[test]
    fn arena_five_base_battle_spawns_have_open_forward_lanes() {
        let arena = parse_arena(ARENA_5).expect("arena should parse");
        let grid = TileGrid::from_arena(&arena).expect("grid should build");

        for spawn in [&arena.p1_spawn, &arena.p2_spawn] {
            let forward = spawn_point_top_left(spawn) + spawn.facing.movement() * TILE_SIZE;
            assert!(
                grid.can_tank_occupy(forward),
                "spawn at ({}, {}) should be able to move {:?}",
                spawn.x,
                spawn.y,
                spawn.facing
            );
        }

        assert_eq!(grid.get(2, 24), Some(TileKind::Brick));
        assert_eq!(grid.get(22, 0), Some(TileKind::Brick));
    }

    #[test]
    fn arena_rejects_base_battle_rules_with_invalid_values() {
        let no_lives = base_battle_arena_text().replacen("lives: 3", "lives: 0", 1);
        assert!(
            parse_arena(&no_lives)
                .err()
                .expect("zero lives should fail")
                .contains("base battle lives must be greater than zero")
        );

        let shifted_base = base_battle_arena_text().replacen(
            "p1_base: (x: 24, y: 24)",
            "p1_base: (x: 23, y: 24)",
            1,
        );
        assert!(
            parse_arena(&shifted_base)
                .err()
                .expect("shifted p1 base should fail")
                .contains("p1 base position (23, 24) must cover a 2x2 base tile area")
        );

        let overlapping_bases = base_battle_arena_text().replacen(
            "p1_base: (x: 24, y: 24)",
            "p1_base: (x: 0, y: 0)",
            1,
        );
        assert!(
            parse_arena(&overlapping_bases)
                .err()
                .expect("overlapping bases should fail")
                .contains("p1 base (0, 0) and p2 base (0, 0) must not overlap")
        );
    }

    #[test]
    fn level_rejects_spawn_points_that_do_not_fit_tanks() {
        let blocked_player = LEVEL_1.replacen(
            "player_spawn: (x: 8, y: 24",
            "player_spawn: (x: 10, y: 24",
            1,
        );
        assert!(
            parse_level(&blocked_player)
                .err()
                .expect("blocked player spawn should fail")
                .contains("player spawn (10, 24) must fit a tank on passable tiles")
        );

        let blocked_enemy = LEVEL_1.replacen(
            "\"..........................\",",
            "\"............BB............\",",
            1,
        );
        assert!(
            parse_level(&blocked_enemy)
                .err()
                .expect("blocked enemy spawn should fail")
                .contains("enemy spawn 2 (12, 0) must fit a tank on passable tiles")
        );
    }

    #[test]
    fn level_rejects_enemy_spawns_outside_classic_top_slots() {
        let shifted_enemy = LEVEL_1.replacen(
            "(x: 12, y: 0, facing: Down)",
            "(x: 8, y: 0, facing: Down)",
            1,
        );

        assert!(
            parse_level(&shifted_enemy)
                .err()
                .expect("shifted enemy spawn should fail")
                .contains(
                    "enemy spawn 2 must be classic top spawn (12, 0, Down), got (8, 0, Down)"
                )
        );
    }

    #[test]
    fn level_rejects_more_than_four_active_enemies() {
        let too_many_active =
            LEVEL_1.replacen("max_enemies_on_screen: 4", "max_enemies_on_screen: 5", 1);

        assert!(
            parse_level(&too_many_active)
                .err()
                .expect("too many active enemies should fail")
                .contains("max_enemies_on_screen must be at most 4, got 5")
        );
    }

    #[test]
    fn level_rejects_base_positions_that_do_not_cover_base_tiles() {
        let shifted_base = LEVEL_1.replacen(
            "base_position: (x: 12, y: 24)",
            "base_position: (x: 11, y: 24)",
            1,
        );
        assert!(
            parse_level(&shifted_base)
                .err()
                .expect("shifted base should fail")
                .contains("base position (11, 24) must cover a 2x2 base tile area")
        );
    }

    #[test]
    fn level_rejects_campaign_base_outside_classic_bottom_center() {
        let shifted_base = LEVEL_1
            .replacen(
                "\"..........BBEEBB..........\",",
                "\"............BBEEBB........\",",
                2,
            )
            .replacen(
                "base_position: (x: 12, y: 24)",
                "base_position: (x: 14, y: 24)",
                1,
            );

        assert!(
            parse_level(&shifted_base)
                .err()
                .expect("shifted campaign base should fail")
                .contains("base position (14, 24) must use classic campaign base (12, 24)")
        );
    }

    #[test]
    fn arena_rejects_spawn_points_that_do_not_fit_tanks() {
        let blocked_p1 = ARENA_1.replacen("p1_spawn: (x: 0, y: 24", "p1_spawn: (x: 4, y: 24", 1);
        assert!(
            parse_arena(&blocked_p1)
                .err()
                .expect("blocked p1 spawn should fail")
                .contains("p1 spawn (4, 24) must fit a tank on passable tiles")
        );
    }

    #[test]
    fn arena_rejects_overlapping_player_spawns() {
        let overlapping_p2 = ARENA_1.replacen(
            "p2_spawn: (x: 24, y: 0, facing: Down)",
            "p2_spawn: (x: 0, y: 24, facing: Down)",
            1,
        );

        assert!(
            parse_arena(&overlapping_p2)
                .err()
                .expect("overlapping player spawns should fail")
                .contains("p1 spawn (0, 24) and p2 spawn (0, 24) must not overlap")
        );
    }

    #[test]
    fn arena_rejects_powerup_spawns_that_are_not_collectible() {
        let blocked_powerup = ARENA_1.replacen("(x: 12, y: 12)", "(x: 4, y: 24)", 1);
        assert!(
            parse_arena(&blocked_powerup)
                .err()
                .expect("blocked power-up spawn should fail")
                .contains("power-up spawn 1 (4, 24) must fit a 16x16 reward on passable tiles")
        );
    }

    #[test]
    fn arena_rejects_duplicate_powerup_spawns() {
        let duplicate_powerup = ARENA_1.replacen(
            "  powerup_spawns: [\n    (x: 12, y: 12),\n  ],",
            "  powerup_spawns: [\n    (x: 12, y: 12),\n    (x: 12, y: 12),\n  ],",
            1,
        );

        assert!(
            parse_arena(&duplicate_powerup)
                .err()
                .expect("duplicate power-up spawn should fail")
                .contains("power-up spawn 2 (12, 12) is configured more than once")
        );
    }

    #[test]
    fn tile_grid_uses_expected_passability() {
        let level = parse_level(LEVEL_1).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        assert!(!TileKind::Brick.tank_passable());
        assert!(!TileKind::Water.tank_passable());
        assert!(TileKind::Forest.tank_passable());
        assert!(TileKind::Ice.tank_passable());
        assert!(grid.can_tank_occupy(Vec2::new(8.0 * TILE_SIZE, 24.0 * TILE_SIZE)));
    }

    #[test]
    fn ice_tiles_modify_tank_movement_speed() {
        let mut grid = TileGrid::empty();
        for y in 4..=5 {
            for x in 4..=5 {
                grid.set(x, y, TileKind::Ice);
            }
        }

        assert_eq!(
            tank_move_speed(PLAYER_SPEED, &grid, Vec2::new(0.0, 0.0)),
            PLAYER_SPEED
        );
        assert_eq!(
            tank_move_speed(
                PLAYER_SPEED,
                &grid,
                Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE)
            ),
            PLAYER_SPEED * ICE_SPEED_MULTIPLIER
        );
        assert!(
            grid.tank_overlaps_tile(Vec2::new(3.0 * TILE_SIZE, 4.0 * TILE_SIZE), TileKind::Ice)
        );
    }

    #[test]
    fn ice_tiles_allow_tanks_and_do_not_block_bullets() {
        let mut grid = TileGrid::empty();
        for y in 4..=5 {
            for x in 4..=5 {
                grid.set(x, y, TileKind::Ice);
            }
        }

        assert!(grid.can_tank_occupy(Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE)));
        assert!(!TileKind::Ice.bullet_blocks());
        assert_eq!(
            bullet_blocking_tile_hit(&grid, Vec2::new(20.0, 32.0), Vec2::new(48.0, 32.0)),
            None
        );
    }

    #[test]
    fn water_tiles_block_tanks_but_do_not_block_bullets() {
        let mut grid = TileGrid::empty();
        for y in 4..=5 {
            for x in 4..=5 {
                grid.set(x, y, TileKind::Water);
            }
        }

        assert!(!grid.can_tank_occupy(Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE)));
        assert!(!TileKind::Water.bullet_blocks());
        assert_eq!(
            bullet_blocking_tile_hit(&grid, Vec2::new(20.0, 32.0), Vec2::new(48.0, 32.0)),
            None
        );
    }

    #[test]
    fn stage_six_authors_ice_corridors() {
        let stage_6 = parse_level(LEVEL_6).expect("level should parse");
        let grid = TileGrid::from_level(&stage_6).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Ice));
    }

    #[test]
    fn stage_seven_authors_forest_pressure_lanes() {
        let stage_7 = parse_level(LEVEL_7).expect("level should parse");
        let grid = TileGrid::from_level(&stage_7).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
    }

    #[test]
    fn stage_eight_authors_steel_maze_pressure() {
        let stage_8 = parse_level(LEVEL_8).expect("level should parse");
        let grid = TileGrid::from_level(&stage_8).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
    }

    #[test]
    fn stage_nine_authors_mixed_terrain_pressure() {
        let stage_9 = parse_level(LEVEL_9).expect("level should parse");
        let grid = TileGrid::from_level(&stage_9).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_9.spawn_interval_secs, 1.5);
    }

    #[test]
    fn stage_ten_authors_fortress_pressure() {
        let stage_10 = parse_level(LEVEL_10).expect("level should parse");
        let grid = TileGrid::from_level(&stage_10).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_10.spawn_interval_secs, 1.4);
        assert_eq!(stage_10.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_eleven_authors_split_lane_pressure() {
        let stage_11 = parse_level(LEVEL_11).expect("level should parse");
        let grid = TileGrid::from_level(&stage_11).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_11.spawn_interval_secs, 1.35);
        assert_eq!(stage_11.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twelve_authors_ice_and_water_choke_points() {
        let stage_12 = parse_level(LEVEL_12).expect("level should parse");
        let grid = TileGrid::from_level(&stage_12).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(stage_12.spawn_interval_secs, 1.3);
        assert_eq!(stage_12.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirteen_authors_forest_screen_pressure() {
        let stage_13 = parse_level(LEVEL_13).expect("level should parse");
        let grid = TileGrid::from_level(&stage_13).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_13.spawn_interval_secs, 1.25);
        assert_eq!(stage_13.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_fourteen_authors_steel_gate_gauntlet() {
        let stage_14 = parse_level(LEVEL_14).expect("level should parse");
        let grid = TileGrid::from_level(&stage_14).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert_eq!(stage_14.spawn_interval_secs, 1.2);
        assert_eq!(stage_14.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_fifteen_authors_mixed_fortress_pressure() {
        let stage_15 = parse_level(LEVEL_15).expect("level should parse");
        let grid = TileGrid::from_level(&stage_15).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert_eq!(stage_15.spawn_interval_secs, 1.15);
        assert_eq!(stage_15.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_sixteen_authors_river_lock_pressure() {
        let stage_16 = parse_level(LEVEL_16).expect("level should parse");
        let grid = TileGrid::from_level(&stage_16).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_16.spawn_interval_secs, 1.1);
        assert_eq!(stage_16.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_seventeen_authors_ice_screen_pressure() {
        let stage_17 = parse_level(LEVEL_17).expect("level should parse");
        let grid = TileGrid::from_level(&stage_17).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(stage_17.spawn_interval_secs, 1.05);
        assert_eq!(stage_17.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_eighteen_authors_steel_axis_pressure() {
        let stage_18 = parse_level(LEVEL_18).expect("level should parse");
        let grid = TileGrid::from_level(&stage_18).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_18.spawn_interval_secs, 1.0);
        assert_eq!(stage_18.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_nineteen_authors_outer_wall_pressure() {
        let stage_19 = parse_level(LEVEL_19).expect("level should parse");
        let grid = TileGrid::from_level(&stage_19).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_19.spawn_interval_secs, 0.95);
        assert_eq!(stage_19.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_authors_mid_campaign_delta_pressure() {
        let stage_20 = parse_level(LEVEL_20).expect("level should parse");
        let grid = TileGrid::from_level(&stage_20).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_20.spawn_interval_secs, 0.9);
        assert_eq!(stage_20.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_one_authors_canal_gate_pressure() {
        let stage_21 = parse_level(LEVEL_21).expect("level should parse");
        let grid = TileGrid::from_level(&stage_21).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_21.spawn_interval_secs, 0.88);
        assert_eq!(stage_21.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_two_authors_forest_ice_axis_pressure() {
        let stage_22 = parse_level(LEVEL_22).expect("level should parse");
        let grid = TileGrid::from_level(&stage_22).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(stage_22.spawn_interval_secs, 0.86);
        assert_eq!(stage_22.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_three_authors_steel_basin_crossfire() {
        let stage_23 = parse_level(LEVEL_23).expect("level should parse");
        let grid = TileGrid::from_level(&stage_23).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_23.spawn_interval_secs, 0.84);
        assert_eq!(stage_23.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_four_authors_ice_water_flank_pressure() {
        let stage_24 = parse_level(LEVEL_24).expect("level should parse");
        let grid = TileGrid::from_level(&stage_24).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(stage_24.spawn_interval_secs, 0.82);
        assert_eq!(stage_24.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_five_authors_steel_water_bastion_pressure() {
        let stage_25 = parse_level(LEVEL_25).expect("level should parse");
        let grid = TileGrid::from_level(&stage_25).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_25.spawn_interval_secs, 0.8);
        assert_eq!(stage_25.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_six_authors_brick_steel_gate_pressure() {
        let stage_26 = parse_level(LEVEL_26).expect("level should parse");
        let grid = TileGrid::from_level(&stage_26).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_26.spawn_interval_secs, 0.78);
        assert_eq!(stage_26.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_seven_authors_forest_water_lane_pressure() {
        let stage_27 = parse_level(LEVEL_27).expect("level should parse");
        let grid = TileGrid::from_level(&stage_27).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_27.spawn_interval_secs, 0.76);
        assert_eq!(stage_27.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_eight_authors_steel_ring_ice_moat_pressure() {
        let stage_28 = parse_level(LEVEL_28).expect("level should parse");
        let grid = TileGrid::from_level(&stage_28).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert_eq!(stage_28.spawn_interval_secs, 0.74);
        assert_eq!(stage_28.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_twenty_nine_authors_brick_fortress_rush_pressure() {
        let stage_29 = parse_level(LEVEL_29).expect("level should parse");
        let grid = TileGrid::from_level(&stage_29).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_29.spawn_interval_secs, 0.72);
        assert_eq!(stage_29.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_authors_late_campaign_bastion_pressure() {
        let stage_30 = parse_level(LEVEL_30).expect("level should parse");
        let grid = TileGrid::from_level(&stage_30).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_30.spawn_interval_secs, 0.7);
        assert_eq!(stage_30.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_one_authors_forest_water_choke_pressure() {
        let stage_31 = parse_level(LEVEL_31).expect("level should parse");
        let grid = TileGrid::from_level(&stage_31).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_31.spawn_interval_secs, 0.68);
        assert_eq!(stage_31.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_two_authors_crossfire_water_lattice_pressure() {
        let stage_32 = parse_level(LEVEL_32).expect("level should parse");
        let grid = TileGrid::from_level(&stage_32).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_32.spawn_interval_secs, 0.66);
        assert_eq!(stage_32.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_three_authors_diagonal_bunker_pressure() {
        let stage_33 = parse_level(LEVEL_33).expect("level should parse");
        let grid = TileGrid::from_level(&stage_33).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_33.spawn_interval_secs, 0.64);
        assert_eq!(stage_33.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_four_authors_base_siege_pressure() {
        let stage_34 = parse_level(LEVEL_34).expect("level should parse");
        let grid = TileGrid::from_level(&stage_34).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_34.spawn_interval_secs, 0.62);
        assert_eq!(stage_34.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_five_authors_first_campaign_finale_pressure() {
        let stage_35 = parse_level(LEVEL_35).expect("level should parse");
        let grid = TileGrid::from_level(&stage_35).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_35.spawn_interval_secs, 0.60);
        assert_eq!(stage_35.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_six_authors_extended_campaign_base_siege() {
        let stage_36 = parse_level(LEVEL_36).expect("level should parse");
        let grid = TileGrid::from_level(&stage_36).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_36.spawn_interval_secs, 0.58);
        assert_eq!(stage_36.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_seven_authors_extended_campaign_water_bulwark() {
        let stage_37 = parse_level(LEVEL_37).expect("level should parse");
        let grid = TileGrid::from_level(&stage_37).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_37.spawn_interval_secs, 0.56);
        assert_eq!(stage_37.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_eight_authors_extended_campaign_ice_crossfire() {
        let stage_38 = parse_level(LEVEL_38).expect("level should parse");
        let grid = TileGrid::from_level(&stage_38).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_38.spawn_interval_secs, 0.54);
        assert_eq!(stage_38.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_thirty_nine_authors_extended_campaign_steel_freeze_lanes() {
        let stage_39 = parse_level(LEVEL_39).expect("level should parse");
        let grid = TileGrid::from_level(&stage_39).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_39.spawn_interval_secs, 0.52);
        assert_eq!(stage_39.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_authors_extended_campaign_iron_keep_pressure() {
        let stage_40 = parse_level(LEVEL_40).expect("level should parse");
        let grid = TileGrid::from_level(&stage_40).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_40.spawn_interval_secs, 0.50);
        assert_eq!(stage_40.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_one_authors_extended_campaign_frozen_crossfire() {
        let stage_41 = parse_level(LEVEL_41).expect("level should parse");
        let grid = TileGrid::from_level(&stage_41).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_41.spawn_interval_secs, 0.48);
        assert_eq!(stage_41.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_two_authors_extended_campaign_ring_fire_lanes() {
        let stage_42 = parse_level(LEVEL_42).expect("level should parse");
        let grid = TileGrid::from_level(&stage_42).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_42.spawn_interval_secs, 0.46);
        assert_eq!(stage_42.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_three_authors_extended_campaign_mirror_gates() {
        let stage_43 = parse_level(LEVEL_43).expect("level should parse");
        let grid = TileGrid::from_level(&stage_43).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_43.spawn_interval_secs, 0.44);
        assert_eq!(stage_43.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_four_authors_extended_campaign_ice_fortress() {
        let stage_44 = parse_level(LEVEL_44).expect("level should parse");
        let grid = TileGrid::from_level(&stage_44).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_44.spawn_interval_secs, 0.42);
        assert_eq!(stage_44.powerup_carriers.len(), 6);
    }

    #[test]
    fn stage_forty_five_authors_extended_campaign_crowned_moat() {
        let stage_45 = parse_level(LEVEL_45).expect("level should parse");
        let grid = TileGrid::from_level(&stage_45).expect("grid should build");
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert!(grid.tiles.contains(&TileKind::Brick));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert_eq!(stage_45.spawn_interval_secs, 0.40);
        assert_eq!(stage_45.powerup_carriers.len(), 6);
    }

    #[test]
    fn forest_renders_as_overlay_above_tanks_and_bullets() {
        assert!(TileKind::Forest.tank_passable());
        assert!(terrain_z(TileKind::Forest) > 7.0);
        assert!(terrain_z(TileKind::Forest) < 8.0);
        assert!(terrain_z(TileKind::Brick) < terrain_z(TileKind::Forest));
        assert!(terrain_z(TileKind::Water) < terrain_z(TileKind::Forest));
    }

    #[test]
    fn campaign_enemy_markers_fit_as_compact_tank_icons() {
        assert_eq!(ENEMY_MARKER_COUNT, 20);
        assert_eq!(ENEMY_MARKER_COLUMNS, 4);
        assert_eq!(enemy_marker_top_left(0), Vec2::new(216.0, 159.0));
        assert_eq!(enemy_marker_top_left(3), Vec2::new(243.0, 159.0));
        assert_eq!(enemy_marker_top_left(4), Vec2::new(216.0, 168.0));

        let last = enemy_marker_top_left(ENEMY_MARKER_COUNT - 1);
        assert_eq!(last, Vec2::new(243.0, 195.0));
        assert!(last.x + ENEMY_MARKER_SIZE <= VIRTUAL_WIDTH - 4.0);
        assert!(last.y + ENEMY_MARKER_SIZE <= BOARD_ORIGIN_Y + board_size());
    }

    #[test]
    fn campaign_enemy_markers_show_undestroyed_enemies() {
        assert_eq!(enemy_markers_remaining(20, 0), 20);
        assert_eq!(enemy_markers_remaining(20, 1), 19);
        assert_eq!(enemy_markers_remaining(20, 4), 16);
        assert_eq!(enemy_markers_remaining(20, 20), 0);
        assert_eq!(enemy_markers_remaining(20, 25), 0);
    }

    #[test]
    fn campaign_enemy_markers_stay_visible_until_first_kill() {
        let mut score_board = ScoreBoard::campaign(20);

        assert_eq!(
            enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed),
            20
        );

        score_board.record_enemy_destroyed(EnemyKind::Basic);
        assert_eq!(
            enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed),
            19
        );
    }

    #[test]
    fn campaign_enemy_marker_uses_basic_enemy_tank_sprite() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        assert_eq!(
            enemy_marker_tank_index(&manifest),
            manifest.tank_index(TankSpriteSet::EnemyBasic, Direction::Down, 0)
        );
    }

    #[test]
    fn campaign_life_icon_fits_status_panel() {
        let top_left = campaign_life_icon_top_left();
        assert_eq!(top_left, Vec2::new(222.0, 123.0));
        assert!(top_left.x >= 212.0);
        assert!(top_left.x + PLAYER_LIFE_ICON_SIZE < 234.0);
        assert!(top_left.y + PLAYER_LIFE_ICON_SIZE < ENEMY_MARKER_TOP);
    }

    #[test]
    fn campaign_life_icon_uses_player_tank_sprite() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        assert_eq!(
            player_life_icon_tank_index(&manifest, PlayerId::One),
            manifest.tank_index(TankSpriteSet::Player1, Direction::Up, 0)
        );
    }

    #[test]
    fn versus_life_icons_fit_status_panel_and_match_players() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let p1_top_left = versus_life_icon_top_left(PlayerId::One);
        let p2_top_left = versus_life_icon_top_left(PlayerId::Two);

        assert_eq!(p1_top_left, Vec2::new(222.0, 73.0));
        assert_eq!(p2_top_left, Vec2::new(222.0, 145.0));
        for top_left in [p1_top_left, p2_top_left] {
            assert!(top_left.x >= 212.0);
            assert!(top_left.x + PLAYER_LIFE_ICON_SIZE < 234.0);
            assert!(top_left.y >= 24.0);
            assert!(top_left.y + PLAYER_LIFE_ICON_SIZE <= BOARD_ORIGIN_Y + board_size());
        }
        assert!(p2_top_left.y + PLAYER_LIFE_ICON_SIZE <= versus_arena_label_top_left().y);
        assert_eq!(
            player_life_icon_tank_index(&manifest, PlayerId::One),
            manifest.tank_index(TankSpriteSet::Player1, Direction::Up, 0)
        );
        assert_eq!(
            player_life_icon_tank_index(&manifest, PlayerId::Two),
            manifest.tank_index(TankSpriteSet::Player2, Direction::Up, 0)
        );
    }

    #[test]
    fn versus_arena_and_objective_labels_fit_status_panel() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let labeled_rows = [
            ("ARENA", versus_arena_label_top_left()),
            ("TARGET", versus_target_label_top_left()),
            ("BASE", versus_base_label_top_left()),
        ];
        let digit_rows = [
            ("99", versus_arena_number_top_left()),
            ("99", versus_target_number_top_left()),
        ];

        for (label, top_left) in labeled_rows {
            assert!(top_left.x >= 212.0);
            assert!(top_left.x + phase_text_width(label) <= VIRTUAL_WIDTH - 4.0);
            assert!(top_left.y >= 24.0);
            assert!(top_left.y + GENERATED_GLYPH_HEIGHT as f32 <= BOARD_ORIGIN_Y + board_size());
            for ch in label.chars() {
                assert_manifest_glyph_is_visible(&manifest, ch);
            }
        }

        for (digits, top_left) in digit_rows {
            assert!(top_left.x >= 212.0);
            assert!(top_left.x + phase_text_width(digits) <= VIRTUAL_WIDTH - 4.0);
            assert!(top_left.y >= 24.0);
            assert!(top_left.y + GENERATED_GLYPH_HEIGHT as f32 <= BOARD_ORIGIN_Y + board_size());
        }
        assert!(versus_arena_number_top_left().y < versus_target_label_top_left().y);
        assert!(versus_target_number_top_left().y > versus_target_label_top_left().y);
        assert!(versus_base_label_top_left().y > versus_arena_number_top_left().y);
    }

    #[test]
    fn status_value_text_tracks_versus_arena_number() {
        let score_board = ScoreBoard::versus(3, 5, 2.0);
        let arena_five = GameStatus {
            arena: 5,
            ..GameStatus::default()
        };
        let late_arena = GameStatus {
            arena: 135,
            ..GameStatus::default()
        };

        assert_eq!(
            status_value_text(
                StatusValue::Arena,
                GameMode::VersusDeathmatch,
                &arena_five,
                &score_board
            ),
            "05"
        );
        assert_eq!(
            status_value_text(
                StatusValue::Arena,
                GameMode::VersusBaseBattle,
                &late_arena,
                &score_board
            ),
            "99"
        );
    }

    #[test]
    fn campaign_score_icon_fits_next_to_score_label() {
        let icon = score_badge_icon_top_left();
        let score_label_right = 214.0 + phase_text_width("SCORE");
        assert_eq!(icon, Vec2::new(244.0, 38.0));
        assert!(icon.x > score_label_right);
        assert!(icon.x + (GENERATED_UI_ICON_SIZE as f32) <= VIRTUAL_WIDTH - 4.0);
        assert!(icon.y + (GENERATED_UI_ICON_SIZE as f32) < 49.0);
    }

    #[test]
    fn score_badge_icon_uses_transparent_pixel_art() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let image = create_score_badge_icon(manifest.ui.score_badge);
        assert_eq!(
            image.texture_descriptor.size.width,
            manifest.ui.score_badge.width as u32
        );
        assert_eq!(
            image.texture_descriptor.size.height,
            manifest.ui.score_badge.height as u32
        );
        let pixels = image.data.as_ref().expect("score icon should have pixels");
        assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 0));
        assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 255));
    }

    #[test]
    fn campaign_stage_icon_and_number_fit_status_panel() {
        let icon = stage_flag_icon_top_left();
        let number = stage_number_top_left();
        assert_eq!(icon, Vec2::new(216.0, 87.0));
        assert_eq!(number, Vec2::new(230.0, 87.0));
        assert!(icon.x >= 212.0);
        assert!(icon.x + (GENERATED_UI_ICON_SIZE as f32) < number.x);
        assert!(number.x + phase_text_width("99") <= VIRTUAL_WIDTH - 8.0);
    }

    #[test]
    fn stage_flag_icon_uses_transparent_pixel_art() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let image = create_stage_flag_icon(manifest.ui.stage_flag);
        assert_eq!(
            image.texture_descriptor.size.width,
            manifest.ui.stage_flag.width as u32
        );
        assert_eq!(
            image.texture_descriptor.size.height,
            manifest.ui.stage_flag.height as u32
        );
        let pixels = image.data.as_ref().expect("stage icon should have pixels");
        assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 0));
        assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 255));
    }

    #[test]
    fn base_sprites_use_manifest_dimensions() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        for (sprite, destroyed) in [
            (manifest.base.intact, false),
            (manifest.base.destroyed, true),
        ] {
            let image = create_base_image(sprite, destroyed);
            assert_eq!(image.texture_descriptor.size.width, sprite.width as u32);
            assert_eq!(image.texture_descriptor.size.height, sprite.height as u32);
            let pixels = image.data.as_ref().expect("base sprite should have pixels");
            assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 255));
        }
    }

    #[test]
    fn campaign_phase_detects_game_over() {
        assert_eq!(campaign_phase(0, 20, 0, 20, 0), GamePhase::GameOver);
    }

    #[test]
    fn campaign_phase_detects_level_clear() {
        assert_eq!(campaign_phase(3, 20, 20, 0, 0), GamePhase::LevelClear);
        assert_eq!(campaign_phase(3, 20, 19, 0, 0), GamePhase::LevelClear);
    }

    #[test]
    fn campaign_phase_stays_playing_while_enemies_remain() {
        assert_eq!(campaign_phase(3, 20, 19, 0, 1), GamePhase::Playing);
        assert_eq!(campaign_phase(3, 20, 5, 10, 4), GamePhase::Playing);
    }

    #[test]
    fn level_clear_scorecard_stays_readable_before_next_stage() {
        assert_eq!(
            campaign_phase_transition_seconds(GamePhase::LevelClear),
            LEVEL_CLEAR_SCORECARD_SECONDS
        );
        assert!(
            campaign_phase_transition_seconds(GamePhase::LevelClear) > LEVEL_CLEAR_DELAY_SECONDS
        );
        assert_eq!(
            campaign_phase_transition_seconds(GamePhase::GameOver),
            LEVEL_CLEAR_DELAY_SECONDS
        );
    }

    #[test]
    fn stage_clear_bonus_rewards_remaining_lives() {
        assert_eq!(stage_clear_bonus(3), 3000);
        assert_eq!(stage_clear_bonus(1), 1000);
        assert_eq!(stage_clear_bonus(0), 0);
        assert_eq!(stage_clear_bonus(-2), 0);
    }

    #[test]
    fn score_board_tracks_enemy_kill_breakdown() {
        let mut score_board = ScoreBoard::campaign(20);

        score_board.record_enemy_destroyed(EnemyKind::Basic);
        score_board.record_enemy_destroyed(EnemyKind::Fast);
        score_board.record_enemy_destroyed(EnemyKind::Fast);
        score_board.record_enemy_destroyed(EnemyKind::Power);
        score_board.record_enemy_destroyed(EnemyKind::Armor);

        assert_eq!(score_board.score, 1200);
        assert_eq!(score_board.enemies_destroyed, 5);
        assert_eq!(score_board.enemy_kills.count(EnemyKind::Basic), 1);
        assert_eq!(score_board.enemy_kills.count(EnemyKind::Fast), 2);
        assert_eq!(score_board.enemy_kills.count(EnemyKind::Power), 1);
        assert_eq!(score_board.enemy_kills.count(EnemyKind::Armor), 1);
        assert_eq!(score_board.enemy_kills.total(), 5);
    }

    #[test]
    fn pause_toggle_only_affects_active_or_paused_game() {
        assert_eq!(
            toggle_pause_phase(GamePhase::ModeSelect),
            GamePhase::ModeSelect
        );
        assert_eq!(
            toggle_pause_phase(GamePhase::StageIntro),
            GamePhase::StageIntro
        );
        assert_eq!(toggle_pause_phase(GamePhase::Playing), GamePhase::Paused);
        assert_eq!(toggle_pause_phase(GamePhase::Paused), GamePhase::Playing);
        assert_eq!(toggle_pause_phase(GamePhase::GameOver), GamePhase::GameOver);
        assert_eq!(
            toggle_pause_phase(GamePhase::LevelClear),
            GamePhase::LevelClear
        );
        assert_eq!(
            toggle_pause_phase(GamePhase::RoundOver),
            GamePhase::RoundOver
        );
        assert_eq!(toggle_pause_phase(GamePhase::Victory), GamePhase::Victory);
    }

    #[test]
    fn paused_phase_freezes_visual_effect_timers_only() {
        assert!(!visual_effects_can_advance(GamePhase::Paused));
        assert!(visual_effects_can_advance(GamePhase::StageIntro));
        assert!(visual_effects_can_advance(GamePhase::Playing));
        assert!(visual_effects_can_advance(GamePhase::LevelClear));
        assert!(visual_effects_can_advance(GamePhase::GameOver));
        assert!(visual_effects_can_advance(GamePhase::RoundOver));
        assert!(visual_effects_can_advance(GamePhase::Victory));
    }

    #[test]
    fn terminal_phases_clear_transient_bullets_and_powerups() {
        assert!(!terminal_phase_clears_transients(GamePhase::ModeSelect));
        assert!(!terminal_phase_clears_transients(GamePhase::StageIntro));
        assert!(!terminal_phase_clears_transients(GamePhase::Playing));
        assert!(!terminal_phase_clears_transients(GamePhase::Paused));
        assert!(terminal_phase_clears_transients(GamePhase::LevelClear));
        assert!(terminal_phase_clears_transients(GamePhase::GameOver));
        assert!(terminal_phase_clears_transients(GamePhase::RoundOver));
        assert!(terminal_phase_clears_transients(GamePhase::Victory));
    }

    #[test]
    fn bullet_clashes_resolve_only_while_playing() {
        assert!(bullet_clashes_can_resolve(GamePhase::Playing));
        assert!(!bullet_clashes_can_resolve(GamePhase::Paused));
        assert!(!bullet_clashes_can_resolve(GamePhase::StageIntro));
        assert!(!bullet_clashes_can_resolve(GamePhase::LevelClear));
        assert!(!bullet_clashes_can_resolve(GamePhase::GameOver));
        assert!(!bullet_clashes_can_resolve(GamePhase::RoundOver));
        assert!(!bullet_clashes_can_resolve(GamePhase::Victory));
        assert!(!bullet_clashes_can_resolve(GamePhase::ModeSelect));
    }

    #[test]
    fn victory_phase_uses_campaign_clear_banner() {
        assert_eq!(
            phase_banner_lines(GamePhase::Victory, None).expect("victory should show a banner"),
            VICTORY_BANNER_LINES.as_slice()
        );
    }

    #[test]
    fn victory_screen_clears_previous_stage_entities_and_runtime_state() {
        let mut app = App::new();
        let mut grid = TileGrid::empty();
        grid.set(10, 24, TileKind::Brick);

        let level = parse_level(LEVEL_1).expect("level should parse");
        let mut enemy_freeze = EnemyFreeze::default();
        enemy_freeze.start();
        let mut versus_freeze = VersusPlayerFreeze::default();
        versus_freeze.start(PlayerId::Two);
        let mut base_reinforcement = BaseReinforcement {
            timer: None,
            saved_tiles: vec![(10, 24, TileKind::Brick)],
        };
        base_reinforcement.start();

        app.insert_resource(GameStatus {
            phase: GamePhase::LevelClear,
            stage: LEVEL_COUNT,
            arena: DEFAULT_VERSUS_ARENA,
            winner: Some(PlayerId::One),
            transition_timer: Timer::from_seconds(LEVEL_CLEAR_SCORECARD_SECONDS, TimerMode::Once),
        });
        app.insert_resource(grid);
        app.insert_resource(EnemyDirector::from_level(&level));
        app.insert_resource(StageRules {
            player_steel_destruction: true,
        });
        app.insert_resource(enemy_freeze);
        app.insert_resource(versus_freeze);
        app.insert_resource(base_reinforcement);
        app.world_mut()
            .spawn((GameEntity, GridTile { x: 10, y: 24 }));
        app.world_mut().spawn((GameEntity, PhaseBanner));
        app.add_systems(Update, enter_victory_screen_for_test);

        app.update();

        let mut game_entities = app.world_mut().query::<&GameEntity>();
        assert_eq!(game_entities.iter(app.world()).count(), 0);

        let status = app.world().resource::<GameStatus>();
        assert_eq!(status.phase, GamePhase::Victory);
        assert_eq!(status.stage, LEVEL_COUNT);
        assert_eq!(status.winner, None);
        assert!(
            app.world()
                .resource::<TileGrid>()
                .tiles
                .iter()
                .all(|tile| *tile == TileKind::Empty)
        );
        let director = app.world().resource::<EnemyDirector>();
        assert!(director.roster.is_empty());
        assert!(director.spawns.is_empty());
        assert_eq!(director.max_active, 0);
        assert_eq!(*app.world().resource::<StageRules>(), StageRules::default());
        assert!(!app.world().resource::<EnemyFreeze>().is_active());
        assert!(
            !app.world()
                .resource::<VersusPlayerFreeze>()
                .is_player_frozen(PlayerId::Two)
        );
        let reinforcement = app.world().resource::<BaseReinforcement>();
        assert!(reinforcement.timer.is_none());
        assert!(reinforcement.saved_tiles.is_empty());
    }

    #[test]
    fn stage_intro_blocks_gameplay_and_shows_ready_banner() {
        let status = GameStatus {
            phase: GamePhase::StageIntro,
            stage: 7,
            ..GameStatus::default()
        };

        assert!(!status.is_playing());
        assert_eq!(
            phase_banner_text(&status, GameMode::Campaign, &ScoreBoard::campaign(3))
                .expect("stage intro should show a banner"),
            ["STAGE 07".to_string(), "READY".to_string()]
        );
    }

    #[test]
    fn stage_intro_banner_clamps_two_digit_stage_label() {
        assert_eq!(
            stage_intro_banner_text(3),
            ["STAGE 03".to_string(), "READY".to_string()]
        );
        assert_eq!(
            stage_intro_banner_text(135),
            ["STAGE 99".to_string(), "READY".to_string()]
        );
    }

    #[test]
    fn level_clear_banner_shows_cleared_stage_number() {
        let status = GameStatus {
            phase: GamePhase::LevelClear,
            stage: 12,
            ..GameStatus::default()
        };
        let mut score_board = ScoreBoard::campaign(20);
        score_board.lives = 2;
        score_board.record_enemy_destroyed(EnemyKind::Basic);
        score_board.record_enemy_destroyed(EnemyKind::Fast);
        score_board.record_enemy_destroyed(EnemyKind::Fast);
        score_board.record_enemy_destroyed(EnemyKind::Power);
        score_board.record_enemy_destroyed(EnemyKind::Armor);

        assert_eq!(
            phase_banner_text(&status, GameMode::Campaign, &score_board)
                .expect("level clear should show a banner"),
            [
                "STAGE 12".to_string(),
                "LEVEL CLEAR".to_string(),
                "100X01 200X02".to_string(),
                "300X01 400X01".to_string(),
                "TOTAL 05".to_string(),
                "BONUS 2000".to_string()
            ]
        );

        let mut late_score_board = ScoreBoard::campaign(20);
        late_score_board.lives = 3;
        assert_eq!(
            level_clear_banner_text(135, &late_score_board),
            [
                "STAGE 99".to_string(),
                "LEVEL CLEAR".to_string(),
                "100X00 200X00".to_string(),
                "300X00 400X00".to_string(),
                "TOTAL 00".to_string(),
                "BONUS 3000".to_string()
            ]
        );
    }

    #[test]
    fn versus_intro_banner_uses_selected_arena_label() {
        let status = GameStatus {
            phase: GamePhase::StageIntro,
            arena: 4,
            ..GameStatus::default()
        };

        assert!(!status.is_playing());
        assert_eq!(
            phase_banner_text(
                &status,
                GameMode::VersusDeathmatch,
                &ScoreBoard::versus(3, 5, 2.0)
            )
            .expect("arena intro should show a banner"),
            [
                "ARENA 04".to_string(),
                "DUEL".to_string(),
                "READY".to_string()
            ]
        );
        assert_eq!(
            arena_intro_banner_text(135, GameMode::VersusBaseBattle),
            [
                "ARENA 99".to_string(),
                "BASE BATTLE".to_string(),
                "READY".to_string()
            ]
        );
    }

    #[test]
    fn arena_labels_distinguish_deathmatch_and_base_battle() {
        let duel = parse_arena(ARENA_1).expect("arena should parse");
        let base = parse_arena(ARENA_5).expect("arena should parse");

        assert_eq!(battle_kind_label(duel.battle_rules), "DUEL");
        assert_eq!(battle_kind_label(base.battle_rules), "BASE");
        assert_eq!(arena_intro_kind_label(GameMode::VersusDeathmatch), "DUEL");
        assert_eq!(
            arena_intro_kind_label(GameMode::VersusBaseBattle),
            "BASE BATTLE"
        );
    }

    #[test]
    fn terminal_phase_banners_show_restart_or_menu_hint() {
        for lines in [
            phase_banner_lines(GamePhase::GameOver, None).expect("game over should show a banner"),
            phase_banner_lines(GamePhase::RoundOver, Some(PlayerId::One))
                .expect("p1 win should show a banner"),
            phase_banner_lines(GamePhase::RoundOver, Some(PlayerId::Two))
                .expect("p2 win should show a banner"),
            phase_banner_lines(GamePhase::Victory, None).expect("victory should show a banner"),
        ] {
            assert!(
                lines.contains(&"PRESS R OR M"),
                "terminal phase banner should explain how to continue"
            );
        }
    }

    #[test]
    fn paused_banner_shows_resume_restart_and_menu_hints() {
        let lines = phase_banner_lines(GamePhase::Paused, None).expect("paused should show banner");
        assert!(lines.contains(&"ESC RESUME"));
        assert!(lines.contains(&"R RESTART"));
        assert!(lines.contains(&"M MENU"));
    }

    #[test]
    fn phase_banner_text_uses_available_pixel_glyphs() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        let statuses = [
            (
                GameStatus {
                    phase: GamePhase::StageIntro,
                    stage: 35,
                    ..GameStatus::default()
                },
                GameMode::Campaign,
            ),
            (
                GameStatus {
                    phase: GamePhase::StageIntro,
                    arena: 4,
                    ..GameStatus::default()
                },
                GameMode::VersusDeathmatch,
            ),
            (
                GameStatus {
                    phase: GamePhase::StageIntro,
                    arena: 5,
                    ..GameStatus::default()
                },
                GameMode::VersusBaseBattle,
            ),
            (
                GameStatus {
                    phase: GamePhase::Paused,
                    ..GameStatus::default()
                },
                GameMode::Campaign,
            ),
            (
                GameStatus {
                    phase: GamePhase::GameOver,
                    ..GameStatus::default()
                },
                GameMode::Campaign,
            ),
            (
                GameStatus {
                    phase: GamePhase::LevelClear,
                    ..GameStatus::default()
                },
                GameMode::Campaign,
            ),
            (
                GameStatus {
                    phase: GamePhase::RoundOver,
                    winner: Some(PlayerId::One),
                    ..GameStatus::default()
                },
                GameMode::VersusDeathmatch,
            ),
            (
                GameStatus {
                    phase: GamePhase::RoundOver,
                    winner: Some(PlayerId::Two),
                    ..GameStatus::default()
                },
                GameMode::VersusDeathmatch,
            ),
            (
                GameStatus {
                    phase: GamePhase::Victory,
                    ..GameStatus::default()
                },
                GameMode::Campaign,
            ),
        ];

        for (status, mode) in statuses {
            let score_board = ScoreBoard::campaign(3);
            let lines =
                phase_banner_text(&status, mode, &score_board).expect("phase should show a banner");
            for line in lines {
                assert!(phase_text_width(&line) > 0.0);
                for ch in line.chars().filter(|ch| *ch != ' ') {
                    assert_manifest_glyph_is_visible(&manifest, ch);
                }
            }
        }
    }

    #[test]
    fn game_starts_at_mode_select() {
        assert_eq!(GameStatus::default().phase, GamePhase::ModeSelect);
        assert_eq!(ModeSelect::default().stage, 1);
        assert_eq!(GameStatus::default().arena, DEFAULT_VERSUS_ARENA);
    }

    #[test]
    fn mode_select_cycles_campaign_battle_music_and_sound() {
        assert_eq!(
            next_mode_select_option(ModeSelectOption::Campaign),
            ModeSelectOption::Battle
        );
        assert_eq!(
            next_mode_select_option(ModeSelectOption::Battle),
            ModeSelectOption::Music
        );
        assert_eq!(
            next_mode_select_option(ModeSelectOption::Music),
            ModeSelectOption::Sound
        );
        assert_eq!(
            next_mode_select_option(ModeSelectOption::Sound),
            ModeSelectOption::Campaign
        );
        assert_eq!(
            previous_mode_select_option(ModeSelectOption::Campaign),
            ModeSelectOption::Sound
        );
        assert_eq!(
            previous_mode_select_option(ModeSelectOption::Sound),
            ModeSelectOption::Music
        );
        assert_eq!(
            GameMode::VersusBaseBattle.mode_select_option(),
            ModeSelectOption::Battle
        );
    }

    #[test]
    fn mode_select_arena_selection_wraps_authored_arenas() {
        assert_eq!(ModeSelect::default().arena, DEFAULT_VERSUS_ARENA);
        assert_eq!(next_arena(1), 2);
        assert_eq!(next_arena(2), 3);
        assert_eq!(next_arena(3), 4);
        assert_eq!(next_arena(4), 5);
        assert_eq!(next_arena(5), 6);
        assert_eq!(next_arena(6), 1);
        assert_eq!(previous_arena(1), 6);
        assert_eq!(previous_arena(2), 1);
        assert_eq!(previous_arena(3), 2);
        assert_eq!(previous_arena(4), 3);
        assert_eq!(previous_arena(5), 4);
        assert_eq!(previous_arena(6), 5);
    }

    #[test]
    fn mode_select_stage_selection_wraps_authored_campaign() {
        assert_eq!(ModeSelect::default().stage, 1);
        assert_eq!(next_stage(1), 2);
        assert_eq!(next_stage(LEVEL_COUNT - 1), LEVEL_COUNT);
        assert_eq!(next_stage(LEVEL_COUNT), 1);
        assert_eq!(previous_stage(1), LEVEL_COUNT);
        assert_eq!(previous_stage(2), 1);
        assert_eq!(previous_stage(LEVEL_COUNT), LEVEL_COUNT - 1);
    }

    #[test]
    fn selected_campaign_stage_clamps_to_authored_campaign_range() {
        let mut mode_select = ModeSelect {
            stage: 12,
            ..ModeSelect::default()
        };
        assert_eq!(selected_campaign_stage(&mode_select), 12);

        mode_select.stage = 0;
        assert_eq!(selected_campaign_stage(&mode_select), 1);

        mode_select.stage = LEVEL_COUNT + 5;
        assert_eq!(selected_campaign_stage(&mode_select), LEVEL_COUNT);
    }

    #[test]
    fn mode_select_cursor_tracks_selected_option() {
        let campaign = mode_select_cursor_translation(ModeSelectOption::Campaign);
        let battle = mode_select_cursor_translation(ModeSelectOption::Battle);
        let music = mode_select_cursor_translation(ModeSelectOption::Music);
        let sound = mode_select_cursor_translation(ModeSelectOption::Sound);
        assert_eq!(campaign.x, battle.x);
        assert!(campaign.y > battle.y);
        assert!(battle.y > music.y);
        assert!(music.y > sound.y);
    }

    #[test]
    fn mode_select_hints_fit_and_use_available_pixel_glyphs() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        assert_eq!(
            MODE_SELECT_HINT_LINES,
            ["WS ARROWS SELECT", "AD ARROWS CHANGE", "SPACE ENTER START"]
        );
        for line in [
            "STAGE", "ARENA", "37", "BASE", "DUEL", "MUSIC", "BGM", "CLASSIC", "SOUND", "ON", "OFF",
        ]
        .into_iter()
        .chain(MODE_SELECT_HINT_LINES)
        {
            assert!(
                phase_text_width(line) <= 208.0,
                "mode select text should fit in the playfield"
            );
            for ch in line.chars().filter(|ch| *ch != ' ') {
                assert_manifest_glyph_is_visible(&manifest, ch);
            }
        }
    }

    #[test]
    fn direction_priority_uses_most_recent_pressed_direction() {
        let mut priority = Vec::new();

        record_direction_press(&mut priority, Direction::Down);
        record_direction_press(&mut priority, Direction::Right);
        record_direction_press(&mut priority, Direction::Up);

        assert_eq!(preferred_direction(&priority), Some(Direction::Up));

        prune_direction_priority(&mut priority, |direction| {
            matches!(direction, Direction::Down | Direction::Right)
        });

        assert_eq!(preferred_direction(&priority), Some(Direction::Right));
    }

    #[test]
    fn direction_priority_repress_moves_direction_to_latest_slot() {
        let mut priority = Vec::new();

        record_direction_press(&mut priority, Direction::Left);
        record_direction_press(&mut priority, Direction::Right);
        record_direction_press(&mut priority, Direction::Left);

        assert_eq!(priority, [Direction::Right, Direction::Left]);
        assert_eq!(preferred_direction(&priority), Some(Direction::Left));
    }

    #[test]
    fn deathmatch_winner_requires_target_score_or_zero_lives() {
        assert_eq!(deathmatch_winner_after_hit(4, 2, 5, PlayerId::One), None);
        assert_eq!(
            deathmatch_winner_after_hit(5, 2, 5, PlayerId::One),
            Some(PlayerId::One)
        );
        assert_eq!(
            deathmatch_winner_after_hit(2, 0, 5, PlayerId::Two),
            Some(PlayerId::Two)
        );
    }

    #[test]
    fn enemy_scores_match_spec() {
        assert_eq!(enemy_score(EnemyKind::Basic), 100);
        assert_eq!(enemy_score(EnemyKind::Fast), 200);
        assert_eq!(enemy_score(EnemyKind::Power), 300);
        assert_eq!(enemy_score(EnemyKind::Armor), 400);
    }

    #[test]
    fn enemy_hit_sound_distinguishes_armor_hits_from_kills() {
        assert_eq!(enemy_hit_sound(2), SoundKind::SteelHit);
        assert_eq!(enemy_hit_sound(1), SoundKind::SteelHit);
        assert_eq!(enemy_hit_sound(0), SoundKind::TankExplosion);
        assert_eq!(enemy_hit_sound(-1), SoundKind::TankExplosion);
    }

    #[test]
    fn power_enemies_fire_faster_bullets() {
        assert_eq!(enemy_bullet_speed(EnemyKind::Basic), BULLET_SPEED);
        assert_eq!(enemy_bullet_speed(EnemyKind::Fast), BULLET_SPEED);
        assert_eq!(enemy_bullet_speed(EnemyKind::Armor), BULLET_SPEED);
        assert_eq!(
            enemy_bullet_speed(EnemyKind::Power),
            POWER_ENEMY_BULLET_SPEED
        );
    }

    #[test]
    fn tank_sprite_indices_separate_players_and_enemy_kind_animation_frames() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::Player1, Direction::Up, 0),
            0
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::Player1, Direction::Up, 1),
            4
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::Player2, Direction::Up, 0),
            8
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::Player2, Direction::Up, 1),
            12
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::EnemyBasic, Direction::Up, 0),
            16
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::EnemyBasic, Direction::Up, 99),
            20
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::EnemyFast, Direction::Up, 0),
            24
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::EnemyPower, Direction::Up, 0),
            32
        );
        assert_eq!(
            animated_tank_sprite_index(&manifest, TankSpriteSet::EnemyArmor, Direction::Up, 0),
            40
        );
        assert_eq!(
            TankSpriteSet::enemy(EnemyKind::Fast),
            TankSpriteSet::EnemyFast
        );
        assert_eq!(
            TankSpriteSet::enemy(EnemyKind::Power),
            TankSpriteSet::EnemyPower
        );
        assert_eq!(
            TankSpriteSet::enemy(EnemyKind::Armor),
            TankSpriteSet::EnemyArmor
        );
    }

    #[test]
    fn enemy_visual_feedback_marks_carriers_and_damaged_armor() {
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Basic, Some(PowerUpKind::Star), 1, 0.05),
            [248, 232, 96]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Basic, Some(PowerUpKind::Star), 1, 0.20),
            [255, 255, 255]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Fast, None, 1, 0.20),
            [112, 216, 128]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Power, None, 1, 0.20),
            [248, 112, 112]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Armor, None, 3, 0.20),
            [168, 184, 216]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Armor, None, 2, 0.20),
            [216, 96, 72]
        );
        assert_eq!(
            enemy_visual_rgb(EnemyKind::Armor, None, 1, 0.20),
            [248, 168, 88]
        );
    }

    #[test]
    fn spawn_protection_expires_after_spawn_shimmer() {
        let frames = SpriteFrameRange { first: 4, last: 7 };
        let duration = spawn_shimmer_duration_secs(frames);
        let mut protection = SpawnProtection::for_spawn_shimmer(frames);

        assert_eq!(duration, SPAWN_SHIMMER_FRAME_SECONDS * 4.0);
        assert!(!protection.tick(Duration::from_secs_f32(
            duration - SPAWN_SHIMMER_FRAME_SECONDS / 2.0
        )));
        assert!(protection.tick(Duration::from_secs_f32(SPAWN_SHIMMER_FRAME_SECONDS)));
    }

    #[test]
    fn player_respawn_delay_expires_after_spawn_shimmer() {
        let frames = SpriteFrameRange { first: 4, last: 7 };
        let duration = spawn_shimmer_duration_secs(frames);
        let mut delay = PlayerRespawnDelay::for_spawn_shimmer(frames);

        assert_eq!(duration, SPAWN_SHIMMER_FRAME_SECONDS * 4.0);
        assert!(!delay.tick(Duration::from_secs_f32(
            duration - SPAWN_SHIMMER_FRAME_SECONDS / 2.0
        )));
        assert!(delay.tick(Duration::from_secs_f32(SPAWN_SHIMMER_FRAME_SECONDS)));
    }

    #[test]
    fn explosion_duration_matches_animation_frames() {
        assert_eq!(
            explosion_duration_secs(SpriteFrameRange { first: 0, last: 3 }),
            EXPLOSION_FRAME_SECONDS * 4.0
        );
    }

    #[test]
    fn destroyed_tank_stays_until_explosion_finishes() {
        let frames = SpriteFrameRange { first: 0, last: 3 };
        let mut destroyed_tank = DestroyedTank::for_explosion(frames);

        assert!(!destroyed_tank.tick(Duration::from_secs_f32(
            explosion_duration_secs(frames) - 0.01
        )));
        assert!(destroyed_tank.tick(Duration::from_secs_f32(0.02)));
    }

    #[test]
    fn parked_tank_translation_is_off_board() {
        let top_left = board_top_left_from_translation(parked_tank_translation(), TANK_SIZE);

        assert_eq!(top_left, parked_tank_top_left());
        assert!(top_left.x + TANK_SIZE < 0.0);
        assert!(top_left.y + TANK_SIZE < 0.0);
    }

    #[test]
    fn pending_player_respawn_waits_for_explosion() {
        let frames = SpriteFrameRange { first: 0, last: 3 };
        let mut pending_respawn = PlayerRespawnPending::for_explosion(frames);

        assert!(!pending_respawn.tick(Duration::from_secs_f32(
            explosion_duration_secs(frames) - 0.01
        )));
        assert!(pending_respawn.tick(Duration::from_secs_f32(0.02)));
    }

    #[test]
    fn initial_player_spawn_starts_with_invulnerability_shield() {
        let mut app = App::new();
        app.insert_resource(test_sprite_assets());
        app.add_systems(Update, spawn_player_with_initial_shield_for_test);

        app.update();

        let mut players = app.world_mut().query::<(&Player, &Tank, &Shield)>();
        let spawned: Vec<(PlayerId, Vec2, f32)> = players
            .iter(app.world())
            .map(|(player, tank, shield)| (player.id, tank.top_left, shield.timer.remaining_secs()))
            .collect();

        assert_eq!(spawned.len(), 1);
        assert_eq!(spawned[0].0, PlayerId::One);
        assert_eq!(spawned[0].1, Vec2::new(64.0, 192.0));
        assert!(
            (spawned[0].2 - TEST_SPAWN_INVULNERABILITY_SECONDS).abs() <= f32::EPSILON,
            "initial shield should use the configured spawn invulnerability"
        );
    }

    #[test]
    fn player_respawn_system_waits_until_respawn_point_is_clear() {
        let mut app = App::new();
        let respawn_top_left = Vec2::new(64.0, 192.0);
        let frames = SpriteFrameRange { first: 0, last: 3 };
        let mut pending_respawn = PlayerRespawnPending::for_explosion(frames);
        assert!(pending_respawn.tick(Duration::from_secs_f32(explosion_duration_secs(frames))));

        app.insert_resource(Time::<()>::default());
        app.insert_resource(test_sprite_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(ScoreBoard::versus(3, 5, 1.5));

        let player_entity = app
            .world_mut()
            .spawn((
                Player { id: PlayerId::One },
                PlayerUpgrade { level: 3 },
                RespawnPoint {
                    top_left: respawn_top_left,
                    facing: Direction::Up,
                },
                pending_respawn,
                Transform::default(),
                Sprite {
                    color: player_upgrade_visual_color(3),
                    ..default()
                },
                TankSpriteState::new(TankSpriteSet::Player1),
            ))
            .id();
        let blocker = app
            .world_mut()
            .spawn(Tank {
                top_left: respawn_top_left,
                facing: Direction::Down,
                speed: 0.0,
            })
            .id();
        app.add_systems(Update, tick_player_respawns);

        app.update();

        assert!(
            app.world()
                .get::<PlayerRespawnPending>(player_entity)
                .is_some()
        );
        assert!(app.world().get::<Tank>(player_entity).is_none());

        app.world_mut().entity_mut(blocker).despawn();
        app.update();

        let tank = app
            .world()
            .get::<Tank>(player_entity)
            .expect("player should respawn once the spawn point clears");
        assert_eq!(tank.top_left, respawn_top_left);
        assert_eq!(tank.facing, Direction::Up);
        assert!(
            app.world()
                .get::<PlayerRespawnPending>(player_entity)
                .is_none()
        );
        assert!(
            app.world()
                .get::<PlayerRespawnDelay>(player_entity)
                .is_some()
        );
        assert!(app.world().get::<Shield>(player_entity).is_some());
        assert_eq!(
            app.world()
                .get::<PlayerUpgrade>(player_entity)
                .expect("player upgrade should remain present")
                .level,
            0
        );
    }

    #[test]
    fn parked_destroyed_tank_is_outside_the_battlefield() {
        let top_left = parked_tank_top_left();
        assert!(top_left.x + TANK_SIZE < 0.0);
        assert!(top_left.y + TANK_SIZE < 0.0);
    }

    #[test]
    fn player_upgrade_visuals_show_star_power_level() {
        assert_eq!(player_upgrade_visual_rgb(0), [255, 255, 255]);
        assert_eq!(player_upgrade_visual_rgb(1), [184, 248, 184]);
        assert_eq!(player_upgrade_visual_rgb(2), [255, 232, 104]);
        assert_eq!(player_upgrade_visual_rgb(3), [255, 176, 104]);
        assert_eq!(player_upgrade_visual_rgb(99), [255, 176, 104]);
    }

    #[test]
    fn helmet_flicker_returns_to_upgrade_visual_between_flashes() {
        assert_eq!(player_shield_visual_rgb(0.05, 2), [160, 220, 255]);
        assert_eq!(player_shield_visual_rgb(0.15, 2), [255, 232, 104]);
    }

    #[test]
    fn player_respawn_resets_star_upgrade_visuals() {
        let mut upgrade = PlayerUpgrade { level: 3 };
        let mut sprite = Sprite {
            color: player_upgrade_visual_color(upgrade.level),
            ..default()
        };

        reset_player_upgrade(&mut upgrade, &mut sprite);

        assert_eq!(upgrade.level, 0);
        assert_eq!(sprite.color, player_upgrade_visual_color(0));
    }

    #[test]
    fn spawn_protection_visual_overrides_enemy_feedback_temporarily() {
        assert_eq!(
            enemy_display_rgb(
                EnemyKind::Armor,
                Some(PowerUpKind::Star),
                1,
                0.02,
                true,
                false
            ),
            [160, 220, 255]
        );
        assert_eq!(
            enemy_display_rgb(
                EnemyKind::Armor,
                Some(PowerUpKind::Star),
                1,
                0.10,
                true,
                false
            ),
            [248, 232, 96]
        );
    }

    #[test]
    fn clock_freeze_visual_tints_enemies_blue() {
        assert_eq!(enemy_frozen_visual_rgb(0.05), [136, 216, 255]);
        assert_eq!(enemy_frozen_visual_rgb(0.18), [216, 248, 255]);
        assert_eq!(
            enemy_display_rgb(
                EnemyKind::Armor,
                Some(PowerUpKind::Star),
                1,
                0.18,
                false,
                true
            ),
            [216, 248, 255]
        );
    }

    #[test]
    fn player_bullet_limit_increases_after_star_upgrades() {
        assert_eq!(player_bullet_limit(0), 1);
        assert_eq!(player_bullet_limit(1), 1);
        assert_eq!(player_bullet_limit(2), 2);
        assert_eq!(player_bullet_limit(3), 2);
    }

    #[test]
    fn player_bullet_speed_increases_after_first_star_upgrade() {
        assert_eq!(player_bullet_speed(0), BULLET_SPEED);
        assert_eq!(player_bullet_speed(1), PLAYER_FAST_BULLET_SPEED);
        assert_eq!(player_bullet_speed(3), PLAYER_FAST_BULLET_SPEED);
    }

    #[test]
    fn player_steel_breaking_requires_full_upgrade_and_stage_rule() {
        let disabled = StageRules::default();
        let enabled = StageRules {
            player_steel_destruction: true,
        };
        assert!(!player_bullets_break_steel(2, enabled));
        assert!(!player_bullets_break_steel(3, disabled));
        assert!(player_bullets_break_steel(3, enabled));
    }

    #[test]
    fn bullet_spawns_from_center_front_of_tank() {
        let tank_top_left = Vec2::new(64.0, 80.0);

        assert_eq!(
            spawn_bullet_position(tank_top_left, Direction::Up),
            Vec2::new(70.0, 76.0)
        );
        assert_eq!(
            spawn_bullet_position(tank_top_left, Direction::Down),
            Vec2::new(70.0, 96.0)
        );
        assert_eq!(
            spawn_bullet_position(tank_top_left, Direction::Left),
            Vec2::new(60.0, 86.0)
        );
        assert_eq!(
            spawn_bullet_position(tank_top_left, Direction::Right),
            Vec2::new(80.0, 86.0)
        );
    }

    #[test]
    fn player_fire_system_uses_upgrade_stats_for_spawned_bullet() {
        let mut app = App::new();
        let tank_top_left = Vec2::new(64.0, 80.0);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);

        app.insert_resource(keys);
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(StageRules {
            player_steel_destruction: true,
        });
        app.insert_resource(VersusPlayerFreeze::default());
        app.world_mut().spawn((
            Tank {
                top_left: tank_top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            PlayerUpgrade { level: 3 },
            Player { id: PlayerId::One },
        ));
        app.add_systems(Update, fire_player_bullet);

        app.update();

        let expected_top_left = spawn_bullet_position(tank_top_left, Direction::Up);
        let mut bullets = app.world_mut().query::<&Bullet>();
        let bullets: Vec<_> = bullets.iter(app.world()).collect();
        assert_eq!(bullets.len(), 1);
        let bullet = bullets[0];
        assert_eq!(bullet.previous_top_left, expected_top_left);
        assert_eq!(bullet.top_left, expected_top_left);
        assert_eq!(bullet.facing, Direction::Up);
        assert_eq!(bullet.owner, Team::Player1);
        assert_eq!(bullet.speed, PLAYER_FAST_BULLET_SPEED);
        assert!(bullet.breaks_steel);
        assert!(!bullet.resolved);
    }

    #[test]
    fn player_fire_system_treats_held_fire_as_ready_input() {
        let mut app = App::new();
        let tank_top_left = Vec2::new(64.0, 80.0);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);
        keys.clear_just_pressed(KeyCode::Space);

        app.insert_resource(keys);
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(StageRules::default());
        app.insert_resource(VersusPlayerFreeze::default());
        spawn_test_player(app.world_mut(), PlayerId::One, tank_top_left, 3);
        app.add_systems(Update, fire_player_bullet);

        app.update();

        let mut bullets = app.world_mut().query::<&Bullet>();
        let bullets: Vec<_> = bullets.iter(app.world()).collect();
        assert_eq!(bullets.len(), 1);
        assert_eq!(bullets[0].owner, Team::Player1);
        assert_eq!(bullets[0].speed, BULLET_SPEED);
    }

    #[test]
    fn player_fire_system_respects_upgrade_bullet_limit() {
        let mut app = App::new();
        let tank_top_left = Vec2::new(64.0, 80.0);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);

        app.insert_resource(keys);
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(StageRules {
            player_steel_destruction: true,
        });
        app.insert_resource(VersusPlayerFreeze::default());
        app.world_mut().spawn((
            Tank {
                top_left: tank_top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            PlayerUpgrade { level: 3 },
            Player { id: PlayerId::One },
        ));
        app.world_mut().spawn(Bullet {
            previous_top_left: Vec2::new(8.0, 8.0),
            top_left: Vec2::new(8.0, 8.0),
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        });
        app.world_mut().spawn(Bullet {
            previous_top_left: Vec2::new(24.0, 8.0),
            top_left: Vec2::new(24.0, 8.0),
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        });
        app.add_systems(Update, fire_player_bullet);

        app.update();

        let mut bullets = app.world_mut().query::<&Bullet>();
        assert_eq!(bullets.iter(app.world()).count(), 2);
    }

    #[test]
    fn bullet_tile_destruction_respects_steel_breaking_flag() {
        assert!(bullet_destroys_tile(TileKind::Brick, false));
        assert!(!bullet_destroys_tile(TileKind::Steel, false));
        assert!(bullet_destroys_tile(TileKind::Steel, true));
        assert!(!bullet_destroys_tile(TileKind::Base, true));
    }

    #[test]
    fn bullet_tile_hit_uses_end_tile_for_normal_steps() {
        let mut grid = TileGrid::empty();
        grid.set(3, 1, TileKind::Steel);

        let hit = bullet_blocking_tile_hit(&grid, Vec2::new(20.0, 8.0), Vec2::new(24.0, 8.0))
            .expect("bullet should hit steel at the end tile");

        assert_eq!(hit.x, 3);
        assert_eq!(hit.y, 1);
        assert_eq!(hit.tile, TileKind::Steel);
        assert_eq!(hit.impact_top_left, Vec2::new(24.0, 8.0));
    }

    #[test]
    fn bullet_tile_hit_sweeps_between_fast_steps() {
        let mut grid = TileGrid::empty();
        grid.set(3, 1, TileKind::Brick);
        grid.set(4, 1, TileKind::Steel);

        let hit = bullet_blocking_tile_hit(&grid, Vec2::new(8.0, 8.0), Vec2::new(36.0, 8.0))
            .expect("bullet should hit the first blocking tile it crosses");

        assert_eq!(hit.x, 3);
        assert_eq!(hit.y, 1);
        assert_eq!(hit.tile, TileKind::Brick);
        assert_eq!(hit.impact_top_left, Vec2::new(24.0, 8.0));
    }

    #[test]
    fn bullet_tile_hit_uses_bullet_rect_for_grazing_bricks() {
        let mut grid = TileGrid::empty();
        grid.set(3, 1, TileKind::Brick);

        let hit = bullet_blocking_tile_hit(&grid, Vec2::new(20.0, 5.0), Vec2::new(36.0, 5.0))
            .expect("bullet rect should graze and hit the brick");

        assert_eq!(hit.x, 3);
        assert_eq!(hit.y, 1);
        assert_eq!(hit.tile, TileKind::Brick);
        assert_eq!(hit.impact_top_left, Vec2::new(24.0, 5.0));
    }

    #[test]
    fn bullet_tile_hit_prefers_side_wall_before_base_when_moving_left() {
        let mut grid = TileGrid::empty();
        grid.set(13, 24, TileKind::Base);
        grid.set(14, 24, TileKind::Brick);

        let hit = bullet_blocking_tile_hit(&grid, Vec2::new(115.0, 192.0), Vec2::new(111.0, 192.0))
            .expect("bullet should hit the wall before the base");

        assert_eq!(hit.x, 14);
        assert_eq!(hit.y, 24);
        assert_eq!(hit.tile, TileKind::Brick);
        assert_eq!(hit.impact_top_left, Vec2::new(111.0, 192.0));
    }

    #[test]
    fn bullet_tile_hit_prefers_bottom_wall_before_base_when_moving_up() {
        let mut grid = TileGrid::empty();
        grid.set(12, 24, TileKind::Base);
        grid.set(12, 25, TileKind::Brick);

        let hit = bullet_blocking_tile_hit(&grid, Vec2::new(96.0, 203.0), Vec2::new(96.0, 199.0))
            .expect("bullet should hit the wall before the base");

        assert_eq!(hit.x, 12);
        assert_eq!(hit.y, 25);
        assert_eq!(hit.tile, TileKind::Brick);
        assert_eq!(hit.impact_top_left, Vec2::new(96.0, 199.0));
    }

    #[test]
    fn bullet_tank_hit_uses_end_rect_for_normal_steps() {
        assert_eq!(
            bullet_tank_hit(
                Vec2::new(20.0, 8.0),
                Vec2::new(24.0, 8.0),
                Vec2::new(24.0, 0.0)
            ),
            Some(Vec2::new(24.0, 8.0))
        );
    }

    #[test]
    fn bullet_tank_hit_sweeps_between_fast_steps() {
        assert_eq!(
            bullet_tank_hit(
                Vec2::new(8.0, 8.0),
                Vec2::new(52.0, 8.0),
                Vec2::new(32.0, 0.0)
            ),
            Some(Vec2::new(32.0, 8.0))
        );
    }

    #[test]
    fn bullet_tank_hit_ignores_missed_lanes() {
        assert_eq!(
            bullet_tank_hit(
                Vec2::new(8.0, 24.0),
                Vec2::new(52.0, 24.0),
                Vec2::new(32.0, 0.0)
            ),
            None
        );
    }

    #[test]
    fn bullet_tank_hit_is_blocked_by_earlier_tile_hit() {
        let mut grid = TileGrid::empty();
        let start = Vec2::new(8.0, 8.0);
        let end = Vec2::new(52.0, 8.0);
        grid.set(3, 1, TileKind::Brick);

        let tile_hit = bullet_blocking_tile_hit(&grid, start, end);
        let tank_hit =
            bullet_tank_hit(start, end, Vec2::new(32.0, 0.0)).expect("tank is later on the path");

        assert_eq!(
            tile_hit
                .expect("brick should be first blocking tile")
                .impact_top_left,
            Vec2::new(24.0, 8.0)
        );
        assert_eq!(tank_hit, Vec2::new(32.0, 8.0));
        assert!(!bullet_hit_is_before_tile(start, tank_hit, tile_hit));
    }

    #[test]
    fn bullet_tank_hit_is_blocked_by_grazed_tile_hit() {
        let mut grid = TileGrid::empty();
        let start = Vec2::new(20.0, 5.0);
        let end = Vec2::new(52.0, 5.0);
        grid.set(3, 1, TileKind::Brick);

        let tile_hit = bullet_blocking_tile_hit(&grid, start, end);
        let tank_hit =
            bullet_tank_hit(start, end, Vec2::new(32.0, 0.0)).expect("tank is later on the path");

        assert_eq!(
            tile_hit
                .expect("grazed brick should be the first blocking tile")
                .impact_top_left,
            Vec2::new(24.0, 5.0)
        );
        assert_eq!(tank_hit, Vec2::new(32.0, 5.0));
        assert!(!bullet_hit_is_before_tile(start, tank_hit, tile_hit));
    }

    #[test]
    fn bullet_tank_hit_beats_later_tile_hit() {
        let mut grid = TileGrid::empty();
        let start = Vec2::new(8.0, 8.0);
        let end = Vec2::new(52.0, 8.0);
        grid.set(6, 1, TileKind::Steel);

        let tile_hit = bullet_blocking_tile_hit(&grid, start, end);
        let tank_hit =
            bullet_tank_hit(start, end, Vec2::new(24.0, 0.0)).expect("tank is earlier on the path");

        assert_eq!(
            tile_hit
                .expect("steel should be the later blocking tile")
                .impact_top_left,
            Vec2::new(48.0, 8.0)
        );
        assert_eq!(tank_hit, Vec2::new(24.0, 8.0));
        assert!(bullet_hit_is_before_tile(start, tank_hit, tile_hit));
    }

    #[test]
    fn enemy_director_uses_level_powerup_carrier_markers() {
        let level = parse_level(LEVEL_5).expect("level should parse");
        let director = EnemyDirector::from_level(&level);
        let carriers: Vec<_> = director
            .roster
            .iter()
            .enumerate()
            .filter_map(|(index, enemy)| enemy.carried_powerup.map(|kind| (index + 1, kind)))
            .collect();

        assert_eq!(
            carriers,
            [
                (3, PowerUpKind::Star),
                (8, PowerUpKind::Helmet),
                (13, PowerUpKind::Shovel),
                (18, PowerUpKind::Tank),
            ]
        );
    }

    #[test]
    fn level_rejects_invalid_powerup_carrier_markers() {
        let duplicate =
            LEVEL_1.replacen("(enemy: 10, kind: Helmet)", "(enemy: 5, kind: Helmet)", 1);
        assert!(
            parse_level(&duplicate)
                .err()
                .expect("duplicate carrier should fail")
                .contains("configured more than once")
        );

        let out_of_range = LEVEL_1.replacen(
            "(enemy: 20, kind: Grenade)",
            "(enemy: 21, kind: Grenade)",
            1,
        );
        assert!(
            parse_level(&out_of_range)
                .err()
                .expect("out-of-range carrier should fail")
                .contains("outside the 1..=20 roster")
        );
    }

    #[test]
    fn powerup_cycle_covers_classic_powerups() {
        assert_eq!(powerup_for_cycle(0), PowerUpKind::Star);
        assert_eq!(powerup_for_cycle(1), PowerUpKind::Helmet);
        assert_eq!(powerup_for_cycle(2), PowerUpKind::Clock);
        assert_eq!(powerup_for_cycle(3), PowerUpKind::Grenade);
        assert_eq!(powerup_for_cycle(4), PowerUpKind::Shovel);
        assert_eq!(powerup_for_cycle(5), PowerUpKind::Tank);
        assert_eq!(powerup_for_cycle(6), PowerUpKind::Star);
    }

    #[test]
    fn tank_powerup_caps_campaign_lives_to_status_digit() {
        let top_left = Vec2::new(64.0, 64.0);
        let mut score_board = ScoreBoard::campaign(20);
        score_board.lives = MAX_PLAYER_LIVES;
        let mut app = powerup_pickup_app(GameMode::Campaign, score_board);

        spawn_test_player(app.world_mut(), PlayerId::One, top_left, MAX_PLAYER_LIVES);
        spawn_test_powerup(app.world_mut(), PowerUpKind::Tank, top_left);

        app.update();

        let score_board = app.world().resource::<ScoreBoard>();
        assert_eq!(score_board.lives, MAX_PLAYER_LIVES);
        assert_eq!(
            status_value_text(
                StatusValue::Lives,
                GameMode::Campaign,
                &GameStatus::default(),
                score_board,
            ),
            "9"
        );
        let mut players = app.world_mut().query::<&PlayerLives>();
        let lives: Vec<i32> = players
            .iter(app.world())
            .map(|lives| lives.current)
            .collect();
        assert_eq!(lives, [MAX_PLAYER_LIVES]);
    }

    #[test]
    fn tank_powerup_updates_versus_collector_lives() {
        let top_left = Vec2::new(96.0, 64.0);
        let mut score_board = ScoreBoard::versus(3, 5, 2.0);
        score_board.set_player_lives(PlayerId::Two, MAX_PLAYER_LIVES - 1);
        let mut app = powerup_pickup_app(GameMode::VersusDeathmatch, score_board);

        spawn_test_player(
            app.world_mut(),
            PlayerId::Two,
            top_left,
            MAX_PLAYER_LIVES - 1,
        );
        spawn_test_powerup(app.world_mut(), PowerUpKind::Tank, top_left);

        app.update();

        let score_board = app.world().resource::<ScoreBoard>();
        assert_eq!(score_board.p1_lives, 3);
        assert_eq!(score_board.p2_lives, MAX_PLAYER_LIVES);
        assert_eq!(
            status_value_text(
                StatusValue::P2Lives,
                GameMode::VersusDeathmatch,
                &GameStatus::default(),
                score_board,
            ),
            "9"
        );
        let mut players = app.world_mut().query::<&PlayerLives>();
        let lives: Vec<i32> = players
            .iter(app.world())
            .map(|lives| lives.current)
            .collect();
        assert_eq!(lives, [MAX_PLAYER_LIVES]);
    }

    #[test]
    fn grenade_drops_powerup_from_destroyed_visible_carrier() {
        let mut app = App::new();
        let carrier_top_left = Vec2::new(64.0, 64.0);

        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(ScoreBoard::campaign(20));
        app.world_mut().spawn((
            Tank {
                top_left: Vec2::new(32.0, 32.0),
                facing: Direction::Down,
                speed: 0.0,
            },
            Transform::default(),
            EnemyTank {
                kind: EnemyKind::Basic,
                carried_powerup: None,
            },
        ));
        app.world_mut().spawn((
            Tank {
                top_left: carrier_top_left,
                facing: Direction::Down,
                speed: 0.0,
            },
            Transform::default(),
            EnemyTank {
                kind: EnemyKind::Power,
                carried_powerup: Some(PowerUpKind::Helmet),
            },
        ));
        app.add_systems(Update, grenade_visible_enemies_for_test);

        app.update();

        let score_board = app.world().resource::<ScoreBoard>();
        assert_eq!(score_board.enemies_destroyed, 2);
        assert_eq!(score_board.score, 400);

        let mut powerups = app.world_mut().query::<(&PowerUp, &Transform)>();
        let drops: Vec<(PowerUpKind, Vec2)> = powerups
            .iter(app.world())
            .map(|(powerup, transform)| {
                (
                    powerup.kind,
                    board_top_left_from_translation(transform.translation, TANK_SIZE),
                )
            })
            .collect();

        assert_eq!(drops, [(PowerUpKind::Helmet, carrier_top_left)]);
    }

    #[test]
    fn versus_grenade_respects_target_shield() {
        let mut app = App::new();
        let p1_top_left = Vec2::new(64.0, 64.0);
        let p2_top_left = Vec2::new(96.0, 64.0);

        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(GameMode::VersusDeathmatch);
        app.insert_resource(TileGrid::empty());
        app.insert_resource(EnemyFreeze::default());
        app.insert_resource(VersusPlayerFreeze::default());
        app.insert_resource(BaseReinforcement::default());
        app.insert_resource(ScoreBoard::versus(3, 5, 2.0));
        app.world_mut().spawn((
            Tank {
                top_left: p1_top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            Player { id: PlayerId::One },
            PlayerUpgrade { level: 0 },
            PlayerLives { current: 3 },
            Health { current: 1 },
            Transform::from_translation(board_object_center(
                p1_top_left.x,
                p1_top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
            Sprite::default(),
        ));
        app.world_mut().spawn((
            Tank {
                top_left: p2_top_left,
                facing: Direction::Down,
                speed: PLAYER_SPEED,
            },
            Player { id: PlayerId::Two },
            PlayerUpgrade { level: 0 },
            PlayerLives { current: 3 },
            Health { current: 1 },
            Transform::from_translation(board_object_center(
                p2_top_left.x,
                p2_top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
            Sprite::default(),
            Shield {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
        app.world_mut().spawn((
            PowerUp {
                kind: PowerUpKind::Grenade,
            },
            Transform::from_translation(board_object_center(
                p1_top_left.x,
                p1_top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
        ));
        app.add_systems(Update, pickup_powerups);

        app.update();

        let status = app.world().resource::<GameStatus>();
        assert_eq!(status.phase, GamePhase::Playing);
        let score_board = app.world().resource::<ScoreBoard>();
        assert_eq!(score_board.p1_score, 0);
        assert_eq!(score_board.p2_lives, 3);

        let mut players = app
            .world_mut()
            .query::<(&Player, &PlayerLives, Option<&Shield>, Option<&Tank>)>();
        let target = players
            .iter(app.world())
            .find(|(player, _, _, _)| player.id == PlayerId::Two)
            .expect("target player should remain");
        assert_eq!(target.1.current, 3);
        assert!(target.2.is_some());
        assert!(target.3.is_some());

        let mut powerups = app.world_mut().query::<&PowerUp>();
        assert_eq!(powerups.iter(app.world()).count(), 0);
    }

    #[test]
    fn versus_powerup_director_uses_arena_spawns_and_rotates_rewards() {
        let arena = parse_arena(ARENA_2).expect("arena should parse");
        let mut director = VersusPowerUpDirector::from_arena(&arena);
        assert!(director.spawn_immediately);

        let first = director.next_spawn();
        assert_eq!(first, Some((Vec2::new(96.0, 96.0), PowerUpKind::Star)));
        assert!(!director.spawn_immediately);

        let second = director.next_spawn();
        assert_eq!(second, Some((Vec2::new(104.0, 96.0), PowerUpKind::Helmet)));

        let third = director.next_spawn();
        assert_eq!(third, Some((Vec2::new(96.0, 96.0), PowerUpKind::Clock)));
    }

    #[test]
    fn arena_three_authors_mixed_terrain_duel_space() {
        let arena = parse_arena(ARENA_3).expect("arena should parse");
        let grid = TileGrid::from_arena(&arena).expect("grid should build");

        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert_eq!(arena.powerup_spawns.len(), 3);
        assert_eq!(arena.powerup_spawns[0].x, 12);
        assert_eq!(arena.powerup_spawns[0].y, 12);
    }

    #[test]
    fn arena_four_authors_water_midline_flank_duel_space() {
        let arena = parse_arena(ARENA_4).expect("arena should parse");
        let grid = TileGrid::from_arena(&arena).expect("grid should build");

        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(arena.powerup_spawns.len(), 3);
        assert_eq!(arena.p1_spawn.x, 2);
        assert_eq!(arena.p1_spawn.y, 24);
        assert_eq!(arena.p2_spawn.x, 24);
        assert_eq!(arena.p2_spawn.y, 0);
    }

    #[test]
    fn arena_six_authors_second_base_battle_lane_mix() {
        let arena = parse_arena(ARENA_6).expect("arena should parse");
        let grid = TileGrid::from_arena(&arena).expect("grid should build");

        assert!(grid.tiles.contains(&TileKind::Water));
        assert!(grid.tiles.contains(&TileKind::Forest));
        assert!(grid.tiles.contains(&TileKind::Ice));
        assert!(grid.tiles.contains(&TileKind::Steel));
        assert_eq!(arena.powerup_spawns.len(), 3);
        assert_eq!(arena.p1_spawn.x, 4);
        assert_eq!(arena.p1_spawn.y, 24);
        assert_eq!(arena.p2_spawn.x, 20);
        assert_eq!(arena.p2_spawn.y, 0);
        let BattleRules::BaseBattle {
            p1_base, p2_base, ..
        } = arena.battle_rules
        else {
            panic!("arena six should be base battle");
        };
        assert_eq!(p1_base, GridPoint { x: 0, y: 24 });
        assert_eq!(p2_base, GridPoint { x: 24, y: 0 });
    }

    #[test]
    fn powerup_visuals_sparkle_between_bright_tints() {
        assert_eq!(powerup_visual_rgb(0.05), [255, 255, 255]);
        assert_eq!(powerup_visual_rgb(0.20), [255, 232, 104]);
        assert_eq!(powerup_visual_rgb(0.35), [255, 255, 255]);
    }

    #[test]
    fn enemy_freeze_expires_after_clock_duration() {
        let mut freeze = EnemyFreeze::default();
        freeze.start();
        assert!(freeze.is_active());
        freeze.tick(Duration::from_secs_f32(CLOCK_SECONDS + 0.1));
        assert!(!freeze.is_active());
    }

    #[test]
    fn clock_freeze_target_depends_on_game_mode() {
        assert_eq!(clock_freeze_target(GameMode::Campaign, PlayerId::One), None);
        assert_eq!(
            clock_freeze_target(GameMode::VersusDeathmatch, PlayerId::One),
            Some(PlayerId::Two)
        );
        assert_eq!(
            clock_freeze_target(GameMode::VersusBaseBattle, PlayerId::Two),
            Some(PlayerId::One)
        );
    }

    #[test]
    fn grenade_targets_opponent_only_in_versus() {
        assert_eq!(
            grenade_player_target(GameMode::Campaign, PlayerId::One),
            None
        );
        assert_eq!(
            grenade_player_target(GameMode::VersusDeathmatch, PlayerId::One),
            Some(PlayerId::Two)
        );
        assert_eq!(
            grenade_player_target(GameMode::VersusBaseBattle, PlayerId::Two),
            Some(PlayerId::One)
        );
    }

    #[test]
    fn versus_player_freeze_targets_one_player_and_expires() {
        let mut freeze = VersusPlayerFreeze::default();
        freeze.start(PlayerId::Two);

        assert!(!freeze.is_player_frozen(PlayerId::One));
        assert!(freeze.is_player_frozen(PlayerId::Two));

        freeze.tick(Duration::from_secs_f32(CLOCK_SECONDS + 0.1));

        assert!(!freeze.is_player_frozen(PlayerId::Two));
    }

    #[test]
    fn frozen_player_visuals_flash_blue_white() {
        assert_eq!(player_frozen_visual_rgb(0.05), [136, 216, 255]);
        assert_eq!(player_frozen_visual_rgb(0.18), [216, 248, 255]);
    }

    #[test]
    fn base_wall_positions_wrap_campaign_base_without_base_tiles() {
        let level = parse_level(LEVEL_1).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        let positions = base_wall_positions(&grid);
        assert!(positions.contains(&(10, 24)));
        assert!(positions.contains(&(15, 25)));
        assert!(!positions.contains(&(12, 24)));
        assert!(
            positions
                .iter()
                .all(|(x, y)| grid.tiles[y * BOARD_TILES + x] != TileKind::Base)
        );
    }

    #[test]
    fn base_center_tracks_campaign_base_tiles() {
        let level = parse_level(LEVEL_1).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        assert_eq!(base_center_from_grid(&grid), Some(Vec2::new(104.0, 200.0)));
    }

    #[test]
    fn base_top_left_tracks_whole_campaign_base() {
        let level = parse_level(LEVEL_1).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        assert_eq!(base_top_left_from_grid(&grid), Some(Vec2::new(96.0, 192.0)));
    }

    #[test]
    fn base_hit_detection_covers_the_whole_two_by_two_base() {
        let top_left = Vec2::new(96.0, 192.0);
        assert!(base_contains_tile(top_left, 12, 24));
        assert!(base_contains_tile(top_left, 13, 24));
        assert!(base_contains_tile(top_left, 12, 25));
        assert!(base_contains_tile(top_left, 13, 25));
        assert!(!base_contains_tile(top_left, 11, 24));
        assert!(!base_contains_tile(top_left, 14, 25));
    }

    #[test]
    fn base_battle_winner_is_the_destroyed_base_opponent() {
        assert_eq!(base_battle_winner_for_base(PlayerId::One), PlayerId::Two);
        assert_eq!(base_battle_winner_for_base(PlayerId::Two), PlayerId::One);
    }

    #[test]
    fn base_battle_bullets_only_destroy_opponent_base() {
        assert!(base_can_be_destroyed_by_bullet(
            GameMode::VersusBaseBattle,
            Team::Player1,
            Some(PlayerId::Two)
        ));
        assert!(!base_can_be_destroyed_by_bullet(
            GameMode::VersusBaseBattle,
            Team::Player1,
            Some(PlayerId::One)
        ));
        assert!(!base_can_be_destroyed_by_bullet(
            GameMode::VersusBaseBattle,
            Team::Player1,
            None
        ));
    }

    #[test]
    fn campaign_base_can_be_destroyed_by_player_or_enemy_bullets() {
        assert!(base_can_be_destroyed_by_bullet(
            GameMode::Campaign,
            Team::Player1,
            None
        ));
        assert!(base_can_be_destroyed_by_bullet(
            GameMode::Campaign,
            Team::Enemy,
            None
        ));
    }

    #[test]
    fn base_destroyed_sounds_include_terminal_jingles() {
        assert_eq!(
            base_destroyed_sounds(GameMode::Campaign, None),
            [SoundKind::BaseDestroyed, SoundKind::GameOver]
        );
        assert_eq!(
            base_destroyed_sounds(GameMode::VersusBaseBattle, Some(PlayerId::One)),
            [SoundKind::BaseDestroyed, SoundKind::LevelClear]
        );
        assert_eq!(base_destroyed_sounds(GameMode::VersusBaseBattle, None), []);
        assert_eq!(base_destroyed_sounds(GameMode::VersusDeathmatch, None), []);
    }

    #[test]
    fn shovel_reinforces_only_the_collectors_base_in_base_battle() {
        let arena = parse_arena(ARENA_5).expect("arena should parse");
        let grid = TileGrid::from_arena(&arena).expect("grid should build");
        let BattleRules::BaseBattle {
            p1_base, p2_base, ..
        } = arena.battle_rules
        else {
            panic!("arena five should be base battle");
        };
        let bases = [
            BaseSprite {
                owner: Some(PlayerId::One),
                top_left: grid_point_top_left(&p1_base),
            },
            BaseSprite {
                owner: Some(PlayerId::Two),
                top_left: grid_point_top_left(&p2_base),
            },
        ];

        let p1_positions = shovel_reinforcement_positions(
            GameMode::VersusBaseBattle,
            PlayerId::One,
            &grid,
            &bases,
        );
        assert!(p1_positions.contains(&(2, 24)));
        assert!(!p1_positions.contains(&(22, 0)));

        let p2_positions = shovel_reinforcement_positions(
            GameMode::VersusBaseBattle,
            PlayerId::Two,
            &grid,
            &bases,
        );
        assert!(p2_positions.contains(&(22, 0)));
        assert!(!p2_positions.contains(&(2, 24)));

        assert!(
            shovel_reinforcement_positions(
                GameMode::VersusDeathmatch,
                PlayerId::One,
                &grid,
                &bases
            )
            .is_empty()
        );
    }

    #[test]
    fn shovel_reinforcement_warns_only_near_expiration() {
        let mut reinforcement = BaseReinforcement {
            timer: None,
            saved_tiles: vec![(10, 24, TileKind::Brick)],
        };
        reinforcement.start();

        assert_eq!(reinforcement.warning_elapsed_secs(), None);
        assert!(reinforcement.contains_position(10, 24));
        assert!(!reinforcement.contains_position(12, 24));

        assert!(!reinforcement.tick(Duration::from_secs_f32(
            SHOVEL_SECONDS - SHOVEL_WARNING_SECONDS + 0.01
        )));
        assert!(reinforcement.warning_elapsed_secs().is_some());
    }

    #[test]
    fn shovel_reinforcement_can_switch_to_a_different_base() {
        let mut app = App::new();
        let mut grid = TileGrid::empty();
        grid.set(2, 24, TileKind::Steel);
        grid.set(22, 0, TileKind::Brick);
        let mut reinforcement = BaseReinforcement {
            timer: None,
            saved_tiles: vec![(2, 24, TileKind::Brick)],
        };
        reinforcement.start();

        app.insert_resource(test_sprite_assets());
        app.insert_resource(grid);
        app.insert_resource(reinforcement);
        app.add_systems(Update, switch_base_reinforcement_for_test);

        app.update();

        let grid = app.world().resource::<TileGrid>();
        assert_eq!(grid.get(2, 24), Some(TileKind::Brick));
        assert_eq!(grid.get(22, 0), Some(TileKind::Steel));

        let reinforcement = app.world().resource::<BaseReinforcement>();
        assert_eq!(reinforcement.saved_tiles, [(22, 0, TileKind::Brick)]);
        assert!(reinforcement.timer.is_some());
    }

    #[test]
    fn repeated_shovel_on_same_base_only_refreshes_timer() {
        let saved_tiles = [(2, 24, TileKind::Brick), (3, 24, TileKind::Steel)];

        assert!(reinforcement_matches_positions(
            &saved_tiles,
            &[(3, 24), (2, 24)]
        ));
        assert!(!reinforcement_matches_positions(
            &saved_tiles,
            &[(22, 0), (2, 24)]
        ));
    }

    #[test]
    fn shovel_warning_visuals_flash_yellow() {
        assert_eq!(shovel_warning_visual_rgb(0.05), [255, 255, 255]);
        assert_eq!(shovel_warning_visual_rgb(0.18), [248, 232, 96]);
    }

    #[test]
    fn same_kind_tile_sync_refreshes_sprite_to_clear_shovel_tint() {
        let mut app = App::new();
        let assets = test_sprite_assets();
        let mut grid = TileGrid::empty();
        grid.set(10, 24, TileKind::Steel);

        let mut tinted_sprite = Sprite::from_atlas_image(
            assets.terrain_image.clone(),
            TextureAtlas {
                layout: assets.terrain_layout.clone(),
                index: assets.manifest.terrain.steel,
            },
        );
        tinted_sprite.color = shovel_warning_visual_color(0.18);

        app.insert_resource(assets);
        app.insert_resource(grid);
        app.world_mut()
            .spawn((tinted_sprite, GridTile { x: 10, y: 24 }, GameEntity));
        app.add_systems(Update, refresh_same_kind_steel_tile_for_test);

        app.update();

        let mut query = app.world_mut().query::<(&GridTile, &Sprite)>();
        let matching_sprites: Vec<&Sprite> = query
            .iter(app.world())
            .filter_map(|(tile, sprite)| (tile.x == 10 && tile.y == 24).then_some(sprite))
            .collect();

        assert_eq!(matching_sprites.len(), 1);
        assert_eq!(matching_sprites[0].color, Color::WHITE);
        assert_eq!(
            matching_sprites[0]
                .texture_atlas
                .as_ref()
                .expect("refreshed terrain sprite should use the terrain atlas")
                .index,
            app.world()
                .resource::<SpriteAssets>()
                .manifest
                .terrain
                .steel
        );
    }

    #[test]
    fn spawn_point_top_left_uses_small_tile_coordinates() {
        let spawn = SpawnPoint {
            x: 8,
            y: 24,
            facing: Direction::Up,
        };
        assert_eq!(spawn_point_top_left(&spawn), Vec2::new(64.0, 192.0));
    }

    #[test]
    fn aligned_fire_direction_requires_shared_lane() {
        let enemy = Vec2::new(104.0, 48.0);
        assert_eq!(
            aligned_fire_direction(enemy, Vec2::new(104.0, 96.0)),
            Some(Direction::Down)
        );
        assert_eq!(
            aligned_fire_direction(enemy, Vec2::new(72.0, 48.0)),
            Some(Direction::Left)
        );
        assert_eq!(aligned_fire_direction(enemy, Vec2::new(112.0, 61.0)), None);
    }

    #[test]
    fn enemy_aim_prefers_aligned_player_before_base() {
        let enemy_top_left = Vec2::new(96.0, 40.0);
        let player_top_lefts = [Vec2::new(48.0, 40.0)];
        let base_center = Some(Vec2::new(104.0, 200.0));
        assert_eq!(
            enemy_aim_direction(enemy_top_left, &player_top_lefts, base_center),
            Some(Direction::Left)
        );
    }

    #[test]
    fn preferred_enemy_direction_pressures_base_without_player() {
        assert_eq!(
            preferred_enemy_direction(
                Vec2::new(24.0, 64.0),
                Direction::Up,
                &[],
                Some(Vec2::new(104.0, 200.0))
            ),
            Direction::Down
        );
    }

    #[test]
    fn enemy_direction_can_roam_instead_of_always_pressuring_base() {
        let top_left = Vec2::new(80.0, 80.0);
        assert!(enemy_should_roam(top_left, Direction::Up));
        assert_eq!(
            enemy_patrol_direction(top_left, Direction::Up),
            Direction::Left
        );
        assert_eq!(
            preferred_enemy_direction(top_left, Direction::Up, &[], Some(Vec2::new(104.0, 200.0))),
            Direction::Left
        );
    }

    #[test]
    fn enemy_patrol_still_pushes_top_spawns_downward() {
        assert_eq!(
            enemy_patrol_direction(Vec2::new(96.0, 0.0), Direction::Left),
            Direction::Down
        );
    }

    #[test]
    fn enemy_alignment_fire_uses_fractional_cooldown() {
        assert!(!enemy_alignment_fire_ready(EnemyKind::Basic, 0.70));
        assert!(enemy_alignment_fire_ready(EnemyKind::Basic, 0.72));
    }

    #[test]
    fn enemy_fire_slots_limit_total_and_per_tank_bullets() {
        assert!(enemy_fire_slot_available(0, 0));
        assert!(enemy_fire_slot_available(ENEMY_BULLET_LIMIT - 1, 0));
        assert!(!enemy_fire_slot_available(ENEMY_BULLET_LIMIT, 0));
        assert!(!enemy_fire_slot_available(1, ENEMY_BULLET_LIMIT_PER_TANK));
    }

    #[test]
    fn enemy_random_fire_is_low_rate_and_deterministic() {
        assert_eq!(enemy_random_fire_rate(EnemyKind::Basic), 4);
        assert_eq!(enemy_random_fire_rate(EnemyKind::Armor), 4);
        assert_eq!(enemy_random_fire_rate(EnemyKind::Fast), 3);
        assert_eq!(enemy_random_fire_rate(EnemyKind::Power), 2);

        assert!(enemy_random_fire_ready(
            Vec2::new(0.0, 0.0),
            Direction::Up,
            EnemyKind::Basic
        ));
        assert!(!enemy_random_fire_ready(
            Vec2::new(TILE_SIZE, 0.0),
            Direction::Up,
            EnemyKind::Basic
        ));
        assert!(enemy_random_fire_ready(
            Vec2::new(0.0, 0.0),
            Direction::Up,
            EnemyKind::Power
        ));
    }

    #[test]
    fn generated_retro_sounds_are_short_and_bounded() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
        for (_, spec) in sound_manifest_specs(&manifest.sounds) {
            let sound = make_manifest_sound(spec);
            assert_eq!(sound.sample_rate, SOUND_SAMPLE_RATE);
            assert!(!sound.samples.is_empty());
            assert!(sound.samples.len() <= SOUND_SAMPLE_RATE as usize);
            assert!(sound.samples.iter().all(|sample| sample.abs() <= 1.0));
        }
    }

    #[test]
    fn generated_background_music_is_longer_loop_and_bounded() {
        let music = make_background_music_sound();
        assert_eq!(music.sample_rate, SOUND_SAMPLE_RATE);
        assert!(music.samples.len() > SOUND_SAMPLE_RATE as usize);
        assert!(music.samples.len() <= (SOUND_SAMPLE_RATE as usize * 5));
        assert!(music.samples.iter().all(|sample| sample.abs() <= 1.0));
    }

    #[test]
    fn sound_sample_count_never_returns_zero() {
        assert_eq!(sample_count(0.0), 1);
    }

    #[test]
    fn tank_position_blocks_other_tanks_but_allows_self() {
        let current = Vec2::new(16.0, 16.0);
        let other = Vec2::new(48.0, 16.0);
        assert!(tank_position_free(current, current, &[current, other]));
        assert!(!tank_position_free(
            Vec2::new(40.0, 16.0),
            current,
            &[current, other]
        ));
        assert!(tank_position_free(
            Vec2::new(72.0, 16.0),
            current,
            &[current, other]
        ));
    }

    #[test]
    fn tank_spawn_position_blocks_any_overlapping_tank() {
        let current = Vec2::new(16.0, 16.0);
        let other = Vec2::new(48.0, 16.0);

        assert!(!tank_spawn_position_free(current, &[current, other]));
        assert!(!tank_spawn_position_free(
            Vec2::new(40.0, 16.0),
            &[current, other]
        ));
        assert!(tank_spawn_position_free(
            Vec2::new(72.0, 16.0),
            &[current, other]
        ));
    }

    #[test]
    fn bullet_overlap_uses_bullet_footprint() {
        assert!(bullet_positions_overlap(
            Vec2::new(10.0, 10.0),
            Vec2::new(13.0, 10.0)
        ));
        assert!(!bullet_positions_overlap(
            Vec2::new(10.0, 10.0),
            Vec2::new(14.0, 10.0)
        ));
    }

    #[test]
    fn bullet_paths_clash_detects_fast_head_on_crossing() {
        assert_eq!(
            bullet_paths_clash_impact(
                Vec2::new(8.0, 8.0),
                Vec2::new(36.0, 8.0),
                Vec2::new(36.0, 8.0),
                Vec2::new(8.0, 8.0)
            ),
            Some(Vec2::new(22.0, 8.0))
        );
    }

    #[test]
    fn bullet_paths_clash_ignores_missed_lanes() {
        assert_eq!(
            bullet_paths_clash_impact(
                Vec2::new(8.0, 8.0),
                Vec2::new(36.0, 8.0),
                Vec2::new(36.0, 16.0),
                Vec2::new(8.0, 16.0)
            ),
            None
        );
    }

    #[test]
    fn bullet_paths_clash_ignores_edge_touch_moving_apart() {
        assert_eq!(
            bullet_paths_clash_impact(
                Vec2::new(8.0, 8.0),
                Vec2::new(8.0, 8.0),
                Vec2::new(12.0, 8.0),
                Vec2::new(16.0, 8.0)
            ),
            None
        );
    }

    #[test]
    fn cancel_colliding_bullets_despawns_live_crossing_bullets() {
        let mut app = App::new();
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.world_mut().spawn(test_bullet(
            Vec2::new(8.0, 8.0),
            Vec2::new(36.0, 8.0),
            false,
        ));
        app.world_mut().spawn(test_bullet(
            Vec2::new(36.0, 8.0),
            Vec2::new(8.0, 8.0),
            false,
        ));
        app.add_systems(Update, cancel_colliding_bullets);

        app.update();

        let mut bullets = app.world_mut().query::<&Bullet>();
        assert_eq!(bullets.iter(app.world()).count(), 0);
    }

    #[test]
    fn cancel_colliding_bullets_ignores_already_resolved_bullets() {
        let mut app = App::new();
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.world_mut()
            .spawn(test_bullet(Vec2::new(8.0, 8.0), Vec2::new(36.0, 8.0), true));
        app.world_mut().spawn(test_bullet(
            Vec2::new(36.0, 8.0),
            Vec2::new(8.0, 8.0),
            false,
        ));
        app.add_systems(Update, cancel_colliding_bullets);

        app.update();

        let mut bullets = app.world_mut().query::<&Bullet>();
        assert_eq!(bullets.iter(app.world()).count(), 2);
    }

    #[test]
    fn cancel_colliding_bullets_keeps_third_bullet_after_earliest_pair_clashes() {
        let mut app = App::new();
        app.insert_resource(test_sprite_assets());
        app.insert_resource(test_sound_assets());
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.world_mut().spawn(test_bullet(
            Vec2::new(8.0, 8.0),
            Vec2::new(44.0, 8.0),
            false,
        ));
        app.world_mut().spawn(test_bullet(
            Vec2::new(44.0, 8.0),
            Vec2::new(8.0, 8.0),
            false,
        ));
        app.world_mut().spawn(test_bullet(
            Vec2::new(24.0, 24.0),
            Vec2::new(24.0, -12.0),
            false,
        ));
        app.add_systems(Update, cancel_colliding_bullets);

        app.update();

        let mut bullets = app.world_mut().query::<&Bullet>();
        let remaining: Vec<(Vec2, Vec2)> = bullets
            .iter(app.world())
            .map(|bullet| (bullet.previous_top_left, bullet.top_left))
            .collect();
        assert_eq!(remaining, [(Vec2::new(44.0, 8.0), Vec2::new(8.0, 8.0))]);
    }

    #[test]
    fn bullet_clash_impact_uses_midpoint_between_bullets() {
        assert_eq!(
            bullet_clash_impact_top_left(Vec2::new(10.0, 12.0), Vec2::new(14.0, 8.0)),
            Vec2::new(12.0, 10.0)
        );
    }

    fn unique_temp_asset_path(name: &str) -> std::path::PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("tank-{nonce}-{name}"))
    }
}
