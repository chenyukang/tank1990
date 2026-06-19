use super::*;

#[derive(Component)]
struct OldStageEntity;

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
const LEVEL_46: &str = include_str!("../assets/levels/046.level.ron");
const LEVEL_47: &str = include_str!("../assets/levels/047.level.ron");
const LEVEL_48: &str = include_str!("../assets/levels/048.level.ron");
const LEVEL_49: &str = include_str!("../assets/levels/049.level.ron");
const LEVEL_50: &str = include_str!("../assets/levels/050.level.ron");
const ARENA_1: &str = include_str!("../assets/arenas/arena_01.ron");
const ARENA_2: &str = include_str!("../assets/arenas/arena_02.ron");
const ARENA_3: &str = include_str!("../assets/arenas/arena_03.ron");
const ARENA_4: &str = include_str!("../assets/arenas/arena_04.ron");
const ARENA_5: &str = include_str!("../assets/arenas/arena_05.ron");
const ARENA_6: &str = include_str!("../assets/arenas/arena_06.ron");
const ARENA_7: &str = include_str!("../assets/arenas/arena_07.ron");
const ARENA_8: &str = include_str!("../assets/arenas/arena_08.ron");
const GITIGNORE: &str = include_str!("../.gitignore");
const TEST_SPAWN_INVULNERABILITY_SECONDS: f32 = 3.25;

fn image_pixel(image: &Image, x: usize, y: usize) -> [u8; 4] {
    let width = image.texture_descriptor.size.width as usize;
    let pixels = image.data.as_ref().expect("image should have pixel data");
    let index = (y * width + x) * 4;
    pixels[index..index + 4]
        .try_into()
        .expect("pixel should have four channels")
}

fn pixels_pixel(pixels: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
    let index = (y * width + x) * 4;
    pixels[index..index + 4]
        .try_into()
        .expect("pixel should have four channels")
}

fn replace_fixture_once(contents: &str, from: &str, to: &str) -> String {
    let normalized = contents.replace("\r\n", "\n");
    assert!(
        normalized.contains(from),
        "fixture replacement pattern should match"
    );
    normalized.replacen(from, to, 1)
}

#[test]
fn fixture_replacement_matches_crlf_inputs() {
    assert_eq!(
        replace_fixture_once("head\r\nbody\r\ntail", "head\nbody", "top"),
        "top\ntail"
    );
}

fn authored_levels() -> [(usize, &'static str); CUSTOM_LEVEL_COUNT] {
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
        (46, LEVEL_46),
        (47, LEVEL_47),
        (48, LEVEL_48),
        (49, LEVEL_49),
        (50, LEVEL_50),
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
        (7, ARENA_7),
        (8, ARENA_8),
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
        stage_flag_icon: image.clone(),
        shield_image: image,
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
        generated_background_music: sound,
        custom_background_music: None,
    }
}

fn test_sound_assets_with_custom_music() -> SoundAssets {
    let mut sounds = test_sound_assets();
    sounds.custom_background_music = Some(SoundHandle::Retro(Handle::<RetroSound>::default()));
    sounds
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

fn spawn_movable_test_player(world: &mut World, id: PlayerId, top_left: Vec2, facing: Direction) {
    world.spawn((
        Tank {
            top_left,
            facing,
            speed: PLAYER_SPEED,
        },
        TankSpriteState::new(TankSpriteSet::player(id)),
        Player { id },
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

fn spawn_test_enemy_tank(
    world: &mut World,
    kind: EnemyKind,
    top_left: Vec2,
    facing: Direction,
) -> Entity {
    world
        .spawn((
            Tank {
                top_left,
                facing,
                speed: enemy_speed(kind),
            },
            EnemyTank {
                kind,
                carried_powerup: None,
            },
            Health {
                current: enemy_health(kind),
            },
            EnemyAi {
                turn_timer: Timer::from_seconds(enemy_turn_interval(kind), TimerMode::Repeating),
                fire_timer: Timer::from_seconds(enemy_fire_interval(kind), TimerMode::Repeating),
                strategy: EnemyAiStrategy::default(),
                difficulty_profile: EnemyDifficultyProfile::default(),
            },
            TankSpriteState::new(TankSpriteSet::enemy(kind)),
            Transform::from_translation(board_object_center(
                top_left.x,
                top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            )),
            Sprite::default(),
        ))
        .id()
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

fn spawn_overlay_effects_for_test(mut commands: Commands, assets: Res<SpriteAssets>) {
    spawn_bullet_impact_effect(&mut commands, &assets, Vec2::new(16.0, 16.0));
    spawn_explosion(&mut commands, &assets, Vec2::new(32.0, 16.0));
    spawn_spawn_effect(&mut commands, &assets, Vec2::new(48.0, 16.0));
    spawn_base_destruction_effect(&mut commands, &assets, Vec2::new(64.0, 16.0));
}

fn spawn_player_with_initial_shield_for_test(mut commands: Commands, assets: Res<SpriteAssets>) {
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

fn spawn_campaign_powerup_for_test(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    active_powerups: Query<Entity, With<PowerUp>>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
) {
    spawn_powerup(
        &mut commands,
        &assets,
        PowerUpKind::Helmet,
        Vec2::new(96.0, 96.0),
        active_powerups.iter(),
        &active_sparkles,
    );
}

#[test]
fn window_scale_menu_defaults_and_cycles_crisp_scales() {
    assert_eq!(ModeSelect::default().window_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(window_scale_label(2), "2X");
    assert_eq!(window_scale_label(3), "3X");
    assert_eq!(window_scale_label(4), "4X");
    assert_eq!(next_window_scale(2), 3);
    assert_eq!(next_window_scale(3), 4);
    assert_eq!(next_window_scale(4), 2);
    assert_eq!(previous_window_scale(2), 4);
    assert_eq!(previous_window_scale(3), 2);
    assert_eq!(previous_window_scale(4), 3);
    assert_eq!(clamp_window_scale(1), 2);
    assert_eq!(clamp_window_scale(5), 4);
}

#[test]
fn music_menu_mode_defaults_to_bgm_and_cycles_available_modes() {
    assert_eq!(ModeSelect::default().audio_mode, AudioMode::Bgm);
    assert!(ModeSelect::default().sound_enabled);
    assert_eq!(next_audio_mode(AudioMode::Bgm, false), AudioMode::Classic);
    assert_eq!(next_audio_mode(AudioMode::Classic, false), AudioMode::Bgm);
    assert_eq!(
        previous_audio_mode(AudioMode::Bgm, false),
        AudioMode::Classic
    );
    assert_eq!(
        previous_audio_mode(AudioMode::Classic, false),
        AudioMode::Bgm
    );
    assert_eq!(next_audio_mode(AudioMode::Bgm, true), AudioMode::Custom);
    assert_eq!(next_audio_mode(AudioMode::Custom, true), AudioMode::Classic);
    assert_eq!(next_audio_mode(AudioMode::Classic, true), AudioMode::Bgm);
    assert_eq!(
        previous_audio_mode(AudioMode::Bgm, true),
        AudioMode::Classic
    );
    assert_eq!(
        previous_audio_mode(AudioMode::Classic, true),
        AudioMode::Custom
    );
    assert_eq!(previous_audio_mode(AudioMode::Custom, true), AudioMode::Bgm);
    assert_eq!(audio_mode_label(AudioMode::Bgm), "BGM");
    assert_eq!(audio_mode_label(AudioMode::Custom), "CUSTOM");
    assert_eq!(audio_mode_label(AudioMode::Classic), "CLASSIC");
    assert!(!toggle_sound_enabled(true));
    assert!(toggle_sound_enabled(false));
    assert_eq!(sound_enabled_label(true), "ON");
    assert_eq!(sound_enabled_label(false), "OFF");
}

#[test]
fn runtime_settings_do_not_read_hidden_environment_switches() {
    let source = include_str!("main.rs");
    let forbidden_reads = [
        ["std", "::", "env", "::", "var", "("].concat(),
        ["std", "::", "env", "::", "var_os", "("].concat(),
        ["env", "::", "var", "("].concat(),
        ["env", "::", "var_os", "("].concat(),
    ];

    for forbidden in forbidden_reads {
        assert!(
            !source.contains(&forbidden),
            "runtime settings must use the main menu, not hidden environment reads: {forbidden}"
        );
    }
}

#[test]
fn background_music_only_plays_during_active_rounds() {
    assert!(background_music_should_play(
        AudioMode::Bgm,
        GamePhase::StageIntro,
        false
    ));
    assert!(background_music_should_play(
        AudioMode::Bgm,
        GamePhase::Playing,
        false
    ));
    assert!(!background_music_should_play(
        AudioMode::Bgm,
        GamePhase::ModeSelect,
        false
    ));
    assert!(!background_music_should_play(
        AudioMode::Bgm,
        GamePhase::Paused,
        false
    ));
    assert!(!background_music_should_play(
        AudioMode::Bgm,
        GamePhase::LevelClear,
        false
    ));
    assert!(!background_music_should_play(
        AudioMode::Bgm,
        GamePhase::GameOver,
        false
    ));
    assert!(background_music_should_play(
        AudioMode::Custom,
        GamePhase::Playing,
        true
    ));
    assert!(!background_music_should_play(
        AudioMode::Custom,
        GamePhase::Playing,
        false
    ));
    assert!(!background_music_should_play(
        AudioMode::Classic,
        GamePhase::Playing,
        true
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
    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Bgm]);

    app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Classic;
    app.update();
    assert_eq!(background_music_entity_count(&mut app), 0);

    app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Bgm;
    app.update();
    assert_eq!(background_music_entity_count(&mut app), 1);
    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Bgm]);

    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Paused;
    app.update();
    assert_eq!(background_music_entity_count(&mut app), 0);
}

#[test]
fn background_music_sync_switches_to_custom_menu_source_when_available() {
    let mut app = App::new();
    app.insert_resource(ModeSelect::default());
    app.insert_resource(test_sound_assets_with_custom_music());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.add_systems(Update, sync_background_music);

    app.update();
    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Bgm]);

    app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Custom;
    app.update();
    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Custom]);

    app.world_mut().resource_mut::<ModeSelect>().audio_mode = AudioMode::Classic;
    app.update();
    assert!(background_music_modes(&mut app).is_empty());
}

#[test]
fn sound_effect_mute_does_not_disable_background_music() {
    let mut sounds = test_sound_assets();
    sounds.sound_enabled = false;
    let mut app = App::new();
    app.insert_resource(ModeSelect::default());
    app.insert_resource(sounds);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.add_systems(Update, sync_background_music);

    app.update();

    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Bgm]);
}

#[test]
fn main_menu_music_setting_drives_custom_background_loop() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Enter);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets_with_custom_music());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Music,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(
        Update,
        (handle_shared_controls, sync_background_music).chain(),
    );

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().audio_mode,
        AudioMode::Custom
    );
    assert!(background_music_modes(&mut app).is_empty());

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Playing;
    app.update();

    assert_eq!(background_music_modes(&mut app), vec![AudioMode::Custom]);
}

#[test]
fn main_menu_music_setting_selects_classic_when_custom_music_is_missing() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Enter);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Music,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(
        Update,
        (handle_shared_controls, sync_background_music).chain(),
    );

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().audio_mode,
        AudioMode::Classic
    );

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Playing;
    app.update();

    assert!(background_music_modes(&mut app).is_empty());
}

fn background_music_entity_count(app: &mut App) -> usize {
    let mut query = app
        .world_mut()
        .query_filtered::<Entity, With<BackgroundMusic>>();
    query.iter(app.world()).count()
}

fn background_music_modes(app: &mut App) -> Vec<AudioMode> {
    let mut query = app.world_mut().query::<&BackgroundMusic>();
    query.iter(app.world()).map(|music| music.mode).collect()
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

#[test]
fn main_menu_sound_setting_mutes_one_shot_audio() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Enter);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Sound,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(
        Update,
        (handle_shared_controls, spawn_fire_sound_for_test).chain(),
    );

    app.update();

    assert!(!app.world().resource::<ModeSelect>().sound_enabled);
    assert!(!app.world().resource::<SoundAssets>().sound_enabled);
    assert_eq!(retro_audio_player_count(&mut app), 0);
}

fn spawn_fire_sound_for_test(mut commands: Commands, sounds: Res<SoundAssets>) {
    play_sound(&mut commands, &sounds, SoundKind::Fire);
}

fn retro_audio_player_count(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&AudioPlayer<RetroSound>>();
    query.iter(app.world()).count()
}

#[test]
fn window_mode_toggle_switches_between_windowed_and_borderless_fullscreen() {
    assert_eq!(
        toggle_window_mode(WindowMode::Windowed),
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    );
    assert_eq!(
        toggle_window_mode(WindowMode::BorderlessFullscreen(MonitorSelection::Primary)),
        WindowMode::Windowed
    );
}

#[test]
fn f_key_toggles_primary_window_fullscreen() {
    reset_window_scale_settings_for_test();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyF);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect::default());
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.world_mut().spawn((
        Window {
            mode: WindowMode::Windowed,
            ..default()
        },
        PrimaryWindow,
    ));
    app.add_systems(Update, handle_fullscreen_toggle);

    app.update();

    let mut windows = app.world_mut().query::<&Window>();
    let window = windows
        .single(app.world())
        .expect("primary window should exist");
    assert_eq!(
        window.mode,
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    );
    assert_eq!(
        app.world().resource::<ModeSelect>().window_scale,
        MAX_WINDOW_SCALE
    );
    assert_eq!(window_scale(), MAX_WINDOW_SCALE as f32);

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::KeyF);
        keys.clear();
        keys.press(KeyCode::KeyF);
    }
    app.update();

    let mut windows = app.world_mut().query::<&Window>();
    let window = windows
        .single(app.world())
        .expect("primary window should exist");
    assert_eq!(window.mode, WindowMode::Windowed);
    assert_eq!(
        app.world().resource::<ModeSelect>().window_scale,
        DEFAULT_WINDOW_SCALE
    );
    assert_eq!(window_scale(), DEFAULT_WINDOW_SCALE as f32);
    let (width, height) = virtual_window_size(DEFAULT_WINDOW_SCALE as f32);
    assert_eq!(window.resolution.width(), width as f32);
    assert_eq!(window.resolution.height(), height as f32);
}

#[test]
fn fullscreen_toggle_rescales_active_game_entities_to_four_x() {
    reset_window_scale_settings_for_test();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyF);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect::default());
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.world_mut().spawn((
        Window {
            mode: WindowMode::Windowed,
            ..default()
        },
        PrimaryWindow,
    ));
    app.world_mut().spawn((
        GameEntity,
        Transform::from_translation(Vec3::new(30.0, -15.0, 0.3))
            .with_scale(Vec3::splat(DEFAULT_WINDOW_SCALE as f32)),
    ));
    app.add_systems(Update, handle_fullscreen_toggle);

    app.update();

    let ratio = MAX_WINDOW_SCALE as f32 / DEFAULT_WINDOW_SCALE as f32;
    let mut transforms = app
        .world_mut()
        .query_filtered::<&Transform, With<GameEntity>>();
    let transform = transforms
        .single(app.world())
        .expect("game entity should exist");
    assert_eq!(transform.translation.x, 30.0 * ratio);
    assert_eq!(transform.translation.y, -15.0 * ratio);
    assert_eq!(transform.scale, Vec3::splat(MAX_WINDOW_SCALE as f32));
    reset_window_scale_settings_for_test();
}

#[test]
fn preserve_scale_fullscreen_policy_keeps_canvas_resolution_unchanged() {
    reset_window_scale_settings_for_test();
    let mut mode_select = ModeSelect {
        window_scale: DEFAULT_WINDOW_SCALE,
        ..ModeSelect::default()
    };
    let mut window = Window {
        mode: WindowMode::Windowed,
        resolution: (390, 308).into(),
        ..default()
    };

    let (old_scale, new_scale) = toggle_window_fullscreen_with_policy(
        &mut window,
        &mut mode_select,
        FullscreenScalePolicy::PreserveCurrentScale,
    );

    assert_eq!(
        window.mode,
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    );
    assert_eq!(old_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(new_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(mode_select.window_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(window.resolution.width(), 390.0);
    assert_eq!(window.resolution.height(), 308.0);

    let (old_scale, new_scale) = toggle_window_fullscreen_with_policy(
        &mut window,
        &mut mode_select,
        FullscreenScalePolicy::PreserveCurrentScale,
    );

    assert_eq!(window.mode, WindowMode::Windowed);
    assert_eq!(old_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(new_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(mode_select.window_scale, DEFAULT_WINDOW_SCALE);
    assert_eq!(window.resolution.width(), 390.0);
    assert_eq!(window.resolution.height(), 308.0);
    reset_window_scale_settings_for_test();
}

#[test]
fn main_menu_scale_setting_resizes_primary_window() {
    reset_window_scale_settings_for_test();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowLeft);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Scale,
        window_scale: MAX_WINDOW_SCALE,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.world_mut().spawn((
        Window {
            resolution: virtual_window_size(MAX_WINDOW_SCALE as f32).into(),
            ..default()
        },
        PrimaryWindow,
    ));
    app.world_mut().spawn((GameEntity, OldStageEntity));
    app.add_systems(Update, handle_shared_controls);

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().window_scale,
        DEFAULT_WINDOW_SCALE
    );
    let mut windows = app.world_mut().query::<&Window>();
    let window = windows
        .single(app.world())
        .expect("primary window should exist");
    let (width, height) = virtual_window_size(DEFAULT_WINDOW_SCALE as f32);
    assert_eq!(window.resolution.width(), width as f32);
    assert_eq!(window.resolution.height(), height as f32);
    assert_eq!(window_scale(), DEFAULT_WINDOW_SCALE as f32);

    let mut old_entities = app.world_mut().query::<&OldStageEntity>();
    assert_eq!(old_entities.iter(app.world()).count(), 0);
    let mut cursors = app.world_mut().query::<&ModeSelectCursor>();
    assert_eq!(cursors.iter(app.world()).count(), 1);
}

#[test]
fn personal_sprite_override_paths_are_asset_root_relative_pngs() {
    assert_eq!(PERSONAL_SPRITE_OVERRIDE_PATHS.len(), 11);
    assert!(PERSONAL_SPRITE_OVERRIDE_PATHS.contains(&PERSONAL_SHIELD_PATH));
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
fn custom_music_is_available_only_from_personal_background_file() {
    assert_eq!(
        personal_background_music_path_if_available(|path| {
            path == PERSONAL_BACKGROUND_MUSIC_SOUND_PATH
        }),
        Some(PERSONAL_BACKGROUND_MUSIC_SOUND_PATH)
    );
    assert_eq!(
        personal_background_music_path_if_available(|path| path == PERSONAL_FIRE_SOUND_PATH),
        None
    );
    assert_eq!(personal_background_music_path_if_available(|_| false), None);
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
        campaign_stage_path(CampaignMapPack::Original, 1),
        "assets/levels_original/001.level.ron"
    );
    assert_eq!(
        campaign_stage_path(CampaignMapPack::Custom, 12),
        "assets/levels/012.level.ron"
    );
    assert_eq!(
        personal_stage_path(1),
        "assets/personal/levels/001.level.ron"
    );
    assert_eq!(
        personal_stage_path(12),
        "assets/personal/levels/012.level.ron"
    );
    assert_eq!(
        personal_campaign_stage_path(CampaignMapPack::Original, 1),
        "assets/personal/levels_original/001.level.ron"
    );
    assert_eq!(
        personal_campaign_stage_path(CampaignMapPack::Custom, 12),
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
fn embedded_distribution_content_matches_authored_defaults() {
    assert_eq!(embedded_asset_manifest_contents(), MANIFEST);
    assert_eq!(embedded_stage_contents(1), Some(LEVEL_1));
    assert_eq!(embedded_stage_contents(CUSTOM_LEVEL_COUNT), Some(LEVEL_50));
    assert_eq!(embedded_stage_contents(0), None);
    assert_eq!(embedded_stage_contents(CUSTOM_LEVEL_COUNT + 1), None);
    assert!(embedded_campaign_stage_contents(CampaignMapPack::Original, 1).is_some());
    assert!(
        embedded_campaign_stage_contents(CampaignMapPack::Original, ORIGINAL_LEVEL_COUNT).is_some()
    );
    assert_eq!(
        embedded_campaign_stage_contents(CampaignMapPack::Original, ORIGINAL_LEVEL_COUNT + 1),
        None
    );
    assert_eq!(embedded_arena_contents(1), Some(ARENA_1));
    assert_eq!(embedded_arena_contents(ARENA_COUNT), Some(ARENA_8));
    assert_eq!(embedded_arena_contents(0), None);
    assert_eq!(embedded_arena_contents(ARENA_COUNT + 1), None);

    parse_asset_manifest(embedded_asset_manifest_contents())
        .expect("embedded manifest should parse");
    parse_level(embedded_stage_contents(1).expect("stage one should be embedded"))
        .expect("embedded stage should parse");
    parse_level(
        embedded_campaign_stage_contents(CampaignMapPack::Original, 1)
            .expect("original stage one should be embedded"),
    )
    .expect("embedded original stage should parse");
    parse_arena(embedded_arena_contents(1).expect("arena one should be embedded"))
        .expect("embedded arena should parse");
}

#[test]
fn runtime_text_prefers_personal_override_before_authored_default() {
    let personal = "assets/personal/levels/001.level.ron";
    let authored = "assets/levels/001.level.ron";
    let (path, contents) = load_runtime_text_with(
        personal,
        authored,
        Some("embedded"),
        |path| path == personal || path == authored,
        |path| Ok(format!("disk:{path}")),
    )
    .expect("runtime text should load");

    assert_eq!(path, personal);
    assert_eq!(
        contents.as_ref(),
        "disk:assets/personal/levels/001.level.ron"
    );
}

#[test]
fn runtime_text_uses_embedded_default_when_asset_files_are_absent() {
    let (path, contents) = load_runtime_text_with(
        "assets/personal/levels/001.level.ron",
        "assets/levels/001.level.ron",
        Some("embedded-stage"),
        |_| false,
        |path| Err(format!("failed to read {path}: missing")),
    )
    .expect("embedded runtime text should load");

    assert_eq!(path, "assets/levels/001.level.ron");
    assert_eq!(contents.as_ref(), "embedded-stage");
}

#[test]
fn runtime_text_still_reports_authored_path_when_no_default_exists() {
    let err = match load_runtime_text_with(
        "assets/personal/levels/099.level.ron",
        "assets/levels/099.level.ron",
        None,
        |_| false,
        |path| Err(format!("failed to read {path}: missing")),
    ) {
        Ok(_) => panic!("missing runtime text should fail"),
        Err(err) => err,
    };

    assert!(err.contains("assets/levels/099.level.ron"));
}

#[test]
fn manifest_text_can_fall_back_to_embedded_default() {
    let contents = load_text_or_embedded_with(
        ASSET_MANIFEST_PATH,
        Some(embedded_asset_manifest_contents()),
        |_| false,
        |path| Err(format!("failed to read {path}: missing")),
    )
    .expect("embedded manifest text should load");
    let manifest = parse_asset_manifest(&contents).expect("embedded manifest should parse");

    assert_eq!(manifest.glyphs.tile_width, GENERATED_GLYPH_WIDTH);
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

    let (original, original_grid) = load_campaign_stage_bundle(CampaignMapPack::Original, 1)
        .expect("original stage bundle should load");
    assert_eq!(original.name, "Stage 1");
    assert_ne!(original.map, level.map);
    assert_eq!(
        original_grid.get(
            original.base_position.x as i32,
            original.base_position.y as i32
        ),
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
        CampaignMapPack::Custom,
        7,
        "assets/levels/007.level.ron",
        "spawn_interval_secs must be positive",
    );

    assert!(err.contains("CUSTOM campaign stage 7"));
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
    assert_eq!(glyph_index(' ', &manifest.glyphs), 0);
    assert_eq!(glyph_index('0', &manifest.glyphs), 1);
    assert_eq!(glyph_index('A', &manifest.glyphs), 11);
    assert_eq!(glyph_index('Z', &manifest.glyphs), 36);

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

    let invalid = replace_fixture_once(
        MANIFEST,
        "    player2: [
      (up: 8, down: 9, left: 10, right: 11),
      (up: 12, down: 13, left: 14, right: 15),
    ],",
        "    player2: [
      (up: 8, down: 9, left: 10, right: 11),
    ],",
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
fn generated_glyph_atlas_adds_transparent_padding_between_characters() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_glyph_atlas(&manifest.glyphs);
    let glyph_count = manifest.glyphs.characters.chars().count();
    let expected_width = manifest.glyphs.tile_width * glyph_count
        + GENERATED_GLYPH_ATLAS_PADDING_X * (glyph_count - 1);

    assert_eq!(image.texture_descriptor.size.width, expected_width as u32);
    assert_eq!(
        image.texture_descriptor.size.height,
        manifest.glyphs.tile_height as u32
    );

    let zero_left = manifest.glyphs.tile_width + GENERATED_GLYPH_ATLAS_PADDING_X;
    assert_eq!(image_pixel(&image, zero_left, 0), [216, 216, 184, 255]);
    assert_eq!(
        image_pixel(&image, zero_left + manifest.glyphs.tile_width, 0),
        [0, 0, 0, 0]
    );
}

#[test]
fn generated_tank_atlas_uses_directional_tread_silhouettes() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_tank_atlas(manifest.atlases.tanks);
    let player1_tread = [88, 80, 40, 255];
    let player1_light = [240, 216, 104, 255];

    assert_eq!(image_pixel(&image, 2, 8), player1_tread);
    assert_eq!(image_pixel(&image, 8, 2), player1_light);

    let left_offset = manifest.atlases.tanks.tile_width * 2;
    assert_eq!(image_pixel(&image, left_offset + 8, 2), player1_tread);
    assert_eq!(image_pixel(&image, left_offset + 2, 8), player1_light);

    let right_offset = manifest.atlases.tanks.tile_width * 3;
    assert_eq!(image_pixel(&image, right_offset + 8, 13), player1_tread);
    assert_eq!(image_pixel(&image, right_offset + 13, 8), player1_light);
}

#[test]
fn generated_bullet_atlas_uses_directional_silhouettes() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_bullet_atlas(manifest.atlases.bullets);
    let light = [248, 248, 216, 255];
    let dark = [128, 112, 64, 255];
    let transparent = [0, 0, 0, 0];

    assert_eq!(image_pixel(&image, 1, 0), light);
    assert_eq!(image_pixel(&image, 1, 3), dark);
    assert_eq!(image_pixel(&image, 0, 0), transparent);

    let down_offset = manifest.atlases.bullets.tile_width;
    assert_eq!(image_pixel(&image, down_offset + 1, 0), dark);
    assert_eq!(image_pixel(&image, down_offset + 1, 3), light);

    let left_offset = manifest.atlases.bullets.tile_width * 2;
    assert_eq!(image_pixel(&image, left_offset, 1), light);
    assert_eq!(image_pixel(&image, left_offset + 3, 1), dark);
    assert_eq!(image_pixel(&image, left_offset, 0), transparent);

    let right_offset = manifest.atlases.bullets.tile_width * 3;
    assert_eq!(image_pixel(&image, right_offset, 1), dark);
    assert_eq!(image_pixel(&image, right_offset + 3, 1), light);
}

#[test]
fn generated_terrain_atlas_uses_distinct_material_patterns() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_terrain_atlas(manifest.atlases.terrain);
    let tile_width = manifest.atlases.terrain.tile_width;

    assert_eq!(image_pixel(&image, 0, 0), [200, 96, 48, 255]);
    assert_eq!(image_pixel(&image, 3, 0), [48, 24, 16, 255]);

    let steel = tile_width;
    assert_eq!(image_pixel(&image, steel, 0), [208, 216, 216, 255]);
    assert_eq!(image_pixel(&image, steel + 2, 2), [72, 80, 88, 255]);
    assert_eq!(image_pixel(&image, steel + 7, 7), [40, 48, 56, 255]);

    let water_a = tile_width * 2;
    let water_b = tile_width * 3;
    assert_eq!(image_pixel(&image, water_a + 2, 1), [104, 176, 232, 255]);
    assert_eq!(image_pixel(&image, water_b + 2, 1), [32, 88, 168, 255]);
    assert_ne!(
        image_pixel(&image, water_a + 2, 1),
        image_pixel(&image, water_b + 2, 1)
    );

    let forest = tile_width * 4;
    assert_eq!(image_pixel(&image, forest, 0), [16, 72, 32, 225]);
    assert_eq!(image_pixel(&image, forest + 1, 1), [88, 176, 80, 240]);

    let ice = tile_width * 5;
    assert_eq!(image_pixel(&image, ice, 0), [224, 248, 255, 255]);
    assert_eq!(image_pixel(&image, ice + 2, 2), [64, 128, 176, 255]);
    assert_eq!(image_pixel(&image, ice + 5, 1), [160, 216, 232, 255]);
}

#[test]
fn generated_images_use_nearest_sampling_to_prevent_web_atlas_bleed() {
    let image = image_from_pixels(1, 1, vec![255, 255, 255, 255]);

    let ImageSampler::Descriptor(descriptor) = image.sampler else {
        panic!("generated images should use an explicit nearest sampler");
    };
    assert_eq!(descriptor.mag_filter, ImageFilterMode::Nearest);
    assert_eq!(descriptor.min_filter, ImageFilterMode::Nearest);
    assert_eq!(descriptor.mipmap_filter, ImageFilterMode::Nearest);
}

#[test]
fn generated_powerup_atlas_uses_readable_classic_icons() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_powerup_atlas(manifest.atlases.powerups);
    let tile_width = manifest.atlases.powerups.tile_width;

    assert_eq!(image_pixel(&image, 8, 2), [255, 248, 160, 255]);
    assert_eq!(image_pixel(&image, 10, 10), [128, 88, 24, 255]);

    let helmet = tile_width;
    assert_eq!(image_pixel(&image, helmet + 6, 6), [216, 248, 248, 255]);
    assert_eq!(image_pixel(&image, helmet + 3, 9), [32, 96, 144, 255]);

    let clock = tile_width * 2;
    assert_eq!(image_pixel(&image, clock + 4, 2), [248, 232, 112, 255]);
    assert_eq!(image_pixel(&image, clock + 2, 8), [56, 152, 224, 255]);
    assert_eq!(image_pixel(&image, clock + 8, 6), [32, 56, 128, 255]);

    let grenade = tile_width * 3;
    assert_eq!(image_pixel(&image, grenade + 13, 0), [255, 96, 48, 255]);
    assert_eq!(image_pixel(&image, grenade + 6, 7), [168, 72, 56, 255]);
    assert_eq!(image_pixel(&image, grenade + 9, 8), [32, 32, 32, 255]);

    let shovel = tile_width * 4;
    assert_eq!(image_pixel(&image, shovel + 8, 2), [120, 72, 32, 255]);
    assert_eq!(image_pixel(&image, shovel + 5, 12), [232, 240, 240, 255]);

    let tank = tile_width * 5;
    assert_eq!(image_pixel(&image, tank + 3, 7), [64, 120, 64, 255]);
    assert_eq!(image_pixel(&image, tank + 6, 5), [104, 240, 120, 255]);
    assert_eq!(image_pixel(&image, tank + 7, 8), [248, 248, 184, 255]);
}

#[test]
fn generated_effect_atlas_uses_readable_animation_frames() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let image = create_effect_atlas(manifest.atlases.effects);
    let tile_width = manifest.atlases.effects.tile_width;

    assert_eq!(image_pixel(&image, 8, 8), [255, 248, 184, 255]);
    assert_eq!(image_pixel(&image, 8, 4), [248, 184, 64, 255]);

    let explosion_smoke = tile_width * 2;
    assert_eq!(
        image_pixel(&image, explosion_smoke + 3, 8),
        [88, 72, 64, 190]
    );

    let spawn = tile_width * 5;
    assert_eq!(image_pixel(&image, spawn + 1, 1), [232, 248, 255, 245]);
    assert_eq!(image_pixel(&image, spawn + 8, 8), [232, 248, 255, 245]);

    let base_flame = tile_width * 9;
    assert_eq!(image_pixel(&image, base_flame + 3, 4), [255, 248, 184, 230]);
    let base_smoke = tile_width * 10;
    assert_eq!(image_pixel(&image, base_smoke + 4, 3), [88, 72, 64, 180]);

    let sparkle = tile_width * 12;
    assert_eq!(image_pixel(&image, sparkle + 7, 7), [255, 255, 255, 220]);
    let sparkle_gold = tile_width * 13;
    assert_eq!(
        image_pixel(&image, sparkle_gold + 8, 8),
        [255, 232, 104, 220]
    );

    let impact = tile_width * 17;
    assert_eq!(image_pixel(&image, impact + 12, 8), [248, 216, 96, 220]);
    let impact_smoke = tile_width * 19;
    assert_eq!(image_pixel(&image, impact_smoke + 8, 8), [72, 56, 48, 130]);
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
        "characters: \" 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
        "characters: \" 00123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
        1,
    );
    assert!(
        parse_asset_manifest(&invalid)
            .expect_err("duplicate glyph should fail")
            .contains("glyphs.characters includes duplicate glyph '0'")
    );

    let invalid = MANIFEST.replacen(
        "characters: \" 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
        "characters: \" 0123456789ABCDEFGHIJKLMNOPQRSTUVWXY?\"",
        1,
    );
    assert!(
        parse_asset_manifest(&invalid)
            .expect_err("unsupported glyph should fail")
            .contains("glyphs.characters includes unsupported blank glyph '?'")
    );

    let invalid = MANIFEST.replacen(
        "characters: \" 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ\"",
        "characters: \" 0123456789ABCDEFGHIJKLMNOPQRSTUVWXY\"",
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
        assert_eq!(level.enemy_ai_strategy, EnemyAiStrategy::Classic);
        assert_eq!(level.difficulty_profile, EnemyDifficultyProfile::Normal);
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
        let grid = TileGrid::from_level(&level).expect("level grid should build");
        assert!(
            grid.can_tank_occupy(Vec2::new(
                CLASSIC_COOP_P2_SPAWN_X as f32 * TILE_SIZE,
                CLASSIC_COOP_P2_SPAWN_Y as f32 * TILE_SIZE
            )),
            "Stage {stage} must leave the co-op P2 spawn open"
        );
    }
}

#[test]
fn original_campaign_levels_match_original_pack_shape() {
    for stage in 1..=ORIGINAL_LEVEL_COUNT {
        let contents = embedded_campaign_stage_contents(CampaignMapPack::Original, stage)
            .expect("original stage should be embedded");
        let level = parse_level(contents).expect("original level should parse");
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
        assert_eq!(
            level.base_position,
            GridPoint {
                x: CLASSIC_BASE_X,
                y: CLASSIC_BASE_Y
            }
        );
        let grid = TileGrid::from_level(&level).expect("original grid should build");
        assert!(grid.can_tank_occupy(Vec2::new(
            CLASSIC_COOP_P2_SPAWN_X as f32 * TILE_SIZE,
            CLASSIC_COOP_P2_SPAWN_Y as f32 * TILE_SIZE
        )));
        for x in (CLASSIC_BASE_X - 1)..=(CLASSIC_BASE_X + 2) {
            assert!(
                grid.get(x as i32, (CLASSIC_BASE_Y - 1) as i32)
                    .is_some_and(TileKind::bullet_blocks),
                "Original stage {stage} should keep the one-tile top fortress wall at ({x}, {})",
                CLASSIC_BASE_Y - 1
            );
        }
        for y in CLASSIC_BASE_Y..=(CLASSIC_BASE_Y + 1) {
            for x in [CLASSIC_BASE_X - 1, CLASSIC_BASE_X + 2] {
                assert!(
                    grid.get(x as i32, y as i32)
                        .is_some_and(TileKind::bullet_blocks),
                    "Original stage {stage} should keep the one-tile side fortress wall at ({x}, {y})"
                );
            }
            for x in CLASSIC_BASE_X..=(CLASSIC_BASE_X + 1) {
                assert_eq!(
                    grid.get(x as i32, y as i32),
                    Some(TileKind::Base),
                    "Original stage {stage} should keep the base at ({x}, {y})"
                );
            }
        }
    }
}

#[test]
fn level_rules_default_to_normal_steel_and_later_levels_enable_upgrade_breaking() {
    let stage_1 = parse_level(LEVEL_1).expect("level should parse");
    assert_eq!(StageRules::from_level(&stage_1), StageRules::default());
    for contents in [
        LEVEL_3, LEVEL_4, LEVEL_5, LEVEL_6, LEVEL_7, LEVEL_8, LEVEL_9, LEVEL_10, LEVEL_11,
        LEVEL_12, LEVEL_13, LEVEL_14, LEVEL_15, LEVEL_16, LEVEL_17, LEVEL_18, LEVEL_19, LEVEL_20,
        LEVEL_21, LEVEL_22, LEVEL_23, LEVEL_24, LEVEL_25, LEVEL_26, LEVEL_27, LEVEL_28, LEVEL_29,
        LEVEL_30, LEVEL_31, LEVEL_32, LEVEL_33, LEVEL_34, LEVEL_35, LEVEL_36, LEVEL_37, LEVEL_38,
        LEVEL_39, LEVEL_40, LEVEL_41, LEVEL_42, LEVEL_43, LEVEL_44, LEVEL_45, LEVEL_46, LEVEL_47,
        LEVEL_48, LEVEL_49, LEVEL_50,
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
fn level_parses_optional_enemy_ai_strategy_and_difficulty_profile() {
    let hard_pathing = LEVEL_1.replacen(
            "max_enemies_on_screen: 4,",
            "max_enemies_on_screen: 4,\n  enemy_ai_strategy: PathToObjective,\n  difficulty_profile: Hard,",
            1,
        );
    let level = parse_level(&hard_pathing).expect("level should parse");
    assert_eq!(level.enemy_ai_strategy, EnemyAiStrategy::PathToObjective);
    assert_eq!(level.difficulty_profile, EnemyDifficultyProfile::Hard);

    let director = EnemyDirector::from_level(&level);
    assert_eq!(director.ai_strategy, EnemyAiStrategy::PathToObjective);
    assert_eq!(director.difficulty_profile, EnemyDifficultyProfile::Hard);
    assert_eq!(
        director.spawn_timer.duration().as_secs_f32(),
        enemy_spawn_interval_for_profile(level.spawn_interval_secs, EnemyDifficultyProfile::Hard)
    );
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
                assert!(matches!(index, 1..=4 | 7));
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

    let shifted_base =
        base_battle_arena_text().replacen("p1_base: (x: 24, y: 24)", "p1_base: (x: 23, y: 24)", 1);
    assert!(
        parse_arena(&shifted_base)
            .err()
            .expect("shifted p1 base should fail")
            .contains("p1 base position (23, 24) must cover a 2x2 base tile area")
    );

    let overlapping_bases =
        base_battle_arena_text().replacen("p1_base: (x: 24, y: 24)", "p1_base: (x: 0, y: 0)", 1);
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
            .contains("enemy spawn 2 must be classic top spawn (12, 0, Down), got (8, 0, Down)")
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
    let duplicate_powerup = replace_fixture_once(
        ARENA_1,
        "  powerup_spawns: [\n    (x: 12, y: 12),\n  ],",
        "  powerup_spawns: [\n    (x: 12, y: 12),\n    (x: 12, y: 12),\n  ],",
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
    assert!(grid.tank_overlaps_tile(Vec2::new(3.0 * TILE_SIZE, 4.0 * TILE_SIZE), TileKind::Ice));
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

fn tank_grid_point_is_open(grid: &TileGrid, x: usize, y: usize) -> bool {
    grid.can_tank_occupy(Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE))
}

fn campaign_base_approach_targets(grid: &TileGrid) -> Vec<(usize, usize)> {
    let mut targets = Vec::new();

    for y in [CLASSIC_BASE_Y - 2, CLASSIC_BASE_Y - 1] {
        for x in (CLASSIC_BASE_X - 3)..=(CLASSIC_BASE_X + 3) {
            if tank_grid_point_is_open(grid, x, y) {
                targets.push((x, y));
            }
        }
    }

    for y in (CLASSIC_BASE_Y - 4)..CLASSIC_BASE_Y {
        for x in [
            CLASSIC_BASE_X - 4,
            CLASSIC_BASE_X - 3,
            CLASSIC_BASE_X + 4,
            CLASSIC_BASE_X + 5,
        ] {
            if tank_grid_point_is_open(grid, x, y) {
                targets.push((x, y));
            }
        }
    }

    targets.sort_unstable();
    targets.dedup();
    targets
}

fn tank_route_reaches_any(
    grid: &TileGrid,
    start: (usize, usize),
    targets: &[(usize, usize)],
) -> bool {
    if !tank_grid_point_is_open(grid, start.0, start.1) || targets.is_empty() {
        return false;
    }

    let target_set: HashSet<(usize, usize)> = targets.iter().copied().collect();
    let max_top_left_tile = BOARD_TILES - 2;
    let width = max_top_left_tile + 1;
    let mut visited = vec![false; width * width];
    let mut queue = VecDeque::from([start]);
    visited[start.1 * width + start.0] = true;

    while let Some((x, y)) = queue.pop_front() {
        if target_set.contains(&(x, y)) {
            return true;
        }

        for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            let next_x = x as isize + dx;
            let next_y = y as isize + dy;
            if !(0..=max_top_left_tile as isize).contains(&next_x)
                || !(0..=max_top_left_tile as isize).contains(&next_y)
            {
                continue;
            }

            let next = (next_x as usize, next_y as usize);
            let index = next.1 * width + next.0;
            if visited[index] || !tank_grid_point_is_open(grid, next.0, next.1) {
                continue;
            }

            visited[index] = true;
            queue.push_back(next);
        }
    }

    false
}

#[test]
fn authored_campaign_levels_use_distinct_classic_terrain_mix() {
    let mut unique_maps = HashSet::new();
    let mut steel_stages = 0;
    let mut water_stages = 0;
    let mut forest_stages = 0;
    let mut ice_stages = 0;

    for (stage, contents) in authored_levels() {
        let level = parse_level(contents).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        assert!(
            unique_maps.insert(level.map.join("\n")),
            "Stage {stage} should not duplicate another authored map"
        );
        assert!(
            grid.tiles.contains(&TileKind::Brick),
            "Stage {stage} should keep destructible cover"
        );

        steel_stages += usize::from(grid.tiles.contains(&TileKind::Steel));
        water_stages += usize::from(grid.tiles.contains(&TileKind::Water));
        forest_stages += usize::from(grid.tiles.contains(&TileKind::Forest));
        ice_stages += usize::from(grid.tiles.contains(&TileKind::Ice));
    }

    assert_eq!(unique_maps.len(), CUSTOM_LEVEL_COUNT);
    assert!(steel_stages >= 25, "steel appears in {steel_stages} stages");
    assert!(water_stages >= 25, "water appears in {water_stages} stages");
    assert!(
        forest_stages >= 30,
        "forest appears in {forest_stages} stages"
    );
    assert!(ice_stages >= 15, "ice appears in {ice_stages} stages");
}

#[test]
fn authored_campaign_levels_use_classic_base_enclosure() {
    for (stage, contents) in authored_levels() {
        let level = parse_level(contents).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");

        for y in [CLASSIC_BASE_Y - 2, CLASSIC_BASE_Y - 1] {
            for x in (CLASSIC_BASE_X - 2)..=(CLASSIC_BASE_X + 3) {
                let tile = grid.get(x as i32, y as i32);
                assert!(
                    tile.is_some_and(TileKind::bullet_blocks),
                    "Stage {stage} should cap the base with a bullet-blocking wall at ({x}, {y})"
                );
                assert_ne!(
                    tile,
                    Some(TileKind::Base),
                    "Stage {stage} should protect the base before bullets reach the eagle"
                );
            }
        }

        for y in CLASSIC_BASE_Y..=(CLASSIC_BASE_Y + 1) {
            for x in [
                CLASSIC_BASE_X - 2,
                CLASSIC_BASE_X - 1,
                CLASSIC_BASE_X + 2,
                CLASSIC_BASE_X + 3,
            ] {
                assert!(
                    grid.get(x as i32, y as i32)
                        .is_some_and(TileKind::bullet_blocks),
                    "Stage {stage} should protect the base side wall at ({x}, {y})"
                );
            }
        }
    }
}

#[test]
fn authored_campaign_levels_keep_enemy_routes_to_base_open() {
    for (stage, contents) in authored_levels() {
        let level = parse_level(contents).expect("level should parse");
        let grid = TileGrid::from_level(&level).expect("grid should build");
        let targets = campaign_base_approach_targets(&grid);
        assert!(
            !targets.is_empty(),
            "Stage {stage} should leave at least one passable base approach"
        );

        for spawn in &level.enemy_spawns {
            let start = (spawn.x, spawn.y);
            assert!(
                tank_route_reaches_any(&grid, start, &targets),
                "Stage {stage} should leave a tank-sized route from enemy spawn {start:?} to the base approach"
            );
        }
    }
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
fn short_effect_animations_render_above_forest_overlay() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.add_systems(Update, spawn_overlay_effects_for_test);

    app.update();

    let forest_z = terrain_z(TileKind::Forest);
    let mut effects = app.world_mut().query::<(&Transform, &SpriteAnimation)>();
    let z_values: Vec<f32> = effects
        .iter(app.world())
        .map(|(transform, _)| transform.translation.z)
        .collect();

    assert_eq!(z_values.len(), 4);
    assert!(
        z_values.iter().all(|z| *z > forest_z),
        "all short effect animations should render above forest overlay: {z_values:?}"
    );
}

#[test]
fn campaign_enemy_markers_fit_as_compact_tank_icons() {
    assert_eq!(ENEMY_MARKER_COUNT, 20);
    assert_eq!(ENEMY_MARKER_COLUMNS, 4);
    assert_eq!(enemy_marker_top_left(0), status_panel_top_left(8.0, 159.0));
    assert_eq!(enemy_marker_top_left(3), status_panel_top_left(35.0, 159.0));
    assert_eq!(enemy_marker_top_left(4), status_panel_top_left(8.0, 168.0));

    let last = enemy_marker_top_left(ENEMY_MARKER_COUNT - 1);
    assert_eq!(last, status_panel_top_left(35.0, 195.0));
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
    let p1_top_left = campaign_life_icon_top_left(PlayerId::One);
    let p2_top_left = campaign_life_icon_top_left(PlayerId::Two);
    assert_eq!(p1_top_left, status_panel_top_left(14.0, 123.0));
    assert_eq!(p2_top_left, status_panel_top_left(14.0, 135.0));
    for top_left in [p1_top_left, p2_top_left] {
        assert!(top_left.x >= STATUS_PANEL_INNER_LEFT);
        assert!(top_left.x + PLAYER_LIFE_ICON_SIZE < status_panel_top_left(26.0, 0.0).x);
        assert!(top_left.y + PLAYER_LIFE_ICON_SIZE < ENEMY_MARKER_TOP);
    }
}

#[test]
fn campaign_life_icon_uses_player_tank_sprite() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
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
fn versus_life_icons_fit_status_panel_and_match_players() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let p1_top_left = versus_life_icon_top_left(PlayerId::One);
    let p2_top_left = versus_life_icon_top_left(PlayerId::Two);

    assert_eq!(p1_top_left, status_panel_top_left(14.0, 73.0));
    assert_eq!(p2_top_left, status_panel_top_left(14.0, 145.0));
    for top_left in [p1_top_left, p2_top_left] {
        assert!(top_left.x >= STATUS_PANEL_INNER_LEFT);
        assert!(top_left.x + PLAYER_LIFE_ICON_SIZE < status_panel_top_left(26.0, 0.0).x);
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
        assert!(top_left.x >= STATUS_PANEL_INNER_LEFT);
        assert!(top_left.x + phase_text_width(label) <= VIRTUAL_WIDTH - 4.0);
        assert!(top_left.y >= 24.0);
        assert!(top_left.y + GENERATED_GLYPH_HEIGHT as f32 <= BOARD_ORIGIN_Y + board_size());
        for ch in label.chars() {
            assert_manifest_glyph_is_visible(&manifest, ch);
        }
    }

    for (digits, top_left) in digit_rows {
        assert!(top_left.x >= STATUS_PANEL_INNER_LEFT);
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
    let score_label_right = status_panel_top_left(6.0, 38.0).x + phase_text_width("SCORE");
    assert_eq!(icon, status_panel_top_left(36.0, 38.0));
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
    assert_eq!(image_pixel(&image, 0, 0), [0, 0, 0, 0]);
    assert_eq!(image_pixel(&image, 3, 0), [255, 232, 128, 255]);
    assert_eq!(image_pixel(&image, 0, 2), [136, 88, 40, 255]);
    assert_eq!(image_pixel(&image, 3, 5), [96, 64, 32, 255]);
    assert_eq!(image_pixel(&image, 1, 7), [96, 64, 32, 255]);
}

#[test]
fn campaign_stage_icon_and_number_fit_status_panel() {
    let icon = stage_flag_icon_top_left();
    let number = stage_number_top_left();
    assert_eq!(icon, status_panel_top_left(8.0, 87.0));
    assert_eq!(number, status_panel_top_left(22.0, 87.0));
    assert!(icon.x >= STATUS_PANEL_INNER_LEFT);
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
    assert_eq!(image_pixel(&image, 0, 0), [0, 0, 0, 0]);
    assert_eq!(image_pixel(&image, 1, 0), [232, 232, 208, 255]);
    assert_eq!(image_pixel(&image, 2, 1), [255, 232, 96, 255]);
    assert_eq!(image_pixel(&image, 6, 3), [255, 232, 96, 255]);
    assert_eq!(image_pixel(&image, 2, 4), [160, 96, 32, 255]);
    assert_eq!(image_pixel(&image, 0, 7), [120, 120, 96, 255]);
}

#[test]
fn shield_overlay_sprite_uses_transparent_ring_pixels() {
    let image = create_shield_image();
    assert_eq!(
        image.texture_descriptor.size.width,
        GENERATED_SHIELD_SIZE as u32
    );
    assert_eq!(
        image.texture_descriptor.size.height,
        GENERATED_SHIELD_SIZE as u32
    );
    let pixels = image
        .data
        .as_ref()
        .expect("shield sprite should have pixels");
    assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] == 0));
    assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] > 0));
    assert_eq!(image_pixel(&image, 0, 0), [0, 0, 0, 0]);
    assert_eq!(image_pixel(&image, 7, 0), [224, 248, 255, 220]);
    assert_eq!(image_pixel(&image, 2, 4), [224, 248, 255, 220]);
    assert_eq!(image_pixel(&image, 13, 4), [48, 128, 200, 180]);
    assert_eq!(image_pixel(&image, 4, 13), [112, 208, 248, 210]);
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
fn generated_base_sprites_have_emblem_and_rubble_pixels() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let intact = create_base_image(manifest.base.intact, false);
    let destroyed = create_base_image(manifest.base.destroyed, true);

    assert_eq!(image_pixel(&intact, 0, 0), [0, 0, 0, 0]);
    assert_eq!(image_pixel(&intact, 7, 3), [240, 232, 160, 255]);
    assert_eq!(image_pixel(&intact, 4, 9), [248, 216, 104, 255]);
    assert_eq!(image_pixel(&intact, 3, 13), [56, 48, 32, 255]);

    assert_eq!(image_pixel(&destroyed, 0, 0), [0, 0, 0, 0]);
    assert_eq!(image_pixel(&destroyed, 7, 2), [80, 72, 64, 255]);
    assert_eq!(image_pixel(&destroyed, 6, 5), [248, 184, 64, 255]);
    assert_eq!(image_pixel(&destroyed, 9, 9), [120, 80, 48, 255]);
    assert_ne!(image_pixel(&destroyed, 7, 3), image_pixel(&intact, 7, 3));
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
    assert!(campaign_phase_transition_seconds(GamePhase::LevelClear) > LEVEL_CLEAR_DELAY_SECONDS);
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
fn level_clear_transition_awards_remaining_life_bonus_once() {
    let mut app = App::new();
    let mut score_board = ScoreBoard::campaign(1);
    score_board.score = 500;
    score_board.lives = 2;
    score_board.enemies_destroyed = 1;

    app.insert_resource(GameMode::Campaign);
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(score_board);
    app.insert_resource(EnemyDirector::inactive());
    app.add_systems(Update, check_game_phase);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::LevelClear);
    assert_eq!(
        app.world().resource::<ScoreBoard>().score,
        500 + stage_clear_bonus(2)
    );

    app.update();

    assert_eq!(
        app.world().resource::<ScoreBoard>().score,
        500 + stage_clear_bonus(2)
    );
}

#[test]
fn level_clear_transition_loads_next_stage_and_clears_old_entities() {
    let mut app = App::new();
    let (next_level, expected_grid) = load_stage_bundle_or_panic(2);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(LEVEL_CLEAR_SCORECARD_SECONDS));

    let level_1 = parse_level(LEVEL_1).expect("level one should parse");
    let mut score_board = ScoreBoard::campaign(1);
    score_board.score = 900;
    score_board.lives = 2;
    score_board.enemies_destroyed = 1;
    score_board.enemy_kills.add(EnemyKind::Armor);
    let mut enemy_freeze = EnemyFreeze::default();
    enemy_freeze.start();
    let mut versus_freeze = VersusPlayerFreeze::default();
    versus_freeze.start(PlayerId::Two);
    let mut base_reinforcement = BaseReinforcement {
        timer: None,
        saved_tiles: vec![(10, 24, TileKind::Brick)],
    };
    base_reinforcement.start();

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::LevelClear,
        map_pack: CampaignMapPack::Custom,
        stage: 1,
        transition_timer: Timer::from_seconds(LEVEL_CLEAR_SCORECARD_SECONDS, TimerMode::Once),
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::from_level(&level_1).expect("level one grid should build"));
    app.insert_resource(EnemyDirector::from_level(&level_1));
    app.insert_resource(score_board);
    app.insert_resource(StageRules {
        player_steel_destruction: true,
    });
    app.insert_resource(ModeSelect {
        map_pack: CampaignMapPack::Custom,
        ai_strategy: ModeSelectAiStrategy::PathToObjective,
        difficulty_profile: ModeSelectDifficultyProfile::Hard,
        ..ModeSelect::default()
    });
    app.insert_resource(enemy_freeze);
    app.insert_resource(versus_freeze);
    app.insert_resource(base_reinforcement);
    app.world_mut()
        .spawn((GameEntity, OldStageEntity, GridTile { x: 10, y: 24 }));
    app.add_systems(Update, advance_after_level_clear);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.stage, 2);
    assert_eq!(status.phase, GamePhase::StageIntro);
    assert_eq!(status.winner, None);
    assert_eq!(
        app.world().resource::<TileGrid>().tiles,
        expected_grid.tiles
    );

    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.score, 900);
    assert_eq!(score_board.lives, 2);
    assert_eq!(score_board.enemies_destroyed, 0);
    assert_eq!(score_board.total_enemies, next_level.enemies.len());
    assert_eq!(score_board.enemy_kills.total(), 0);

    let director = app.world().resource::<EnemyDirector>();
    assert_eq!(director.roster.len(), next_level.enemies.len());
    assert_eq!(director.spawns.len(), next_level.enemy_spawns.len());
    assert_eq!(director.max_active, next_level.max_enemies_on_screen);
    assert_eq!(director.ai_strategy, EnemyAiStrategy::PathToObjective);
    assert_eq!(director.difficulty_profile, EnemyDifficultyProfile::Hard);
    assert_eq!(
        *app.world().resource::<StageRules>(),
        StageRules::from_level(&next_level)
    );
    assert!(!app.world().resource::<EnemyFreeze>().is_active());
    assert!(
        !app.world()
            .resource::<VersusPlayerFreeze>()
            .is_player_frozen(PlayerId::Two)
    );
    let reinforcement = app.world().resource::<BaseReinforcement>();
    assert!(reinforcement.timer.is_none());
    assert!(reinforcement.saved_tiles.is_empty());

    let mut old_entities = app.world_mut().query::<&OldStageEntity>();
    assert_eq!(old_entities.iter(app.world()).count(), 0);
    let mut game_entities = app.world_mut().query::<&GameEntity>();
    assert!(game_entities.iter(app.world()).count() > 0);
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
fn coop_campaign_score_board_tracks_separate_lives_and_total() {
    let mut score_board = ScoreBoard::coop_campaign(20);

    assert_eq!(score_board.p1_lives, 3);
    assert_eq!(score_board.p2_lives, 3);
    assert_eq!(score_board.lives, 6);

    score_board.set_coop_player_lives(PlayerId::One, 1);
    assert_eq!(score_board.p1_lives, 1);
    assert_eq!(score_board.p2_lives, 3);
    assert_eq!(score_board.lives, 4);
    assert_eq!(
        status_value_text(
            StatusValue::Lives,
            GameMode::CoopCampaign,
            &GameStatus::default(),
            &score_board,
        ),
        "1"
    );
    assert_eq!(
        status_value_text(
            StatusValue::P2Lives,
            GameMode::CoopCampaign,
            &GameStatus::default(),
            &score_board,
        ),
        "3"
    );
}

#[test]
fn coop_campaign_next_stage_spawns_only_surviving_players() {
    let mut score_board = ScoreBoard::coop_campaign(20);
    score_board.set_coop_player_lives(PlayerId::One, 0);
    score_board.set_coop_player_lives(PlayerId::Two, 2);

    assert_eq!(
        campaign_player_spawns_from_score(GameMode::CoopCampaign, &score_board),
        vec![(PlayerId::Two, 2)]
    );
    assert_eq!(
        campaign_player_spawns_from_score(GameMode::Campaign, &score_board),
        vec![(PlayerId::One, 2)]
    );
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
fn pause_toggle_accepts_p_escape_and_pause_keys() {
    for key in [KeyCode::KeyP, KeyCode::Escape, KeyCode::Pause] {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(key);

        assert!(pause_toggle_requested(&keys), "{key:?} should pause");
    }

    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyR);
    assert!(!pause_toggle_requested(&keys));
}

#[test]
fn shared_control_m_returns_to_mode_select_and_clears_runtime_state() {
    let mut app = App::new();
    let level = parse_level(LEVEL_1).expect("level should parse");
    let arena = parse_arena(ARENA_5).expect("arena should parse");
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyM);
    let mut grid = TileGrid::empty();
    grid.set(10, 24, TileKind::Brick);
    let mut enemy_freeze = EnemyFreeze::default();
    enemy_freeze.start();
    let mut versus_freeze = VersusPlayerFreeze::default();
    versus_freeze.start(PlayerId::Two);
    let mut base_reinforcement = BaseReinforcement {
        timer: None,
        saved_tiles: vec![(10, 24, TileKind::Brick)],
    };
    base_reinforcement.start();

    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusBaseBattle);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        map_pack: CampaignMapPack::Custom,
        stage: 12,
        arena: 5,
        winner: Some(PlayerId::Two),
        transition_timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
    app.insert_resource(grid);
    app.insert_resource(EnemyDirector::from_level(&level));
    app.insert_resource(ScoreBoard::versus(3, 5, 2.0));
    app.insert_resource(StageRules {
        player_steel_destruction: true,
    });
    app.insert_resource(VersusPowerUpDirector::from_arena(&arena));
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Scale,
        map_pack: CampaignMapPack::Custom,
        stage: 1,
        arena: 1,
        view_mode: TankViewMode::TwoD,
        view_assist: true,
        view_target: PlayerId::One,
        ai_strategy: ModeSelectAiStrategy::Auto,
        difficulty_profile: ModeSelectDifficultyProfile::Auto,
        audio_mode: AudioMode::Classic,
        sound_enabled: false,
        window_scale: DEFAULT_WINDOW_SCALE,
    });
    app.insert_resource(enemy_freeze);
    app.insert_resource(versus_freeze);
    app.insert_resource(base_reinforcement);
    app.world_mut()
        .spawn((GameEntity, OldStageEntity, GridTile { x: 10, y: 24 }));
    app.add_systems(Update, handle_shared_controls);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::ModeSelect);
    assert_eq!(status.stage, 12);
    assert_eq!(status.arena, 5);
    assert_eq!(status.winner, None);

    let mode_select = app.world().resource::<ModeSelect>();
    assert_eq!(mode_select.selected, ModeSelectOption::Battle);
    assert_eq!(mode_select.stage, 12);
    assert_eq!(mode_select.arena, 5);
    assert_eq!(mode_select.audio_mode, AudioMode::Classic);
    assert!(!mode_select.sound_enabled);

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
    assert_eq!(app.world().resource::<ScoreBoard>().total_enemies, 0);
    assert!(
        app.world()
            .resource::<VersusPowerUpDirector>()
            .spawn_points
            .is_empty()
    );
    assert!(!app.world().resource::<EnemyFreeze>().is_active());
    assert!(
        !app.world()
            .resource::<VersusPlayerFreeze>()
            .is_player_frozen(PlayerId::Two)
    );
    let reinforcement = app.world().resource::<BaseReinforcement>();
    assert!(reinforcement.timer.is_none());
    assert!(reinforcement.saved_tiles.is_empty());

    let mut old_entities = app.world_mut().query::<&OldStageEntity>();
    assert_eq!(old_entities.iter(app.world()).count(), 0);
    let mut cursors = app.world_mut().query::<&ModeSelectCursor>();
    assert_eq!(cursors.iter(app.world()).count(), 1);
}

#[test]
fn shared_control_r_restarts_current_campaign_stage() {
    let mut app = App::new();
    let current_stage = 2;
    let (expected_level, expected_grid) = load_stage_bundle_or_panic(current_stage);
    let previous_level = parse_level(LEVEL_1).expect("level should parse");
    let arena = parse_arena(ARENA_1).expect("arena should parse");
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyR);
    let mut grid = TileGrid::empty();
    grid.set(10, 24, TileKind::Brick);
    let mut score_board = ScoreBoard::campaign(3);
    score_board.score = 1200;
    score_board.lives = 1;
    score_board.enemies_destroyed = 2;
    score_board.enemy_kills.add(EnemyKind::Fast);
    let mut enemy_freeze = EnemyFreeze::default();
    enemy_freeze.start();
    let mut versus_freeze = VersusPlayerFreeze::default();
    versus_freeze.start(PlayerId::Two);
    let mut base_reinforcement = BaseReinforcement {
        timer: None,
        saved_tiles: vec![(10, 24, TileKind::Brick)],
    };
    base_reinforcement.start();

    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Paused,
        map_pack: CampaignMapPack::Custom,
        stage: current_stage,
        arena: 5,
        winner: Some(PlayerId::Two),
        transition_timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
    app.insert_resource(grid);
    app.insert_resource(EnemyDirector::from_level(&previous_level));
    app.insert_resource(score_board);
    app.insert_resource(StageRules {
        player_steel_destruction: true,
    });
    app.insert_resource(VersusPowerUpDirector::from_arena(&arena));
    app.insert_resource(ModeSelect::default());
    app.insert_resource(enemy_freeze);
    app.insert_resource(versus_freeze);
    app.insert_resource(base_reinforcement);
    app.world_mut()
        .spawn((GameEntity, OldStageEntity, GridTile { x: 10, y: 24 }));
    app.add_systems(Update, handle_shared_controls);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::StageIntro);
    assert_eq!(status.stage, current_stage);
    assert_eq!(status.winner, None);
    assert_eq!(
        app.world().resource::<TileGrid>().tiles,
        expected_grid.tiles
    );

    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.score, 0);
    assert_eq!(score_board.lives, 3);
    assert_eq!(score_board.enemies_destroyed, 0);
    assert_eq!(score_board.total_enemies, expected_level.enemies.len());
    assert_eq!(score_board.enemy_kills.total(), 0);

    let director = app.world().resource::<EnemyDirector>();
    assert_eq!(director.roster.len(), expected_level.enemies.len());
    assert_eq!(director.spawns.len(), expected_level.enemy_spawns.len());
    assert_eq!(director.max_active, expected_level.max_enemies_on_screen);
    assert_eq!(
        *app.world().resource::<StageRules>(),
        StageRules::from_level(&expected_level)
    );
    assert!(
        app.world()
            .resource::<VersusPowerUpDirector>()
            .spawn_points
            .is_empty()
    );
    assert!(!app.world().resource::<EnemyFreeze>().is_active());
    assert!(
        !app.world()
            .resource::<VersusPlayerFreeze>()
            .is_player_frozen(PlayerId::Two)
    );
    let reinforcement = app.world().resource::<BaseReinforcement>();
    assert!(reinforcement.timer.is_none());
    assert!(reinforcement.saved_tiles.is_empty());

    let mut old_entities = app.world_mut().query::<&OldStageEntity>();
    assert_eq!(old_entities.iter(app.world()).count(), 0);
    let mut game_entities = app.world_mut().query::<&GameEntity>();
    assert!(game_entities.iter(app.world()).count() > 0);
}

#[test]
fn shared_control_r_restarts_current_versus_round() {
    let mut app = App::new();
    let arena_index = 5;
    let (expected_arena, expected_grid) = load_arena_bundle_or_panic(arena_index);
    let previous_level = parse_level(LEVEL_1).expect("level should parse");
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyR);
    let mut grid = TileGrid::empty();
    grid.set(10, 24, TileKind::Brick);
    let mut score_board = ScoreBoard::versus(1, 7, 0.5);
    score_board.p1_score = 4;
    score_board.p2_score = 3;
    score_board.p1_lives = 0;
    score_board.p2_lives = 1;
    let mut enemy_freeze = EnemyFreeze::default();
    enemy_freeze.start();
    let mut versus_freeze = VersusPlayerFreeze::default();
    versus_freeze.start(PlayerId::One);
    let mut base_reinforcement = BaseReinforcement {
        timer: None,
        saved_tiles: vec![(10, 24, TileKind::Steel)],
    };
    base_reinforcement.start();

    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusDeathmatch);
    app.insert_resource(GameStatus {
        phase: GamePhase::RoundOver,
        map_pack: CampaignMapPack::Custom,
        stage: 12,
        arena: arena_index,
        winner: Some(PlayerId::Two),
        transition_timer: Timer::from_seconds(1.0, TimerMode::Once),
    });
    app.insert_resource(grid);
    app.insert_resource(EnemyDirector::from_level(&previous_level));
    app.insert_resource(score_board);
    app.insert_resource(StageRules {
        player_steel_destruction: true,
    });
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect::default());
    app.insert_resource(enemy_freeze);
    app.insert_resource(versus_freeze);
    app.insert_resource(base_reinforcement);
    app.world_mut()
        .spawn((GameEntity, OldStageEntity, GridTile { x: 10, y: 24 }));
    app.add_systems(Update, handle_shared_controls);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::StageIntro);
    assert_eq!(status.arena, arena_index);
    assert_eq!(status.winner, None);
    assert_eq!(
        app.world().resource::<GameMode>(),
        &GameMode::VersusBaseBattle
    );
    assert_eq!(
        app.world().resource::<TileGrid>().tiles,
        expected_grid.tiles
    );

    let BattleRules::BaseBattle {
        lives,
        respawn_invulnerability_secs,
        ..
    } = expected_arena.battle_rules
    else {
        panic!("arena five should restart as base battle");
    };
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.p1_score, 0);
    assert_eq!(score_board.p2_score, 0);
    assert_eq!(score_board.p1_lives, lives);
    assert_eq!(score_board.p2_lives, lives);
    assert_eq!(score_board.target_score, 0);
    assert_eq!(
        score_board.respawn_invulnerability_secs,
        respawn_invulnerability_secs
    );

    let director = app.world().resource::<EnemyDirector>();
    assert!(director.roster.is_empty());
    assert!(director.spawns.is_empty());
    assert_eq!(director.max_active, 0);
    assert_eq!(*app.world().resource::<StageRules>(), StageRules::default());
    assert_eq!(
        app.world().resource::<VersusPowerUpDirector>().spawn_points,
        expected_arena
            .powerup_spawns
            .iter()
            .map(grid_point_top_left)
            .collect::<Vec<_>>()
    );
    assert!(!app.world().resource::<EnemyFreeze>().is_active());
    assert!(
        !app.world()
            .resource::<VersusPlayerFreeze>()
            .is_player_frozen(PlayerId::One)
    );
    let reinforcement = app.world().resource::<BaseReinforcement>();
    assert!(reinforcement.timer.is_none());
    assert!(reinforcement.saved_tiles.is_empty());

    let mut old_entities = app.world_mut().query::<&OldStageEntity>();
    assert_eq!(old_entities.iter(app.world()).count(), 0);
    let mut game_entities = app.world_mut().query::<&GameEntity>();
    assert!(game_entities.iter(app.world()).count() > 0);
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
fn player_spawn_delay_ticks_with_visible_spawn_shimmer() {
    assert!(!player_spawn_delay_can_tick(GamePhase::ModeSelect));
    assert!(player_spawn_delay_can_tick(GamePhase::StageIntro));
    assert!(player_spawn_delay_can_tick(GamePhase::Playing));
    assert!(!player_spawn_delay_can_tick(GamePhase::Paused));
    assert!(!player_spawn_delay_can_tick(GamePhase::LevelClear));
    assert!(!player_spawn_delay_can_tick(GamePhase::GameOver));
    assert!(!player_spawn_delay_can_tick(GamePhase::RoundOver));
    assert!(!player_spawn_delay_can_tick(GamePhase::Victory));
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
        map_pack: CampaignMapPack::Custom,
        stage: CUSTOM_LEVEL_COUNT,
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
    assert_eq!(status.stage, CUSTOM_LEVEL_COUNT);
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
    assert!(lines.contains(&"P ESC RESUME"));
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
fn mode_select_cycles_campaign_coop_battle_view_assist_ai_difficulty_music_sound_scale_stage_and_arena()
 {
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Campaign),
        ModeSelectOption::CoopCampaign
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::CoopCampaign),
        ModeSelectOption::Battle
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Battle),
        ModeSelectOption::MapPack
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::MapPack),
        ModeSelectOption::ViewMode
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::ViewMode),
        ModeSelectOption::ViewAssist
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::ViewAssist),
        ModeSelectOption::AiStrategy
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::AiStrategy),
        ModeSelectOption::Difficulty
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Difficulty),
        ModeSelectOption::Music
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Music),
        ModeSelectOption::Sound
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Sound),
        ModeSelectOption::Scale
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Scale),
        ModeSelectOption::Stage
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Stage),
        ModeSelectOption::Arena
    );
    assert_eq!(
        next_mode_select_option(ModeSelectOption::Arena),
        ModeSelectOption::Campaign
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Campaign),
        ModeSelectOption::Arena
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Arena),
        ModeSelectOption::Stage
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Stage),
        ModeSelectOption::Scale
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::CoopCampaign),
        ModeSelectOption::Campaign
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Battle),
        ModeSelectOption::CoopCampaign
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::MapPack),
        ModeSelectOption::Battle
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::ViewMode),
        ModeSelectOption::MapPack
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::ViewAssist),
        ModeSelectOption::ViewMode
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::AiStrategy),
        ModeSelectOption::ViewAssist
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Difficulty),
        ModeSelectOption::AiStrategy
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Music),
        ModeSelectOption::Difficulty
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Scale),
        ModeSelectOption::Sound
    );
    assert_eq!(
        previous_mode_select_option(ModeSelectOption::Sound),
        ModeSelectOption::Music
    );
    assert_eq!(
        GameMode::CoopCampaign.mode_select_option(),
        ModeSelectOption::CoopCampaign
    );
    assert_eq!(
        GameMode::VersusBaseBattle.mode_select_option(),
        ModeSelectOption::Battle
    );
}

#[test]
fn mode_select_view_values_cycle_and_label() {
    assert_eq!(ModeSelect::default().view_mode, TankViewMode::TwoD);
    assert!(ModeSelect::default().view_assist);
    assert_eq!(ModeSelect::default().view_target, PlayerId::One);
    assert_eq!(
        next_tank_view_mode(TankViewMode::TwoD),
        TankViewMode::ThreeD
    );
    assert_eq!(
        next_tank_view_mode(TankViewMode::ThreeD),
        TankViewMode::TwoD
    );
    assert_eq!(
        previous_tank_view_mode(TankViewMode::TwoD),
        TankViewMode::ThreeD
    );
    assert_eq!(tank_view_mode_label(TankViewMode::ThreeD), "3D");
    assert_eq!(view_assist_label(true), "ON");
    assert_eq!(view_assist_label(false), "OFF");
    assert_eq!(
        active_3d_view_target(GameMode::Campaign, PlayerId::Two),
        PlayerId::One
    );
    assert_eq!(
        active_3d_view_target(GameMode::CoopCampaign, PlayerId::Two),
        PlayerId::Two
    );
    assert_eq!(
        active_3d_view_target(GameMode::VersusDeathmatch, PlayerId::Two),
        PlayerId::Two
    );
    assert_eq!(
        resolved_3d_view_target(GameMode::CoopCampaign, PlayerId::Two, [PlayerId::One]),
        PlayerId::One
    );
    assert_eq!(
        resolved_3d_view_target(GameMode::VersusDeathmatch, PlayerId::One, [PlayerId::Two]),
        PlayerId::Two
    );
    assert_eq!(
        resolved_3d_view_target(
            GameMode::CoopCampaign,
            PlayerId::Two,
            std::iter::empty::<PlayerId>()
        ),
        PlayerId::Two
    );
}

#[test]
fn view_3d_renders_only_during_gameplay_phases() {
    let mode_select = ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    };
    let mode_select_2d = ModeSelect::default();
    assert!(!view_3d_should_render(&mode_select, &GameStatus::default()));
    assert!(!view_3d_should_render(
        &mode_select_2d,
        &GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        }
    ));
    assert!(view_3d_should_render(
        &mode_select,
        &GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        }
    ));
}

#[test]
fn view_hotkeys_toggle_mode_and_target() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyV);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(ModeSelect::default());
    app.insert_resource(GameMode::CoopCampaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.add_systems(Update, handle_view_hotkeys);

    app.update();
    assert_eq!(
        app.world().resource::<ModeSelect>().view_mode,
        TankViewMode::ThreeD
    );

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::KeyV);
        keys.clear();
        keys.press(KeyCode::Tab);
    }
    app.update();
    assert_eq!(
        app.world().resource::<ModeSelect>().view_target,
        PlayerId::Two
    );

    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.release(KeyCode::Tab);
        keys.clear();
        keys.press(KeyCode::Digit3);
    }
    app.update();
    assert_eq!(
        app.world().resource::<ModeSelect>().view_mode,
        TankViewMode::TwoD
    );
}

#[test]
fn view_hotkeys_do_not_switch_target_in_single_player_campaign() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Tab);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_target: PlayerId::One,
        ..ModeSelect::default()
    });
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.add_systems(Update, handle_view_hotkeys);

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().view_target,
        PlayerId::One
    );
}

#[test]
fn view_hotkeys_do_not_toggle_mode_select_view_without_redrawing_menu() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyV);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::TwoD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::ModeSelect,
        ..GameStatus::default()
    });
    app.add_systems(Update, handle_view_hotkeys);

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().view_mode,
        TankViewMode::TwoD
    );
}

#[test]
fn sync_view_cameras_activates_only_the_selected_view() {
    let mut app = App::new();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.world_mut().spawn((Camera::default(), Main2dCamera));
    app.world_mut().spawn((
        Camera {
            is_active: false,
            ..default()
        },
        Tank3dCamera,
    ));
    app.world_mut().spawn((
        Camera {
            is_active: false,
            ..default()
        },
        View3dHudCamera,
    ));
    app.add_systems(Update, sync_view_cameras);

    app.update();

    let mut cameras = app.world_mut().query::<(
        &Camera,
        Option<&Main2dCamera>,
        Option<&Tank3dCamera>,
        Option<&View3dHudCamera>,
    )>();
    let states: Vec<(bool, bool, bool, bool)> = cameras
        .iter(app.world())
        .map(|(camera, is_2d, is_3d, is_hud)| {
            (
                camera.is_active,
                is_2d.is_some(),
                is_3d.is_some(),
                is_hud.is_some(),
            )
        })
        .collect();
    assert!(states.iter().any(|(active, is_2d, _, _)| !active && *is_2d));
    assert!(states.iter().any(|(active, _, is_3d, _)| *active && *is_3d));
    assert!(
        states
            .iter()
            .any(|(active, _, _, is_hud)| *active && *is_hud)
    );

    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::ModeSelect;
    app.update();

    let states: Vec<(bool, bool, bool, bool)> = cameras
        .iter(app.world())
        .map(|(camera, is_2d, is_3d, is_hud)| {
            (
                camera.is_active,
                is_2d.is_some(),
                is_3d.is_some(),
                is_hud.is_some(),
            )
        })
        .collect();
    assert!(states.iter().any(|(active, is_2d, _, _)| *active && *is_2d));
    assert!(states.iter().any(|(active, _, is_3d, _)| !active && *is_3d));
    assert!(
        states
            .iter()
            .any(|(active, _, _, is_hud)| !active && *is_hud)
    );
}

#[test]
fn view_3d_status_lines_track_campaign_and_versus_state() {
    let mut campaign_score = ScoreBoard::campaign(20);
    campaign_score.score = 123;
    campaign_score.lives = 2;
    campaign_score.enemies_destroyed = 3;
    let game_status = GameStatus {
        phase: GamePhase::Playing,
        stage: 7,
        ..GameStatus::default()
    };
    let mode_select = ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_assist: false,
        view_target: PlayerId::Two,
        ..ModeSelect::default()
    };
    let campaign_view_target = active_3d_view_target(GameMode::Campaign, mode_select.view_target);

    assert_eq!(
        view_3d_status_lines(
            GameMode::Campaign,
            &game_status,
            &campaign_score,
            &mode_select,
            campaign_view_target
        ),
        vec![
            "P1 SCORE 000123".to_string(),
            "LIFE 2".to_string(),
            "STAGE 07".to_string(),
            "ENEMY 17".to_string(),
            "VIEW P1".to_string(),
            "ASSIST OFF".to_string(),
        ]
    );
    assert_eq!(campaign_view_target, PlayerId::One);

    let mut versus_score = ScoreBoard::versus(3, 5, 2.0);
    versus_score.p1_score = 4;
    versus_score.p2_score = 2;
    versus_score.p1_lives = 1;
    let versus_status = GameStatus {
        phase: GamePhase::Playing,
        arena: 8,
        ..GameStatus::default()
    };

    assert_eq!(
        view_3d_status_lines(
            GameMode::VersusDeathmatch,
            &versus_status,
            &versus_score,
            &ModeSelect::default(),
            active_3d_view_target(GameMode::VersusDeathmatch, PlayerId::One)
        ),
        vec![
            "P1 04 LIFE 1".to_string(),
            "P2 02 LIFE 3".to_string(),
            "ARENA 08".to_string(),
            "TARGET 05".to_string(),
            "VIEW P1".to_string(),
            "ASSIST ON".to_string(),
        ]
    );
}

#[test]
fn view_3d_enemy_direction_indicator_uses_target_facing() {
    let target = Vec2::new(96.0, 96.0);

    assert_eq!(
        view_3d_enemy_direction_indicator(target, Direction::Up, Vec2::new(96.0, 64.0))
            .map(|indicator| indicator.direction),
        Some(View3dEnemyDirection::Front)
    );
    assert_eq!(
        view_3d_enemy_direction_indicator(target, Direction::Up, Vec2::new(96.0, 128.0))
            .map(|indicator| indicator.direction),
        Some(View3dEnemyDirection::Back)
    );
    assert_eq!(
        view_3d_enemy_direction_indicator(target, Direction::Up, Vec2::new(128.0, 96.0))
            .map(|indicator| indicator.direction),
        Some(View3dEnemyDirection::Right)
    );
    assert_eq!(
        view_3d_enemy_direction_indicator(target, Direction::Up, Vec2::new(64.0, 96.0))
            .map(|indicator| indicator.direction),
        Some(View3dEnemyDirection::Left)
    );
    assert_eq!(
        view_3d_enemy_direction_indicator(target, Direction::Right, Vec2::new(96.0, 128.0))
            .map(|indicator| indicator.direction),
        Some(View3dEnemyDirection::Right)
    );
}

#[test]
fn view_3d_enemy_direction_indicators_track_nearest_threat_per_direction() {
    let target = Tank {
        top_left: Vec2::new(96.0, 96.0),
        facing: Direction::Up,
        speed: PLAYER_SPEED,
    };
    let player = Player { id: PlayerId::One };
    let p2_tank = Tank {
        top_left: Vec2::new(96.0, 128.0),
        facing: Direction::Up,
        speed: PLAYER_SPEED,
    };
    let player_two = Player { id: PlayerId::Two };
    let far_enemy_tank = Tank {
        top_left: Vec2::new(96.0, 32.0),
        facing: Direction::Down,
        speed: enemy_speed(EnemyKind::Basic),
    };
    let near_enemy_tank = Tank {
        top_left: Vec2::new(96.0, 72.0),
        facing: Direction::Down,
        speed: enemy_speed(EnemyKind::Fast),
    };
    let right_enemy_tank = Tank {
        top_left: Vec2::new(128.0, 96.0),
        facing: Direction::Left,
        speed: enemy_speed(EnemyKind::Power),
    };
    let far_enemy = EnemyTank {
        kind: EnemyKind::Basic,
        carried_powerup: None,
    };
    let near_enemy = EnemyTank {
        kind: EnemyKind::Fast,
        carried_powerup: None,
    };
    let right_enemy = EnemyTank {
        kind: EnemyKind::Power,
        carried_powerup: None,
    };

    let campaign = view_3d_enemy_direction_indicators(
        GameMode::Campaign,
        [
            (&target, Some(&player), None),
            (&p2_tank, Some(&player_two), None),
            (&far_enemy_tank, None, Some(&far_enemy)),
            (&near_enemy_tank, None, Some(&near_enemy)),
            (&right_enemy_tank, None, Some(&right_enemy)),
        ],
        PlayerId::One,
    );
    assert_eq!(
        campaign,
        vec![
            View3dEnemyDirectionIndicator {
                direction: View3dEnemyDirection::Front,
                distance_tiles: 3
            },
            View3dEnemyDirectionIndicator {
                direction: View3dEnemyDirection::Right,
                distance_tiles: 4
            },
        ]
    );

    let versus = view_3d_enemy_direction_indicators(
        GameMode::VersusDeathmatch,
        [
            (&target, Some(&player), None),
            (&p2_tank, Some(&player_two), None),
        ],
        PlayerId::One,
    );
    assert_eq!(
        versus,
        vec![View3dEnemyDirectionIndicator {
            direction: View3dEnemyDirection::Back,
            distance_tiles: 4
        }]
    );
}

#[test]
fn view_3d_minimap_image_uses_board_sized_pixel_grid() {
    let image = create_3d_minimap_image();

    assert_eq!(
        image.texture_descriptor.size.width,
        VIEW_3D_MINIMAP_SIZE as u32
    );
    assert_eq!(
        image.texture_descriptor.size.height,
        VIEW_3D_MINIMAP_SIZE as u32
    );
    assert_eq!(
        image
            .data
            .as_ref()
            .expect("minimap should keep pixels")
            .len(),
        VIEW_3D_MINIMAP_SIZE * VIEW_3D_MINIMAP_SIZE * 4
    );
    assert_eq!(image_pixel(&image, 0, 0), [0, 0, 0, 0]);
}

#[test]
fn view_3d_minimap_center_anchors_to_window_top_right() {
    let windowed_size = Vec2::new(VIRTUAL_WIDTH * 3.0, VIRTUAL_HEIGHT * 3.0);
    let windowed_center = view_3d_minimap_center(windowed_size, 3.0);
    let windowed_panel_half = ((VIEW_3D_MINIMAP_SIZE as f32 + 6.0) * 3.0) / 2.0;
    assert_eq!(
        windowed_center + Vec2::splat(windowed_panel_half),
        windowed_size / 2.0 - Vec2::splat(9.0)
    );

    let fullscreen_size = Vec2::new(2048.0, 1280.0);
    let fullscreen_center = view_3d_minimap_center(fullscreen_size, 4.0);
    let panel_half = ((VIEW_3D_MINIMAP_SIZE as f32 + 6.0) * 4.0) / 2.0;

    assert_eq!(
        fullscreen_center + Vec2::splat(panel_half),
        fullscreen_size / 2.0 - Vec2::splat(12.0)
    );
    assert!(fullscreen_center.x > windowed_center.x);
}

#[test]
fn window_top_left_center_anchors_to_actual_window_edge() {
    let window_size = Vec2::new(1254.0, 923.0);
    let size = Vec2::new(20.0, 28.0);

    assert_eq!(
        window_top_left_center(Vec2::ZERO, size, window_size, 21.0),
        Vec3::new(-617.0, 447.5, 21.0)
    );
    assert_eq!(
        window_top_left_center(Vec2::new(12.0, 8.0), size, window_size, 21.0),
        Vec3::new(-605.0, 439.5, 21.0)
    );
}

#[test]
fn view_3d_minimap_pixels_encode_tiles_units_and_target_player() {
    let mut grid = TileGrid::empty();
    grid.set(1, 2, TileKind::Brick);
    grid.set(3, 4, TileKind::Steel);
    grid.set(5, 6, TileKind::Water);

    let player_tank = Tank {
        top_left: Vec2::new(0.0, 0.0),
        facing: Direction::Up,
        speed: PLAYER_SPEED,
    };
    let player = Player { id: PlayerId::One };
    let enemy_tank = Tank {
        top_left: Vec2::new(64.0, 16.0),
        facing: Direction::Down,
        speed: PLAYER_SPEED,
    };
    let enemy = EnemyTank {
        kind: EnemyKind::Basic,
        carried_powerup: None,
    };
    let carrier_tank = Tank {
        top_left: Vec2::new(80.0, 16.0),
        facing: Direction::Down,
        speed: PLAYER_SPEED,
    };
    let carrier_enemy = EnemyTank {
        kind: EnemyKind::Basic,
        carried_powerup: Some(PowerUpKind::Grenade),
    };
    let bullet = Bullet {
        previous_top_left: Vec2::new(100.0, 4.0),
        top_left: Vec2::new(100.0, 4.0),
        facing: Direction::Right,
        owner: Team::Player1,
        speed: BULLET_SPEED,
        breaks_steel: false,
        resolved: false,
    };
    let base = BaseSprite {
        owner: None,
        top_left: Vec2::new(80.0, 80.0),
    };
    let player_two_base = BaseSprite {
        owner: Some(PlayerId::Two),
        top_left: Vec2::new(96.0, 80.0),
    };
    let powerup = PowerUp {
        kind: PowerUpKind::Helmet,
    };
    let powerup_transform = Transform::from_translation(board_object_center(
        112.0,
        80.0,
        Vec2::splat(TANK_SIZE),
        0.0,
    ));

    let pixels = render_3d_minimap_pixels(
        &grid,
        [
            (&player_tank, Some(&player), None::<&EnemyTank>),
            (&enemy_tank, None::<&Player>, Some(&enemy)),
            (&carrier_tank, None::<&Player>, Some(&carrier_enemy)),
        ],
        [&bullet],
        [&base, &player_two_base],
        [(&powerup, &powerup_transform)],
        PlayerId::One,
    );
    let cell = VIEW_3D_MINIMAP_CELL_PIXELS;

    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, cell + 1, 2 * cell + 1),
        [152, 64, 36, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, cell, 2 * cell),
        [8, 8, 8, 210]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 3 * cell + 1, 4 * cell + 1),
        [144, 152, 160, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 5 * cell + 1, 6 * cell + 1),
        [28, 96, 184, 245]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, cell, cell),
        [255, 255, 255, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, cell + 1, cell + 1),
        [184, 248, 184, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 9 * cell, 3 * cell),
        [0, 0, 0, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 9 * cell + 1, 3 * cell + 1),
        [248, 88, 80, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 11 * cell + 1, 3 * cell + 1),
        [224, 64, 56, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 12 * cell, 0),
        [0, 0, 0, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 12 * cell + 1, 1),
        [184, 248, 184, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 11 * cell + 1, 11 * cell + 1),
        [248, 216, 96, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 13 * cell + 1, 11 * cell + 1),
        [112, 184, 255, 255]
    );
    assert_eq!(
        pixels_pixel(&pixels, VIEW_3D_MINIMAP_SIZE, 15 * cell + 1, 11 * cell + 1),
        [112, 216, 248, 255]
    );
}

#[test]
fn view_3d_camera_path_detects_tall_tile_obstructions() {
    let mut grid = TileGrid::empty();
    let from = Vec2::new(104.0, 104.0);
    let to = Vec2::new(104.0, 138.0);

    assert!(!camera_path_is_obstructed(&grid, from, to));

    grid.set(13, 15, TileKind::Brick);
    assert!(camera_path_is_obstructed(&grid, from, to));

    grid.set(13, 15, TileKind::Water);
    assert!(!camera_path_is_obstructed(&grid, from, to));

    grid.set(13, 15, TileKind::Forest);
    assert!(camera_path_is_obstructed(&grid, from, to));
}

#[test]
fn view_3d_chase_camera_moves_closer_without_raising_when_path_is_obstructed() {
    let tank = Tank {
        top_left: Vec2::new(96.0, 96.0),
        facing: Direction::Up,
        speed: PLAYER_SPEED,
    };
    let mut grid = TileGrid::empty();

    let clear = chase_camera_transform(&tank, &grid);
    assert!((clear.translation.y - 23.7).abs() < 0.01);

    grid.set(13, 15, TileKind::Steel);
    let obstructed = chase_camera_transform(&tank, &grid);
    assert!((obstructed.translation.y - clear.translation.y).abs() < 0.01);
    assert!(obstructed.translation.z < clear.translation.z);
}

#[test]
fn view_3d_turning_between_camera_modes_keeps_eye_height_stable() {
    let tank = Tank {
        top_left: Vec2::new(96.0, 96.0),
        facing: Direction::Up,
        speed: PLAYER_SPEED,
    };
    let chase = chase_camera_transform_with_forward_and_height_mode(
        &tank,
        Vec2::Y,
        View3dCameraHeightMode::Chase,
    );
    let tactical = chase_camera_transform_with_forward_and_height_mode(
        &tank,
        Direction::Right.movement(),
        View3dCameraHeightMode::Tactical,
    );

    assert!((tactical.translation.y - chase.translation.y).abs() < 0.01);
}

#[test]
fn view_3d_camera_state_smooths_direction_reversals() {
    let mut state = View3dCameraState::default();
    let initial = state.smoothed_forward(PlayerId::One, Direction::Left.movement(), 1.0 / 60.0);
    assert!((initial - Direction::Left.movement()).length() < 0.001);

    let reversed = state.smoothed_forward(PlayerId::One, Direction::Right.movement(), 1.0 / 60.0);

    assert!((reversed - Direction::Right.movement()).length() > 0.5);
    assert!(reversed.dot(Direction::Left.movement()) > 0.9);
}

#[test]
fn view_3d_camera_height_mode_waits_for_turn_to_settle_before_raising() {
    let calls = std::cell::Cell::new(0);
    let mut state = View3dCameraState::default();
    let initial = state.smoothed_forward(PlayerId::One, Direction::Left.movement(), 1.0 / 60.0);
    assert_eq!(
        state.stable_height_mode(initial, Direction::Left.movement(), || {
            calls.set(calls.get() + 1);
            View3dCameraHeightMode::Chase
        },),
        View3dCameraHeightMode::Chase
    );
    assert_eq!(calls.get(), 1);

    let turning = state.smoothed_forward(PlayerId::One, Direction::Right.movement(), 1.0 / 60.0);
    assert_eq!(
        state.stable_height_mode(turning, Direction::Right.movement(), || {
            calls.set(calls.get() + 1);
            View3dCameraHeightMode::Tactical
        },),
        View3dCameraHeightMode::Chase
    );
    assert_eq!(calls.get(), 1);

    let mut settled = turning;
    for _ in 0..12 {
        settled = state.smoothed_forward(PlayerId::One, Direction::Right.movement(), 1.0 / 60.0);
    }
    assert_eq!(
        state.stable_height_mode(settled, Direction::Right.movement(), || {
            calls.set(calls.get() + 1);
            View3dCameraHeightMode::Tactical
        },),
        View3dCameraHeightMode::Tactical
    );
    assert_eq!(calls.get(), 2);
}

#[test]
fn view_3d_camera_height_mode_stays_tactical_while_turning_down() {
    let calls = std::cell::Cell::new(0);
    let mut state = View3dCameraState::default();
    let initial = state.smoothed_forward(PlayerId::One, Direction::Left.movement(), 1.0 / 60.0);
    assert_eq!(
        state.stable_height_mode(initial, Direction::Left.movement(), || {
            calls.set(calls.get() + 1);
            View3dCameraHeightMode::Tactical
        },),
        View3dCameraHeightMode::Tactical
    );
    assert_eq!(calls.get(), 1);

    let turning = state.smoothed_forward(PlayerId::One, Direction::Right.movement(), 1.0 / 60.0);
    assert_eq!(
        state.stable_height_mode(turning, Direction::Right.movement(), || {
            calls.set(calls.get() + 1);
            View3dCameraHeightMode::Chase
        },),
        View3dCameraHeightMode::Tactical
    );
    assert_eq!(calls.get(), 1);
}

#[test]
fn view_3d_camera_state_resets_when_followed_player_changes() {
    let mut state = View3dCameraState::default();
    state.smoothed_forward(PlayerId::One, Direction::Left.movement(), 1.0 / 60.0);

    let switched = state.smoothed_forward(PlayerId::Two, Direction::Right.movement(), 1.0 / 60.0);

    assert!((switched - Direction::Right.movement()).length() < 0.001);
}

#[test]
fn view_3d_camera_state_smooths_transform_motion() {
    let mut state = View3dCameraState::default();
    let first = Transform::from_xyz(0.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y);
    let second = Transform::from_xyz(100.0, 20.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y);

    let initial = state.smoothed_transform(PlayerId::One, first, 1.0 / 60.0);
    let smoothed = state.smoothed_transform(PlayerId::One, second, 1.0 / 60.0);

    assert!((initial.translation - first.translation).length() < 0.001);
    assert!(smoothed.translation.x > first.translation.x);
    assert!(smoothed.translation.x < second.translation.x);
}

#[test]
fn update_3d_chase_camera_uses_campaign_active_target() {
    let p1_tank = Tank {
        top_left: Vec2::new(32.0, 64.0),
        facing: Direction::Right,
        speed: PLAYER_SPEED,
    };
    let p2_tank = Tank {
        top_left: Vec2::new(120.0, 32.0),
        facing: Direction::Left,
        speed: PLAYER_SPEED,
    };
    let expected = chase_camera_transform(&p1_tank, &TileGrid::empty());
    let mut app = App::new();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_target: PlayerId::Two,
        ..ModeSelect::default()
    });
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut()
        .spawn((p1_tank, Player { id: PlayerId::One }));
    app.world_mut()
        .spawn((p2_tank, Player { id: PlayerId::Two }));
    app.world_mut().spawn((Transform::default(), Tank3dCamera));
    app.add_systems(Update, update_3d_chase_camera);

    app.update();

    let mut cameras = app.world_mut().query::<&Transform>();
    let camera = cameras
        .iter(app.world())
        .find(|transform| transform.translation != Vec3::ZERO)
        .expect("3D camera should move to the active target");
    assert!((camera.translation - expected.translation).length() < 0.01);
}

#[test]
fn update_3d_chase_camera_falls_back_to_available_two_player_target() {
    let p1_tank = Tank {
        top_left: Vec2::new(32.0, 64.0),
        facing: Direction::Right,
        speed: PLAYER_SPEED,
    };
    let expected = chase_camera_transform(&p1_tank, &TileGrid::empty());
    let mut app = App::new();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_target: PlayerId::Two,
        ..ModeSelect::default()
    });
    app.insert_resource(GameMode::CoopCampaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut()
        .spawn((p1_tank, Player { id: PlayerId::One }));
    app.world_mut().spawn((Transform::default(), Tank3dCamera));
    app.add_systems(Update, update_3d_chase_camera);

    app.update();

    let mut cameras = app.world_mut().query::<&Transform>();
    let camera = cameras
        .iter(app.world())
        .find(|transform| transform.translation != Vec3::ZERO)
        .expect("3D camera should follow the available player");
    assert!((camera.translation - expected.translation).length() < 0.01);
}

#[test]
fn view_3d_aim_guide_length_stops_at_bullet_blocking_tiles() {
    let tank = Tank {
        top_left: Vec2::new(32.0, 32.0),
        facing: Direction::Right,
        speed: PLAYER_SPEED,
    };
    let mut grid = TileGrid::empty();

    let open_length = aim_guide_length(&grid, &tank);
    assert_eq!(open_length, TILE_SIZE * 4.0);

    grid.set(8, 5, TileKind::Brick);
    let blocked_by_brick = aim_guide_length(&grid, &tank);
    assert!(blocked_by_brick < open_length);
    assert!(blocked_by_brick > TILE_SIZE);

    grid.set(8, 5, TileKind::Forest);
    assert_eq!(aim_guide_length(&grid, &tank), open_length);

    grid.set(8, 5, TileKind::Base);
    assert_eq!(aim_guide_length(&grid, &tank), blocked_by_brick);
}

#[test]
fn view_3d_protection_kind_prioritizes_spawn_over_shield() {
    assert_eq!(tank_3d_protection_kind(false, false), None);
    assert_eq!(
        tank_3d_protection_kind(true, false),
        Some(Tank3dProtectionKind::Shield)
    );
    assert_eq!(
        tank_3d_protection_kind(false, true),
        Some(Tank3dProtectionKind::Spawn)
    );
    assert_eq!(
        tank_3d_protection_kind(true, true),
        Some(Tank3dProtectionKind::Spawn)
    );
}

#[test]
fn view_3d_effect_kind_reuses_sprite_animation_ranges() {
    fn animation_for(frames: SpriteFrameRange) -> SpriteAnimation {
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            despawn_on_finish: true,
        }
    }

    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");

    assert_eq!(
        view_3d_effect_kind(&animation_for(manifest.explosion_frames()), &manifest),
        Some(View3dEffectKind::Explosion)
    );
    assert_eq!(
        view_3d_effect_kind(
            &animation_for(manifest.base_destruction_frames()),
            &manifest
        ),
        Some(View3dEffectKind::BaseDestruction)
    );
    assert_eq!(
        view_3d_effect_kind(&animation_for(manifest.bullet_impact_frames()), &manifest),
        Some(View3dEffectKind::BulletImpact)
    );
    assert_eq!(
        view_3d_effect_kind(&animation_for(manifest.spawn_shimmer_frames()), &manifest),
        Some(View3dEffectKind::SpawnShimmer)
    );
    assert_eq!(
        view_3d_effect_kind(&animation_for(manifest.powerup_sparkle_frames()), &manifest),
        Some(View3dEffectKind::PowerUpSparkle)
    );
    assert_eq!(
        view_3d_effect_kind(
            &animation_for(SpriteFrameRange {
                first: 90,
                last: 91
            }),
            &manifest
        ),
        None
    );

    assert_eq!(
        view_3d_effect_size(View3dEffectKind::BulletImpact),
        BULLET_SIZE
    );
    assert_eq!(view_3d_effect_size(View3dEffectKind::Explosion), TANK_SIZE);
}

#[test]
fn directed_bullet_impact_effect_records_3d_direction() {
    fn spawn_directed_impact_for_test(mut commands: Commands, assets: Res<SpriteAssets>) {
        spawn_directed_bullet_impact_effect(
            &mut commands,
            &assets,
            Vec2::new(16.0, 16.0),
            Direction::Left,
        );
    }

    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.add_systems(Update, spawn_directed_impact_for_test);

    app.update();

    let mut impacts = app.world_mut().query::<&BulletImpactDirection>();
    let directions = impacts
        .iter(app.world())
        .map(|impact| impact.direction)
        .collect::<Vec<_>>();
    assert_eq!(directions, vec![Direction::Left]);
}

#[test]
fn view_3d_powerup_material_kind_preserves_powerup_identity() {
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Star),
        PowerUp3dMaterialKind::Star
    );
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Helmet),
        PowerUp3dMaterialKind::Helmet
    );
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Clock),
        PowerUp3dMaterialKind::Clock
    );
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Grenade),
        PowerUp3dMaterialKind::Grenade
    );
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Shovel),
        PowerUp3dMaterialKind::Shovel
    );
    assert_eq!(
        powerup_3d_material_kind(PowerUpKind::Tank),
        PowerUp3dMaterialKind::Tank
    );
}

#[test]
fn view_3d_base_material_kind_preserves_base_owner_identity() {
    assert_eq!(base_3d_material_kind(None), Base3dMaterialKind::Neutral);
    assert_eq!(
        base_3d_material_kind(Some(PlayerId::One)),
        Base3dMaterialKind::PlayerOne
    );
    assert_eq!(
        base_3d_material_kind(Some(PlayerId::Two)),
        Base3dMaterialKind::PlayerTwo
    );
}

#[test]
fn view_3d_minimap_base_color_preserves_base_owner_identity() {
    assert_eq!(minimap_base_color(None), [248, 216, 96, 255]);
    assert_eq!(
        minimap_base_color(Some(PlayerId::One)),
        [144, 248, 152, 255]
    );
    assert_eq!(
        minimap_base_color(Some(PlayerId::Two)),
        [112, 184, 255, 255]
    );
}

#[test]
fn view_3d_minimap_powerup_color_preserves_powerup_identity() {
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Star),
        [248, 216, 72, 255]
    );
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Helmet),
        [112, 216, 248, 255]
    );
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Clock),
        [96, 208, 255, 255]
    );
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Grenade),
        [224, 64, 56, 255]
    );
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Shovel),
        [184, 232, 160, 255]
    );
    assert_eq!(
        minimap_powerup_color(PowerUpKind::Tank),
        [96, 240, 112, 255]
    );
}

#[test]
fn view_3d_minimap_enemy_color_marks_powerup_carriers() {
    let normal_enemy = EnemyTank {
        kind: EnemyKind::Basic,
        carried_powerup: None,
    };
    let carrier_enemy = EnemyTank {
        kind: EnemyKind::Basic,
        carried_powerup: Some(PowerUpKind::Grenade),
    };

    assert_eq!(minimap_enemy_color(&normal_enemy), [248, 88, 80, 255]);
    assert_eq!(minimap_enemy_color(&carrier_enemy), [224, 64, 56, 255]);
}

#[test]
fn view_3d_bullet_material_and_minimap_color_preserve_owner_identity() {
    assert_eq!(
        bullet_3d_material_kind(Team::Player1),
        Bullet3dMaterialKind::PlayerOne
    );
    assert_eq!(
        bullet_3d_material_kind(Team::Player2),
        Bullet3dMaterialKind::PlayerTwo
    );
    assert_eq!(
        bullet_3d_material_kind(Team::Enemy),
        Bullet3dMaterialKind::Enemy
    );

    assert_eq!(minimap_bullet_color(Team::Player1), [184, 248, 184, 255]);
    assert_eq!(minimap_bullet_color(Team::Player2), [136, 216, 255, 255]);
    assert_eq!(minimap_bullet_color(Team::Enemy), [248, 88, 80, 255]);
}

#[test]
fn view_3d_tank_material_kind_preserves_freeze_and_damage_state() {
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::One), None, None, 1, false, false),
        Tank3dMaterialKind::PlayerOne
    );
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::Two), Some(0), None, 1, false, false),
        Tank3dMaterialKind::PlayerUpgrade0
    );
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::One), Some(1), None, 1, false, false),
        Tank3dMaterialKind::PlayerUpgrade1
    );
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::Two), Some(2), None, 1, false, false),
        Tank3dMaterialKind::PlayerUpgrade2
    );
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::One), Some(99), None, 1, false, false),
        Tank3dMaterialKind::PlayerUpgrade3
    );
    assert_eq!(
        tank_3d_material_kind(Some(PlayerId::Two), Some(3), None, 1, false, true),
        Tank3dMaterialKind::Frozen
    );
    assert_eq!(
        tank_3d_material_kind(None, None, Some(EnemyKind::Fast), 1, false, false),
        Tank3dMaterialKind::EnemyFast
    );
    assert_eq!(
        tank_3d_material_kind(None, None, Some(EnemyKind::Armor), 1, false, false),
        Tank3dMaterialKind::EnemyPower
    );
    assert_eq!(
        tank_3d_material_kind(None, None, Some(EnemyKind::Armor), 3, false, false),
        Tank3dMaterialKind::EnemyArmor
    );
    assert_eq!(
        tank_3d_material_kind(None, None, Some(EnemyKind::Basic), 1, true, false),
        Tank3dMaterialKind::Frozen
    );
}

#[test]
fn sloped_box_mesh_insets_top_face_for_less_blocky_models() {
    let mesh = sloped_box_mesh(Vec2::new(12.0, 10.0), Vec2::new(8.0, 6.0), 4.0);
    let positions = mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("mesh should have positions")
        .as_float3()
        .expect("positions should be vec3");
    let bottom_max_x = positions
        .iter()
        .filter(|position| position[1] < 0.0)
        .map(|position| position[0].abs())
        .fold(0.0, f32::max);
    let top_max_x = positions
        .iter()
        .filter(|position| position[1] > 0.0)
        .map(|position| position[0].abs())
        .fold(0.0, f32::max);

    assert_eq!(mesh.count_vertices(), 24);
    assert!(top_max_x < bottom_max_x);
}

#[test]
fn tank_barrel_transform_rotates_cylinder_axis_to_facing() {
    for (direction, expected_axis) in [
        (Direction::Up, Vec3::NEG_Z),
        (Direction::Down, Vec3::Z),
        (Direction::Left, Vec3::NEG_X),
        (Direction::Right, Vec3::X),
    ] {
        let transform = tank_barrel_transform(Vec2::new(96.0, 96.0), direction);
        let axis = transform.rotation * Vec3::Y;

        assert!(
            axis.dot(expected_axis) > 0.999,
            "{direction:?} barrel should align with {expected_axis:?}, got {axis:?}"
        );
    }
}

#[test]
fn tank_front_plate_transform_rotates_local_front_to_facing() {
    for (direction, expected_axis) in [
        (Direction::Up, Vec3::NEG_Z),
        (Direction::Down, Vec3::Z),
        (Direction::Left, Vec3::NEG_X),
        (Direction::Right, Vec3::X),
    ] {
        let transform = tank_front_plate_transform(Vec2::new(96.0, 96.0), direction);
        let axis = transform.rotation * Vec3::NEG_Z;

        assert!(
            axis.dot(expected_axis) > 0.999,
            "{direction:?} front plate should align with {expected_axis:?}, got {axis:?}"
        );
    }
}

#[test]
fn tank_muzzle_transform_sits_ahead_of_barrel() {
    for (direction, expected_axis) in [
        (Direction::Up, Vec3::NEG_Z),
        (Direction::Down, Vec3::Z),
        (Direction::Left, Vec3::NEG_X),
        (Direction::Right, Vec3::X),
    ] {
        let barrel = tank_barrel_transform(Vec2::new(96.0, 96.0), direction);
        let muzzle = tank_muzzle_transform(Vec2::new(96.0, 96.0), direction);
        let delta = (muzzle.translation - barrel.translation).normalize_or_zero();

        assert!(
            delta.dot(expected_axis) > 0.999,
            "{direction:?} muzzle should sit ahead of barrel, got {delta:?}"
        );
    }
}

#[test]
fn bullet_3d_transform_rotates_capsule_axis_to_facing() {
    for (direction, expected_axis) in [
        (Direction::Up, Vec3::NEG_Z),
        (Direction::Down, Vec3::Z),
        (Direction::Left, Vec3::NEG_X),
        (Direction::Right, Vec3::X),
    ] {
        let bullet = Bullet {
            previous_top_left: Vec2::new(64.0, 64.0),
            top_left: Vec2::new(64.0, 64.0),
            facing: direction,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        };
        let transform = bullet_3d_transform(&bullet);
        let axis = transform.rotation * Vec3::Y;

        assert!(
            axis.dot(expected_axis) > 0.999,
            "{direction:?} bullet should align with {expected_axis:?}, got {axis:?}"
        );
    }
}

#[test]
fn bullet_trail_3d_transform_sits_behind_bullet() {
    for (direction, expected_axis) in [
        (Direction::Up, Vec3::NEG_Z),
        (Direction::Down, Vec3::Z),
        (Direction::Left, Vec3::NEG_X),
        (Direction::Right, Vec3::X),
    ] {
        let bullet = Bullet {
            previous_top_left: Vec2::new(64.0, 64.0),
            top_left: Vec2::new(64.0, 64.0),
            facing: direction,
            owner: Team::Enemy,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        };
        let bullet_transform = bullet_3d_transform(&bullet);
        let trail_transform = bullet_trail_3d_transform(&bullet);
        let delta = bullet_transform.translation - trail_transform.translation;
        let horizontal_delta = Vec3::new(delta.x, 0.0, delta.z).normalize_or_zero();

        assert!(
            horizontal_delta.dot(expected_axis) > 0.999,
            "{direction:?} trail should sit behind the bullet, got {horizontal_delta:?}"
        );
    }
}

#[test]
fn bullet_trail_3d_transform_uses_swept_segment_when_available() {
    let bullet = Bullet {
        previous_top_left: Vec2::new(64.0, 64.0),
        top_left: Vec2::new(68.0, 64.0),
        facing: Direction::Right,
        owner: Team::Player1,
        speed: BULLET_SPEED,
        breaks_steel: false,
        resolved: false,
    };

    let trail_transform = bullet_trail_3d_transform(&bullet);
    let expected_center = Vec2::new(68.0, 66.0);
    let expected_translation = Vec3::new(
        expected_center.x - board_size() / 2.0,
        trail_transform.translation.y,
        expected_center.y - board_size() / 2.0,
    );

    assert!(
        trail_transform.translation.distance(expected_translation) < 0.001,
        "trail should center on the swept bullet segment"
    );
    assert!(
        (trail_transform.scale.y - 2.0).abs() < 0.001,
        "trail length should match the 4px swept segment"
    );
}

#[test]
fn water_tile_exposed_edges_skip_internal_water_borders() {
    let mut grid = TileGrid::empty();
    grid.set(4, 4, TileKind::Water);
    grid.set(5, 4, TileKind::Water);

    assert_eq!(
        water_tile_exposed_edges(&grid, 4, 4),
        vec![Direction::Up, Direction::Down, Direction::Left]
    );
    assert_eq!(
        water_tile_exposed_edges(&grid, 5, 4),
        vec![Direction::Up, Direction::Right, Direction::Down]
    );
}

#[test]
fn movement_block_contact_distance_uses_full_tank_footprint() {
    let mut grid = TileGrid::empty();
    grid.set(6, 4, TileKind::Brick);
    let tank = Tank {
        top_left: Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE),
        facing: Direction::Right,
        speed: PLAYER_SPEED,
    };

    assert_eq!(
        movement_block_contact_distance(&grid, &tank, TILE_SIZE * 3.0),
        Some(0.0)
    );
}

#[test]
fn movement_block_contact_distance_ignores_clear_paths() {
    let grid = TileGrid::empty();
    let tank = Tank {
        top_left: Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE),
        facing: Direction::Right,
        speed: PLAYER_SPEED,
    };

    assert_eq!(
        movement_block_contact_distance(&grid, &tank, TILE_SIZE * 3.0),
        None
    );
}

#[test]
fn sync_3d_dynamic_scene_spawns_sprite_animation_effects() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        SpriteAnimation {
            first: manifest.explosion_frames().first,
            last: manifest.explosion_frames().last,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        Transform::from_translation(board_object_center(32.0, 32.0, Vec2::splat(TANK_SIZE), 8.1)),
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    let effect_names = names
        .iter(app.world())
        .map(|(name, _)| name.as_str().to_owned())
        .collect::<Vec<_>>();
    for expected_part in ["Flash", "Core", "Smoke", "Spark"] {
        assert!(
            effect_names
                .iter()
                .any(|name| name.starts_with(&format!("3D Effect Explosion {expected_part}"))),
            "3D explosion should include {expected_part}; got {effect_names:?}"
        );
    }
}

#[test]
fn sync_3d_dynamic_scene_orients_directed_bullet_impact_effects() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        SpriteAnimation {
            first: manifest.bullet_impact_frames().first,
            last: manifest.bullet_impact_frames().last,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        Transform::from_translation(board_object_center(
            48.0,
            48.0,
            Vec2::splat(BULLET_SIZE),
            8.1,
        )),
        BulletImpactDirection {
            direction: Direction::Right,
        },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app
        .world_mut()
        .query::<(&Name, &Transform, &View3dDynamic)>();
    let surface_mark = names
        .iter(app.world())
        .find_map(|(name, transform, _)| {
            name.as_str()
                .starts_with("3D Effect BulletImpact SurfaceMark")
                .then_some(*transform)
        })
        .expect("directed impact should spawn a 3D surface mark");

    assert!(
        surface_mark.scale.x < surface_mark.scale.z,
        "right-facing impact should make a thin X surface mark"
    );
}

#[test]
fn sync_3d_dynamic_scene_marks_bullet_owner() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn(Bullet {
        previous_top_left: Vec2::new(32.0, 32.0),
        top_left: Vec2::new(32.0, 32.0),
        facing: Direction::Right,
        owner: Team::Player2,
        speed: BULLET_SPEED,
        breaks_steel: false,
        resolved: false,
    });
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    let bullet_names = names
        .iter(app.world())
        .map(|(name, _)| name.as_str().to_owned())
        .collect::<Vec<_>>();
    assert!(
        bullet_names
            .iter()
            .any(|name| name.starts_with("3D Bullet PlayerTwo"))
    );
    assert!(
        bullet_names
            .iter()
            .any(|name| name.starts_with("3D Bullet Trail PlayerTwo"))
    );
    assert!(
        bullet_names
            .iter()
            .any(|name| name.starts_with("3D Bullet Glow PlayerTwo"))
    );
}

#[test]
fn sync_3d_dynamic_scene_marks_frozen_enemies() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    let mut enemy_freeze = EnemyFreeze::default();
    enemy_freeze.start();
    app.insert_resource(enemy_freeze);
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Down,
            speed: enemy_speed(EnemyKind::Basic),
        },
        EnemyTank {
            kind: EnemyKind::Basic,
            carried_powerup: None,
        },
        Health { current: 1 },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str().starts_with("3D Tank Frozen"))
    );
}

#[test]
fn sync_3d_dynamic_scene_marks_player_upgrade_level() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Down,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
        PlayerUpgrade { level: 2 },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str().starts_with("3D Tank PlayerUpgrade2"))
    );
}

#[test]
fn sync_3d_dynamic_scene_spawns_player_identity_marker_without_assist() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_assist: false,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Down,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::Two },
        PlayerUpgrade { level: 3 },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    let names = names
        .iter(app.world())
        .map(|(name, _)| name.as_str().to_string())
        .collect::<Vec<_>>();
    assert!(
        names
            .iter()
            .any(|name| name.starts_with("3D Tank PlayerUpgrade3"))
    );
    for expected_part in [
        "3D Tank Side Armor",
        "3D Tank Track Pad",
        "3D Tank Exhaust",
        "3D Tank Turret Ring",
        "3D Tank Periscope",
        "3D Tank Antenna",
        "3D Tank Headlight",
    ] {
        assert!(
            names.iter().any(|name| name == expected_part),
            "3D tank should include {expected_part}; got {names:?}"
        );
    }
    assert!(names.iter().any(|name| name == "3D Player Marker Two"));
    assert!(!names.iter().any(|name| name == "3D Aim Guide"));
}

#[test]
fn sync_3d_dynamic_scene_marks_resolved_view_target_player() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(GameMode::CoopCampaign);
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_target: PlayerId::Two,
        view_assist: false,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Down,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    let names = names
        .iter(app.world())
        .map(|(name, _)| name.as_str().to_string())
        .collect::<Vec<_>>();
    assert!(names.iter().any(|name| name == "3D View Target One"));
    assert!(!names.iter().any(|name| name == "3D View Target Two"));
}

#[test]
fn sync_3d_dynamic_scene_spawns_player_aim_guide_when_assist_enabled() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_assist: true,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Right,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str() == "3D Aim Guide")
    );
}

#[test]
fn sync_3d_dynamic_scene_spawns_movement_block_for_view_target_footprint() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_assist: false,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    let mut grid = TileGrid::empty();
    grid.set(6, 4, TileKind::Brick);
    app.insert_resource(grid);
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE),
            facing: Direction::Right,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str() == "3D Movement Block")
    );
}

#[test]
fn sync_3d_dynamic_scene_spawns_powerups_with_kind_names() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    spawn_test_powerup(app.world_mut(), PowerUpKind::Helmet, Vec2::new(32.0, 32.0));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str().starts_with("3D PowerUp Helmet"))
    );
}

#[test]
fn sync_3d_dynamic_scene_spawns_enemy_powerup_carrier_marker() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_assist: true,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(32.0, 32.0),
            facing: Direction::Down,
            speed: enemy_speed(EnemyKind::Basic),
        },
        EnemyTank {
            kind: EnemyKind::Basic,
            carried_powerup: Some(PowerUpKind::Helmet),
        },
        Health { current: 1 },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    assert!(
        names
            .iter(app.world())
            .any(|(name, _)| name.as_str() == "3D Carrier PowerUp Helmet")
    );
}

#[test]
fn sync_3d_dynamic_scene_marks_owned_bases() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn(BaseSprite {
        owner: Some(PlayerId::Two),
        top_left: Vec2::new(96.0, 192.0),
    });
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_dynamic_scene);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dDynamic)>();
    let names = names
        .iter(app.world())
        .map(|(name, _)| name.as_str().to_string())
        .collect::<Vec<_>>();
    assert!(
        names
            .iter()
            .any(|name| name.starts_with("3D Base PlayerTwo"))
    );
    for expected_part in [
        "3D Base Plinth",
        "3D Base Front Lip",
        "3D Base Crest Body",
        "3D Base Crest Wing",
    ] {
        assert!(
            names.iter().any(|name| name == expected_part),
            "3D base should include {expected_part}; got {names:?}"
        );
    }
}

#[test]
fn sync_3d_hud_spawns_and_clears_overlay_entities() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(TileGrid::empty());
    app.add_systems(Update, sync_3d_hud);

    app.update();
    let mut hud_entities = app.world_mut().query::<&View3dHud>();
    assert!(hud_entities.iter(app.world()).count() > 0);

    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::ModeSelect;
    app.update();
    assert_eq!(hud_entities.iter(app.world()).count(), 0);
}

#[test]
fn view_3d_enemy_reserve_marker_count_only_tracks_campaign_remaining_enemies() {
    let mut score_board = ScoreBoard::campaign(20);
    score_board.enemies_destroyed = 3;
    assert_eq!(
        view_3d_enemy_reserve_marker_count(GameMode::Campaign, &score_board),
        17
    );

    let mut oversized = ScoreBoard::coop_campaign(25);
    oversized.enemies_destroyed = 1;
    assert_eq!(
        view_3d_enemy_reserve_marker_count(GameMode::CoopCampaign, &oversized),
        ENEMY_MARKER_COUNT
    );

    let versus = ScoreBoard::versus(3, 5, 2.0);
    assert_eq!(
        view_3d_enemy_reserve_marker_count(GameMode::VersusDeathmatch, &versus),
        0
    );
    assert_eq!(
        view_3d_enemy_reserve_marker_count(GameMode::VersusBaseBattle, &versus),
        0
    );
}

#[test]
fn sync_3d_hud_spawns_enemy_reserve_markers_for_remaining_campaign_enemies() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    let mut score_board = ScoreBoard::campaign(20);
    score_board.enemies_destroyed = 4;
    app.insert_resource(score_board);
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(TileGrid::empty());
    app.add_systems(Update, sync_3d_hud);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dHud)>();
    let reserve_markers = names
        .iter(app.world())
        .filter(|(name, _)| name.as_str().starts_with("3D Enemy Reserve"))
        .count();
    assert_eq!(reserve_markers, 16);
}

#[test]
fn sync_3d_hud_spawns_enemy_direction_indicators_for_threats() {
    let mut app = App::new();
    app.init_resource::<Assets<Mesh>>();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Image>>();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        view_target: PlayerId::One,
        ..ModeSelect::default()
    });
    app.insert_resource(TileGrid::empty());
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(96.0, 96.0),
            facing: Direction::Up,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
    ));
    app.world_mut().spawn((
        Tank {
            top_left: Vec2::new(96.0, 64.0),
            facing: Direction::Down,
            speed: enemy_speed(EnemyKind::Basic),
        },
        EnemyTank {
            kind: EnemyKind::Basic,
            carried_powerup: None,
        },
    ));
    app.add_systems(Startup, setup_3d_view);
    app.add_systems(Update, sync_3d_hud);

    app.update();

    let mut names = app.world_mut().query::<(&Name, &View3dHud)>();
    let direction_markers = names
        .iter(app.world())
        .filter(|(name, _)| name.as_str().starts_with("3D Enemy Direction"))
        .count();
    assert_eq!(direction_markers, 1);
}

#[test]
fn mode_select_ai_and_difficulty_values_cycle_and_label() {
    assert_eq!(
        next_mode_select_ai_strategy(ModeSelectAiStrategy::Auto),
        ModeSelectAiStrategy::Classic
    );
    assert_eq!(
        next_mode_select_ai_strategy(ModeSelectAiStrategy::Classic),
        ModeSelectAiStrategy::PathToObjective
    );
    assert_eq!(
        next_mode_select_ai_strategy(ModeSelectAiStrategy::PathToObjective),
        ModeSelectAiStrategy::Auto
    );
    assert_eq!(
        previous_mode_select_ai_strategy(ModeSelectAiStrategy::Auto),
        ModeSelectAiStrategy::PathToObjective
    );
    assert_eq!(
        previous_mode_select_ai_strategy(ModeSelectAiStrategy::Classic),
        ModeSelectAiStrategy::Auto
    );
    assert_eq!(
        mode_select_ai_strategy_label(ModeSelectAiStrategy::PathToObjective),
        "PATH"
    );

    assert_eq!(
        ModeSelect::default().difficulty_profile,
        ModeSelectDifficultyProfile::Easy
    );
    assert_eq!(
        next_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Easy),
        ModeSelectDifficultyProfile::Normal
    );
    assert_eq!(
        next_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Normal),
        ModeSelectDifficultyProfile::Hard
    );
    assert_eq!(
        next_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Hard),
        ModeSelectDifficultyProfile::Auto
    );
    assert_eq!(
        next_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Auto),
        ModeSelectDifficultyProfile::Easy
    );
    assert_eq!(
        previous_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Easy),
        ModeSelectDifficultyProfile::Auto
    );
    assert_eq!(
        previous_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Auto),
        ModeSelectDifficultyProfile::Hard
    );
    assert_eq!(
        previous_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Normal),
        ModeSelectDifficultyProfile::Easy
    );
    assert_eq!(
        previous_mode_select_difficulty_profile(ModeSelectDifficultyProfile::Hard),
        ModeSelectDifficultyProfile::Normal
    );
    assert_eq!(
        mode_select_difficulty_profile_label(ModeSelectDifficultyProfile::Easy),
        "EASY"
    );
    assert_eq!(
        mode_select_difficulty_profile_label(ModeSelectDifficultyProfile::Normal),
        "NORMAL"
    );
}

#[test]
fn mode_select_auto_ai_uses_level_defaults_and_explicit_values_override() {
    let level_contents = LEVEL_1.replacen(
            "max_enemies_on_screen: 4,",
            "max_enemies_on_screen: 4,\n  enemy_ai_strategy: PathToObjective,\n  difficulty_profile: Hard,",
            1,
        );
    let level = parse_level(&level_contents).expect("level should parse");
    let mode_select = ModeSelect {
        difficulty_profile: ModeSelectDifficultyProfile::Auto,
        ..ModeSelect::default()
    };
    assert_eq!(
        selected_enemy_ai_strategy(&mode_select, &level),
        EnemyAiStrategy::PathToObjective
    );
    assert_eq!(
        selected_enemy_difficulty_profile(&mode_select, &level),
        EnemyDifficultyProfile::Hard
    );

    let mode_select = ModeSelect::default();
    assert_eq!(
        selected_enemy_difficulty_profile(&mode_select, &level),
        EnemyDifficultyProfile::Easy
    );

    let mode_select = ModeSelect {
        ai_strategy: ModeSelectAiStrategy::Classic,
        difficulty_profile: ModeSelectDifficultyProfile::Normal,
        ..ModeSelect::default()
    };
    assert_eq!(
        selected_enemy_ai_strategy(&mode_select, &level),
        EnemyAiStrategy::Classic
    );
    assert_eq!(
        selected_enemy_difficulty_profile(&mode_select, &level),
        EnemyDifficultyProfile::Normal
    );
}

#[test]
fn main_menu_ai_and_difficulty_settings_cycle_from_ui() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowRight);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::AiStrategy,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(Update, handle_shared_controls);

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().ai_strategy,
        ModeSelectAiStrategy::Classic
    );
    let mut cursors = app.world_mut().query::<&ModeSelectCursor>();
    assert_eq!(cursors.iter(app.world()).count(), 1);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowLeft);
    app.world_mut().resource_mut::<ModeSelect>().selected = ModeSelectOption::Difficulty;
    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().difficulty_profile,
        ModeSelectDifficultyProfile::Auto
    );
    let mut cursors = app.world_mut().query::<&ModeSelectCursor>();
    assert_eq!(cursors.iter(app.world()).count(), 1);
}

#[test]
fn main_menu_view_settings_cycle_from_ui() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowRight);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::ViewMode,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(Update, handle_shared_controls);

    app.update();

    assert_eq!(
        app.world().resource::<ModeSelect>().view_mode,
        TankViewMode::ThreeD
    );

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::ArrowRight);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::ArrowRight);
    app.world_mut().resource_mut::<ModeSelect>().selected = ModeSelectOption::ViewAssist;
    app.update();

    assert!(!app.world().resource::<ModeSelect>().view_assist);
}

#[test]
fn mode_select_arena_selection_wraps_authored_arenas() {
    assert_eq!(ModeSelect::default().arena, DEFAULT_VERSUS_ARENA);
    assert_eq!(next_arena(1), 2);
    assert_eq!(next_arena(2), 3);
    assert_eq!(next_arena(3), 4);
    assert_eq!(next_arena(4), 5);
    assert_eq!(next_arena(5), 6);
    assert_eq!(next_arena(6), 7);
    assert_eq!(next_arena(7), 8);
    assert_eq!(next_arena(8), 1);
    assert_eq!(previous_arena(1), 8);
    assert_eq!(previous_arena(2), 1);
    assert_eq!(previous_arena(3), 2);
    assert_eq!(previous_arena(4), 3);
    assert_eq!(previous_arena(5), 4);
    assert_eq!(previous_arena(6), 5);
    assert_eq!(previous_arena(7), 6);
    assert_eq!(previous_arena(8), 7);
}

#[test]
fn main_menu_arena_row_changes_selected_arena() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowRight);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusDeathmatch);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::Arena,
        arena: 1,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(Update, handle_shared_controls);

    app.update();

    assert_eq!(app.world().resource::<ModeSelect>().arena, 2);
}

#[test]
fn mode_select_stage_selection_wraps_authored_campaign() {
    assert_eq!(ModeSelect::default().stage, 1);
    assert_eq!(next_stage(1, CampaignMapPack::Custom), 2);
    assert_eq!(
        next_stage(CUSTOM_LEVEL_COUNT - 1, CampaignMapPack::Custom),
        CUSTOM_LEVEL_COUNT
    );
    assert_eq!(next_stage(CUSTOM_LEVEL_COUNT, CampaignMapPack::Custom), 1);
    assert_eq!(
        previous_stage(1, CampaignMapPack::Custom),
        CUSTOM_LEVEL_COUNT
    );
    assert_eq!(previous_stage(2, CampaignMapPack::Custom), 1);
    assert_eq!(
        previous_stage(CUSTOM_LEVEL_COUNT, CampaignMapPack::Custom),
        CUSTOM_LEVEL_COUNT - 1
    );
    assert_eq!(
        next_stage(ORIGINAL_LEVEL_COUNT, CampaignMapPack::Original),
        1
    );
    assert_eq!(
        previous_stage(1, CampaignMapPack::Original),
        ORIGINAL_LEVEL_COUNT
    );
}

#[test]
fn selected_campaign_stage_clamps_to_authored_campaign_range() {
    let mut mode_select = ModeSelect {
        map_pack: CampaignMapPack::Custom,
        stage: 12,
        ..ModeSelect::default()
    };
    assert_eq!(selected_campaign_stage(&mode_select), 12);

    mode_select.stage = 0;
    assert_eq!(selected_campaign_stage(&mode_select), 1);

    mode_select.stage = CUSTOM_LEVEL_COUNT + 5;
    assert_eq!(selected_campaign_stage(&mode_select), CUSTOM_LEVEL_COUNT);

    mode_select.map_pack = CampaignMapPack::Original;
    mode_select.stage = CUSTOM_LEVEL_COUNT;
    assert_eq!(selected_campaign_stage(&mode_select), ORIGINAL_LEVEL_COUNT);
}

#[test]
fn main_menu_map_pack_row_cycles_and_clamps_stage() {
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowRight);

    let mut app = App::new();
    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(GameStatus::default());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(EnemyDirector::inactive());
    app.insert_resource(ScoreBoard::campaign(0));
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPowerUpDirector::inactive());
    app.insert_resource(ModeSelect {
        selected: ModeSelectOption::MapPack,
        map_pack: CampaignMapPack::Custom,
        stage: CUSTOM_LEVEL_COUNT,
        ..ModeSelect::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.add_systems(Update, handle_shared_controls);

    app.update();

    let mode_select = app.world().resource::<ModeSelect>();
    assert_eq!(mode_select.map_pack, CampaignMapPack::Original);
    assert_eq!(mode_select.stage, ORIGINAL_LEVEL_COUNT);
    let mut cursors = app.world_mut().query::<&ModeSelectCursor>();
    assert_eq!(cursors.iter(app.world()).count(), 1);
}

#[test]
fn mode_select_cursor_tracks_selected_option() {
    let cursor = |selected| {
        let mode_select = ModeSelect {
            selected,
            ..ModeSelect::default()
        };
        mode_select_cursor_translation(&ModeSelectDisplay::from_mode_select(&mode_select))
    };
    let campaign = cursor(ModeSelectOption::Campaign);
    let coop = cursor(ModeSelectOption::CoopCampaign);
    let battle = cursor(ModeSelectOption::Battle);
    let map = cursor(ModeSelectOption::MapPack);
    let view = cursor(ModeSelectOption::ViewMode);
    let assist = cursor(ModeSelectOption::ViewAssist);
    let ai = cursor(ModeSelectOption::AiStrategy);
    let difficulty = cursor(ModeSelectOption::Difficulty);
    let music = cursor(ModeSelectOption::Music);
    let sound = cursor(ModeSelectOption::Sound);
    let scale = cursor(ModeSelectOption::Scale);
    let stage = cursor(ModeSelectOption::Stage);
    let arena = cursor(ModeSelectOption::Arena);
    assert!(campaign.y > coop.y);
    assert!(coop.y > battle.y);
    assert!(battle.y > map.y);
    assert!(map.y > view.y);
    assert!(view.y > assist.y);
    assert!(assist.y > ai.y);
    assert!(ai.y > difficulty.y);
    assert!(difficulty.y > music.y);
    assert!(music.y > sound.y);
    assert!(sound.y > scale.y);
    assert!(scale.y > stage.y);
    assert!(stage.y > arena.y);
}

#[test]
fn mode_select_cursor_stays_left_of_selected_text() {
    let mode_select = ModeSelect {
        selected: ModeSelectOption::Campaign,
        ..ModeSelect::default()
    };
    let display = ModeSelectDisplay::from_mode_select(&mode_select);
    let option = mode_select_option_top_left(&display, ModeSelectOption::Campaign);
    let cursor_top_left =
        virtual_top_left_from_translation(mode_select_cursor_translation(&display), TANK_SIZE);

    assert_eq!(cursor_top_left.y, option.y - 4.0);
    assert_eq!(
        option.x - (cursor_top_left.x + TANK_SIZE),
        MODE_SELECT_CURSOR_GAP - TANK_SIZE
    );
    assert!(cursor_top_left.x + TANK_SIZE < option.x);
}

#[test]
fn mode_select_text_rows_are_centered_on_the_virtual_screen() {
    let mode_select = ModeSelect::default();
    let display = ModeSelectDisplay::from_mode_select(&mode_select);
    let center_x = VIRTUAL_WIDTH / 2.0;
    assert_eq!(
        mode_select_centered_x("TANK 1990") + phase_text_width("TANK 1990") / 2.0,
        center_x
    );

    for option in [
        ModeSelectOption::Campaign,
        ModeSelectOption::CoopCampaign,
        ModeSelectOption::Battle,
        ModeSelectOption::MapPack,
        ModeSelectOption::ViewMode,
        ModeSelectOption::ViewAssist,
        ModeSelectOption::AiStrategy,
        ModeSelectOption::Difficulty,
        ModeSelectOption::Music,
        ModeSelectOption::Sound,
        ModeSelectOption::Scale,
        ModeSelectOption::Stage,
        ModeSelectOption::Arena,
    ] {
        let text = mode_select_option_text(&display, option);
        let top_left = mode_select_option_top_left(&display, option);
        assert_eq!(
            top_left.x + phase_text_width(&text) / 2.0,
            center_x,
            "{text} should be centered"
        );
    }
}

#[test]
fn mode_select_separates_mode_choices_from_settings() {
    let campaign_to_coop = mode_select_option_y(ModeSelectOption::CoopCampaign)
        - mode_select_option_y(ModeSelectOption::Campaign);
    let coop_to_battle = mode_select_option_y(ModeSelectOption::Battle)
        - mode_select_option_y(ModeSelectOption::CoopCampaign);
    let battle_to_map = mode_select_option_y(ModeSelectOption::MapPack)
        - mode_select_option_y(ModeSelectOption::Battle);
    let map_to_view = mode_select_option_y(ModeSelectOption::ViewMode)
        - mode_select_option_y(ModeSelectOption::MapPack);
    let view_to_assist = mode_select_option_y(ModeSelectOption::ViewAssist)
        - mode_select_option_y(ModeSelectOption::ViewMode);
    let ai_to_difficulty = mode_select_option_y(ModeSelectOption::Difficulty)
        - mode_select_option_y(ModeSelectOption::AiStrategy);

    assert_eq!(campaign_to_coop, coop_to_battle);
    assert_eq!(map_to_view, campaign_to_coop);
    assert_eq!(view_to_assist, campaign_to_coop);
    assert_eq!(ai_to_difficulty, campaign_to_coop);
    assert!(battle_to_map > campaign_to_coop);
}

#[test]
fn mode_select_hints_fit_and_use_available_pixel_glyphs() {
    let manifest = parse_asset_manifest(MANIFEST).expect("manifest should parse");
    assert_eq!(MODE_SELECT_HINT_LINES, ["", "", "SPACE ENTER START"]);
    for line in [
        "TANK 1990",
        "1 PLAYER",
        "2 PLAYERS",
        "BATTLE",
        "MAP",
        "ORIGINAL",
        "MAP ORIGINAL",
        "MAP CUSTOM",
        "VIEW",
        "2D",
        "3D",
        "ASSIST",
        "STAGE",
        "STAGE 50",
        "ARENA",
        "ARENA 08 DUEL",
        "37",
        "BASE",
        "DUEL",
        "AI",
        "DIFF",
        "EASY",
        "AUTO",
        "PATH",
        "NORMAL",
        "HARD",
        "MUSIC",
        "BGM",
        "CUSTOM",
        "CLASSIC",
        "SOUND",
        "ON",
        "OFF",
        "SCALE",
        "2X",
        "3X",
        "4X",
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
fn mode_select_hints_leave_bottom_margin_inside_menu_panel() {
    let last_hint_bottom = MODE_SELECT_HINT_TOP
        + (MODE_SELECT_HINT_LINES.len() as f32 - 1.0) * MODE_SELECT_HINT_LINE_STEP
        + GENERATED_GLYPH_HEIGHT as f32;
    let menu_panel_bottom = 16.0 + MODE_SELECT_WIDTH;

    assert!(last_hint_bottom <= menu_panel_bottom - 3.0);
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
fn deathmatch_hit_reaching_target_score_ends_round_for_shooter() {
    let mut app = App::new();
    let p2_top_left = Vec2::new(64.0, 80.0);
    let bullet_top_left = p2_top_left + Vec2::splat(4.0);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusDeathmatch);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::versus(3, 1, 2.0));
    spawn_test_player(app.world_mut(), PlayerId::Two, p2_top_left, 3);
    app.world_mut().spawn((
        Bullet {
            previous_top_left: bullet_top_left,
            top_left: bullet_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::RoundOver);
    assert_eq!(status.winner, Some(PlayerId::One));
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.p1_score, 1);
    assert_eq!(score_board.p2_score, 0);
    assert_eq!(score_board.p2_lives, 2);

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut players = app
        .world_mut()
        .query::<(&Player, &PlayerLives, Option<&Tank>, Option<&DestroyedTank>)>();
    let p2 = players
        .iter(app.world())
        .find(|(player, _, _, _)| player.id == PlayerId::Two)
        .expect("P2 should still exist as a destroyed player");
    assert_eq!(p2.1.current, 2);
    assert!(p2.2.is_none());
    assert!(p2.3.is_some());
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 1);
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
fn spawn_protected_enemy_absorbs_player_bullet_without_damage_or_score() {
    let enemy_top_left = Vec2::new(96.0, 0.0);
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.world_mut().spawn((
        Tank {
            top_left: enemy_top_left,
            facing: Direction::Down,
            speed: enemy_speed(EnemyKind::Basic),
        },
        Transform::from_translation(board_object_center(
            enemy_top_left.x,
            enemy_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0,
        )),
        EnemyTank {
            kind: EnemyKind::Basic,
            carried_powerup: None,
        },
        Health { current: 1 },
        SpawnProtection::for_spawn_shimmer(SpriteFrameRange { first: 4, last: 7 }),
    ));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: enemy_top_left,
            top_left: enemy_top_left,
            facing: Direction::Down,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            enemy_top_left.x,
            enemy_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut enemies = app.world_mut().query::<(&EnemyTank, &Health)>();
    let enemies: Vec<(EnemyKind, i32)> = enemies
        .iter(app.world())
        .map(|(enemy, health)| (enemy.kind, health.current))
        .collect();
    assert_eq!(enemies, [(EnemyKind::Basic, 1)]);
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.score, 0);
    assert_eq!(score_board.enemies_destroyed, 0);
}

#[test]
fn coop_campaign_player_two_bullet_destroys_enemy() {
    let enemy_top_left = Vec2::new(96.0, 64.0);
    let mut app = App::new();

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::CoopCampaign);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::coop_campaign(20));
    spawn_test_enemy_tank(
        app.world_mut(),
        EnemyKind::Basic,
        enemy_top_left,
        Direction::Down,
    );
    app.world_mut().spawn((
        Bullet {
            previous_top_left: enemy_top_left,
            top_left: enemy_top_left,
            facing: Direction::Down,
            owner: Team::Player2,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            enemy_top_left.x,
            enemy_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.score, enemy_score(EnemyKind::Basic));
    assert_eq!(score_board.enemies_destroyed, 1);
    let mut enemies = app.world_mut().query::<&EnemyTank>();
    assert_eq!(enemies.iter(app.world()).count(), 0);
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
}

#[test]
fn coop_campaign_player_bullets_do_not_hit_teammates() {
    let p2_top_left = Vec2::new(80.0, 96.0);
    let mut app = App::new();

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::CoopCampaign);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::coop_campaign(20));
    spawn_test_player(app.world_mut(), PlayerId::Two, p2_top_left, 3);
    app.world_mut().spawn((
        Bullet {
            previous_top_left: p2_top_left,
            top_left: p2_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            p2_top_left.x,
            p2_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::Playing);
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.p2_lives, 3);
    assert_eq!(score_board.lives, 6);
    let mut players = app.world_mut().query::<(&Player, &PlayerLives, &Tank)>();
    let p2 = players
        .iter(app.world())
        .find(|(player, _, _)| player.id == PlayerId::Two)
        .expect("P2 should remain active");
    assert_eq!(p2.1.current, 3);
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 1);
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
fn initial_spawn_delay_expires_during_stage_intro() {
    let frames = SpriteFrameRange { first: 4, last: 7 };
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(spawn_shimmer_duration_secs(frames)));
    let mut app = App::new();

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::StageIntro,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    let player = app
        .world_mut()
        .spawn(PlayerRespawnDelay::for_spawn_shimmer(frames))
        .id();
    app.add_systems(Update, tick_player_respawns);

    app.update();

    assert!(
        app.world().get::<PlayerRespawnDelay>(player).is_none(),
        "stage intro should consume the control delay while the visible shimmer plays"
    );
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
fn pending_player_respawn_waits_for_playing_phase() {
    let frames = SpriteFrameRange { first: 0, last: 3 };
    let respawn_top_left = Vec2::new(64.0, 192.0);
    let mut pending_respawn = PlayerRespawnPending::for_explosion(frames);
    assert!(pending_respawn.tick(Duration::from_secs_f32(explosion_duration_secs(frames))));

    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::StageIntro,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::versus(3, 5, 1.5));
    let player = app
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
            Sprite::default(),
            TankSpriteState::new(TankSpriteSet::Player1),
        ))
        .id();
    app.add_systems(Update, tick_player_respawns);

    app.update();

    assert!(
        app.world().get::<PlayerRespawnPending>(player).is_some(),
        "pending respawns should not materialize before active play resumes"
    );
    assert!(app.world().get::<Tank>(player).is_none());
    assert!(app.world().get::<PlayerRespawnDelay>(player).is_none());
}

#[test]
fn initial_player_spawn_starts_with_invulnerability_shield() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.add_systems(Update, spawn_player_with_initial_shield_for_test);

    app.update();

    let mut players = app
        .world_mut()
        .query::<(&Player, &Tank, &Shield, Option<&PlayerRespawnDelay>)>();
    let spawned: Vec<(PlayerId, Vec2, f32, bool)> = players
        .iter(app.world())
        .map(|(player, tank, shield, delay)| {
            (
                player.id,
                tank.top_left,
                shield.timer.remaining_secs(),
                delay.is_some(),
            )
        })
        .collect();

    assert_eq!(spawned.len(), 1);
    assert_eq!(spawned[0].0, PlayerId::One);
    assert_eq!(spawned[0].1, Vec2::new(64.0, 192.0));
    assert!(
        (spawned[0].2 - TEST_SPAWN_INVULNERABILITY_SECONDS).abs() <= f32::EPSILON,
        "initial shield should use the configured spawn invulnerability"
    );
    assert!(
        spawned[0].3,
        "initial spawn should delay control while the shimmer plays"
    );

    let mut effects = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(
        effects.iter(app.world()).count(),
        1,
        "initial spawn should show one shimmer effect"
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
fn shield_timer_waits_through_intro_and_pause_then_ticks_in_playing() {
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(2.0));
    let mut app = App::new();

    app.insert_resource(time);
    app.insert_resource(GameStatus {
        phase: GamePhase::StageIntro,
        ..GameStatus::default()
    });
    let player = app
        .world_mut()
        .spawn((
            Player { id: PlayerId::One },
            PlayerUpgrade { level: 0 },
            Shield {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            Sprite::default(),
        ))
        .id();
    app.add_systems(Update, tick_shields);

    app.update();

    assert!(
        app.world().get::<Shield>(player).is_some(),
        "stage intro should not consume spawn invulnerability"
    );
    assert!(
        (app.world()
            .get::<Shield>(player)
            .expect("shield should remain")
            .timer
            .remaining_secs()
            - 2.0)
            .abs()
            <= f32::EPSILON
    );

    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Paused;
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(2.0));
    app.update();

    assert!(
        app.world().get::<Shield>(player).is_some(),
        "paused play should preserve spawn invulnerability"
    );
    assert!(
        (app.world()
            .get::<Shield>(player)
            .expect("shield should remain")
            .timer
            .remaining_secs()
            - 2.0)
            .abs()
            <= f32::EPSILON
    );

    app.world_mut().resource_mut::<GameStatus>().phase = GamePhase::Playing;
    app.world_mut()
        .resource_mut::<Time>()
        .advance_by(Duration::from_secs_f32(2.0));
    app.update();

    assert!(
        app.world().get::<Shield>(player).is_none(),
        "active play should consume and eventually remove the shield"
    );
    assert_eq!(
        app.world()
            .get::<Sprite>(player)
            .expect("player sprite should remain")
            .color,
        player_upgrade_visual_color(0)
    );
}

#[test]
fn paused_shield_visual_renders_without_ticking_timer() {
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(1.0));
    let mut app = App::new();
    let top_left = Vec2::new(64.0, 96.0);

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Paused,
        ..GameStatus::default()
    });
    let player = app
        .world_mut()
        .spawn((
            Player { id: PlayerId::One },
            PlayerUpgrade { level: 0 },
            Tank {
                top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            Shield {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            Sprite::default(),
        ))
        .id();
    app.add_systems(Update, (tick_shields, sync_shield_visuals).chain());

    app.update();

    assert!(
        (app.world()
            .get::<Shield>(player)
            .expect("paused shield should remain")
            .timer
            .remaining_secs()
            - 2.0)
            .abs()
            <= f32::EPSILON
    );
    let mut visuals = app
        .world_mut()
        .query::<(&ShieldVisual, &Transform, &Sprite)>();
    let spawned: Vec<(Entity, Vec3, Color)> = visuals
        .iter(app.world())
        .map(|(visual, transform, sprite)| (visual.owner, transform.translation, sprite.color))
        .collect();

    assert_eq!(spawned.len(), 1);
    assert_eq!(spawned[0].0, player);
    assert_eq!(spawned[0].1, shield_visual_translation(top_left));
    assert_eq!(spawned[0].2, shield_visual_color(0.0));
}

#[test]
fn shield_visuals_follow_active_player_shields_and_cleanup() {
    let mut app = App::new();
    let top_left = Vec2::new(64.0, 96.0);
    let moved_top_left = Vec2::new(72.0, 96.0);

    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    let player = app
        .world_mut()
        .spawn((
            Player { id: PlayerId::One },
            Tank {
                top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            Shield {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ))
        .id();
    app.add_systems(Update, sync_shield_visuals);

    app.update();

    let mut visuals = app
        .world_mut()
        .query::<(&ShieldVisual, &Transform, &Sprite)>();
    let spawned: Vec<(Entity, Vec3, Color)> = visuals
        .iter(app.world())
        .map(|(visual, transform, sprite)| (visual.owner, transform.translation, sprite.color))
        .collect();
    assert_eq!(spawned.len(), 1);
    assert_eq!(spawned[0].0, player);
    assert_eq!(spawned[0].1, shield_visual_translation(top_left));
    assert_eq!(spawned[0].2, shield_visual_color(0.0));

    {
        let mut tank = app
            .world_mut()
            .get_mut::<Tank>(player)
            .expect("player tank should exist");
        tank.top_left = moved_top_left;
    }
    {
        let mut shield = app
            .world_mut()
            .get_mut::<Shield>(player)
            .expect("player shield should exist");
        shield.timer.tick(Duration::from_secs_f32(0.11));
    }

    app.update();

    let mut visuals = app
        .world_mut()
        .query::<(&ShieldVisual, &Transform, &Sprite)>();
    let updated: Vec<(Entity, Vec3, Color)> = visuals
        .iter(app.world())
        .map(|(visual, transform, sprite)| (visual.owner, transform.translation, sprite.color))
        .collect();
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].0, player);
    assert_eq!(updated[0].1, shield_visual_translation(moved_top_left));
    assert_eq!(updated[0].2, shield_visual_color(0.11));

    app.world_mut().entity_mut(player).remove::<Shield>();
    app.update();

    let mut visuals = app.world_mut().query::<&ShieldVisual>();
    assert_eq!(visuals.iter(app.world()).count(), 0);
}

#[test]
fn shield_visuals_clear_during_terminal_phases() {
    let mut app = App::new();
    let top_left = Vec2::new(64.0, 96.0);

    app.insert_resource(test_sprite_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::LevelClear,
        ..GameStatus::default()
    });
    let player = app
        .world_mut()
        .spawn((
            Player { id: PlayerId::One },
            Tank {
                top_left,
                facing: Direction::Up,
                speed: PLAYER_SPEED,
            },
            Shield {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ))
        .id();
    app.world_mut().spawn((
        ShieldVisual { owner: player },
        Transform::from_translation(shield_visual_translation(top_left)),
        Sprite::default(),
    ));
    app.add_systems(Update, sync_shield_visuals);

    app.update();

    let mut visuals = app.world_mut().query::<&ShieldVisual>();
    assert_eq!(visuals.iter(app.world()).count(), 0);
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
fn player_spawn_delay_blocks_firing_until_control_resumes() {
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
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.world_mut().spawn((
        Tank {
            top_left: tank_top_left,
            facing: Direction::Up,
            speed: PLAYER_SPEED,
        },
        PlayerUpgrade { level: 0 },
        Player { id: PlayerId::One },
        PlayerRespawnDelay::for_spawn_shimmer(SpriteFrameRange { first: 4, last: 7 }),
    ));
    app.add_systems(Update, fire_player_bullet);

    app.update();

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
}

#[test]
fn player_move_system_blocks_tank_from_entering_water_tile() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(16.0, 16.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));
    let mut grid = TileGrid::empty();
    grid.set(4, 2, TileKind::Water);
    grid.set(4, 3, TileKind::Water);

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app
        .world_mut()
        .query::<(&Tank, &Transform, &TankSpriteState)>();
    let (tank, transform, sprite_state) = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left);
    assert_eq!(tank.facing, Direction::Right);
    assert_eq!(
        transform.translation,
        board_object_center(
            tank_top_left.x,
            tank_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0
        )
    );
    assert_eq!(sprite_state.frame, 0);
}

#[test]
fn player_move_system_allows_ice_and_applies_speed_boost() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(4.0 * TILE_SIZE, 4.0 * TILE_SIZE);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));
    let mut grid = TileGrid::empty();
    for y in 4..=5 {
        for x in 4..=5 {
            grid.set(x, y, TileKind::Ice);
        }
    }

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();

    let expected_top_left =
        round_vec2(tank_top_left + Vec2::new(TILE_SIZE * ICE_SPEED_MULTIPLIER, 0.0));
    let mut players = app.world_mut().query::<(&Tank, &Transform)>();
    let (tank, transform) = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, expected_top_left);
    assert_eq!(tank.facing, Direction::Right);
    assert_eq!(
        transform.translation,
        board_object_center(
            expected_top_left.x,
            expected_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0
        )
    );
}

#[test]
fn player_turning_snaps_slight_lane_drift_back_to_grid() {
    let mut app = App::new();
    let off_lane_top_left = Vec2::new(16.0, 17.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(1.0 / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(
        app.world_mut(),
        PlayerId::One,
        off_lane_top_left,
        Direction::Up,
    );
    app.add_systems(Update, move_player_tank);

    app.update();

    let expected_top_left = Vec2::new(17.0, 16.0);
    let mut players = app.world_mut().query::<(&Tank, &Transform)>();
    let (tank, transform) = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, expected_top_left);
    assert_eq!(tank.facing, Direction::Right);
    assert_eq!(
        transform.translation,
        board_object_center(
            expected_top_left.x,
            expected_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0
        )
    );
    assert_eq!(tank.top_left.y.rem_euclid(TILE_SIZE), 0.0);
}

#[test]
fn tank_move_candidate_lane_assists_into_narrow_vertical_gap() {
    let mut grid = TileGrid::empty();
    for y in 4..=5 {
        for x in [0, 1, 4, 5] {
            grid.set(x, y, TileKind::Brick);
        }
    }
    let current = Vec2::new(19.0, 6.0 * TILE_SIZE);

    let next = tank_move_candidate(current, Direction::Up, TILE_SIZE, &grid, &[current])
        .expect("lane assist should align the tank with the open two-tile gap");

    assert_eq!(next, Vec2::new(16.0, 6.0 * TILE_SIZE));
    assert_eq!(next.x.rem_euclid(TILE_SIZE), 0.0);
}

#[test]
fn tank_move_candidate_does_not_lane_assist_into_closed_gap() {
    let mut grid = TileGrid::empty();
    for y in 4..=5 {
        for x in [0, 1, 2, 4, 5] {
            grid.set(x, y, TileKind::Brick);
        }
    }
    let current = Vec2::new(19.0, 6.0 * TILE_SIZE);

    assert_eq!(
        tank_move_candidate(current, Direction::Up, TILE_SIZE, &grid, &[current]),
        None
    );
}

#[test]
fn player_3d_w_moves_forward_in_current_facing() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyW);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(
        app.world_mut(),
        PlayerId::One,
        tank_top_left,
        Direction::Right,
    );
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left + Vec2::new(TILE_SIZE, 0.0));
    assert_eq!(tank.facing, Direction::Right);
}

#[test]
fn player_3d_a_rotates_left_without_world_left_motion() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyA);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left);
    assert_eq!(tank.facing, Direction::Left);

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear_just_pressed(KeyCode::KeyA);
    app.update();

    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left);
    assert_eq!(tank.facing, Direction::Left);
}

#[test]
fn player_3d_held_turn_does_not_repeat_without_new_key_press() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyA);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(1.0 / 60.0));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();
    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.clear_just_pressed(KeyCode::KeyA);
    }
    {
        let mut time = app.world_mut().resource_mut::<Time>();
        time.advance_by(Duration::from_secs_f32(1.0));
    }
    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left);
    assert_eq!(tank.facing, Direction::Left);
}

#[test]
fn player_3d_turn_press_is_consumed_once_across_fixed_ticks() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(1.0 / 60.0));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();
    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left);
    assert_eq!(tank.facing, Direction::Right);
}

#[test]
fn player_3d_turn_can_repeat_after_key_release() {
    let mut control = PlayerControl::default();
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);

    let first = player_3d_tank_motion(&keys, &mut control, PlayerId::One, Direction::Up)
        .expect("first press should turn");
    assert_eq!(first.facing, Direction::Right);

    keys.clear_just_pressed(KeyCode::KeyD);
    keys.release(KeyCode::KeyD);
    assert_eq!(
        player_3d_tank_motion(&keys, &mut control, PlayerId::One, first.facing),
        None
    );

    keys.press(KeyCode::KeyD);
    let second = player_3d_tank_motion(&keys, &mut control, PlayerId::One, first.facing)
        .expect("new press after release should turn again");
    assert_eq!(second.facing, Direction::Down);
}

#[test]
fn player_3d_turns_and_moves_forward_in_new_facing_on_same_frame() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyW);
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left + Vec2::new(TILE_SIZE, 0.0));
    assert_eq!(tank.facing, Direction::Right);
}

#[test]
fn player_3d_s_reverses_without_flipping_facing() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(32.0, 32.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyS);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ModeSelect {
        view_mode: TankViewMode::ThreeD,
        ..ModeSelect::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(
        app.world_mut(),
        PlayerId::One,
        tank_top_left,
        Direction::Right,
    );
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app.world_mut().query::<&Tank>();
    let tank = players.single(app.world()).unwrap();
    assert_eq!(tank.top_left, tank_top_left - Vec2::new(TILE_SIZE, 0.0));
    assert_eq!(tank.facing, Direction::Right);
}

#[test]
fn player_move_system_blocks_tank_from_entering_solid_tiles() {
    for tile in [TileKind::Brick, TileKind::Steel, TileKind::Base] {
        let mut app = App::new();
        let tank_top_left = Vec2::new(16.0, 16.0);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyD);
        let mut time = Time::<()>::default();
        time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));
        let mut grid = TileGrid::empty();
        grid.set(4, 2, tile);
        grid.set(4, 3, tile);

        app.insert_resource(time);
        app.insert_resource(keys);
        app.insert_resource(PlayerControl::default());
        app.insert_resource(test_sprite_assets());
        app.insert_resource(grid);
        app.insert_resource(GameStatus {
            phase: GamePhase::Playing,
            ..GameStatus::default()
        });
        app.insert_resource(VersusPlayerFreeze::default());
        spawn_movable_test_player(app.world_mut(), PlayerId::One, tank_top_left, Direction::Up);
        app.add_systems(Update, move_player_tank);

        app.update();

        let mut players = app.world_mut().query::<(&Tank, &Transform)>();
        let (tank, transform) = players.single(app.world()).unwrap();
        assert_eq!(tank.top_left, tank_top_left, "{tile:?} should block tank");
        assert_eq!(tank.facing, Direction::Right);
        assert_eq!(
            transform.translation,
            board_object_center(
                tank_top_left.x,
                tank_top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0
            )
        );
    }
}

#[test]
fn player_move_system_blocks_tank_from_entering_other_player() {
    let mut app = App::new();
    let p1_top_left = Vec2::new(16.0, 16.0);
    let p2_top_left = Vec2::new(32.0, 16.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, p1_top_left, Direction::Up);
    spawn_movable_test_player(app.world_mut(), PlayerId::Two, p2_top_left, Direction::Left);
    app.add_systems(Update, move_player_tank);

    app.update();

    let mut players = app.world_mut().query::<(&Player, &Tank, &Transform)>();
    let players: Vec<(PlayerId, Vec2, Direction, Vec3)> = players
        .iter(app.world())
        .map(|(player, tank, transform)| {
            (player.id, tank.top_left, tank.facing, transform.translation)
        })
        .collect();
    let p1 = players
        .iter()
        .find(|(player, _, _, _)| *player == PlayerId::One)
        .expect("P1 should exist");
    let p2 = players
        .iter()
        .find(|(player, _, _, _)| *player == PlayerId::Two)
        .expect("P2 should exist");

    assert_eq!(
        *p1,
        (
            PlayerId::One,
            p1_top_left,
            Direction::Right,
            board_object_center(p1_top_left.x, p1_top_left.y, Vec2::splat(TANK_SIZE), 6.0)
        )
    );
    assert_eq!(
        *p2,
        (
            PlayerId::Two,
            p2_top_left,
            Direction::Left,
            board_object_center(p2_top_left.x, p2_top_left.y, Vec2::splat(TANK_SIZE), 6.0)
        )
    );
}

#[test]
fn enemy_move_system_blocks_tank_from_entering_player() {
    let mut app = App::new();
    let enemy_top_left = Vec2::new(16.0, 16.0);
    let player_top_left = Vec2::new(32.0, 16.0);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(
        TILE_SIZE / enemy_speed(EnemyKind::Basic),
    ));

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(EnemyFreeze::default());
    spawn_test_player(app.world_mut(), PlayerId::One, player_top_left, 3);
    spawn_test_enemy_tank(
        app.world_mut(),
        EnemyKind::Basic,
        enemy_top_left,
        Direction::Right,
    );
    app.add_systems(Update, move_enemy_tanks);

    app.update();

    let mut enemies = app
        .world_mut()
        .query::<(&EnemyTank, &Tank, &Transform, &TankSpriteState)>();
    let (_, tank, transform, sprite_state) = enemies.single(app.world()).unwrap();
    assert_eq!(tank.top_left, enemy_top_left);
    assert_eq!(tank.facing, Direction::Down);
    assert_eq!(
        transform.translation,
        board_object_center(
            enemy_top_left.x,
            enemy_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0
        )
    );
    assert_eq!(sprite_state.frame, 0);
}

#[test]
fn versus_move_controls_drive_players_independently() {
    let mut app = App::new();
    let p1_top_left = Vec2::new(16.0, 16.0);
    let p2_top_left = Vec2::new(80.0, 16.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    keys.press(KeyCode::ArrowDown);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(TileGrid::empty());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_movable_test_player(app.world_mut(), PlayerId::One, p1_top_left, Direction::Up);
    spawn_movable_test_player(app.world_mut(), PlayerId::Two, p2_top_left, Direction::Left);
    app.add_systems(Update, (update_player_control, move_player_tank).chain());

    app.update();

    let mut players = app.world_mut().query::<(&Player, &Tank, &Transform)>();
    let players: Vec<(PlayerId, Vec2, Direction, Vec3)> = players
        .iter(app.world())
        .map(|(player, tank, transform)| {
            (player.id, tank.top_left, tank.facing, transform.translation)
        })
        .collect();
    let p1 = players
        .iter()
        .find(|(player, _, _, _)| *player == PlayerId::One)
        .expect("P1 should exist");
    let p2 = players
        .iter()
        .find(|(player, _, _, _)| *player == PlayerId::Two)
        .expect("P2 should exist");
    let p1_next = p1_top_left + Vec2::new(TILE_SIZE, 0.0);
    let p2_next = p2_top_left + Vec2::new(0.0, TILE_SIZE);

    assert_eq!(
        *p1,
        (
            PlayerId::One,
            p1_next,
            Direction::Right,
            board_object_center(p1_next.x, p1_next.y, Vec2::splat(TANK_SIZE), 6.0)
        )
    );
    assert_eq!(
        *p2,
        (
            PlayerId::Two,
            p2_next,
            Direction::Down,
            board_object_center(p2_next.x, p2_next.y, Vec2::splat(TANK_SIZE), 6.0)
        )
    );
}

#[test]
fn versus_fire_controls_spawn_each_players_bullet_independently() {
    let mut app = App::new();
    let p1_top_left = Vec2::new(32.0, 48.0);
    let p2_top_left = Vec2::new(112.0, 48.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Space);
    keys.press(KeyCode::Enter);

    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.world_mut().spawn((
        Tank {
            top_left: p1_top_left,
            facing: Direction::Right,
            speed: PLAYER_SPEED,
        },
        PlayerUpgrade { level: 0 },
        Player { id: PlayerId::One },
    ));
    app.world_mut().spawn((
        Tank {
            top_left: p2_top_left,
            facing: Direction::Left,
            speed: PLAYER_SPEED,
        },
        PlayerUpgrade { level: 0 },
        Player { id: PlayerId::Two },
    ));
    app.add_systems(Update, fire_player_bullet);

    app.update();

    let mut bullets = app.world_mut().query::<&Bullet>();
    let bullets: Vec<_> = bullets.iter(app.world()).collect();
    assert_eq!(bullets.len(), 2);

    let p1_bullet = bullets
        .iter()
        .find(|bullet| bullet.owner == Team::Player1)
        .expect("P1 should fire a player-one bullet");
    let p2_bullet = bullets
        .iter()
        .find(|bullet| bullet.owner == Team::Player2)
        .expect("P2 should fire a player-two bullet");
    let expected_p1_top_left = spawn_bullet_position(p1_top_left, Direction::Right);
    let expected_p2_top_left = spawn_bullet_position(p2_top_left, Direction::Left);

    assert_eq!(p1_bullet.previous_top_left, expected_p1_top_left);
    assert_eq!(p1_bullet.top_left, expected_p1_top_left);
    assert_eq!(p1_bullet.facing, Direction::Right);
    assert_eq!(p1_bullet.speed, BULLET_SPEED);
    assert!(!p1_bullet.breaks_steel);
    assert!(!p1_bullet.resolved);

    assert_eq!(p2_bullet.previous_top_left, expected_p2_top_left);
    assert_eq!(p2_bullet.top_left, expected_p2_top_left);
    assert_eq!(p2_bullet.facing, Direction::Left);
    assert_eq!(p2_bullet.speed, BULLET_SPEED);
    assert!(!p2_bullet.breaks_steel);
    assert!(!p2_bullet.resolved);
}

#[test]
fn player_two_can_fire_with_right_shift() {
    let mut app = App::new();
    let tank_top_left = Vec2::new(112.0, 48.0);
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ShiftRight);

    app.insert_resource(keys);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.world_mut().spawn((
        Tank {
            top_left: tank_top_left,
            facing: Direction::Down,
            speed: PLAYER_SPEED,
        },
        PlayerUpgrade { level: 0 },
        Player { id: PlayerId::Two },
    ));
    app.add_systems(Update, fire_player_bullet);

    app.update();

    let expected_top_left = spawn_bullet_position(tank_top_left, Direction::Down);
    let mut bullets = app.world_mut().query::<&Bullet>();
    let bullets: Vec<_> = bullets.iter(app.world()).collect();
    assert_eq!(bullets.len(), 1);
    assert_eq!(bullets[0].owner, Team::Player2);
    assert_eq!(bullets[0].previous_top_left, expected_top_left);
    assert_eq!(bullets[0].top_left, expected_top_left);
    assert_eq!(bullets[0].facing, Direction::Down);
    assert_eq!(bullets[0].speed, BULLET_SPEED);
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
fn player_held_fire_refires_after_previous_bullet_is_gone() {
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
    app.insert_resource(StageRules::default());
    app.insert_resource(VersusPlayerFreeze::default());
    spawn_test_player(app.world_mut(), PlayerId::One, tank_top_left, 3);
    app.add_systems(Update, fire_player_bullet);

    app.update();

    let expected_top_left = spawn_bullet_position(tank_top_left, Direction::Up);
    let mut bullets = app.world_mut().query::<(Entity, &Bullet)>();
    let first_bullets: Vec<(Entity, Team, Vec2)> = bullets
        .iter(app.world())
        .map(|(entity, bullet)| (entity, bullet.owner, bullet.top_left))
        .collect();
    assert_eq!(first_bullets.len(), 1);
    assert_eq!(first_bullets[0].1, Team::Player1);
    assert_eq!(first_bullets[0].2, expected_top_left);

    app.update();

    let mut bullets = app.world_mut().query_filtered::<Entity, With<Bullet>>();
    assert_eq!(bullets.iter(app.world()).count(), 1);

    app.world_mut().entity_mut(first_bullets[0].0).despawn();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear_just_pressed(KeyCode::Space);
    app.update();

    let mut bullets = app.world_mut().query::<(Entity, &Bullet)>();
    let bullets: Vec<(Entity, Team, Vec2)> = bullets
        .iter(app.world())
        .map(|(entity, bullet)| (entity, bullet.owner, bullet.top_left))
        .collect();
    assert_eq!(bullets.len(), 1);
    assert_ne!(bullets[0].0, first_bullets[0].0);
    assert_eq!(bullets[0].1, Team::Player1);
    assert_eq!(bullets[0].2, expected_top_left);
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
fn normal_bullet_hit_on_steel_despawns_bullet_without_changing_tile() {
    let mut app = App::new();
    let bullet_top_left = Vec2::new(24.0, 8.0);
    let mut grid = TileGrid::empty();
    grid.set(3, 1, TileKind::Steel);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.world_mut()
        .spawn((GridTile { x: 3, y: 1 }, Sprite::default()));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: bullet_top_left,
            top_left: bullet_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    assert_eq!(
        app.world().resource::<TileGrid>().get(3, 1),
        Some(TileKind::Steel)
    );
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut tiles = app.world_mut().query::<&GridTile>();
    let remaining_tiles: Vec<(usize, usize)> = tiles
        .iter(app.world())
        .map(|tile| (tile.x, tile.y))
        .collect();
    assert_eq!(remaining_tiles, [(3, 1)]);
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 1);
}

#[test]
fn steel_breaking_bullet_hit_destroys_tile_and_sprite() {
    let mut app = App::new();
    let bullet_top_left = Vec2::new(24.0, 8.0);
    let mut grid = TileGrid::empty();
    grid.set(3, 1, TileKind::Steel);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.world_mut()
        .spawn((GridTile { x: 3, y: 1 }, Sprite::default()));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: bullet_top_left,
            top_left: bullet_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: true,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    assert_eq!(
        app.world().resource::<TileGrid>().get(3, 1),
        Some(TileKind::Empty)
    );
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut tiles = app.world_mut().query::<&GridTile>();
    assert_eq!(tiles.iter(app.world()).count(), 0);
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 1);
}

#[test]
fn brick_hit_updates_grid_and_removes_sprite_immediately() {
    let mut app = App::new();
    let bullet_top_left = Vec2::new(24.0, 8.0);
    let mut grid = TileGrid::empty();
    grid.set(3, 1, TileKind::Brick);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.world_mut()
        .spawn((GridTile { x: 3, y: 1 }, Sprite::default()));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: bullet_top_left,
            top_left: bullet_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let grid = app.world().resource::<TileGrid>();
    assert_eq!(grid.get(3, 1), Some(TileKind::Empty));
    assert!(grid.can_tank_occupy(Vec2::new(24.0, 8.0)));
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut tiles = app.world_mut().query::<&GridTile>();
    assert_eq!(tiles.iter(app.world()).count(), 0);
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 1);
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
fn enemy_spawning_waits_for_clear_spawn_area_without_consuming_roster() {
    let level = parse_level(LEVEL_1).expect("level should parse");
    let grid = TileGrid::from_level(&level).expect("grid should build");
    let mut app = App::new();
    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(EnemyDirector::from_level(&level));

    let blockers: Vec<Entity> = level
        .enemy_spawns
        .iter()
        .map(|spawn| {
            app.world_mut()
                .spawn(Tank {
                    top_left: spawn_point_top_left(spawn),
                    facing: Direction::Down,
                    speed: 0.0,
                })
                .id()
        })
        .collect();
    app.add_systems(Update, spawn_enemies);

    app.update();

    assert_eq!(enemy_tank_count(&mut app), 0);
    let director = app.world().resource::<EnemyDirector>();
    assert_eq!(director.spawned_count, 0);
    assert_eq!(director.roster.len(), level.enemies.len());

    for blocker in blockers {
        app.world_mut().entity_mut(blocker).despawn();
    }
    app.update();

    assert_eq!(enemy_tank_count(&mut app), 1);
    let spawned_top_left = spawned_enemy_top_lefts(&mut app);
    assert_eq!(
        spawned_top_left,
        vec![spawn_point_top_left(&level.enemy_spawns[0])]
    );
    let director = app.world().resource::<EnemyDirector>();
    assert_eq!(director.spawned_count, 1);
    assert_eq!(director.roster.len(), level.enemies.len() - 1);
}

fn enemy_tank_count(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&EnemyTank>();
    query.iter(app.world()).count()
}

fn spawned_enemy_top_lefts(app: &mut App) -> Vec<Vec2> {
    let mut query = app.world_mut().query::<(&Tank, &EnemyTank)>();
    query
        .iter(app.world())
        .map(|(tank, _)| tank.top_left)
        .collect()
}

#[test]
fn level_rejects_invalid_powerup_carrier_markers() {
    let duplicate = LEVEL_1.replacen("(enemy: 10, kind: Helmet)", "(enemy: 5, kind: Helmet)", 1);
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
fn star_powerup_caps_player_upgrade_level() {
    let top_left = Vec2::new(64.0, 64.0);
    let mut app = powerup_pickup_app(GameMode::Campaign, ScoreBoard::campaign(20));

    app.world_mut().spawn((
        Tank {
            top_left,
            facing: Direction::Up,
            speed: PLAYER_SPEED,
        },
        Player { id: PlayerId::One },
        PlayerUpgrade { level: 2 },
        PlayerLives { current: 3 },
        Health { current: 1 },
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0,
        )),
        Sprite {
            color: player_upgrade_visual_color(2),
            ..default()
        },
    ));
    spawn_test_powerup(app.world_mut(), PowerUpKind::Star, top_left);

    app.update();

    let mut players = app.world_mut().query::<(&PlayerUpgrade, &Sprite)>();
    let upgraded: Vec<(u8, Color)> = players
        .iter(app.world())
        .map(|(upgrade, sprite)| (upgrade.level, sprite.color))
        .collect();
    assert_eq!(upgraded, [(3, player_upgrade_visual_color(3))]);

    spawn_test_powerup(app.world_mut(), PowerUpKind::Star, top_left);
    app.update();

    let mut players = app.world_mut().query::<&PlayerUpgrade>();
    let capped: Vec<u8> = players
        .iter(app.world())
        .map(|upgrade| upgrade.level)
        .collect();
    assert_eq!(capped, [3]);
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
fn campaign_powerup_drop_replaces_existing_powerup_and_sparkle() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    spawn_test_powerup(app.world_mut(), PowerUpKind::Star, Vec2::new(32.0, 32.0));
    app.world_mut().spawn(PowerUpSparkle);
    app.add_systems(Update, spawn_campaign_powerup_for_test);

    app.update();

    let mut powerups = app.world_mut().query::<(&PowerUp, &Transform)>();
    let active: Vec<(PowerUpKind, Vec2)> = powerups
        .iter(app.world())
        .map(|(powerup, transform)| {
            (
                powerup.kind,
                board_top_left_from_translation(transform.translation, TANK_SIZE),
            )
        })
        .collect();
    assert_eq!(active, [(PowerUpKind::Helmet, Vec2::new(96.0, 96.0))]);

    let mut sparkles = app.world_mut().query::<&PowerUpSparkle>();
    assert_eq!(sparkles.iter(app.world()).count(), 1);
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
fn arena_seven_authors_forest_island_deathmatch_space() {
    let arena = parse_arena(ARENA_7).expect("arena should parse");
    let grid = TileGrid::from_arena(&arena).expect("grid should build");

    assert!(grid.tiles.contains(&TileKind::Water));
    assert!(grid.tiles.contains(&TileKind::Forest));
    assert!(grid.tiles.contains(&TileKind::Ice));
    assert!(grid.tiles.contains(&TileKind::Steel));
    assert_eq!(arena.powerup_spawns.len(), 3);
    assert_eq!(arena.p1_spawn.x, 4);
    assert_eq!(arena.p1_spawn.y, 24);
    assert_eq!(arena.p2_spawn.x, 22);
    assert_eq!(arena.p2_spawn.y, 0);
    let BattleRules::Deathmatch { target_score, .. } = arena.battle_rules else {
        panic!("arena seven should be deathmatch");
    };
    assert_eq!(target_score, 5);
}

#[test]
fn arena_eight_authors_base_battle_bridge_lanes() {
    let arena = parse_arena(ARENA_8).expect("arena should parse");
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
        panic!("arena eight should be base battle");
    };
    assert_eq!(p1_base, GridPoint { x: 0, y: 24 });
    assert_eq!(p2_base, GridPoint { x: 24, y: 0 });
}

#[test]
fn powerup_visuals_sparkle_between_bright_tints() {
    assert_eq!(
        powerup_visual_rgb(PowerUpKind::Clock, 0.05),
        [255, 255, 255]
    );
    assert_eq!(
        powerup_visual_rgb(PowerUpKind::Clock, 0.20),
        [216, 240, 255]
    );
    assert_eq!(
        powerup_visual_rgb(PowerUpKind::Grenade, 0.20),
        [255, 224, 184]
    );
    assert_eq!(powerup_visual_rgb(PowerUpKind::Tank, 0.20), [216, 255, 216]);
    assert_eq!(
        powerup_visual_rgb(PowerUpKind::Clock, 0.35),
        [255, 255, 255]
    );
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
fn versus_clock_pickup_freezes_opponent_movement_and_fire() {
    let mut app = App::new();
    let p1_top_left = Vec2::new(32.0, 32.0);
    let p2_top_left = Vec2::new(96.0, 32.0);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(TILE_SIZE / PLAYER_SPEED));
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::ArrowDown);
    keys.press(KeyCode::Enter);

    app.insert_resource(time);
    app.insert_resource(keys);
    app.insert_resource(PlayerControl::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(GameMode::VersusDeathmatch);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(StageRules::default());
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
        TankSpriteState::new(TankSpriteSet::Player1),
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
            facing: Direction::Left,
            speed: PLAYER_SPEED,
        },
        TankSpriteState::new(TankSpriteSet::Player2),
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
    ));
    spawn_test_powerup(app.world_mut(), PowerUpKind::Clock, p1_top_left);
    app.add_systems(
        Update,
        (
            pickup_powerups,
            update_player_control,
            move_player_tank,
            fire_player_bullet,
        )
            .chain(),
    );

    app.update();

    assert!(
        app.world()
            .resource::<VersusPlayerFreeze>()
            .is_player_frozen(PlayerId::Two)
    );
    let mut players = app.world_mut().query::<(&Player, &Tank)>();
    let p2 = players
        .iter(app.world())
        .find(|(player, _)| player.id == PlayerId::Two)
        .map(|(_, tank)| (tank.top_left, tank.facing))
        .expect("P2 should remain spawned");
    assert_eq!(p2.0, p2_top_left);
    assert_eq!(p2.1, Direction::Left);
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
}

#[test]
fn campaign_clock_pickup_freezes_enemy_spawn_movement_and_fire() {
    let level = parse_level(LEVEL_1).expect("level should parse");
    let mut app = App::new();
    let player_top_left = Vec2::new(32.0, 32.0);
    let enemy_top_left = Vec2::new(96.0, 32.0);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(
        enemy_fire_interval(EnemyKind::Basic) + 0.1,
    ));

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(TileGrid::empty());
    app.insert_resource(StageRules::default());
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(VersusPlayerFreeze::default());
    app.insert_resource(BaseReinforcement::default());
    app.insert_resource(ScoreBoard::campaign(level.enemies.len()));
    app.insert_resource(EnemyDirector::from_level(&level));
    spawn_test_player(app.world_mut(), PlayerId::One, player_top_left, 3);
    spawn_test_powerup(app.world_mut(), PowerUpKind::Clock, player_top_left);
    let enemy = spawn_test_enemy_tank(
        app.world_mut(),
        EnemyKind::Basic,
        enemy_top_left,
        Direction::Left,
    );
    app.add_systems(
        Update,
        (
            pickup_powerups,
            spawn_enemies,
            move_enemy_tanks,
            fire_enemy_bullets,
        )
            .chain(),
    );

    app.update();

    assert!(app.world().resource::<EnemyFreeze>().is_active());
    assert_eq!(enemy_tank_count(&mut app), 1);
    let tank = app
        .world()
        .get::<Tank>(enemy)
        .expect("existing enemy should remain frozen in place");
    assert_eq!(tank.top_left, enemy_top_left);
    assert_eq!(tank.facing, Direction::Left);
    let director = app.world().resource::<EnemyDirector>();
    assert_eq!(director.spawned_count, 0);
    assert_eq!(director.roster.len(), level.enemies.len());
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
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
fn base_battle_own_bullet_hits_own_base_without_ending_round() {
    let mut app = App::new();
    let base_top_left = Vec2::new(96.0, 192.0);
    let mut grid = TileGrid::empty();
    grid.set(12, 24, TileKind::Base);
    grid.set(13, 24, TileKind::Base);
    grid.set(12, 25, TileKind::Base);
    grid.set(13, 25, TileKind::Base);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusBaseBattle);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::versus(3, 5, 2.0));
    app.world_mut().spawn((
        BaseSprite {
            owner: Some(PlayerId::One),
            top_left: base_top_left,
        },
        Sprite::default(),
    ));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: base_top_left,
            top_left: base_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            base_top_left.x,
            base_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::Playing);
    assert_eq!(status.winner, None);
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.p1_score, 0);
    assert_eq!(score_board.p2_score, 0);

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut bases = app.world_mut().query::<&BaseSprite>();
    assert_eq!(bases.iter(app.world()).count(), 1);
}

#[test]
fn base_battle_opponent_base_hit_ends_round_for_shooter() {
    let mut app = App::new();
    let base_top_left = Vec2::new(96.0, 192.0);
    let mut grid = TileGrid::empty();
    grid.set(12, 24, TileKind::Base);
    grid.set(13, 24, TileKind::Base);
    grid.set(12, 25, TileKind::Base);
    grid.set(13, 25, TileKind::Base);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::VersusBaseBattle);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::versus(3, 5, 2.0));
    app.world_mut().spawn((
        BaseSprite {
            owner: Some(PlayerId::Two),
            top_left: base_top_left,
        },
        Sprite::default(),
    ));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: base_top_left,
            top_left: base_top_left,
            facing: Direction::Right,
            owner: Team::Player1,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            base_top_left.x,
            base_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::RoundOver);
    assert_eq!(status.winner, Some(PlayerId::One));
    let score_board = app.world().resource::<ScoreBoard>();
    assert_eq!(score_board.p1_score, 0);
    assert_eq!(score_board.p2_score, 0);

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut bases = app.world_mut().query::<&BaseSprite>();
    assert_eq!(bases.iter(app.world()).count(), 1);
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 2);
}

#[test]
fn enemy_bullet_destroying_campaign_base_enters_game_over() {
    let mut app = App::new();
    let base_top_left = Vec2::new(96.0, 192.0);
    let mut grid = TileGrid::empty();
    grid.set(12, 24, TileKind::Base);
    grid.set(13, 24, TileKind::Base);
    grid.set(12, 25, TileKind::Base);
    grid.set(13, 25, TileKind::Base);

    app.insert_resource(Time::<()>::default());
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameMode::Campaign);
    app.insert_resource(grid);
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(ScoreBoard::campaign(20));
    app.world_mut().spawn((
        BaseSprite {
            owner: None,
            top_left: base_top_left,
        },
        Sprite::default(),
    ));
    app.world_mut().spawn((
        Bullet {
            previous_top_left: base_top_left,
            top_left: base_top_left,
            facing: Direction::Right,
            owner: Team::Enemy,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        Transform::from_translation(board_object_center(
            base_top_left.x,
            base_top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        )),
    ));
    app.add_systems(Update, move_bullets);

    app.update();

    let status = app.world().resource::<GameStatus>();
    assert_eq!(status.phase, GamePhase::GameOver);
    assert_eq!(status.winner, None);
    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
    let mut bases = app.world_mut().query::<&BaseSprite>();
    assert_eq!(bases.iter(app.world()).count(), 1);
    let mut animations = app.world_mut().query::<&SpriteAnimation>();
    assert_eq!(animations.iter(app.world()).count(), 2);
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

    let p1_positions =
        shovel_reinforcement_positions(GameMode::VersusBaseBattle, PlayerId::One, &grid, &bases);
    assert!(p1_positions.contains(&(2, 24)));
    assert!(!p1_positions.contains(&(22, 0)));

    let p2_positions =
        shovel_reinforcement_positions(GameMode::VersusBaseBattle, PlayerId::Two, &grid, &bases);
    assert!(p2_positions.contains(&(22, 0)));
    assert!(p2_positions.contains(&(24, 2)));
    assert!(!p2_positions.contains(&(2, 24)));

    assert!(
        shovel_reinforcement_positions(GameMode::VersusDeathmatch, PlayerId::One, &grid, &bases)
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
            EnemyKind::Basic,
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
    assert!(enemy_should_roam(EnemyKind::Basic, top_left, Direction::Up));
    assert_eq!(
        enemy_patrol_direction(EnemyKind::Basic, top_left, Direction::Up),
        Direction::Left
    );
    assert_eq!(
        preferred_enemy_direction(
            EnemyKind::Basic,
            top_left,
            Direction::Up,
            &[],
            Some(Vec2::new(104.0, 200.0))
        ),
        Direction::Left
    );
}

#[test]
fn path_to_objective_strategy_routes_around_simple_obstacles() {
    let mut grid = TileGrid::empty();
    grid.set(2, 0, TileKind::Water);
    grid.set(2, 1, TileKind::Water);
    let enemy_top_left = Vec2::new(0.0, 0.0);
    let player_top_lefts = [Vec2::new(32.0, 16.0)];

    assert_eq!(
        path_direction_to_targets(&grid, enemy_top_left, &[GridPoint { x: 4, y: 2 }]),
        Some(Direction::Down)
    );
    assert_eq!(
        select_enemy_direction(
            EnemyAiStrategy::PathToObjective,
            EnemyDifficultyProfile::Normal,
            EnemyKind::Basic,
            enemy_top_left,
            Direction::Right,
            &player_top_lefts,
            None,
            &grid,
        ),
        Direction::Down
    );
    assert_eq!(
        preferred_enemy_direction(
            EnemyKind::Basic,
            enemy_top_left,
            Direction::Right,
            &player_top_lefts,
            None,
        ),
        Direction::Right
    );
}

#[test]
fn enemy_patrol_still_pushes_top_spawns_downward() {
    assert_eq!(
        enemy_patrol_direction(EnemyKind::Armor, Vec2::new(96.0, 0.0), Direction::Left),
        Direction::Down
    );
}

#[test]
fn enemy_type_personality_tunes_turning_and_roaming() {
    assert!(enemy_turn_interval(EnemyKind::Fast) < enemy_turn_interval(EnemyKind::Basic));
    assert!(enemy_turn_interval(EnemyKind::Basic) < enemy_turn_interval(EnemyKind::Armor));
    assert_eq!(enemy_turn_interval(EnemyKind::Power), 1.0);
    assert!(
        enemy_turn_interval_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Easy)
            > enemy_turn_interval(EnemyKind::Basic)
    );
    assert!(
        enemy_turn_interval_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Hard)
            < enemy_turn_interval(EnemyKind::Basic)
    );
    assert!(
        enemy_fire_interval_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Easy)
            > enemy_fire_interval(EnemyKind::Basic)
    );
    assert!(
        enemy_fire_interval_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Hard)
            < enemy_fire_interval(EnemyKind::Basic)
    );
    assert!(
        enemy_speed_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Easy)
            < enemy_speed(EnemyKind::Basic)
    );
    assert!(
        enemy_speed_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Hard)
            > enemy_speed(EnemyKind::Basic)
    );
    assert!(
        enemy_spawn_interval_for_profile(1.0, EnemyDifficultyProfile::Easy)
            > enemy_spawn_interval_for_profile(1.0, EnemyDifficultyProfile::Normal)
    );

    assert!(enemy_roam_rate(EnemyKind::Fast) < enemy_roam_rate(EnemyKind::Basic));
    assert!(enemy_roam_rate(EnemyKind::Basic) < enemy_roam_rate(EnemyKind::Armor));
    assert_eq!(enemy_roam_rate(EnemyKind::Power), 3);
    assert!(
        enemy_roam_rate_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Easy)
            < enemy_roam_rate(EnemyKind::Basic)
    );
    assert!(
        enemy_roam_rate_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Hard)
            > enemy_roam_rate(EnemyKind::Basic)
    );
    assert!(
        enemy_random_fire_rate_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Easy)
            > enemy_random_fire_rate(EnemyKind::Basic)
    );
    assert!(
        enemy_random_fire_rate_for_profile(EnemyKind::Basic, EnemyDifficultyProfile::Hard)
            < enemy_random_fire_rate(EnemyKind::Basic)
    );

    let top_left = Vec2::new(80.0, 80.0);
    assert!(enemy_should_roam(EnemyKind::Basic, top_left, Direction::Up));
    assert!(!enemy_should_roam(
        EnemyKind::Armor,
        top_left,
        Direction::Up
    ));
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
fn enemy_fire_system_respects_per_tank_bullet_limit() {
    let mut app = App::new();
    let enemy_one_top_left = Vec2::new(64.0, 32.0);
    let enemy_two_top_left = Vec2::new(112.0, 32.0);
    let mut time = Time::<()>::default();
    time.advance_by(Duration::from_secs_f32(enemy_fire_interval(
        EnemyKind::Basic,
    )));

    app.insert_resource(time);
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.insert_resource(EnemyFreeze::default());
    app.insert_resource(TileGrid::empty());
    spawn_test_player(
        app.world_mut(),
        PlayerId::One,
        enemy_one_top_left + Vec2::new(0.0, 64.0),
        3,
    );
    spawn_test_player(
        app.world_mut(),
        PlayerId::Two,
        enemy_two_top_left + Vec2::new(0.0, 64.0),
        3,
    );
    let enemy_one = spawn_test_enemy_tank(
        app.world_mut(),
        EnemyKind::Basic,
        enemy_one_top_left,
        Direction::Up,
    );
    let enemy_two = spawn_test_enemy_tank(
        app.world_mut(),
        EnemyKind::Basic,
        enemy_two_top_left,
        Direction::Up,
    );
    app.world_mut().spawn((
        Bullet {
            previous_top_left: Vec2::new(8.0, 8.0),
            top_left: Vec2::new(8.0, 8.0),
            facing: Direction::Down,
            owner: Team::Enemy,
            speed: BULLET_SPEED,
            breaks_steel: false,
            resolved: false,
        },
        EnemyBulletSource { shooter: enemy_one },
    ));
    app.add_systems(Update, fire_enemy_bullets);

    app.update();

    let mut sources = app.world_mut().query::<&EnemyBulletSource>();
    let shooters: Vec<Entity> = sources
        .iter(app.world())
        .map(|source| source.shooter)
        .collect();
    assert_eq!(
        shooters
            .iter()
            .filter(|shooter| **shooter == enemy_one)
            .count(),
        1
    );
    assert_eq!(
        shooters
            .iter()
            .filter(|shooter| **shooter == enemy_two)
            .count(),
        1
    );

    let expected_enemy_two_bullet = spawn_bullet_position(enemy_two_top_left, Direction::Down);
    let mut bullets = app.world_mut().query::<&Bullet>();
    let bullets: Vec<_> = bullets.iter(app.world()).collect();
    assert_eq!(bullets.len(), 2);
    let spawned = bullets
        .iter()
        .find(|bullet| bullet.top_left == expected_enemy_two_bullet)
        .expect("second enemy should fire when its own slot is free");
    assert_eq!(spawned.previous_top_left, expected_enemy_two_bullet);
    assert_eq!(spawned.facing, Direction::Down);
    assert_eq!(spawned.owner, Team::Enemy);
    assert_eq!(spawned.speed, BULLET_SPEED);
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
        Vec2::new(24.0, 32.0),
        Vec2::new(24.0, -4.0),
        false,
    ));
    app.add_systems(Update, cancel_colliding_bullets);

    app.update();

    let mut bullets = app.world_mut().query::<&Bullet>();
    let remaining: Vec<(Vec2, Vec2)> = bullets
        .iter(app.world())
        .map(|bullet| (bullet.previous_top_left, bullet.top_left))
        .collect();
    assert_eq!(remaining, [(Vec2::new(24.0, 32.0), Vec2::new(24.0, -4.0))]);
}

#[test]
fn cancel_colliding_bullets_despawns_simultaneous_clash_cluster() {
    let mut app = App::new();
    app.insert_resource(test_sprite_assets());
    app.insert_resource(test_sound_assets());
    app.insert_resource(GameStatus {
        phase: GamePhase::Playing,
        ..GameStatus::default()
    });
    app.world_mut().spawn(test_bullet(
        Vec2::new(8.0, 8.0),
        Vec2::new(20.0, 8.0),
        false,
    ));
    app.world_mut().spawn(test_bullet(
        Vec2::new(10.0, 8.0),
        Vec2::new(10.0, 20.0),
        false,
    ));
    app.world_mut().spawn(test_bullet(
        Vec2::new(9.0, 10.0),
        Vec2::new(20.0, 10.0),
        false,
    ));
    app.add_systems(Update, cancel_colliding_bullets);

    app.update();

    let mut bullets = app.world_mut().query::<&Bullet>();
    assert_eq!(bullets.iter(app.world()).count(), 0);
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
