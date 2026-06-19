use crate::*;

#[derive(Component)]
pub(crate) struct ModeSelectCursor;

#[derive(Component)]
pub(crate) struct ModeSelectStageGlyph {
    pub(crate) digit: usize,
}

#[derive(Component)]
pub(crate) struct ModeSelectArenaGlyph {
    pub(crate) digit: usize,
}

#[derive(Component)]
pub(crate) struct ModeSelectBattleKindGlyph {
    pub(crate) digit: usize,
}

#[derive(Component)]
pub(crate) struct ModeSelectMusicGlyph {
    pub(crate) digit: usize,
}

#[derive(Component)]
pub(crate) struct ModeSelectSoundGlyph {
    pub(crate) digit: usize,
}

pub(crate) struct ModeSelectDisplay {
    selected: ModeSelectOption,
    map_pack: CampaignMapPack,
    stage: usize,
    arena: usize,
    view_mode: TankViewMode,
    view_assist: bool,
    ai_strategy: ModeSelectAiStrategy,
    difficulty_profile: ModeSelectDifficultyProfile,
    audio_mode: AudioMode,
    sound_enabled: bool,
    window_scale: u32,
}

impl ModeSelectDisplay {
    pub(crate) fn from_mode_select(mode_select: &ModeSelect) -> Self {
        Self {
            selected: mode_select.selected,
            map_pack: mode_select.map_pack,
            stage: mode_select.stage,
            arena: mode_select.arena,
            view_mode: mode_select.view_mode,
            view_assist: mode_select.view_assist,
            ai_strategy: mode_select.ai_strategy,
            difficulty_profile: mode_select.difficulty_profile,
            audio_mode: mode_select.audio_mode,
            sound_enabled: mode_select.sound_enabled,
            window_scale: mode_select.window_scale,
        }
    }
}

pub(crate) fn spawn_mode_select_screen(
    commands: &mut Commands,
    assets: &SpriteAssets,
    mode_select: &ModeSelect,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb_u8(16, 16, 14),
            Vec2::new(208.0 * window_scale(), 208.0 * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(
            Vec2::new(MODE_SELECT_LEFT, 16.0),
            Vec2::new(208.0, 208.0),
            0.0,
        )),
        GameEntity,
    ));

    spawn_pixel_text(
        commands,
        assets,
        "TANK 1990",
        mode_select_centered_top_left("TANK 1990", MODE_SELECT_TITLE_Y),
        0.3,
    );
    let display = ModeSelectDisplay::from_mode_select(mode_select);
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
    ] {
        let text = mode_select_option_text(&display, option);
        spawn_pixel_text(
            commands,
            assets,
            &text,
            mode_select_option_top_left(&display, option),
            0.3,
        );
    }
    let stage_top_left = mode_select_option_top_left(&display, ModeSelectOption::Stage);
    spawn_pixel_text(commands, assets, "STAGE", stage_top_left, 0.3);
    spawn_mode_select_stage_digits(
        commands,
        assets,
        mode_select.stage,
        Vec2::new(stage_top_left.x + GLYPH_ADVANCE * 6.0, stage_top_left.y),
        0.3,
    );
    let arena_top_left = mode_select_option_top_left(&display, ModeSelectOption::Arena);
    spawn_pixel_text(commands, assets, "ARENA", arena_top_left, 0.3);
    spawn_mode_select_arena_digits(
        commands,
        assets,
        mode_select.arena,
        Vec2::new(arena_top_left.x + GLYPH_ADVANCE * 6.0, arena_top_left.y),
        0.3,
    );
    spawn_mode_select_battle_kind(
        commands,
        assets,
        mode_select.arena,
        Vec2::new(arena_top_left.x + GLYPH_ADVANCE * 9.0, arena_top_left.y),
        0.3,
    );
    spawn_mode_select_hints(commands, assets);
    spawn_mode_select_cursor(commands, assets, &display);
}

fn spawn_mode_select_hints(commands: &mut Commands, assets: &SpriteAssets) {
    for (index, line) in MODE_SELECT_HINT_LINES.iter().enumerate() {
        let text_width = phase_text_width(line);
        spawn_pixel_text(
            commands,
            assets,
            line,
            Vec2::new(
                MODE_SELECT_LEFT + (MODE_SELECT_WIDTH - text_width) / 2.0,
                MODE_SELECT_HINT_TOP + index as f32 * 9.0,
            ),
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

fn spawn_mode_select_cursor(
    commands: &mut Commands,
    assets: &SpriteAssets,
    display: &ModeSelectDisplay,
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
        Transform::from_translation(mode_select_cursor_translation(display))
            .with_scale(Vec3::splat(window_scale())),
        ModeSelectCursor,
        GameEntity,
    ));
}

pub(crate) fn mode_select_ai_strategy_label(strategy: ModeSelectAiStrategy) -> &'static str {
    match strategy {
        ModeSelectAiStrategy::Auto => "AUTO",
        ModeSelectAiStrategy::Classic => "CLASSIC",
        ModeSelectAiStrategy::PathToObjective => "PATH",
    }
}

pub(crate) fn mode_select_difficulty_profile_label(
    profile: ModeSelectDifficultyProfile,
) -> &'static str {
    match profile {
        ModeSelectDifficultyProfile::Easy => "EASY",
        ModeSelectDifficultyProfile::Auto => "AUTO",
        ModeSelectDifficultyProfile::Normal => "NORMAL",
        ModeSelectDifficultyProfile::Hard => "HARD",
    }
}

pub(crate) fn update_mode_select_stage_digits(
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

pub(crate) fn mode_select_option_text(
    display: &ModeSelectDisplay,
    option: ModeSelectOption,
) -> String {
    match option {
        ModeSelectOption::Campaign => "1 PLAYER".to_string(),
        ModeSelectOption::CoopCampaign => "2 PLAYERS".to_string(),
        ModeSelectOption::Battle => "BATTLE".to_string(),
        ModeSelectOption::MapPack => {
            format!("MAP {}", campaign_map_pack_label(display.map_pack))
        }
        ModeSelectOption::ViewMode => format!("VIEW {}", tank_view_mode_label(display.view_mode)),
        ModeSelectOption::ViewAssist => {
            format!("ASSIST {}", view_assist_label(display.view_assist))
        }
        ModeSelectOption::AiStrategy => {
            format!("AI {}", mode_select_ai_strategy_label(display.ai_strategy))
        }
        ModeSelectOption::Difficulty => {
            format!(
                "DIFF {}",
                mode_select_difficulty_profile_label(display.difficulty_profile)
            )
        }
        ModeSelectOption::Music => format!("MUSIC {}", audio_mode_label(display.audio_mode)),
        ModeSelectOption::Sound => format!("SOUND {}", sound_enabled_label(display.sound_enabled)),
        ModeSelectOption::Scale => format!("SCALE {}", window_scale_label(display.window_scale)),
        ModeSelectOption::Stage => format!("STAGE {:02}", display.stage.min(99)),
        ModeSelectOption::Arena => {
            format!(
                "ARENA {:02} {}",
                display.arena.min(99),
                battle_kind_label_for_arena(display.arena)
            )
        }
    }
}

pub(crate) fn mode_select_option_top_left(
    display: &ModeSelectDisplay,
    option: ModeSelectOption,
) -> Vec2 {
    mode_select_centered_top_left(
        &mode_select_option_text(display, option),
        mode_select_option_y(option),
    )
}

pub(crate) fn mode_select_option_y(option: ModeSelectOption) -> f32 {
    match option {
        ModeSelectOption::Campaign => 46.0,
        ModeSelectOption::CoopCampaign => 57.0,
        ModeSelectOption::Battle => 68.0,
        ModeSelectOption::MapPack => 87.0,
        ModeSelectOption::ViewMode => 98.0,
        ModeSelectOption::ViewAssist => 109.0,
        ModeSelectOption::AiStrategy => 120.0,
        ModeSelectOption::Difficulty => 131.0,
        ModeSelectOption::Music => 142.0,
        ModeSelectOption::Sound => 153.0,
        ModeSelectOption::Scale => 164.0,
        ModeSelectOption::Stage => 178.0,
        ModeSelectOption::Arena => 189.0,
    }
}

pub(crate) fn mode_select_cursor_translation(display: &ModeSelectDisplay) -> Vec3 {
    let option = mode_select_option_top_left(display, display.selected);
    virtual_center_scaled(
        Vec2::new((option.x - MODE_SELECT_CURSOR_GAP).max(0.0), option.y - 4.0),
        Vec2::splat(TANK_SIZE),
        0.3,
    )
}

pub(crate) fn update_mode_select_cursor(
    cursors: &mut Query<&mut Transform, With<ModeSelectCursor>>,
    display: &ModeSelectDisplay,
) {
    let translation = mode_select_cursor_translation(display);
    for mut transform in cursors {
        transform.translation = translation;
    }
}

pub(crate) fn update_mode_select_arena_digits(
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

pub(crate) fn update_mode_select_battle_kind(
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

pub(crate) fn update_mode_select_music_value(
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

pub(crate) fn update_mode_select_sound_value(
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
