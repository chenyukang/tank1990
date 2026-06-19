use crate::*;

#[derive(Component)]
pub(crate) struct StatusGlyph {
    kind: StatusValue,
    digit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StatusValue {
    Score,
    Lives,
    Stage,
    P2Score,
    P2Lives,
    Arena,
    Target,
}

#[derive(Component)]
pub(crate) struct EnemyMarker {
    index: usize,
}

pub(crate) fn spawn_screen_frame(commands: &mut Commands, assets: &SpriteAssets, mode: GameMode) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(80, 80, 72),
            Vec2::new(
                STATUS_PANEL_WIDTH * window_scale(),
                STATUS_PANEL_HEIGHT * window_scale(),
            ),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(STATUS_PANEL_LEFT, STATUS_PANEL_TOP),
            Vec2::new(STATUS_PANEL_WIDTH, STATUS_PANEL_HEIGHT),
            0.0,
        )),
        GameEntity,
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(36, 36, 32),
            Vec2::new(
                STATUS_PANEL_INNER_WIDTH * window_scale(),
                STATUS_PANEL_INNER_HEIGHT * window_scale(),
            ),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(STATUS_PANEL_INNER_LEFT, STATUS_PANEL_INNER_TOP),
            Vec2::new(STATUS_PANEL_INNER_WIDTH, STATUS_PANEL_INNER_HEIGHT),
            0.1,
        )),
        GameEntity,
    ));

    match mode {
        GameMode::Campaign => spawn_campaign_status_panel(commands, assets, false),
        GameMode::CoopCampaign => spawn_campaign_status_panel(commands, assets, true),
        GameMode::VersusDeathmatch => spawn_versus_status_panel(commands, assets, true),
        GameMode::VersusBaseBattle => spawn_versus_status_panel(commands, assets, false),
    }
}

fn spawn_campaign_status_panel(commands: &mut Commands, assets: &SpriteAssets, show_p2: bool) {
    spawn_pixel_text(
        commands,
        assets,
        "P1",
        status_panel_top_left(6.0, 26.0),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "SCORE",
        status_panel_top_left(6.0, 38.0),
        0.3,
    );
    spawn_score_badge_icon(commands, assets);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Score,
        6,
        status_panel_top_left(6.0, 49.0),
        0.3,
    );

    spawn_pixel_text(
        commands,
        assets,
        "STAGE",
        status_panel_top_left(6.0, 76.0),
        0.3,
    );
    spawn_stage_flag_icon(commands, assets);
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Stage,
        2,
        stage_number_top_left(),
        0.3,
    );

    spawn_pixel_text(
        commands,
        assets,
        "LIFE",
        status_panel_top_left(6.0, 112.0),
        0.3,
    );
    spawn_player_life_icon(
        commands,
        assets,
        PlayerId::One,
        campaign_life_icon_top_left(PlayerId::One),
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Lives,
        1,
        status_panel_top_left(26.0, 123.0),
        0.3,
    );
    if show_p2 {
        spawn_player_life_icon(
            commands,
            assets,
            PlayerId::Two,
            campaign_life_icon_top_left(PlayerId::Two),
        );
        spawn_status_digits(
            commands,
            assets,
            StatusValue::P2Lives,
            1,
            status_panel_top_left(26.0, 135.0),
            0.3,
        );
    }

    spawn_pixel_text(
        commands,
        assets,
        "ENEMY",
        status_panel_top_left(6.0, 148.0),
        0.3,
    );
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
    spawn_pixel_text(
        commands,
        assets,
        "P1",
        status_panel_top_left(6.0, 26.0),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "SCORE",
        status_panel_top_left(6.0, 38.0),
        0.3,
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::Score,
        2,
        status_panel_top_left(18.0, 49.0),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "LIFE",
        status_panel_top_left(6.0, 62.0),
        0.3,
    );
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
        status_panel_top_left(26.0, 73.0),
        0.3,
    );

    spawn_pixel_text(
        commands,
        assets,
        "P2",
        status_panel_top_left(6.0, 98.0),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "SCORE",
        status_panel_top_left(6.0, 110.0),
        0.3,
    );
    spawn_status_digits(
        commands,
        assets,
        StatusValue::P2Score,
        2,
        status_panel_top_left(18.0, 121.0),
        0.3,
    );
    spawn_pixel_text(
        commands,
        assets,
        "LIFE",
        status_panel_top_left(6.0, 134.0),
        0.3,
    );
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
        status_panel_top_left(26.0, 145.0),
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

pub(crate) fn update_status_panel(
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

pub(crate) fn status_value_text(
    kind: StatusValue,
    mode: GameMode,
    game_status: &GameStatus,
    score_board: &ScoreBoard,
) -> String {
    match kind {
        StatusValue::Score => match mode {
            GameMode::Campaign | GameMode::CoopCampaign => {
                format!("{:06}", score_board.score.min(999_999))
            }
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
                format!("{:02}", score_board.p1_score.min(99))
            }
        },
        StatusValue::Lives => match mode {
            GameMode::Campaign => format!("{}", score_board.lives.clamp(0, MAX_PLAYER_LIVES)),
            GameMode::CoopCampaign | GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
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

pub(crate) fn enemy_markers_remaining(total_enemies: usize, enemies_destroyed: usize) -> usize {
    total_enemies.saturating_sub(enemies_destroyed)
}

pub(crate) fn enemy_marker_tank_index(manifest: &AssetManifest) -> usize {
    animated_tank_sprite_index(manifest, TankSpriteSet::EnemyBasic, Direction::Down, 0)
}

pub(crate) fn campaign_life_icon_top_left(player: PlayerId) -> Vec2 {
    match player {
        PlayerId::One => status_panel_top_left(14.0, 123.0),
        PlayerId::Two => status_panel_top_left(14.0, 135.0),
    }
}

pub(crate) fn versus_life_icon_top_left(player: PlayerId) -> Vec2 {
    match player {
        PlayerId::One => status_panel_top_left(14.0, 73.0),
        PlayerId::Two => status_panel_top_left(14.0, 145.0),
    }
}

pub(crate) fn versus_arena_label_top_left() -> Vec2 {
    status_panel_top_left(6.0, 158.0)
}

pub(crate) fn versus_arena_number_top_left() -> Vec2 {
    status_panel_top_left(18.0, 169.0)
}

pub(crate) fn versus_target_label_top_left() -> Vec2 {
    status_panel_top_left(6.0, 184.0)
}

pub(crate) fn versus_target_number_top_left() -> Vec2 {
    status_panel_top_left(18.0, 195.0)
}

pub(crate) fn versus_base_label_top_left() -> Vec2 {
    status_panel_top_left(12.0, 190.0)
}

pub(crate) fn player_life_icon_tank_index(manifest: &AssetManifest, player: PlayerId) -> usize {
    animated_tank_sprite_index(manifest, TankSpriteSet::player(player), Direction::Up, 0)
}

pub(crate) fn score_badge_icon_top_left() -> Vec2 {
    status_panel_top_left(36.0, 38.0)
}

pub(crate) fn stage_flag_icon_top_left() -> Vec2 {
    status_panel_top_left(8.0, 87.0)
}

pub(crate) fn stage_number_top_left() -> Vec2 {
    status_panel_top_left(22.0, 87.0)
}
