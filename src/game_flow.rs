use crate::*;

pub(crate) fn enter_mode_select(
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
    mode_select.map_pack = game_status.map_pack;
    mode_select.stage = game_status
        .stage
        .clamp(1, mode_select.map_pack.stage_count());
    mode_select.arena = game_status.arena.clamp(1, ARENA_COUNT);
    spawn_mode_select_screen(commands, assets, mode_select);

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

pub(crate) fn respawn_mode_select_screen(
    commands: &mut Commands,
    assets: &SpriteAssets,
    mode_select: &ModeSelect,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_mode_select_screen(commands, assets, mode_select);
}

fn campaign_score_board(game_mode: GameMode, total_enemies: usize) -> ScoreBoard {
    if game_mode.is_coop_campaign() {
        ScoreBoard::coop_campaign(total_enemies)
    } else {
        ScoreBoard::campaign(total_enemies)
    }
}

fn campaign_player_spawns_for_restart(game_mode: GameMode) -> Vec<(PlayerId, i32)> {
    if game_mode.is_coop_campaign() {
        vec![(PlayerId::One, 3), (PlayerId::Two, 3)]
    } else {
        vec![(PlayerId::One, 3)]
    }
}

pub(crate) fn campaign_player_spawns_from_score(
    game_mode: GameMode,
    score_board: &ScoreBoard,
) -> Vec<(PlayerId, i32)> {
    if game_mode.is_coop_campaign() {
        [
            (PlayerId::One, score_board.p1_lives.max(0)),
            (PlayerId::Two, score_board.p2_lives.max(0)),
        ]
        .into_iter()
        .filter(|(_, lives)| *lives > 0)
        .collect()
    } else {
        vec![(PlayerId::One, score_board.lives.max(1))]
    }
}

pub(crate) fn restart_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_mode: GameMode,
    game_status: &mut GameStatus,
    tile_grid: &mut TileGrid,
    director: &mut EnemyDirector,
    score_board: &mut ScoreBoard,
    stage_rules: &mut StageRules,
    versus_powerups: &mut VersusPowerUpDirector,
    enemy_freeze: &mut EnemyFreeze,
    versus_freeze: &mut VersusPlayerFreeze,
    base_reinforcement: &mut BaseReinforcement,
    mode_select: &ModeSelect,
    game_entities: &Query<Entity, With<GameEntity>>,
) {
    let (level, new_tile_grid) =
        load_campaign_stage_bundle_or_panic(game_status.map_pack, game_status.stage);
    info!("Loaded {}", level.name);

    for entity in game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(commands, assets, game_mode);
    let player_spawns = campaign_player_spawns_for_restart(game_mode);
    spawn_level(
        commands,
        assets,
        &level,
        &new_tile_grid,
        &player_spawns,
        DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
    );
    play_sound(commands, sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    let ai_strategy = selected_enemy_ai_strategy(mode_select, &level);
    let difficulty_profile = selected_enemy_difficulty_profile(mode_select, &level);
    *director = EnemyDirector::from_level_with_ai(&level, ai_strategy, difficulty_profile);
    *score_board = campaign_score_board(game_mode, level.enemies.len());
    *stage_rules = StageRules::from_level(&level);
    *versus_powerups = VersusPowerUpDirector::inactive();
    enemy_freeze.reset();
    versus_freeze.reset();
    base_reinforcement.reset();
    game_status.phase = GamePhase::StageIntro;
    game_status.winner = None;
    game_status.transition_timer = Timer::from_seconds(STAGE_INTRO_SECONDS, TimerMode::Once);
}

pub(crate) fn start_versus_round(
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

pub(crate) fn check_game_phase(
    mut commands: Commands,
    game_mode: Res<GameMode>,
    sounds: Res<SoundAssets>,
    mut game_status: ResMut<GameStatus>,
    mut score_board: ResMut<ScoreBoard>,
    director: Res<EnemyDirector>,
    active_enemies: Query<&EnemyTank>,
) {
    if !game_mode.is_campaign() || !game_status.is_playing() {
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

pub(crate) fn clear_terminal_transients(
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

pub(crate) fn advance_after_stage_intro(time: Res<Time>, mut game_status: ResMut<GameStatus>) {
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

pub(crate) fn advance_after_level_clear(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_mode: Res<GameMode>,
    mode_select: Res<ModeSelect>,
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

    if game_status.stage >= game_status.map_pack.stage_count() {
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
    let (level, new_tile_grid) =
        load_campaign_stage_bundle_or_panic(game_status.map_pack, next_stage);
    info!("Loaded {}", level.name);

    for entity in &game_entities {
        commands.entity(entity).despawn();
    }

    spawn_screen_frame(&mut commands, &assets, *game_mode);
    let player_spawns = campaign_player_spawns_from_score(*game_mode, &score_board);
    spawn_level(
        &mut commands,
        &assets,
        &level,
        &new_tile_grid,
        &player_spawns,
        DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
    );
    play_sound(&mut commands, &sounds, SoundKind::StageStart);

    *tile_grid = new_tile_grid;
    let ai_strategy = selected_enemy_ai_strategy(&mode_select, &level);
    let difficulty_profile = selected_enemy_difficulty_profile(&mode_select, &level);
    *director = EnemyDirector::from_level_with_ai(&level, ai_strategy, difficulty_profile);
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

pub(crate) fn enter_victory_screen(
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
