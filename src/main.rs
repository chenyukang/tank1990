#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::RenderAssetUsages;
use bevy::audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings, Source, Volume};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PresentMode;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

const LEVEL_COUNT: usize = 3;
const LEVEL_CLEAR_DELAY_SECONDS: f32 = 2.0;
const VERSUS_ARENA: usize = 1;

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
const SNAP_DISTANCE: f32 = 2.0;
const GLYPHS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const POWERUP_DROP_INTERVAL: usize = 5;
const HELMET_SECONDS: f32 = 6.0;
const SOUND_SAMPLE_RATE: u32 = 22_050;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerControl::default())
        .insert_resource(GameMode::Campaign)
        .insert_resource(GameStatus::default())
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
                animate_sprites,
                tick_shields,
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
            phase: GamePhase::Playing,
            stage: 1,
            arena: VERSUS_ARENA,
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

#[derive(Resource)]
struct EnemyDirector {
    roster: VecDeque<EnemyKind>,
    spawns: Vec<SpawnPoint>,
    spawn_timer: Timer,
    max_active: usize,
    spawn_cursor: usize,
}

impl EnemyDirector {
    fn from_level(level: &LevelDefinition) -> Self {
        Self {
            roster: level.enemies.iter().copied().collect(),
            spawns: level.enemy_spawns.clone(),
            spawn_timer: Timer::from_seconds(level.spawn_interval_secs, TimerMode::Repeating),
            max_active: level.max_enemies_on_screen,
            spawn_cursor: 0,
        }
    }

    fn inactive() -> Self {
        Self {
            roster: VecDeque::new(),
            spawns: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_active: 0,
            spawn_cursor: 0,
        }
    }
}

#[derive(Resource, Clone)]
struct TileGrid {
    tiles: Vec<TileKind>,
}

impl TileGrid {
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
    spawn_interval_secs: f32,
    max_enemies_on_screen: usize,
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
}

#[derive(Component)]
struct EnemyAi {
    turn_timer: Timer,
    fire_timer: Timer,
}

#[derive(Component)]
struct Tank {
    top_left: Vec2,
    facing: Direction,
    speed: f32,
}

#[derive(Component)]
struct Bullet {
    top_left: Vec2,
    facing: Direction,
    owner: Team,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PowerUpKind {
    Star,
    Helmet,
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
    let level = load_stage_definition(1).expect("level should load");
    info!("Loaded {}", level.name);
    let tile_grid = TileGrid::from_level(&level).expect("level map should be valid");
    let enemy_director = EnemyDirector::from_level(&level);
    let score_board = ScoreBoard::campaign(level.enemies.len());

    spawn_screen_frame(&mut commands, &sprite_assets, GameMode::Campaign);
    spawn_level(
        &mut commands,
        &sprite_assets,
        &level,
        &tile_grid,
        score_board.lives,
    );
    play_sound(&mut commands, &sound_assets, SoundKind::StageStart);

    commands.insert_resource(sprite_assets);
    commands.insert_resource(sound_assets);
    commands.insert_resource(tile_grid);
    commands.insert_resource(enemy_director);
    commands.insert_resource(score_board);
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
    game_entities: Query<Entity, With<GameEntity>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_status.phase = toggle_pause_phase(game_status.phase);
    }

    if keys.just_pressed(KeyCode::KeyM) {
        match *game_mode {
            GameMode::Campaign => start_versus_round(
                &mut commands,
                &assets,
                &sounds,
                &mut game_mode,
                &mut game_status,
                &mut tile_grid,
                &mut director,
                &mut score_board,
                &game_entities,
            ),
            GameMode::VersusDeathmatch => {
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
                    &game_entities,
                );
            }
        }
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
                &game_entities,
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
                &game_entities,
            ),
        }
    }
}

fn restart_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    score_board: &mut ScoreBoard,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let level = load_stage_definition(game_status.stage).expect("level should load");
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
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let arena = load_arena_definition(VERSUS_ARENA).expect("arena should load");
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
    game_status.phase = GamePhase::Playing;
    game_status.arena = VERSUS_ARENA;
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
    parse_level(&contents)
}

fn load_arena(path: &str) -> Result<ArenaDefinition, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    parse_arena(&contents)
}

fn parse_level(contents: &str) -> Result<LevelDefinition, String> {
    let level: LevelDefinition =
        ron::from_str(contents).map_err(|err| format!("failed to parse level: {err}"))?;

    TileGrid::from_level(&level)?;
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

    Ok(level)
}

fn parse_arena(contents: &str) -> Result<ArenaDefinition, String> {
    let arena: ArenaDefinition =
        ron::from_str(contents).map_err(|err| format!("failed to parse arena: {err}"))?;

    TileGrid::from_arena(&arena)?;
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
    for point in &arena.powerup_spawns {
        if point.x >= BOARD_TILES || point.y >= BOARD_TILES {
            return Err(format!(
                "powerup spawn ({}, {}) is outside the battlefield",
                point.x, point.y
            ));
        }
    }

    Ok(arena)
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
    text: &str,
    top_left: Vec2,
    z: f32,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(48, 48, 40),
            Vec2::new(132.0 * WINDOW_SCALE, 17.0 * WINDOW_SCALE),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(36.0, top_left.y - 5.0),
            Vec2::new(132.0, 17.0),
            z - 0.1,
        )),
        PhaseBanner,
        GameEntity,
    ));
    spawn_pixel_text_inner(commands, assets, text, top_left, z, true);
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
            if let Some(index) = terrain_sprite_index(tile) {
                commands.spawn((
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
            }
        }
    }
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
                index: tank_sprite_index(player_id.team(), spawn.facing),
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
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    mut tank_queries: ParamSet<(
        Query<&Tank>,
        Query<(&mut Tank, &mut Sprite, &mut Transform, &Player), With<Player>>,
    )>,
) {
    if !game_status.is_playing() {
        return;
    }

    let occupied: Vec<Vec2> = tank_queries.p0().iter().map(|tank| tank.top_left).collect();

    for (mut tank, mut sprite, mut transform, player) in &mut tank_queries.p1() {
        let Some(direction) =
            held_direction(&keys, player_last_direction(&control, player.id), player.id)
        else {
            continue;
        };

        tank.facing = direction;
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = tank_sprite_index(player.id.team(), direction);
        }

        let mut next = tank.top_left;
        snap_to_lane(&mut next, direction);
        next += direction.movement() * tank.speed * time.delta_secs();
        next = round_vec2(next);

        if grid.can_tank_occupy(next) && tank_position_free(next, tank.top_left, &occupied) {
            tank.top_left = next;
            transform.translation =
                board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    mut director: ResMut<EnemyDirector>,
    active_enemies: Query<&EnemyTank>,
    tanks: Query<&Tank>,
) {
    if !game_status.is_playing()
        || director.roster.is_empty()
        || active_enemies.iter().count() >= director.max_active
    {
        return;
    }

    let first_spawn = director.roster.len() == 20;
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

        let kind = director
            .roster
            .pop_front()
            .expect("checked non-empty roster above");

        commands.spawn((
            Sprite::from_atlas_image(
                assets.tank_image.clone(),
                TextureAtlas {
                    layout: assets.tank_layout.clone(),
                    index: tank_sprite_index(Team::Enemy, spawn.facing),
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
            EnemyTank { kind },
            EnemyAi {
                turn_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
                fire_timer: Timer::from_seconds(enemy_fire_interval(kind), TimerMode::Repeating),
            },
            GameEntity,
        ));
        spawn_spawn_effect(&mut commands, &assets, top_left);
        break;
    }
}

fn move_enemy_tanks(
    time: Res<Time>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    mut tank_queries: ParamSet<(
        Query<(&Tank, Option<&Player>)>,
        Query<
            (&mut Tank, &mut Sprite, &mut Transform, &mut EnemyAi),
            (With<EnemyTank>, Without<Player>),
        >,
    )>,
) {
    if !game_status.is_playing() {
        return;
    }

    let occupied: Vec<(Vec2, bool)> = tank_queries
        .p0()
        .iter()
        .map(|(tank, player)| (tank.top_left, player.is_some()))
        .collect();
    let player_top_left = occupied
        .iter()
        .find_map(|(top_left, is_player)| is_player.then_some(*top_left));
    let occupied_positions: Vec<Vec2> = occupied.iter().map(|(top_left, _)| *top_left).collect();

    for (mut tank, mut sprite, mut transform, mut ai) in &mut tank_queries.p1() {
        ai.turn_timer.tick(time.delta());
        if ai.turn_timer.just_finished() {
            tank.facing = preferred_enemy_direction(tank.top_left, tank.facing, player_top_left);
        }

        let mut next = tank.top_left;
        snap_to_lane(&mut next, tank.facing);
        next += tank.facing.movement() * tank.speed * time.delta_secs();
        next = round_vec2(next);

        if grid.can_tank_occupy(next)
            && tank_position_free(next, tank.top_left, &occupied_positions)
        {
            tank.top_left = next;
            transform.translation =
                board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
        } else {
            tank.facing = next_direction(tank.facing);
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = tank_sprite_index(Team::Enemy, tank.facing);
        }
    }
}

fn fire_enemy_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    enemy_bullets: Query<&Bullet>,
    mut enemies: Query<(&Tank, &EnemyTank, &mut EnemyAi)>,
) {
    if !game_status.is_playing()
        || enemy_bullets
            .iter()
            .filter(|bullet| bullet.owner == Team::Enemy)
            .count()
            >= 4
    {
        return;
    }

    for (tank, enemy, mut ai) in &mut enemies {
        ai.fire_timer.tick(time.delta());
        if !ai.fire_timer.just_finished() {
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
                owner: Team::Enemy,
            },
            GameEntity,
        ));
        play_sound(&mut commands, &sounds, SoundKind::Fire);

        if enemy.kind == EnemyKind::Power {
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
    players: Query<(&Tank, &PlayerUpgrade, &Player), With<Player>>,
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
    mut base_sprites: Query<&mut Sprite, With<BaseSprite>>,
    mut enemy_tanks: Query<
        (Entity, &Tank, &EnemyTank, &mut Health),
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
        bullet.top_left += facing.movement() * BULLET_SPEED * time.delta_secs();
        bullet.top_left = round_vec2(bullet.top_left);

        let center = bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0);
        if center.x < 0.0 || center.y < 0.0 || center.x >= board_size() || center.y >= board_size()
        {
            commands.entity(entity).despawn();
            continue;
        }

        if *game_mode == GameMode::Campaign && bullet.owner.is_player() {
            let mut hit_enemy = false;
            for (enemy_entity, enemy_tank, enemy, mut health) in &mut enemy_tanks {
                if rects_overlap(
                    bullet.top_left,
                    Vec2::splat(BULLET_SIZE),
                    enemy_tank.top_left,
                    Vec2::splat(TANK_SIZE),
                ) {
                    health.current -= 1;
                    if health.current <= 0 {
                        score_board.score += enemy_score(enemy.kind);
                        score_board.enemies_destroyed += 1;
                        spawn_explosion(&mut commands, &assets, enemy_tank.top_left);
                        play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                        if should_drop_powerup(score_board.enemies_destroyed) {
                            spawn_powerup(
                                &mut commands,
                                &assets,
                                powerup_for_drop(score_board.enemies_destroyed),
                                enemy_tank.top_left,
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
            if tile == TileKind::Brick {
                grid.set(tile_x, tile_y, TileKind::Empty);
                play_sound(&mut commands, &sounds, SoundKind::BrickHit);
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
                spawn_explosion(
                    &mut commands,
                    &assets,
                    Vec2::new(tile_x as f32 * TILE_SIZE, tile_y as f32 * TILE_SIZE),
                );
                play_sound(&mut commands, &sounds, SoundKind::BaseDestroyed);
                for mut sprite in &mut base_sprites {
                    sprite.image = assets.base_destroyed.clone();
                }
            } else if tile == TileKind::Steel {
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

fn pickup_powerups(
    mut commands: Commands,
    game_status: Res<GameStatus>,
    sounds: Res<SoundAssets>,
    powerups: Query<(Entity, &PowerUp, &Transform)>,
    mut players: Query<(Entity, &Tank, &mut PlayerUpgrade), With<Player>>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (powerup_entity, powerup, transform) in &powerups {
        let powerup_top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        for (player_entity, tank, mut upgrade) in &mut players {
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
            }
            commands.entity(powerup_entity).despawn();
            play_sound(&mut commands, &sounds, SoundKind::PowerupPickup);
            break;
        }
    }
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

fn tick_shields(
    mut commands: Commands,
    time: Res<Time>,
    mut shielded: Query<(Entity, &mut Shield, &mut Sprite), With<Player>>,
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
    score_board.enemies_destroyed = 0;
    score_board.total_enemies = level.enemies.len();
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

    let message = match game_status.phase {
        GamePhase::Playing => return,
        GamePhase::Paused => "PAUSED",
        GamePhase::GameOver => "GAME OVER",
        GamePhase::LevelClear => "LEVEL CLEAR",
        GamePhase::RoundOver => match game_status.winner {
            Some(PlayerId::One) => "P1 WIN",
            Some(PlayerId::Two) => "P2 WIN",
            None => "GAME OVER",
        },
        GamePhase::Victory => "VICTORY",
    };
    let text_width = message.chars().count() as f32 * 6.0 - 1.0;
    spawn_phase_text(
        &mut commands,
        &assets,
        message,
        Vec2::new((208.0 - text_width) / 2.0, 111.0),
        9.0,
    );
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
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: 0,
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
            first: 0,
            last: 3,
            timer: Timer::from_seconds(0.07, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

fn spawn_spawn_effect(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: 4,
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
            first: 4,
            last: 7,
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
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
) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.powerup_image.clone(),
            TextureAtlas {
                layout: assets.powerup_layout.clone(),
                index: powerup_sprite_index(kind),
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
}

fn preferred_enemy_direction(
    top_left: Vec2,
    current: Direction,
    player_top_left: Option<Vec2>,
) -> Direction {
    if let Some(player) = player_top_left {
        let delta = player - top_left;
        if delta.x.abs() > delta.y.abs() && delta.x.abs() > 24.0 {
            return if delta.x < 0.0 {
                Direction::Left
            } else {
                Direction::Right
            };
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

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
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

fn enemy_score(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Basic => 100,
        EnemyKind::Fast => 200,
        EnemyKind::Power => 300,
        EnemyKind::Armor => 400,
    }
}

fn player_bullet_limit(upgrade_level: u8) -> usize {
    if upgrade_level >= 2 { 2 } else { 1 }
}

fn should_drop_powerup(enemies_destroyed: usize) -> bool {
    enemies_destroyed > 0 && enemies_destroyed.is_multiple_of(POWERUP_DROP_INTERVAL)
}

fn powerup_for_drop(enemies_destroyed: usize) -> PowerUpKind {
    if (enemies_destroyed / POWERUP_DROP_INTERVAL).is_multiple_of(2) {
        PowerUpKind::Helmet
    } else {
        PowerUpKind::Star
    }
}

fn powerup_sprite_index(kind: PowerUpKind) -> usize {
    match kind {
        PowerUpKind::Star => 0,
        PowerUpKind::Helmet => 1,
    }
}

fn tank_sprite_index(team: Team, direction: Direction) -> usize {
    let base = match team {
        Team::Player1 => 0,
        Team::Player2 => 4,
        Team::Enemy => 4,
    };
    base + direction.tank_sprite_index()
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
        TileKind::Forest => 5.0,
        TileKind::Water => 1.0,
        _ => 2.0,
    }
}

fn terrain_sprite_index(tile: TileKind) -> Option<usize> {
    match tile {
        TileKind::Brick => Some(0),
        TileKind::Steel => Some(1),
        TileKind::Water => Some(2),
        TileKind::Forest => Some(3),
        TileKind::Ice => Some(4),
        TileKind::Empty | TileKind::Base => None,
    }
}

fn create_sprite_assets(
    images: &mut Assets<Image>,
    atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> SpriteAssets {
    let terrain_image = images.add(create_terrain_atlas());
    let terrain_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(8),
        5,
        1,
        None,
        None,
    ));

    let tank_image = images.add(create_tank_atlas());
    let tank_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        8,
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
        8,
        1,
        None,
        None,
    ));

    let powerup_image = images.add(create_powerup_atlas());
    let powerup_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        2,
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
    let mut pixels = vec![0; 8 * 5 * 8 * 4];

    draw_brick(&mut pixels, 40, 0);
    draw_steel(&mut pixels, 40, 8);
    draw_water(&mut pixels, 40, 16);
    draw_forest(&mut pixels, 40, 24);
    draw_ice(&mut pixels, 40, 32);

    image_from_pixels(40, 8, pixels)
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

fn draw_water(pixels: &mut [u8], width: usize, x_offset: usize) {
    fill_rect(pixels, width, x_offset, 0, 8, 8, [24, 64, 144, 255]);
    for y in [1, 4, 6] {
        for x in 0..8 {
            if (x + y) % 3 != 0 {
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
    let mut pixels = vec![0; 16 * 8 * 16 * 4];
    let player_palette = TankPalette {
        dark: [48, 56, 24, 255],
        body: [184, 160, 64, 255],
        light: [240, 216, 104, 255],
        tread: [88, 80, 40, 255],
    };
    let enemy_palette = TankPalette {
        dark: [64, 24, 24, 255],
        body: [176, 56, 40, 255],
        light: [232, 104, 72, 255],
        tread: [88, 40, 32, 255],
    };

    draw_tank(&mut pixels, 128, 0, Direction::Up, player_palette);
    draw_tank(&mut pixels, 128, 16, Direction::Down, player_palette);
    draw_tank(&mut pixels, 128, 32, Direction::Left, player_palette);
    draw_tank(&mut pixels, 128, 48, Direction::Right, player_palette);
    draw_tank(&mut pixels, 128, 64, Direction::Up, enemy_palette);
    draw_tank(&mut pixels, 128, 80, Direction::Down, enemy_palette);
    draw_tank(&mut pixels, 128, 96, Direction::Left, enemy_palette);
    draw_tank(&mut pixels, 128, 112, Direction::Right, enemy_palette);
    image_from_pixels(128, 16, pixels)
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
) {
    fill_rect(pixels, width, x_offset + 2, 2, 4, 12, palette.tread);
    fill_rect(pixels, width, x_offset + 10, 2, 4, 12, palette.tread);
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
    let mut pixels = vec![0; 16 * 8 * 16 * 4];
    for frame in 0..4 {
        draw_explosion_frame(&mut pixels, 128, frame * 16, frame);
    }
    for frame in 0..4 {
        draw_spawn_frame(&mut pixels, 128, 64 + frame * 16, frame);
    }
    image_from_pixels(128, 16, pixels)
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

fn create_powerup_atlas() -> Image {
    let mut pixels = vec![0; 16 * 2 * 16 * 4];
    draw_star_powerup(&mut pixels, 32, 0);
    draw_helmet_powerup(&mut pixels, 32, 16);
    image_from_pixels(32, 16, pixels)
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

    const LEVEL_1: &str = include_str!("../assets/levels/001.level.ron");
    const LEVEL_2: &str = include_str!("../assets/levels/002.level.ron");
    const LEVEL_3: &str = include_str!("../assets/levels/003.level.ron");
    const ARENA_1: &str = include_str!("../assets/arenas/arena_01.ron");

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
    fn authored_level_files_match_classic_shape() {
        for (stage, contents) in [(1, LEVEL_1), (2, LEVEL_2), (3, LEVEL_3)] {
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
        }
    }

    #[test]
    fn authored_arena_file_matches_deathmatch_shape() {
        let arena = parse_arena(ARENA_1).expect("arena should parse");
        assert_eq!(arena.name, "Arena 1");
        assert_eq!(arena.map.len(), BOARD_TILES);
        assert!(
            arena
                .map
                .iter()
                .all(|row| row.chars().count() == BOARD_TILES)
        );
        assert_eq!(arena.powerup_spawns.len(), 1);

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
    fn player_bullet_limit_increases_after_star_upgrades() {
        assert_eq!(player_bullet_limit(0), 1);
        assert_eq!(player_bullet_limit(1), 1);
        assert_eq!(player_bullet_limit(2), 2);
        assert_eq!(player_bullet_limit(3), 2);
    }

    #[test]
    fn powerups_drop_on_classic_carrier_cadence() {
        assert!(!should_drop_powerup(0));
        assert!(!should_drop_powerup(4));
        assert!(should_drop_powerup(5));
        assert_eq!(powerup_for_drop(5), PowerUpKind::Star);
        assert_eq!(powerup_for_drop(10), PowerUpKind::Helmet);
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
}
