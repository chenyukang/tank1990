use crate::*;

#[derive(Resource)]
pub(super) struct PlayerControl {
    pub(super) p1_last_direction: Direction,
    pub(super) p2_last_direction: Direction,
    pub(super) p1_direction_priority: Vec<Direction>,
    pub(super) p2_direction_priority: Vec<Direction>,
    pub(super) p1_consumed_3d_turns: Vec<Direction>,
    pub(super) p2_consumed_3d_turns: Vec<Direction>,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            p1_last_direction: Direction::Up,
            p2_last_direction: Direction::Down,
            p1_direction_priority: Vec::new(),
            p2_direction_priority: Vec::new(),
            p1_consumed_3d_turns: Vec::new(),
            p2_consumed_3d_turns: Vec::new(),
        }
    }
}

impl PlayerControl {
    pub(crate) fn update_from_input(&mut self, keys: &ButtonInput<KeyCode>) {
        update_direction_priority(
            keys,
            PlayerId::One,
            &mut self.p1_direction_priority,
            &mut self.p1_last_direction,
        );
        update_direction_priority(
            keys,
            PlayerId::Two,
            &mut self.p2_direction_priority,
            &mut self.p2_last_direction,
        );
        prune_3d_turn_consumptions(keys, PlayerId::One, &mut self.p1_consumed_3d_turns);
        prune_3d_turn_consumptions(keys, PlayerId::Two, &mut self.p2_consumed_3d_turns);
    }

    pub(crate) fn tank_motion(
        &mut self,
        keys: &ButtonInput<KeyCode>,
        player: PlayerId,
        current_facing: Direction,
        use_3d_controls: bool,
    ) -> Option<PlayerTankMotion> {
        if use_3d_controls {
            return self.tank_motion_3d(keys, player, current_facing);
        }

        held_direction(keys, self.last_direction(player), player).map(|direction| {
            PlayerTankMotion {
                facing: direction,
                movement: Some(direction),
            }
        })
    }

    fn tank_motion_3d(
        &mut self,
        keys: &ButtonInput<KeyCode>,
        player: PlayerId,
        current_facing: Direction,
    ) -> Option<PlayerTankMotion> {
        let last_direction = self.last_direction(player);
        let consumed_turns = self.consumed_3d_turns(player);
        prune_3d_turn_consumptions(keys, player, consumed_turns);
        let turn = just_pressed_3d_turn_input(keys, last_direction, player, consumed_turns);
        let facing = turn
            .map(|turn| apply_3d_turn(current_facing, turn))
            .unwrap_or(current_facing);

        let movement = held_3d_throttle_direction(keys, last_direction, player, facing);
        if turn.is_none() && movement.is_none() {
            return None;
        }

        Some(PlayerTankMotion { facing, movement })
    }

    fn last_direction(&self, player: PlayerId) -> Direction {
        match player {
            PlayerId::One => self.p1_last_direction,
            PlayerId::Two => self.p2_last_direction,
        }
    }

    fn consumed_3d_turns(&mut self, player: PlayerId) -> &mut Vec<Direction> {
        match player {
            PlayerId::One => &mut self.p1_consumed_3d_turns,
            PlayerId::Two => &mut self.p2_consumed_3d_turns,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlayerTankMotion {
    pub(crate) facing: Direction,
    pub(crate) movement: Option<Direction>,
}

pub(crate) fn update_player_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut control: ResMut<PlayerControl>,
) {
    control.update_from_input(&keys);
}

#[cfg(test)]
pub(crate) fn player_3d_tank_motion(
    keys: &ButtonInput<KeyCode>,
    control: &mut PlayerControl,
    player: PlayerId,
    current_facing: Direction,
) -> Option<PlayerTankMotion> {
    control.tank_motion_3d(keys, player, current_facing)
}

pub(crate) fn handle_shared_controls(
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
        Query<&mut Window, With<PrimaryWindow>>,
    )>,
) {
    if game_status.phase == GamePhase::ModeSelect {
        if keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp) {
            mode_select.selected = previous_mode_select_option(mode_select.selected);
            update_mode_select_cursor(
                &mut menu_queries.p1(),
                &ModeSelectDisplay::from_mode_select(&mode_select),
            );
        }

        if keys.just_pressed(KeyCode::KeyS) || keys.just_pressed(KeyCode::ArrowDown) {
            mode_select.selected = next_mode_select_option(mode_select.selected);
            update_mode_select_cursor(
                &mut menu_queries.p1(),
                &ModeSelectDisplay::from_mode_select(&mode_select),
            );
        }

        if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
            match mode_select.selected {
                ModeSelectOption::Campaign
                | ModeSelectOption::CoopCampaign
                | ModeSelectOption::Stage => {
                    mode_select.stage = previous_stage(mode_select.stage, mode_select.map_pack);
                    update_mode_select_stage_digits(
                        &mut menu_queries.p2(),
                        &assets.manifest.glyphs,
                        mode_select.stage,
                    );
                }
                ModeSelectOption::Battle | ModeSelectOption::Arena => {
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
                ModeSelectOption::MapPack => {
                    mode_select.map_pack = previous_campaign_map_pack(mode_select.map_pack);
                    mode_select.stage = selected_campaign_stage(&mode_select);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewMode => {
                    mode_select.view_mode = previous_tank_view_mode(mode_select.view_mode);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewAssist => {
                    mode_select.view_assist = toggle_view_assist(mode_select.view_assist);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::AiStrategy => {
                    mode_select.ai_strategy =
                        previous_mode_select_ai_strategy(mode_select.ai_strategy);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Difficulty => {
                    mode_select.difficulty_profile =
                        previous_mode_select_difficulty_profile(mode_select.difficulty_profile);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode =
                        previous_available_audio_mode(mode_select.audio_mode, &sounds);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
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
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Scale => {
                    let scale = previous_window_scale(mode_select.window_scale);
                    change_mode_select_window_scale(&mut mode_select, scale);
                    resize_primary_window(&mut menu_queries.p7(), mode_select.window_scale);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
            }
        }

        if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
            match mode_select.selected {
                ModeSelectOption::Campaign
                | ModeSelectOption::CoopCampaign
                | ModeSelectOption::Stage => {
                    mode_select.stage = next_stage(mode_select.stage, mode_select.map_pack);
                    update_mode_select_stage_digits(
                        &mut menu_queries.p2(),
                        &assets.manifest.glyphs,
                        mode_select.stage,
                    );
                }
                ModeSelectOption::Battle | ModeSelectOption::Arena => {
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
                ModeSelectOption::MapPack => {
                    mode_select.map_pack = next_campaign_map_pack(mode_select.map_pack);
                    mode_select.stage = selected_campaign_stage(&mode_select);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewMode => {
                    mode_select.view_mode = next_tank_view_mode(mode_select.view_mode);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewAssist => {
                    mode_select.view_assist = toggle_view_assist(mode_select.view_assist);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::AiStrategy => {
                    mode_select.ai_strategy = next_mode_select_ai_strategy(mode_select.ai_strategy);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Difficulty => {
                    mode_select.difficulty_profile =
                        next_mode_select_difficulty_profile(mode_select.difficulty_profile);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode =
                        next_available_audio_mode(mode_select.audio_mode, &sounds);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
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
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Scale => {
                    let scale = next_window_scale(mode_select.window_scale);
                    change_mode_select_window_scale(&mut mode_select, scale);
                    resize_primary_window(&mut menu_queries.p7(), mode_select.window_scale);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
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
                    game_status.map_pack = mode_select.map_pack;
                    game_status.stage = selected_campaign_stage(&mode_select);
                    *game_mode = GameMode::Campaign;
                    restart_level(
                        &mut commands,
                        &assets,
                        &sounds,
                        *game_mode,
                        &mut game_status,
                        &mut tile_grid,
                        &mut director,
                        &mut score_board,
                        &mut stage_rules,
                        &mut versus_powerups,
                        &mut enemy_freeze,
                        &mut versus_freeze,
                        &mut base_reinforcement,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::CoopCampaign => {
                    game_status.map_pack = mode_select.map_pack;
                    game_status.stage = selected_campaign_stage(&mode_select);
                    *game_mode = GameMode::CoopCampaign;
                    restart_level(
                        &mut commands,
                        &assets,
                        &sounds,
                        *game_mode,
                        &mut game_status,
                        &mut tile_grid,
                        &mut director,
                        &mut score_board,
                        &mut stage_rules,
                        &mut versus_powerups,
                        &mut enemy_freeze,
                        &mut versus_freeze,
                        &mut base_reinforcement,
                        &mode_select,
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
                ModeSelectOption::MapPack => {
                    mode_select.map_pack = next_campaign_map_pack(mode_select.map_pack);
                    mode_select.stage = selected_campaign_stage(&mode_select);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewMode => {
                    mode_select.view_mode = next_tank_view_mode(mode_select.view_mode);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::ViewAssist => {
                    mode_select.view_assist = toggle_view_assist(mode_select.view_assist);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::AiStrategy => {
                    mode_select.ai_strategy = next_mode_select_ai_strategy(mode_select.ai_strategy);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Difficulty => {
                    mode_select.difficulty_profile =
                        next_mode_select_difficulty_profile(mode_select.difficulty_profile);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Music => {
                    mode_select.audio_mode =
                        next_available_audio_mode(mode_select.audio_mode, &sounds);
                    update_mode_select_music_value(
                        &mut menu_queries.p5(),
                        &assets.manifest.glyphs,
                        mode_select.audio_mode,
                    );
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
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
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Scale => {
                    let scale = next_window_scale(mode_select.window_scale);
                    change_mode_select_window_scale(&mut mode_select, scale);
                    resize_primary_window(&mut menu_queries.p7(), mode_select.window_scale);
                    respawn_mode_select_screen(
                        &mut commands,
                        &assets,
                        &mode_select,
                        &menu_queries.p0(),
                    );
                }
                ModeSelectOption::Stage => {
                    mode_select.stage = next_stage(mode_select.stage, mode_select.map_pack);
                    update_mode_select_stage_digits(
                        &mut menu_queries.p2(),
                        &assets.manifest.glyphs,
                        mode_select.stage,
                    );
                }
                ModeSelectOption::Arena => {
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
            }
        }
        return;
    }

    if pause_toggle_requested(&keys) {
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
            GameMode::Campaign | GameMode::CoopCampaign => restart_level(
                &mut commands,
                &assets,
                &sounds,
                *game_mode,
                &mut game_status,
                &mut tile_grid,
                &mut director,
                &mut score_board,
                &mut stage_rules,
                &mut versus_powerups,
                &mut enemy_freeze,
                &mut versus_freeze,
                &mut base_reinforcement,
                &mode_select,
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

pub(crate) fn toggle_pause_phase(phase: GamePhase) -> GamePhase {
    match phase {
        GamePhase::Playing => GamePhase::Paused,
        GamePhase::Paused => GamePhase::Playing,
        phase => phase,
    }
}

pub(crate) fn pause_toggle_requested(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::KeyP)
        || keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Pause)
}

fn apply_3d_turn(facing: Direction, turn: Direction) -> Direction {
    match turn {
        Direction::Left => facing.turn_left(),
        Direction::Right => facing.turn_right(),
        Direction::Up | Direction::Down => facing,
    }
}

fn just_pressed_3d_turn_input(
    keys: &ButtonInput<KeyCode>,
    last_direction: Direction,
    player: PlayerId,
    consumed_turns: &mut Vec<Direction>,
) -> Option<Direction> {
    let left = direction_just_pressed(keys, Direction::Left, player)
        && !consumed_turns.contains(&Direction::Left);
    let right = direction_just_pressed(keys, Direction::Right, player)
        && !consumed_turns.contains(&Direction::Right);
    let turn = match (left, right) {
        (true, false) => Some(Direction::Left),
        (false, true) => Some(Direction::Right),
        (true, true) if matches!(last_direction, Direction::Left | Direction::Right) => {
            Some(last_direction)
        }
        _ => None,
    };
    if let Some(turn) = turn {
        record_3d_turn_consumption(consumed_turns, turn);
    }
    turn
}

fn direction_just_pressed(
    keys: &ButtonInput<KeyCode>,
    direction: Direction,
    player: PlayerId,
) -> bool {
    direction_key_pairs(player)
        .into_iter()
        .any(|(key, candidate)| candidate == direction && keys.just_pressed(key))
}

fn held_3d_throttle_direction(
    keys: &ButtonInput<KeyCode>,
    last_direction: Direction,
    player: PlayerId,
    facing: Direction,
) -> Option<Direction> {
    let forward = direction_is_held(keys, Direction::Up, player);
    let backward = direction_is_held(keys, Direction::Down, player);
    match (forward, backward) {
        (true, false) => Some(facing),
        (false, true) => Some(facing.opposite()),
        (true, true) if last_direction == Direction::Up => Some(facing),
        (true, true) if last_direction == Direction::Down => Some(facing.opposite()),
        _ => None,
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

fn record_3d_turn_consumption(consumed_turns: &mut Vec<Direction>, direction: Direction) {
    consumed_turns.retain(|consumed| *consumed != direction);
    consumed_turns.push(direction);
}

fn prune_3d_turn_consumptions(
    keys: &ButtonInput<KeyCode>,
    player: PlayerId,
    consumed_turns: &mut Vec<Direction>,
) {
    consumed_turns.retain(|direction| direction_is_held(keys, *direction, player));
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

pub(crate) fn record_direction_press(priority: &mut Vec<Direction>, direction: Direction) {
    priority.retain(|held| *held != direction);
    priority.push(direction);
}

pub(crate) fn prune_direction_priority(
    priority: &mut Vec<Direction>,
    is_held: impl Fn(Direction) -> bool,
) {
    priority.retain(|direction| is_held(*direction));
}

pub(crate) fn preferred_direction(priority: &[Direction]) -> Option<Direction> {
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

pub(crate) fn player_fire_pressed(keys: &ButtonInput<KeyCode>, player: PlayerId) -> bool {
    match player {
        PlayerId::One => keys.pressed(KeyCode::Space),
        PlayerId::Two => keys.pressed(KeyCode::Enter) || keys.pressed(KeyCode::ShiftRight),
    }
}
