#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::RenderAssetUsages;
use bevy::audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings, Source, Volume};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PresentMode;
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

const ASSET_MANIFEST_PATH: &str = "assets/manifest.ron";
const LEVEL_COUNT: usize = 35;
const LEVEL_CLEAR_DELAY_SECONDS: f32 = 2.0;
const ARENA_COUNT: usize = 3;
const DEFAULT_VERSUS_ARENA: usize = 1;
const TANK_ATLAS_TILES: usize = 48;
const TANK_ANIMATION_FRAMES: usize = 2;
const TERRAIN_ATLAS_TILES: usize = 6;
const EFFECT_ATLAS_TILES: usize = 16;
const POWERUP_ATLAS_TILES: usize = 6;

const VIRTUAL_WIDTH: f32 = 256.0;
const VIRTUAL_HEIGHT: f32 = 240.0;
const WINDOW_SCALE: f32 = 3.0;

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
const SNAP_DISTANCE: f32 = 2.0;
const GLYPHS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static PAUSED_BANNER_LINES: [&str; 2] = ["PAUSED", "PRESS ESC"];
static GAME_OVER_BANNER_LINES: [&str; 2] = ["GAME OVER", "PRESS R OR M"];
static LEVEL_CLEAR_BANNER_LINES: [&str; 1] = ["LEVEL CLEAR"];
static P1_WIN_BANNER_LINES: [&str; 2] = ["P1 WIN", "PRESS R OR M"];
static P2_WIN_BANNER_LINES: [&str; 2] = ["P2 WIN", "PRESS R OR M"];
static VICTORY_BANNER_LINES: [&str; 3] = ["VICTORY", "ALL STAGES CLEAR", "PRESS R OR M"];
const HELMET_SECONDS: f32 = 6.0;
const CLOCK_SECONDS: f32 = 6.0;
const SHOVEL_SECONDS: f32 = 10.0;
const ENEMY_ALIGNMENT_FIRE_FRACTION: f32 = 0.45;
const ENEMY_SPAWN_PROTECTION_SECONDS: f32 = 0.35;
const PLAYER_RESPAWN_DELAY_SECONDS: f32 = 0.35;
const VERSUS_POWERUP_INTERVAL_SECONDS: f32 = 8.0;
const SOUND_SAMPLE_RATE: u32 = 22_050;
const ICE_SPEED_MULTIPLIER: f32 = 1.18;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerControl::default())
        .insert_resource(GameMode::Campaign)
        .insert_resource(ModeSelect::default())
        .insert_resource(GameStatus::default())
        .insert_resource(EnemyFreeze::default())
        .insert_resource(BaseReinforcement::default())
        .insert_resource(VersusPowerUpDirector::inactive())
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tank 1990 Bevy Remake".into(),
                        resolution: (
                            (VIRTUAL_WIDTH * WINDOW_SCALE) as u32,
                            (VIRTUAL_HEIGHT * WINDOW_SCALE) as u32,
                        )
                            .into(),
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
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct AssetManifest {
    tanks: TankSpriteManifest,
    terrain: TerrainSpriteManifest,
    effects: EffectSpriteManifest,
    powerups: PowerUpSpriteManifest,
}

impl AssetManifest {
    fn tank_index(&self, set: TankSpriteSet, direction: Direction, frame: usize) -> usize {
        self.tanks.frames_for(set)[frame.min(TANK_ANIMATION_FRAMES - 1)].index(direction)
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

#[derive(Resource)]
struct SoundAssets {
    fire: Handle<RetroSound>,
    brick_hit: Handle<RetroSound>,
    steel_hit: Handle<RetroSound>,
    tank_explosion: Handle<RetroSound>,
    base_destroyed: Handle<RetroSound>,
    powerup_pickup: Handle<RetroSound>,
    stage_start: Handle<RetroSound>,
    level_clear: Handle<RetroSound>,
    game_over: Handle<RetroSound>,
}

#[derive(Clone, Copy, Debug)]
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
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            p1_last_direction: Direction::Up,
            p2_last_direction: Direction::Down,
        }
    }
}

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
enum GameMode {
    Campaign,
    VersusDeathmatch,
}

#[derive(Resource)]
struct ModeSelect {
    selected: GameMode,
    arena: usize,
}

impl Default for ModeSelect {
    fn default() -> Self {
        Self {
            selected: GameMode::Campaign,
            arena: DEFAULT_VERSUS_ARENA,
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
            p1_score: 0,
            p2_score: 0,
            p1_lives: 3,
            p2_lives: 0,
            target_score: 0,
            respawn_invulnerability_secs: 2.0,
        }
    }

    fn versus(lives: i32, target_score: u32, respawn_invulnerability_secs: f32) -> Self {
        Self {
            score: 0,
            lives,
            enemies_destroyed: 0,
            total_enemies: 0,
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

    fn set_player_lives(&mut self, player: PlayerId, lives: i32) {
        match player {
            PlayerId::One => self.p1_lives = lives,
            PlayerId::Two => self.p2_lives = lives,
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

    fn tank_sprite_index(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Right => 3,
        }
    }

    fn bullet_sprite_index(self) -> usize {
        self.tank_sprite_index()
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
}

#[derive(Clone, Deserialize)]
struct SpawnPoint {
    x: usize,
    y: usize,
    facing: Direction,
}

#[derive(Deserialize)]
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
    fn enemy() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_PROTECTION_SECONDS, TimerMode::Once),
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
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(PLAYER_RESPAWN_DELAY_SECONDS, TimerMode::Once),
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
    top_left: Vec2,
    facing: Direction,
    owner: Team,
    speed: f32,
    breaks_steel: bool,
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
struct BaseSprite;

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
struct ModeSelectArenaGlyph {
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
    mut images: ResMut<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut retro_sounds: ResMut<Assets<RetroSound>>,
) {
    commands.spawn(Camera2d);

    let sprite_assets = create_sprite_assets(&mut images, &mut atlas_layouts);
    let sound_assets = create_sound_assets(&mut retro_sounds);
    spawn_mode_select_screen(
        &mut commands,
        &sprite_assets,
        GameMode::Campaign,
        DEFAULT_VERSUS_ARENA,
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
    sounds: Res<SoundAssets>,
    mut game_mode: ResMut<GameMode>,
    mut game_status: ResMut<GameStatus>,
    mut tile_grid: ResMut<TileGrid>,
    mut director: ResMut<EnemyDirector>,
    mut score_board: ResMut<ScoreBoard>,
    mut stage_rules: ResMut<StageRules>,
    mut versus_powerups: ResMut<VersusPowerUpDirector>,
    mut mode_select: ResMut<ModeSelect>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    mut menu_queries: ParamSet<(
        Query<Entity, With<GameEntity>>,
        Query<&mut Transform, With<ModeSelectCursor>>,
        Query<(&ModeSelectArenaGlyph, &mut Sprite)>,
    )>,
) {
    if game_status.phase == GamePhase::ModeSelect {
        if keys.just_pressed(KeyCode::KeyW)
            || keys.just_pressed(KeyCode::ArrowUp)
            || keys.just_pressed(KeyCode::KeyS)
            || keys.just_pressed(KeyCode::ArrowDown)
        {
            mode_select.selected = other_mode(mode_select.selected);
            update_mode_select_cursor(&mut menu_queries.p1(), mode_select.selected);
        }

        if mode_select.selected == GameMode::VersusDeathmatch
            && (keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft))
        {
            mode_select.arena = previous_arena(mode_select.arena);
            update_mode_select_arena_digits(&mut menu_queries.p2(), mode_select.arena);
        }

        if mode_select.selected == GameMode::VersusDeathmatch
            && (keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight))
        {
            mode_select.arena = next_arena(mode_select.arena);
            update_mode_select_arena_digits(&mut menu_queries.p2(), mode_select.arena);
        }

        if keys.just_pressed(KeyCode::Space)
            || keys.just_pressed(KeyCode::Enter)
            || keys.just_pressed(KeyCode::ShiftRight)
        {
            match mode_select.selected {
                GameMode::Campaign => {
                    game_status.stage = 1;
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
                        &mut base_reinforcement,
                        &menu_queries.p0(),
                    );
                }
                GameMode::VersusDeathmatch => {
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
                        &mut base_reinforcement,
                        &menu_queries.p0(),
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
                &mut base_reinforcement,
                &menu_queries.p0(),
            ),
            GameMode::VersusDeathmatch => start_versus_round(
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
    base_reinforcement: &mut BaseReinforcement,
    selected_mode: GameMode,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    mode_select.selected = selected_mode;
    mode_select.arena = game_status.arena.clamp(1, ARENA_COUNT);
    spawn_mode_select_screen(commands, assets, mode_select.selected, mode_select.arena);

    *tile_grid = TileGrid::empty();
    *director = EnemyDirector::inactive();
    *score_board = ScoreBoard::campaign(0);
    *stage_rules = StageRules::default();
    *versus_powerups = VersusPowerUpDirector::inactive();
    enemy_freeze.reset();
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
    base_reinforcement: &mut BaseReinforcement,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let level = load_stage_definition(game_status.stage).expect("level should load");
    info!("Loaded {}", level.name);
    let new_tile_grid = TileGrid::from_level(&level).expect("level map should be valid");

    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(commands, assets, GameMode::Campaign);
    spawn_level(commands, assets, &level, &new_tile_grid, 3);
    play_sound(commands, sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    *director = EnemyDirector::from_level(&level);
    *score_board = ScoreBoard::campaign(level.enemies.len());
    *stage_rules = StageRules::from_level(&level);
    *versus_powerups = VersusPowerUpDirector::inactive();
    enemy_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::Playing;
    game_status.winner = None;
    game_status.transition_timer.reset();
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
    base_reinforcement: &mut BaseReinforcement,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let arena_index = game_status.arena.clamp(1, ARENA_COUNT);
    let arena = load_arena_definition(arena_index).unwrap_or_else(|err| {
        panic!("failed to load versus arena {arena_index}: {err}");
    });
    info!("Loaded {}", arena.name);
    let new_tile_grid = TileGrid::from_arena(&arena).expect("arena map should be valid");
    let BattleRules::Deathmatch {
        target_score,
        lives,
        respawn_invulnerability_secs,
    } = arena.battle_rules;

    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(commands, assets, GameMode::VersusDeathmatch);
    spawn_arena(commands, assets, &arena, &new_tile_grid, lives);
    play_sound(commands, sounds, SoundKind::StageStart);

    *game_mode = GameMode::VersusDeathmatch;
    *tile_grid = new_tile_grid;
    *director = EnemyDirector::inactive();
    *score_board = ScoreBoard::versus(lives, target_score, respawn_invulnerability_secs);
    *stage_rules = StageRules::default();
    *versus_powerups = VersusPowerUpDirector::from_arena(&arena);
    enemy_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::Playing;
    game_status.arena = arena_index;
    game_status.winner = None;
    game_status.transition_timer.reset();
}

fn stage_path(stage: usize) -> String {
    format!("assets/levels/{stage:03}.level.ron")
}

fn load_stage_definition(stage: usize) -> Result<LevelDefinition, String> {
    load_level(&stage_path(stage))
}

fn arena_path(arena: usize) -> String {
    format!("assets/arenas/arena_{arena:02}.ron")
}

fn load_arena_definition(arena: usize) -> Result<ArenaDefinition, String> {
    load_arena(&arena_path(arena))
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
    validate_tank_frames(&manifest.tanks)?;

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
    if level.max_enemies_on_screen == 0 {
        return Err("max_enemies_on_screen must be greater than zero".to_string());
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
    let BattleRules::Deathmatch {
        target_score,
        lives,
        respawn_invulnerability_secs,
    } = arena.battle_rules;
    if target_score == 0 {
        return Err("deathmatch target_score must be greater than zero".to_string());
    }
    if lives <= 0 {
        return Err("deathmatch lives must be greater than zero".to_string());
    }
    if respawn_invulnerability_secs <= 0.0 {
        return Err("deathmatch respawn_invulnerability_secs must be positive".to_string());
    }
    validate_tank_spawn(&grid, "p1 spawn", &arena.p1_spawn)?;
    validate_tank_spawn(&grid, "p2 spawn", &arena.p2_spawn)?;
    for (index, point) in arena.powerup_spawns.iter().enumerate() {
        validate_powerup_spawn(&grid, index + 1, point)?;
    }

    Ok(arena)
}

fn spawn_mode_select_screen(
    commands: &mut Commands,
    assets: &SpriteAssets,
    selected: GameMode,
    arena: usize,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(16, 16, 14),
            Vec2::new(208.0 * WINDOW_SCALE, 208.0 * WINDOW_SCALE),
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
        mode_select_option_top_left(GameMode::Campaign),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "BATTLE",
        mode_select_option_top_left(GameMode::VersusDeathmatch),
        0.3,
    );
    spawn_pixel_text(commands, assets, "ARENA", Vec2::new(77.0, 145.0), 0.3);
    spawn_mode_select_arena_digits(commands, assets, arena, Vec2::new(113.0, 145.0), 0.3);
    spawn_mode_select_cursor(commands, assets, selected);
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
                    index: glyph_index(ch),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * 6.0, top_left.y),
                Vec2::new(5.0, 7.0),
                z,
            ))
            .with_scale(Vec3::splat(WINDOW_SCALE)),
            ModeSelectArenaGlyph { digit },
            GameEntity,
        ));
    }
}

fn spawn_mode_select_cursor(commands: &mut Commands, assets: &SpriteAssets, selected: GameMode) {
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
            .with_scale(Vec3::splat(WINDOW_SCALE)),
        ModeSelectCursor,
        GameEntity,
    ));
}

fn spawn_screen_frame(commands: &mut Commands, assets: &SpriteAssets, mode: GameMode) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(80, 80, 72),
            Vec2::new(48.0 * WINDOW_SCALE, 208.0 * WINDOW_SCALE),
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
            Vec2::new(40.0 * WINDOW_SCALE, 192.0 * WINDOW_SCALE),
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
        GameMode::VersusDeathmatch => spawn_versus_status_panel(commands, assets),
    }
}

fn spawn_campaign_status_panel(commands: &mut Commands, assets: &SpriteAssets) {
    spawn_pixel_text(commands, assets, "P1", Vec2::new(214.0, 26.0), 0.3);
    spawn_pixel_text(commands, assets, "SCORE", Vec2::new(214.0, 38.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Score,
        6,
        Vec2::new(214.0, 49.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "STAGE", Vec2::new(214.0, 76.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Stage,
        2,
        Vec2::new(224.0, 87.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "LIFE", Vec2::new(214.0, 112.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Lives,
        1,
        Vec2::new(234.0, 123.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "ENEMY", Vec2::new(214.0, 148.0), 0.3);
    for index in 0..20 {
        let col = index % 2;
        let row = index / 2;
        commands.spawn((
            Sprite::from_color(
                Color::srgb_u8(184, 184, 160),
                Vec2::new(4.0 * WINDOW_SCALE, 4.0 * WINDOW_SCALE),
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(219.0 + col as f32 * 9.0, 160.0 + row as f32 * 5.0),
                Vec2::new(4.0, 4.0),
                0.3,
            )),
            Visibility::Visible,
            EnemyMarker { index },
            GameEntity,
        ));
    }
}

fn spawn_versus_status_panel(commands: &mut Commands, assets: &SpriteAssets) {
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
    spawn_status_digits(
        commands,
        assets,
        StatusValue::P2Lives,
        1,
        Vec2::new(234.0, 145.0),
        0.3,
    );

    spawn_pixel_text(commands, assets, "TARGET", Vec2::new(214.0, 174.0), 0.3);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Target,
        2,
        Vec2::new(226.0, 185.0),
        0.3,
    );
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
                    index: glyph_index('0'),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + digit as f32 * 6.0, top_left.y),
                Vec2::new(5.0, 7.0),
                z,
            ))
            .with_scale(Vec3::splat(WINDOW_SCALE)),
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
    lines: &[&str],
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
            Vec2::new(132.0 * WINDOW_SCALE, background_height * WINDOW_SCALE),
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
                    index: glyph_index(ch),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + index as f32 * 6.0, top_left.y),
                Vec2::new(5.0, 7.0),
                z,
            ))
            .with_scale(Vec3::splat(WINDOW_SCALE)),
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
) {
    spawn_terrain(commands, assets, tile_grid);

    commands.spawn((
        Sprite::from_image(assets.base_intact.clone()),
        Transform::from_translation(board_object_center(
            level.base_position.x as f32 * TILE_SIZE,
            level.base_position.y as f32 * TILE_SIZE,
            Vec2::splat(TANK_SIZE),
            4.0,
        ))
        .with_scale(Vec3::splat(WINDOW_SCALE)),
        BaseSprite,
        GameEntity,
    ));

    spawn_player_tank(
        commands,
        assets,
        &level.player_spawn,
        PlayerId::One,
        player_lives,
    );
}

fn spawn_arena(
    commands: &mut Commands,
    assets: &SpriteAssets,
    arena: &ArenaDefinition,
    tile_grid: &TileGrid,
    player_lives: i32,
) {
    spawn_terrain(commands, assets, tile_grid);
    spawn_player_tank(
        commands,
        assets,
        &arena.p1_spawn,
        PlayerId::One,
        player_lives,
    );
    spawn_player_tank(
        commands,
        assets,
        &arena.p2_spawn,
        PlayerId::Two,
        player_lives,
    );
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
            .with_scale(Vec3::splat(WINDOW_SCALE)),
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
    if tile_grid.tiles[y * BOARD_TILES + x] == tile {
        return;
    }

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

fn spawn_player_tank(
    commands: &mut Commands,
    assets: &SpriteAssets,
    spawn: &SpawnPoint,
    player_id: PlayerId,
    player_lives: i32,
) {
    let player_top_left = Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE);

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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
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
        Player { id: player_id },
        GameEntity,
    ));
}

fn update_player_control(keys: Res<ButtonInput<KeyCode>>, mut control: ResMut<PlayerControl>) {
    for (key, direction) in [
        (KeyCode::KeyW, Direction::Up),
        (KeyCode::KeyS, Direction::Down),
        (KeyCode::KeyA, Direction::Left),
        (KeyCode::KeyD, Direction::Right),
    ] {
        if keys.just_pressed(key) {
            control.p1_last_direction = direction;
        }
    }

    for (key, direction) in [
        (KeyCode::ArrowUp, Direction::Up),
        (KeyCode::ArrowDown, Direction::Down),
        (KeyCode::ArrowLeft, Direction::Left),
        (KeyCode::ArrowRight, Direction::Right),
    ] {
        if keys.just_pressed(key) {
            control.p2_last_direction = direction;
        }
    }
}

fn move_player_tank(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    control: Res<PlayerControl>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
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

        if !grid.can_tank_occupy(top_left)
            || tanks
                .iter()
                .any(|tank| tank_rects_overlap(tank.top_left, top_left))
        {
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
            .with_scale(Vec3::splat(WINDOW_SCALE)),
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
            SpawnProtection::enemy(),
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
        if !ai.fire_timer.just_finished() && !snap_fire_ready {
            continue;
        }

        if let Some(direction) = aim_direction {
            tank.facing = direction;
            set_tank_sprite_direction(&mut sprite, tank_sprite, tank.facing, &assets.manifest);
        }
        let bullet_top_left = spawn_bullet_position(tank.top_left, tank.facing);
        commands.spawn((
            Sprite::from_atlas_image(
                assets.bullet_image.clone(),
                TextureAtlas {
                    layout: assets.bullet_layout.clone(),
                    index: tank.facing.bullet_sprite_index(),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(WINDOW_SCALE)),
            Bullet {
                top_left: bullet_top_left,
                facing: tank.facing,
                owner: Team::Enemy,
                speed: enemy_bullet_speed(enemy.kind),
                breaks_steel: false,
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
    players: Query<(&Tank, &PlayerUpgrade, &Player), (With<Player>, Without<PlayerRespawnDelay>)>,
    bullets: Query<&Bullet>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (tank, upgrade, player) in &players {
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
                    index: tank.facing.bullet_sprite_index(),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(WINDOW_SCALE)),
            Bullet {
                top_left: bullet_top_left,
                facing: tank.facing,
                owner,
                speed: player_bullet_speed(upgrade.level),
                breaks_steel: player_bullets_break_steel(upgrade.level, *stage_rules),
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
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform)>,
    tile_sprites: Query<(Entity, &GridTile)>,
    active_powerups: Query<Entity, With<PowerUp>>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    mut base_sprites: Query<&mut Sprite, With<BaseSprite>>,
    mut enemy_tanks: Query<
        (
            Entity,
            &Tank,
            &EnemyTank,
            &mut Health,
            Option<&SpawnProtection>,
        ),
        (With<EnemyTank>, Without<Player>),
    >,
    mut player_tanks: Query<
        (
            Entity,
            &mut Tank,
            &mut Transform,
            &RespawnPoint,
            &mut PlayerLives,
            &mut Health,
            Option<&Shield>,
            &Player,
        ),
        (With<Player>, Without<EnemyTank>, Without<Bullet>),
    >,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut bullet, mut transform) in &mut bullets {
        let facing = bullet.facing;
        let speed = bullet.speed;
        bullet.top_left += facing.movement() * speed * time.delta_secs();
        bullet.top_left = round_vec2(bullet.top_left);

        let center = bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0);
        if center.x < 0.0 || center.y < 0.0 || center.x >= board_size() || center.y >= board_size()
        {
            commands.entity(entity).despawn();
            continue;
        }

        if *game_mode == GameMode::Campaign && bullet.owner.is_player() {
            let mut hit_enemy = false;
            for (enemy_entity, enemy_tank, enemy, mut health, spawn_protection) in &mut enemy_tanks
            {
                if rects_overlap(
                    bullet.top_left,
                    Vec2::splat(BULLET_SIZE),
                    enemy_tank.top_left,
                    Vec2::splat(TANK_SIZE),
                ) {
                    if spawn_protection.is_some() {
                        commands.entity(entity).despawn();
                        play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                        hit_enemy = true;
                        break;
                    }

                    health.current -= 1;
                    if health.current <= 0 {
                        score_board.score += enemy_score(enemy.kind);
                        score_board.enemies_destroyed += 1;
                        spawn_explosion(&mut commands, &assets, enemy_tank.top_left);
                        play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                        if let Some(powerup_kind) = enemy.carried_powerup {
                            spawn_powerup(
                                &mut commands,
                                &assets,
                                powerup_kind,
                                enemy_tank.top_left,
                                &active_powerups,
                                &active_sparkles,
                            );
                        }
                        commands.entity(enemy_entity).despawn();
                    }
                    commands.entity(entity).despawn();
                    hit_enemy = true;
                    break;
                }
            }
            if hit_enemy {
                continue;
            }
        }

        if *game_mode == GameMode::VersusDeathmatch
            && let Some(shooter) = bullet.owner.player_id()
        {
            let mut hit_player = false;
            for (
                player_entity,
                mut player_tank,
                mut player_transform,
                respawn,
                mut lives,
                mut player_health,
                shield,
                player,
            ) in &mut player_tanks
            {
                if player.id == shooter
                    || !rects_overlap(
                        bullet.top_left,
                        Vec2::splat(BULLET_SIZE),
                        player_tank.top_left,
                        Vec2::splat(TANK_SIZE),
                    )
                {
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
                        respawn,
                        &mut lives,
                        &mut player_health,
                        player.id,
                        Some(shooter),
                        GameMode::VersusDeathmatch,
                    );
                }
                commands.entity(entity).despawn();
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
                respawn,
                mut lives,
                mut player_health,
                shield,
                player,
            ) in &mut player_tanks
            {
                if !rects_overlap(
                    bullet.top_left,
                    Vec2::splat(BULLET_SIZE),
                    player_tank.top_left,
                    Vec2::splat(TANK_SIZE),
                ) {
                    continue;
                }

                if shield.is_some() {
                    commands.entity(entity).despawn();
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
                    respawn,
                    &mut lives,
                    &mut player_health,
                    player.id,
                    None,
                    GameMode::Campaign,
                );
                commands.entity(entity).despawn();
                hit_player = true;
                break;
            }
            if hit_player {
                continue;
            }
        }

        let tile_x = (center.x / TILE_SIZE).floor() as usize;
        let tile_y = (center.y / TILE_SIZE).floor() as usize;
        let tile = grid.tiles[tile_y * BOARD_TILES + tile_x];

        if tile.bullet_blocks() {
            if bullet_destroys_tile(tile, bullet.breaks_steel) {
                grid.set(tile_x, tile_y, TileKind::Empty);
                play_sound(
                    &mut commands,
                    &sounds,
                    if tile == TileKind::Steel {
                        SoundKind::SteelHit
                    } else {
                        SoundKind::BrickHit
                    },
                );
                for (tile_entity, grid_tile) in &tile_sprites {
                    if grid_tile.x == tile_x && grid_tile.y == tile_y {
                        commands.entity(tile_entity).despawn();
                        break;
                    }
                }
            }

            if *game_mode == GameMode::Campaign
                && tile == TileKind::Base
                && game_status.is_playing()
            {
                game_status.phase = GamePhase::GameOver;
                let base_top_left = base_top_left_from_grid(&grid).unwrap_or(Vec2::new(
                    tile_x as f32 * TILE_SIZE,
                    tile_y as f32 * TILE_SIZE,
                ));
                spawn_base_destruction_effect(&mut commands, &assets, base_top_left);
                play_sound(&mut commands, &sounds, SoundKind::BaseDestroyed);
                for mut sprite in &mut base_sprites {
                    sprite.image = assets.base_destroyed.clone();
                }
            } else if tile == TileKind::Steel && !bullet.breaks_steel {
                play_sound(&mut commands, &sounds, SoundKind::SteelHit);
            }

            commands.entity(entity).despawn();
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

fn resolve_player_destroyed(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_status: &mut GameStatus,
    score_board: &mut ScoreBoard,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
    respawn: &RespawnPoint,
    lives: &mut PlayerLives,
    health: &mut Health,
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
                    return;
                }
            }
        }
    }

    tank.top_left = respawn.top_left;
    tank.facing = respawn.facing;
    transform.translation = board_object_center(
        respawn.top_left.x,
        respawn.top_left.y,
        Vec2::splat(TANK_SIZE),
        6.0,
    );
    commands.entity(player_entity).insert(Shield {
        timer: Timer::from_seconds(score_board.respawn_invulnerability_secs, TimerMode::Once),
    });
    commands
        .entity(player_entity)
        .insert(PlayerRespawnDelay::new());
    spawn_spawn_effect(commands, assets, respawn.top_left);
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

fn cancel_colliding_bullets(mut commands: Commands, bullets: Query<(Entity, &Bullet)>) {
    let bullets: Vec<(Entity, Vec2)> = bullets
        .iter()
        .map(|(entity, bullet)| (entity, bullet.top_left))
        .collect();

    for i in 0..bullets.len() {
        for j in (i + 1)..bullets.len() {
            if bullet_positions_overlap(bullets[i].1, bullets[j].1) {
                commands.entity(bullets[i].0).despawn();
                commands.entity(bullets[j].0).despawn();
            }
        }
    }
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
    if *game_mode != GameMode::VersusDeathmatch
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
    game_status: Res<GameStatus>,
    game_mode: Res<GameMode>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    mut tile_grid: ResMut<TileGrid>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    powerups: Query<(Entity, &PowerUp, &Transform)>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    tile_sprites: Query<(Entity, &GridTile)>,
    mut players: Query<
        (Entity, &Tank, &Player, &mut PlayerUpgrade, &mut PlayerLives),
        With<Player>,
    >,
    enemy_tanks: Query<(Entity, &Tank, &EnemyTank), With<EnemyTank>>,
    mut score_board: ResMut<ScoreBoard>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (powerup_entity, powerup, transform) in &powerups {
        let powerup_top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        for (player_entity, tank, player, mut upgrade, mut lives) in &mut players {
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
                }
                PowerUpKind::Helmet => {
                    commands.entity(player_entity).insert(Shield {
                        timer: Timer::from_seconds(HELMET_SECONDS, TimerMode::Once),
                    });
                }
                PowerUpKind::Clock => {
                    enemy_freeze.start();
                }
                PowerUpKind::Grenade => {
                    destroy_visible_enemies(
                        &mut commands,
                        &assets,
                        &sounds,
                        &mut score_board,
                        &enemy_tanks,
                    );
                }
                PowerUpKind::Shovel => {
                    if *game_mode == GameMode::Campaign {
                        reinforce_base_walls(
                            &mut commands,
                            &assets,
                            &mut tile_grid,
                            &tile_sprites,
                            &mut base_reinforcement,
                        );
                    }
                }
                PowerUpKind::Tank => {
                    lives.current += 1;
                    match *game_mode {
                        GameMode::Campaign => {
                            score_board.lives = lives.current;
                        }
                        GameMode::VersusDeathmatch => {
                            score_board.set_player_lives(player.id, lives.current);
                        }
                    }
                }
            }
            commands.entity(powerup_entity).despawn();
            despawn_powerup_sparkles(&mut commands, &active_sparkles);
            play_sound(&mut commands, &sounds, SoundKind::PowerupPickup);
            break;
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
    mut base_reinforcement: ResMut<BaseReinforcement>,
    tile_sprites: Query<(Entity, &GridTile)>,
) {
    if !game_status.is_playing() {
        return;
    }

    enemy_freeze.tick(time.delta());

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

fn update_powerup_visuals(time: Res<Time>, mut powerups: Query<&mut Sprite, With<PowerUp>>) {
    let [r, g, b] = powerup_visual_rgb(time.elapsed_secs());
    for mut sprite in &mut powerups {
        sprite.color = Color::srgb_u8(r, g, b);
    }
}

fn destroy_visible_enemies(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    score_board: &mut ScoreBoard,
    enemy_tanks: &Query<(Entity, &Tank, &EnemyTank), With<EnemyTank>>,
) {
    let mut destroyed_any = false;
    for (enemy_entity, enemy_tank, enemy) in enemy_tanks {
        score_board.score += enemy_score(enemy.kind);
        score_board.enemies_destroyed += 1;
        spawn_explosion(commands, assets, enemy_tank.top_left);
        commands.entity(enemy_entity).despawn();
        destroyed_any = true;
    }

    if destroyed_any {
        play_sound(commands, sounds, SoundKind::TankExplosion);
    }
}

fn reinforce_base_walls(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    base_reinforcement: &mut BaseReinforcement,
) {
    let positions = base_wall_positions(tile_grid);
    if base_reinforcement.saved_tiles.is_empty() {
        base_reinforcement.saved_tiles = positions
            .iter()
            .map(|(x, y)| (*x, *y, tile_grid.tiles[y * BOARD_TILES + x]))
            .collect();
    }

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
    mut animations: Query<(Entity, &mut Sprite, &mut SpriteAnimation)>,
) {
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
    game_status: Res<GameStatus>,
    mut respawning_players: Query<(Entity, &mut PlayerRespawnDelay)>,
) {
    if !game_status.is_playing() {
        return;
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
    mut shielded: Query<
        (Entity, &mut Shield, &mut Sprite),
        (With<Player>, Without<PlayerRespawnDelay>),
    >,
) {
    for (entity, mut shield, mut sprite) in &mut shielded {
        shield.timer.tick(time.delta());
        sprite.color = if shield.timer.elapsed_secs() % 0.25 < 0.125 {
            Color::srgb_u8(160, 220, 255)
        } else {
            Color::WHITE
        };

        if shield.timer.is_finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<Shield>();
        }
    }
}

fn update_enemy_visual_feedback(
    time: Res<Time>,
    mut enemies: Query<(&EnemyTank, &Health, Option<&SpawnProtection>, &mut Sprite)>,
) {
    for (enemy, health, spawn_protection, mut sprite) in &mut enemies {
        sprite.color = enemy_visual_color(
            enemy.kind,
            enemy.carried_powerup,
            health.current,
            time.elapsed_secs(),
            spawn_protection.is_some(),
        );
    }
}

fn check_game_phase(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    sounds: Res<SoundAssets>,
    mut game_status: ResMut<GameStatus>,
    score_board: Res<ScoreBoard>,
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
        game_status.transition_timer.reset();
        match next_phase {
            GamePhase::LevelClear => play_sound(&mut commands, &sounds, SoundKind::LevelClear),
            GamePhase::GameOver => play_sound(&mut commands, &sounds, SoundKind::GameOver),
            _ => {}
        }
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
    mut base_reinforcement: ResMut<BaseReinforcement>,
    game_entities: Query<Entity, With<GameEntity>>,
    banners: Query<Entity, With<PhaseBanner>>,
) {
    if game_status.phase != GamePhase::LevelClear {
        return;
    }

    game_status.transition_timer.tick(time.delta());
    if !game_status.transition_timer.just_finished() {
        return;
    }

    if game_status.stage >= LEVEL_COUNT {
        for entity in &banners {
            commands.entity(entity).despawn();
        }
        game_status.phase = GamePhase::Victory;
        game_status.transition_timer.reset();
        return;
    }

    let next_stage = game_status.stage + 1;
    let level = load_stage_definition(next_stage).expect("next level should load");
    info!("Loaded {}", level.name);
    let new_tile_grid = TileGrid::from_level(&level).expect("next level map should be valid");

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
    );
    play_sound(&mut commands, &sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    *director = EnemyDirector::from_level(&level);
    *stage_rules = StageRules::from_level(&level);
    score_board.enemies_destroyed = 0;
    score_board.total_enemies = level.enemies.len();
    enemy_freeze.reset();
    base_reinforcement.reset();
    game_status.stage = next_stage;
    game_status.phase = GamePhase::Playing;
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
        let text = match glyph.kind {
            StatusValue::Score => match *game_mode {
                GameMode::Campaign => format!("{:06}", score_board.score.min(999_999)),
                GameMode::VersusDeathmatch => format!("{:02}", score_board.p1_score.min(99)),
            },
            StatusValue::Lives => match *game_mode {
                GameMode::Campaign => format!("{}", score_board.lives.clamp(0, 9)),
                GameMode::VersusDeathmatch => format!("{}", score_board.p1_lives.clamp(0, 9)),
            },
            StatusValue::Stage => format!("{:02}", game_status.stage.min(99)),
            StatusValue::P2Score => format!("{:02}", score_board.p2_score.min(99)),
            StatusValue::P2Lives => format!("{}", score_board.p2_lives.clamp(0, 9)),
            StatusValue::Target => format!("{:02}", score_board.target_score.min(99)),
        };

        if let Some(ch) = text.chars().nth(glyph.digit)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = glyph_index(ch);
        }
    }

    let enemies_remaining = score_board
        .total_enemies
        .saturating_sub(score_board.enemies_destroyed);
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

    let Some(lines) = phase_banner_lines(game_status.phase, game_status.winner) else {
        return;
    };
    spawn_phase_text(&mut commands, &assets, lines, 114.5, 9.0);
}

fn phase_text_width(text: &str) -> f32 {
    text.chars().count() as f32 * 6.0 - 1.0
}

fn phase_banner_lines(
    phase: GamePhase,
    winner: Option<PlayerId>,
) -> Option<&'static [&'static str]> {
    match phase {
        GamePhase::ModeSelect | GamePhase::Playing => None,
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

fn toggle_pause_phase(phase: GamePhase) -> GamePhase {
    match phase {
        GamePhase::Playing => GamePhase::Paused,
        GamePhase::Paused => GamePhase::Playing,
        phase => phase,
    }
}

fn other_mode(mode: GameMode) -> GameMode {
    match mode {
        GameMode::Campaign => GameMode::VersusDeathmatch,
        GameMode::VersusDeathmatch => GameMode::Campaign,
    }
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

fn mode_select_option_top_left(mode: GameMode) -> Vec2 {
    match mode {
        GameMode::Campaign => Vec2::new(82.0, 105.0),
        GameMode::VersusDeathmatch => Vec2::new(88.0, 125.0),
    }
}

fn mode_select_cursor_translation(mode: GameMode) -> Vec3 {
    let option = mode_select_option_top_left(mode);
    board_object_center(
        60.0,
        option.y - 4.0 - BOARD_ORIGIN_Y,
        Vec2::splat(TANK_SIZE),
        0.3,
    )
}

fn update_mode_select_cursor(
    cursors: &mut Query<&mut Transform, With<ModeSelectCursor>>,
    selected: GameMode,
) {
    let translation = mode_select_cursor_translation(selected);
    for mut transform in cursors {
        transform.translation = translation;
    }
}

fn update_mode_select_arena_digits(
    glyphs: &mut Query<(&ModeSelectArenaGlyph, &mut Sprite)>,
    arena: usize,
) {
    let text = format!("{:02}", arena.min(99));
    for (glyph, mut sprite) in glyphs {
        if let Some(ch) = text.chars().nth(glyph.digit)
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = glyph_index(ch);
        }
    }
}

fn player_last_direction(control: &PlayerControl, player: PlayerId) -> Direction {
    match player {
        PlayerId::One => control.p1_last_direction,
        PlayerId::Two => control.p2_last_direction,
    }
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
        PlayerId::One => keys.just_pressed(KeyCode::Space),
        PlayerId::Two => {
            keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::ShiftRight)
        }
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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.07, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
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
    active_powerups: &Query<Entity, With<PowerUp>>,
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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
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
        .with_scale(Vec3::splat(WINDOW_SCALE)),
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
            return axis_direction_toward(own_center, base_center);
        }
    }

    if top_left.y < 20.0 {
        Direction::Down
    } else if top_left.x < 80.0 {
        Direction::Right
    } else if top_left.x > 112.0 {
        Direction::Left
    } else {
        current
    }
}

fn enemy_alignment_fire_ready(kind: EnemyKind, elapsed_secs: f32) -> bool {
    elapsed_secs >= enemy_fire_interval(kind) * ENEMY_ALIGNMENT_FIRE_FRACTION
}

fn enemy_fire_slot_available(active_enemy_bullets: usize, active_for_tank: usize) -> bool {
    active_enemy_bullets < ENEMY_BULLET_LIMIT && active_for_tank < ENEMY_BULLET_LIMIT_PER_TANK
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

fn enemy_visual_color(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
) -> Color {
    let [r, g, b] = enemy_display_rgb(kind, carried_powerup, health, elapsed_secs, spawn_protected);
    Color::srgb_u8(r, g, b)
}

fn enemy_display_rgb(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
) -> [u8; 3] {
    if spawn_protected && elapsed_secs % 0.16 < 0.08 {
        return [160, 220, 255];
    }

    enemy_visual_rgb(kind, carried_powerup, health, elapsed_secs)
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

fn validate_level_positions(level: &LevelDefinition, grid: &TileGrid) -> Result<(), String> {
    validate_tank_spawn(grid, "player spawn", &level.player_spawn)?;

    for (index, spawn) in level.enemy_spawns.iter().enumerate() {
        let label = format!("enemy spawn {}", index + 1);
        validate_tank_spawn(grid, &label, spawn)?;
    }

    validate_base_position(grid, &level.base_position)
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

fn validate_base_position(grid: &TileGrid, point: &GridPoint) -> Result<(), String> {
    if point.x >= BOARD_TILES - 1 || point.y >= BOARD_TILES - 1 {
        return Err(format!(
            "base position ({}, {}) must fit a 2x2 base inside the battlefield",
            point.x, point.y
        ));
    }

    for y in point.y..=(point.y + 1) {
        for x in point.x..=(point.x + 1) {
            if grid.get(x as i32, y as i32) != Some(TileKind::Base) {
                return Err(format!(
                    "base position ({}, {}) must cover a 2x2 base tile area",
                    point.x, point.y
                ));
            }
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

fn bullet_positions_overlap(a: Vec2, b: Vec2) -> bool {
    rects_overlap(a, Vec2::splat(BULLET_SIZE), b, Vec2::splat(BULLET_SIZE))
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
    let center_x = translation.x / WINDOW_SCALE + VIRTUAL_WIDTH / 2.0;
    let center_y = VIRTUAL_HEIGHT / 2.0 - translation.y / WINDOW_SCALE;
    Vec2::new(
        center_x - object_size / 2.0 - BOARD_ORIGIN_X,
        center_y - object_size / 2.0 - BOARD_ORIGIN_Y,
    )
}

fn virtual_center_scaled(top_left: Vec2, size: Vec2, z: f32) -> Vec3 {
    let center = top_left + size / 2.0;
    Vec3::new(
        (center.x - VIRTUAL_WIDTH / 2.0) * WINDOW_SCALE,
        (VIRTUAL_HEIGHT / 2.0 - center.y) * WINDOW_SCALE,
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

fn create_sprite_assets(
    images: &mut Assets<Image>,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> SpriteAssets {
    let manifest = load_asset_manifest(ASSET_MANIFEST_PATH).expect("asset manifest should load");
    let terrain_image = images.add(create_terrain_atlas());
    let terrain_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(8),
        TERRAIN_ATLAS_TILES as u32,
        1,
        None,
        None,
    ));

    let tank_image = images.add(create_tank_atlas());
    let tank_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        TANK_ATLAS_TILES as u32,
        1,
        None,
        None,
    ));

    let bullet_image = images.add(create_bullet_atlas());
    let bullet_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(4),
        4,
        1,
        None,
        None,
    ));

    let effect_image = images.add(create_effect_atlas());
    let effect_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        EFFECT_ATLAS_TILES as u32,
        1,
        None,
        None,
    ));

    let powerup_image = images.add(create_powerup_atlas());
    let powerup_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        POWERUP_ATLAS_TILES as u32,
        1,
        None,
        None,
    ));

    let glyph_image = images.add(create_glyph_atlas());
    let glyph_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(5, 7),
        36,
        1,
        None,
        None,
    ));

    let base_intact = images.add(create_base_image(false));
    let base_destroyed = images.add(create_base_image(true));

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
    }
}

fn create_sound_assets(sounds: &mut Assets<RetroSound>) -> SoundAssets {
    SoundAssets {
        fire: sounds.add(make_sweep_sound(0.08, 920.0, 420.0, 0.22)),
        brick_hit: sounds.add(make_noise_sound(0.07, 0.18, 0x1234_5678)),
        steel_hit: sounds.add(make_sweep_sound(0.08, 1380.0, 1780.0, 0.16)),
        tank_explosion: sounds.add(make_noise_sound(0.24, 0.30, 0xBEEF_9001)),
        base_destroyed: sounds.add(make_layered_sound(&[
            SoundNote {
                duration_secs: 0.14,
                frequency: 180.0,
                volume: 0.24,
            },
            SoundNote {
                duration_secs: 0.16,
                frequency: 120.0,
                volume: 0.22,
            },
            SoundNote {
                duration_secs: 0.20,
                frequency: 80.0,
                volume: 0.20,
            },
        ])),
        powerup_pickup: sounds.add(make_layered_sound(&[
            SoundNote {
                duration_secs: 0.05,
                frequency: 660.0,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.05,
                frequency: 880.0,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.08,
                frequency: 1320.0,
                volume: 0.16,
            },
        ])),
        stage_start: sounds.add(make_layered_sound(&[
            SoundNote {
                duration_secs: 0.07,
                frequency: 392.0,
                volume: 0.16,
            },
            SoundNote {
                duration_secs: 0.07,
                frequency: 523.25,
                volume: 0.16,
            },
            SoundNote {
                duration_secs: 0.12,
                frequency: 659.25,
                volume: 0.15,
            },
        ])),
        level_clear: sounds.add(make_layered_sound(&[
            SoundNote {
                duration_secs: 0.08,
                frequency: 523.25,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.08,
                frequency: 659.25,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.08,
                frequency: 783.99,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.18,
                frequency: 1046.5,
                volume: 0.16,
            },
        ])),
        game_over: sounds.add(make_layered_sound(&[
            SoundNote {
                duration_secs: 0.12,
                frequency: 392.0,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.12,
                frequency: 261.63,
                volume: 0.18,
            },
            SoundNote {
                duration_secs: 0.24,
                frequency: 130.81,
                volume: 0.16,
            },
        ])),
    }
}

#[derive(Clone, Copy)]
struct SoundNote {
    duration_secs: f32,
    frequency: f32,
    volume: f32,
}

fn play_sound(commands: &mut Commands, sounds: &SoundAssets, kind: SoundKind) {
    let handle = match kind {
        SoundKind::Fire => sounds.fire.clone(),
        SoundKind::BrickHit => sounds.brick_hit.clone(),
        SoundKind::SteelHit => sounds.steel_hit.clone(),
        SoundKind::TankExplosion => sounds.tank_explosion.clone(),
        SoundKind::BaseDestroyed => sounds.base_destroyed.clone(),
        SoundKind::PowerupPickup => sounds.powerup_pickup.clone(),
        SoundKind::StageStart => sounds.stage_start.clone(),
        SoundKind::LevelClear => sounds.level_clear.clone(),
        SoundKind::GameOver => sounds.game_over.clone(),
    };
    commands.spawn((
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(0.45)),
    ));
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

fn create_terrain_atlas() -> Image {
    let width = 8 * TERRAIN_ATLAS_TILES;
    let mut pixels = vec![0; width * 8 * 4];

    draw_brick(&mut pixels, width, 0);
    draw_steel(&mut pixels, width, 8);
    draw_water(&mut pixels, width, 16, 0);
    draw_water(&mut pixels, width, 24, 1);
    draw_forest(&mut pixels, width, 32);
    draw_ice(&mut pixels, width, 40);

    image_from_pixels(width, 8, pixels)
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

fn create_tank_atlas() -> Image {
    let width = 16 * TANK_ATLAS_TILES;
    let group_stride = 16 * TANK_ANIMATION_FRAMES * 4;
    let mut pixels = vec![0; width * 16 * 4];
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
    image_from_pixels(width, 16, pixels)
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

fn create_bullet_atlas() -> Image {
    let mut pixels = vec![0; 4 * 4 * 4 * 4];
    for x_offset in [0, 4, 8, 12] {
        fill_rect(&mut pixels, 16, x_offset, 0, 4, 4, [248, 248, 216, 255]);
        set_pixel(&mut pixels, 16, x_offset, 0, [128, 112, 64, 255]);
        set_pixel(&mut pixels, 16, x_offset + 3, 3, [128, 112, 64, 255]);
    }
    image_from_pixels(16, 4, pixels)
}

fn create_effect_atlas() -> Image {
    let width = 16 * EFFECT_ATLAS_TILES;
    let mut pixels = vec![0; width * 16 * 4];
    for frame in 0..4 {
        draw_explosion_frame(&mut pixels, width, frame * 16, frame);
    }
    for frame in 0..4 {
        draw_spawn_frame(&mut pixels, width, 64 + frame * 16, frame);
    }
    for frame in 0..4 {
        draw_base_destruction_frame(&mut pixels, width, 128 + frame * 16, frame);
    }
    for frame in 0..4 {
        draw_powerup_sparkle_frame(&mut pixels, width, 192 + frame * 16, frame);
    }
    image_from_pixels(width, 16, pixels)
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

fn create_powerup_atlas() -> Image {
    let mut pixels = vec![0; 16 * 6 * 16 * 4];
    draw_star_powerup(&mut pixels, 96, 0);
    draw_helmet_powerup(&mut pixels, 96, 16);
    draw_clock_powerup(&mut pixels, 96, 32);
    draw_grenade_powerup(&mut pixels, 96, 48);
    draw_shovel_powerup(&mut pixels, 96, 64);
    draw_tank_powerup(&mut pixels, 96, 80);
    image_from_pixels(96, 16, pixels)
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

fn create_glyph_atlas() -> Image {
    let glyph_width = 5;
    let glyph_height = 7;
    let width = glyph_width * GLYPHS.len();
    let mut pixels = vec![0; width * glyph_height * 4];

    for (glyph, ch) in GLYPHS.chars().enumerate() {
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

fn glyph_index(ch: char) -> usize {
    GLYPHS.find(ch).unwrap_or(0)
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
        'I' => [
            "#####", "..#..", "..#..", "..#..", "..#..", "..#..", "#####",
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
        'Y' => [
            "#...#", "#...#", ".#.#.", "..#..", "..#..", "..#..", "..#..",
        ],
        _ => [
            ".....", ".....", ".....", ".....", ".....", ".....", ".....",
        ],
    }
}

fn create_base_image(destroyed: bool) -> Image {
    let mut pixels = vec![0; 16 * 16 * 4];
    if destroyed {
        fill_rect(&mut pixels, 16, 3, 9, 10, 4, [96, 72, 48, 255]);
        fill_rect(&mut pixels, 16, 5, 5, 3, 4, [160, 48, 24, 255]);
        fill_rect(&mut pixels, 16, 9, 4, 2, 6, [184, 88, 32, 255]);
        fill_rect(&mut pixels, 16, 2, 12, 12, 2, [48, 40, 32, 255]);
    } else {
        fill_rect(&mut pixels, 16, 4, 9, 8, 4, [160, 120, 72, 255]);
        fill_rect(&mut pixels, 16, 5, 6, 6, 4, [192, 152, 88, 255]);
        fill_rect(&mut pixels, 16, 7, 3, 2, 4, [224, 192, 112, 255]);
        fill_rect(&mut pixels, 16, 3, 13, 10, 1, [72, 56, 32, 255]);
    }
    image_from_pixels(16, 16, pixels)
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
    const ARENA_1: &str = include_str!("../assets/arenas/arena_01.ron");
    const ARENA_2: &str = include_str!("../assets/arenas/arena_02.ron");
    const ARENA_3: &str = include_str!("../assets/arenas/arena_03.ron");

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
        ]
    }

    fn authored_arenas() -> [(usize, &'static str); ARENA_COUNT] {
        [(1, ARENA_1), (2, ARENA_2), (3, ARENA_3)]
    }

    #[test]
    fn stage_paths_use_three_digit_level_numbers() {
        assert_eq!(stage_path(1), "assets/levels/001.level.ron");
        assert_eq!(stage_path(12), "assets/levels/012.level.ron");
    }

    #[test]
    fn arena_paths_use_two_digit_arena_numbers() {
        assert_eq!(arena_path(1), "assets/arenas/arena_01.ron");
        assert_eq!(arena_path(12), "assets/arenas/arena_12.ron");
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
    fn authored_asset_manifest_matches_generated_atlases() {
        let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
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

        assert_eq!(manifest.powerup_index(PowerUpKind::Star), 0);
        assert_eq!(manifest.powerup_index(PowerUpKind::Helmet), 1);
        assert_eq!(manifest.powerup_index(PowerUpKind::Clock), 2);
        assert_eq!(manifest.powerup_index(PowerUpKind::Grenade), 3);
        assert_eq!(manifest.powerup_index(PowerUpKind::Shovel), 4);
        assert_eq!(manifest.powerup_index(PowerUpKind::Tank), 5);
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
            "powerup_sparkle: (first: 12, last: 16)",
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
            assert!(!level.powerup_carriers.is_empty());
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
    fn authored_arena_file_matches_deathmatch_shape() {
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

            let BattleRules::Deathmatch {
                target_score,
                lives,
                respawn_invulnerability_secs,
            } = arena.battle_rules;
            assert_eq!(target_score, 5);
            assert_eq!(lives, 3);
            assert_eq!(respawn_invulnerability_secs, 2.0);

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
            "(x: 12, y: 0, facing: Down)",
            "(x: 12, y: 24, facing: Down)",
            1,
        );
        assert!(
            parse_level(&blocked_enemy)
                .err()
                .expect("blocked enemy spawn should fail")
                .contains("enemy spawn 2 (12, 24) must fit a tank on passable tiles")
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
    fn tile_grid_uses_expected_passability() {
        let level = parse_level(LEVEL_1).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        assert!(!TileKind::Brick.tank_passable());
        assert!(!TileKind::Water.tank_passable());
        assert!(TileKind::Forest.tank_passable());
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
    fn forest_renders_as_overlay_above_tanks_and_bullets() {
        assert!(TileKind::Forest.tank_passable());
        assert!(terrain_z(TileKind::Forest) > 7.0);
        assert!(terrain_z(TileKind::Forest) < 8.0);
        assert!(terrain_z(TileKind::Brick) < terrain_z(TileKind::Forest));
        assert!(terrain_z(TileKind::Water) < terrain_z(TileKind::Forest));
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
    fn pause_toggle_only_affects_active_or_paused_game() {
        assert_eq!(
            toggle_pause_phase(GamePhase::ModeSelect),
            GamePhase::ModeSelect
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
    fn victory_phase_uses_campaign_clear_banner() {
        assert_eq!(
            phase_banner_lines(GamePhase::Victory, None).expect("victory should show a banner"),
            VICTORY_BANNER_LINES.as_slice()
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
    fn paused_banner_shows_resume_hint() {
        let lines = phase_banner_lines(GamePhase::Paused, None).expect("paused should show banner");
        assert!(lines.contains(&"PRESS ESC"));
    }

    #[test]
    fn phase_banner_text_uses_available_pixel_glyphs() {
        let banners = [
            phase_banner_lines(GamePhase::Paused, None).expect("paused should show a banner"),
            phase_banner_lines(GamePhase::GameOver, None).expect("game over should show a banner"),
            phase_banner_lines(GamePhase::LevelClear, None)
                .expect("level clear should show a banner"),
            phase_banner_lines(GamePhase::RoundOver, Some(PlayerId::One))
                .expect("p1 win should show a banner"),
            phase_banner_lines(GamePhase::RoundOver, Some(PlayerId::Two))
                .expect("p2 win should show a banner"),
            phase_banner_lines(GamePhase::Victory, None).expect("victory should show a banner"),
        ];

        for lines in banners {
            for line in lines {
                assert!(phase_text_width(line) > 0.0);
                for ch in line.chars().filter(|ch| *ch != ' ') {
                    assert!(
                        glyph_pattern(ch).iter().any(|row| row.contains('#')),
                        "glyph {ch} should render"
                    );
                }
            }
        }
    }

    #[test]
    fn game_starts_at_mode_select() {
        assert_eq!(GameStatus::default().phase, GamePhase::ModeSelect);
        assert_eq!(GameStatus::default().arena, DEFAULT_VERSUS_ARENA);
    }

    #[test]
    fn mode_select_toggles_between_campaign_and_battle() {
        assert_eq!(other_mode(GameMode::Campaign), GameMode::VersusDeathmatch);
        assert_eq!(other_mode(GameMode::VersusDeathmatch), GameMode::Campaign);
    }

    #[test]
    fn mode_select_arena_selection_wraps_authored_arenas() {
        assert_eq!(ModeSelect::default().arena, DEFAULT_VERSUS_ARENA);
        assert_eq!(next_arena(1), 2);
        assert_eq!(next_arena(2), 3);
        assert_eq!(next_arena(3), 1);
        assert_eq!(previous_arena(1), 3);
        assert_eq!(previous_arena(2), 1);
        assert_eq!(previous_arena(3), 2);
    }

    #[test]
    fn mode_select_cursor_tracks_selected_option() {
        let campaign = mode_select_cursor_translation(GameMode::Campaign);
        let battle = mode_select_cursor_translation(GameMode::VersusDeathmatch);
        assert_eq!(campaign.x, battle.x);
        assert!(campaign.y > battle.y);
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
        let mut protection = SpawnProtection::enemy();
        assert!(!protection.tick(Duration::from_secs_f32(
            ENEMY_SPAWN_PROTECTION_SECONDS - 0.01
        )));
        assert!(protection.tick(Duration::from_secs_f32(0.02)));
    }

    #[test]
    fn player_respawn_delay_expires_after_spawn_shimmer() {
        let mut delay = PlayerRespawnDelay::new();
        assert!(!delay.tick(Duration::from_secs_f32(PLAYER_RESPAWN_DELAY_SECONDS - 0.01)));
        assert!(delay.tick(Duration::from_secs_f32(0.02)));
    }

    #[test]
    fn spawn_protection_visual_overrides_enemy_feedback_temporarily() {
        assert_eq!(
            enemy_display_rgb(EnemyKind::Armor, Some(PowerUpKind::Star), 1, 0.02, true),
            [160, 220, 255]
        );
        assert_eq!(
            enemy_display_rgb(EnemyKind::Armor, Some(PowerUpKind::Star), 1, 0.10, true),
            [248, 232, 96]
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
    fn bullet_tile_destruction_respects_steel_breaking_flag() {
        assert!(bullet_destroys_tile(TileKind::Brick, false));
        assert!(!bullet_destroys_tile(TileKind::Steel, false));
        assert!(bullet_destroys_tile(TileKind::Steel, true));
        assert!(!bullet_destroys_tile(TileKind::Base, true));
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
    fn generated_retro_sounds_are_short_and_bounded() {
        let sounds = [
            make_sweep_sound(0.08, 920.0, 420.0, 0.22),
            make_noise_sound(0.07, 0.18, 0x1234_5678),
            make_sweep_sound(0.08, 1380.0, 1780.0, 0.16),
            make_noise_sound(0.24, 0.30, 0xBEEF_9001),
            make_layered_sound(&[
                SoundNote {
                    duration_secs: 0.14,
                    frequency: 180.0,
                    volume: 0.24,
                },
                SoundNote {
                    duration_secs: 0.16,
                    frequency: 120.0,
                    volume: 0.22,
                },
            ]),
        ];

        for sound in sounds {
            assert_eq!(sound.sample_rate, SOUND_SAMPLE_RATE);
            assert!(!sound.samples.is_empty());
            assert!(sound.samples.len() <= SOUND_SAMPLE_RATE as usize);
            assert!(sound.samples.iter().all(|sample| sample.abs() <= 1.0));
        }
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

    fn unique_temp_asset_path(name: &str) -> std::path::PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("tank-{nonce}-{name}"))
    }
}
