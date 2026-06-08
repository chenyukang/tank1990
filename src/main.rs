use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PresentMode;
use serde::Deserialize;
use std::collections::VecDeque;
use std::fs;

const LEVEL_COUNT: usize = 3;
const LEVEL_CLEAR_DELAY_SECONDS: f32 = 2.0;

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

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerControl::default())
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

#[derive(Resource, Default)]
struct PlayerControl {
    last_direction: Direction,
}

#[derive(Resource)]
struct GameStatus {
    phase: GamePhase,
    stage: usize,
    transition_timer: Timer,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self {
            phase: GamePhase::Playing,
            stage: 1,
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
    Victory,
}

#[derive(Resource)]
struct ScoreBoard {
    score: u32,
    lives: i32,
    enemies_destroyed: usize,
    total_enemies: usize,
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
}

#[derive(Resource, Clone)]
struct TileGrid {
    tiles: Vec<TileKind>,
}

impl TileGrid {
    fn from_level(level: &LevelDefinition) -> Result<Self, String> {
        if level.map.len() != BOARD_TILES {
            return Err(format!(
                "expected {BOARD_TILES} map rows, got {}",
                level.map.len()
            ));
        }

        let mut tiles = Vec::with_capacity(BOARD_TILES * BOARD_TILES);
        for (y, row) in level.map.iter().enumerate() {
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
struct Player;

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
    Player,
    Enemy,
}

#[derive(Component)]
struct Health {
    current: i32,
}

#[derive(Component)]
struct RespawnPoint(Vec2);

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
) {
    commands.spawn(Camera2d);

    let sprite_assets = create_sprite_assets(&mut images, &mut atlas_layouts);
    let level = load_stage_definition(1).expect("level should load");
    info!("Loaded {}", level.name);
    let tile_grid = TileGrid::from_level(&level).expect("level map should be valid");
    let enemy_director = EnemyDirector::from_level(&level);
    let score_board = ScoreBoard {
        score: 0,
        lives: 3,
        enemies_destroyed: 0,
        total_enemies: level.enemies.len(),
    };

    spawn_screen_frame(&mut commands, &sprite_assets);
    spawn_level(
        &mut commands,
        &sprite_assets,
        &level,
        &tile_grid,
        score_board.lives,
    );

    commands.insert_resource(sprite_assets);
    commands.insert_resource(tile_grid);
    commands.insert_resource(enemy_director);
    commands.insert_resource(score_board);
}

fn handle_shared_controls(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    mut game_status: ResMut<GameStatus>,
    mut tile_grid: ResMut<TileGrid>,
    mut director: ResMut<EnemyDirector>,
    mut score_board: ResMut<ScoreBoard>,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_status.phase = toggle_pause_phase(game_status.phase);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        restart_level(
            &mut commands,
            &assets,
            &mut game_status,
            &mut tile_grid,
            &mut director,
            &mut score_board,
            &game_entities,
        );
    }
}

fn restart_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
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

    spawn_screen_frame(commands, assets);
    spawn_level(commands, assets, &level, &new_tile_grid, 3);

    *tile_grid = new_tile_grid;
    *director = EnemyDirector::from_level(&level);
    *score_board = ScoreBoard {
        score: 0,
        lives: 3,
        enemies_destroyed: 0,
        total_enemies: level.enemies.len(),
    };
    game_status.phase = GamePhase::Playing;
    game_status.transition_timer.reset();
}

fn stage_path(stage: usize) -> String {
    format!("assets/levels/{stage:03}.level.ron")
}

fn load_stage_definition(stage: usize) -> Result<LevelDefinition, String> {
    load_level(&stage_path(stage))
}

fn load_level(path: &str) -> Result<LevelDefinition, String> {
    let contents =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    parse_level(&contents)
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

fn spawn_screen_frame(commands: &mut Commands, assets: &SpriteAssets) {
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

    let player_top_left = Vec2::new(
        level.player_spawn.x as f32 * TILE_SIZE,
        level.player_spawn.y as f32 * TILE_SIZE,
    );

    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: tank_sprite_index(Team::Player, level.player_spawn.facing),
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
            facing: level.player_spawn.facing,
            speed: PLAYER_SPEED,
        },
        Health { current: 1 },
        RespawnPoint(player_top_left),
        PlayerLives {
            current: player_lives,
        },
        PlayerUpgrade { level: 0 },
        Player,
        GameEntity,
    ));
}

fn update_player_control(keys: Res<ButtonInput<KeyCode>>, mut control: ResMut<PlayerControl>) {
    for (key, direction) in [
        (KeyCode::KeyW, Direction::Up),
        (KeyCode::ArrowUp, Direction::Up),
        (KeyCode::KeyS, Direction::Down),
        (KeyCode::ArrowDown, Direction::Down),
        (KeyCode::KeyA, Direction::Left),
        (KeyCode::ArrowLeft, Direction::Left),
        (KeyCode::KeyD, Direction::Right),
        (KeyCode::ArrowRight, Direction::Right),
    ] {
        if keys.just_pressed(key) {
            control.last_direction = direction;
        }
    }
}

fn move_player_tank(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    control: Res<PlayerControl>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    mut player: Query<(&mut Tank, &mut Sprite, &mut Transform), With<Player>>,
    enemies: Query<&Tank, (With<EnemyTank>, Without<Player>)>,
) {
    if !game_status.is_playing() {
        return;
    }

    let Ok((mut tank, mut sprite, mut transform)) = player.single_mut() else {
        return;
    };
    let Some(direction) = held_direction(&keys, control.last_direction) else {
        return;
    };

    tank.facing = direction;
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.index = tank_sprite_index(Team::Player, direction);
    }

    let mut next = tank.top_left;
    snap_to_lane(&mut next, direction);
    next += direction.movement() * tank.speed * time.delta_secs();
    next = round_vec2(next);

    let occupied: Vec<Vec2> = enemies.iter().map(|tank| tank.top_left).collect();
    if grid.can_tank_occupy(next) && tank_position_free(next, tank.top_left, &occupied) {
        tank.top_left = next;
        transform.translation = board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
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

        if enemy.kind == EnemyKind::Power {
            break;
        }
    }
}

fn fire_player_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    game_status: Res<GameStatus>,
    player: Query<(&Tank, &PlayerUpgrade), With<Player>>,
    bullets: Query<&Bullet>,
) {
    if !game_status.is_playing()
        || !(keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter))
    {
        return;
    }

    let Ok((tank, upgrade)) = player.single() else {
        return;
    };
    let active_player_bullets = bullets
        .iter()
        .filter(|bullet| bullet.owner == Team::Player)
        .count();
    if active_player_bullets >= player_bullet_limit(upgrade.level) {
        return;
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
            owner: Team::Player,
        },
        GameEntity,
    ));
}

fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
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
            &mut Tank,
            &mut Transform,
            &RespawnPoint,
            &mut PlayerLives,
            &mut Health,
            Option<&Shield>,
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

        if bullet.owner == Team::Player {
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
        } else {
            let Ok((
                mut player_tank,
                mut player_transform,
                respawn,
                mut lives,
                mut player_health,
                shield,
            )) = player_tanks.single_mut()
            else {
                continue;
            };

            if rects_overlap(
                bullet.top_left,
                Vec2::splat(BULLET_SIZE),
                player_tank.top_left,
                Vec2::splat(TANK_SIZE),
            ) {
                if shield.is_some() {
                    commands.entity(entity).despawn();
                    continue;
                }

                spawn_explosion(&mut commands, &assets, player_tank.top_left);
                lives.current -= 1;
                score_board.lives = lives.current;
                if lives.current <= 0 {
                    game_status.phase = GamePhase::GameOver;
                } else {
                    player_health.current = 1;
                    player_tank.top_left = respawn.0;
                    player_tank.facing = Direction::Up;
                    player_transform.translation =
                        board_object_center(respawn.0.x, respawn.0.y, Vec2::splat(TANK_SIZE), 6.0);
                }
                commands.entity(entity).despawn();
                continue;
            }
        }

        let tile_x = (center.x / TILE_SIZE).floor() as usize;
        let tile_y = (center.y / TILE_SIZE).floor() as usize;
        let tile = grid.tiles[tile_y * BOARD_TILES + tile_x];

        if tile.bullet_blocks() {
            if tile == TileKind::Brick {
                grid.set(tile_x, tile_y, TileKind::Empty);
                for (tile_entity, grid_tile) in &tile_sprites {
                    if grid_tile.x == tile_x && grid_tile.y == tile_y {
                        commands.entity(tile_entity).despawn();
                        break;
                    }
                }
            }

            if tile == TileKind::Base && game_status.is_playing() {
                game_status.phase = GamePhase::GameOver;
                spawn_explosion(
                    &mut commands,
                    &assets,
                    Vec2::new(tile_x as f32 * TILE_SIZE, tile_y as f32 * TILE_SIZE),
                );
                for mut sprite in &mut base_sprites {
                    sprite.image = assets.base_destroyed.clone();
                }
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
    powerups: Query<(Entity, &PowerUp, &Transform)>,
    mut players: Query<(Entity, &Tank, &mut PlayerUpgrade), With<Player>>,
) {
    if !game_status.is_playing() {
        return;
    }

    let Ok((player_entity, tank, mut upgrade)) = players.single_mut() else {
        return;
    };

    for (powerup_entity, powerup, transform) in &powerups {
        let powerup_top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
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
    mut game_status: ResMut<GameStatus>,
    score_board: Res<ScoreBoard>,
    director: Res<EnemyDirector>,
    active_enemies: Query<&EnemyTank>,
) {
    if !game_status.is_playing() {
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
    }
}

fn advance_after_level_clear(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
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

    spawn_screen_frame(&mut commands, &assets);
    spawn_level(
        &mut commands,
        &assets,
        &level,
        &new_tile_grid,
        score_board.lives.max(1),
    );

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
    game_status: Res<GameStatus>,
    score_board: Res<ScoreBoard>,
    mut glyphs: Query<(&StatusGlyph, &mut Sprite)>,
    mut markers: Query<(&EnemyMarker, &mut Visibility)>,
    banners: Query<Entity, With<PhaseBanner>>,
) {
    for (glyph, mut sprite) in &mut glyphs {
        let text = match glyph.kind {
            StatusValue::Score => format!("{:06}", score_board.score.min(999_999)),
            StatusValue::Lives => format!("{}", score_board.lives.clamp(0, 9)),
            StatusValue::Stage => format!("{:02}", game_status.stage.min(99)),
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

fn held_direction(keys: &ButtonInput<KeyCode>, last_direction: Direction) -> Option<Direction> {
    if direction_is_held(keys, last_direction) {
        return Some(last_direction);
    }

    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .find(|direction| direction_is_held(keys, *direction))
}

fn direction_is_held(keys: &ButtonInput<KeyCode>, direction: Direction) -> bool {
    match direction {
        Direction::Up => keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp),
        Direction::Down => keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown),
        Direction::Left => keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft),
        Direction::Right => keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight),
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
    enemies_destroyed > 0 && enemies_destroyed % POWERUP_DROP_INTERVAL == 0
}

fn powerup_for_drop(enemies_destroyed: usize) -> PowerUpKind {
    if enemies_destroyed / POWERUP_DROP_INTERVAL % 2 == 0 {
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
        Team::Player => 0,
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
    let radius = [2, 4, 6, 7][frame] as i32;
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
        'V' => [
            "#...#", "#...#", "#...#", "#...#", "#...#", ".#.#.", "..#..",
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

    #[test]
    fn stage_paths_use_three_digit_level_numbers() {
        assert_eq!(stage_path(1), "assets/levels/001.level.ron");
        assert_eq!(stage_path(12), "assets/levels/012.level.ron");
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
        assert_eq!(toggle_pause_phase(GamePhase::Victory), GamePhase::Victory);
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
