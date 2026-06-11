use super::*;

pub(super) fn atlas_tile_size(manifest: GeneratedAtlasManifest) -> UVec2 {
    UVec2::new(manifest.tile_width as u32, manifest.tile_height as u32)
}

pub(super) fn personal_asset_disk_path(asset_path: &str) -> PathBuf {
    Path::new(ASSET_ROOT_DIR).join(asset_path)
}

pub(super) fn personal_asset_exists(asset_path: &str) -> bool {
    personal_asset_disk_path(asset_path).is_file()
}

pub(super) fn image_handle_or_generated(
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

pub(super) fn create_sprite_assets(
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
    let shield_image =
        image_handle_or_generated(asset_server, images, PERSONAL_SHIELD_PATH, || {
            create_shield_image()
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
        shield_image,
    }
}

pub(super) fn sound_handle_or_generated(
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

pub(super) fn create_sound_assets(
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
        generated_background_music: SoundHandle::Retro(sounds.add(make_background_music_sound())),
        custom_background_music: custom_background_music_handle(asset_server),
    }
}

pub(super) fn personal_background_music_path_if_available(
    exists: impl Fn(&str) -> bool,
) -> Option<&'static str> {
    exists(PERSONAL_BACKGROUND_MUSIC_SOUND_PATH).then_some(PERSONAL_BACKGROUND_MUSIC_SOUND_PATH)
}

pub(super) fn custom_background_music_handle(asset_server: &AssetServer) -> Option<SoundHandle> {
    personal_background_music_path_if_available(personal_asset_exists)
        .map(|path| SoundHandle::File(asset_server.load(path)))
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub(super) struct SoundNote {
    pub(super) duration_secs: f32,
    pub(super) frequency: f32,
    pub(super) volume: f32,
}

pub(super) fn play_sound(commands: &mut Commands, sounds: &SoundAssets, kind: SoundKind) {
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

pub(super) fn sync_background_music(
    mut commands: Commands,
    mode_select: Res<ModeSelect>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    music: Query<(Entity, &BackgroundMusic)>,
) {
    let should_play = background_music_should_play(
        mode_select.audio_mode,
        game_status.phase,
        custom_background_music_available(&sounds),
    );
    if should_play {
        let mut matching_music_is_playing = false;
        for (entity, background_music) in &music {
            if background_music.mode == mode_select.audio_mode {
                matching_music_is_playing = true;
            } else {
                commands.entity(entity).despawn();
            }
        }
        if !matching_music_is_playing {
            play_background_music(&mut commands, &sounds, mode_select.audio_mode);
        }
        return;
    }

    for (entity, _) in &music {
        commands.entity(entity).despawn();
    }
}

pub(super) fn background_music_should_play(
    mode: AudioMode,
    phase: GamePhase,
    custom_available: bool,
) -> bool {
    matches!(phase, GamePhase::StageIntro | GamePhase::Playing)
        && background_music_mode_has_loop(mode, custom_available)
}

pub(super) fn background_music_mode_has_loop(mode: AudioMode, custom_available: bool) -> bool {
    match mode {
        AudioMode::Bgm => true,
        AudioMode::Custom => custom_available,
        AudioMode::Classic => false,
    }
}

pub(super) fn play_background_music(
    commands: &mut Commands,
    sounds: &SoundAssets,
    mode: AudioMode,
) {
    let playback = PlaybackSettings::LOOP.with_volume(Volume::Linear(BACKGROUND_MUSIC_VOLUME));
    let Some(handle) = background_music_handle_for_mode(sounds, mode) else {
        return;
    };
    match handle {
        SoundHandle::Retro(handle) => {
            commands.spawn((
                AudioPlayer(handle.clone()),
                playback,
                BackgroundMusic { mode },
            ));
        }
        SoundHandle::File(handle) => {
            commands.spawn((
                AudioPlayer(handle.clone()),
                playback,
                BackgroundMusic { mode },
            ));
        }
    }
}

pub(super) fn make_manifest_sound(spec: &RetroSoundSpec) -> RetroSound {
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

pub(super) fn make_sweep_sound(
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

pub(super) fn make_noise_sound(duration_secs: f32, volume: f32, seed: u32) -> RetroSound {
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

pub(super) fn make_layered_sound(notes: &[SoundNote]) -> RetroSound {
    let mut samples = Vec::new();
    for note in notes {
        append_square_note(&mut samples, *note);
    }
    sound_from_samples(samples)
}

pub(super) fn make_background_music_sound() -> RetroSound {
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

pub(super) fn append_background_music_step(
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

pub(super) fn background_music_percussion(step: u32, t: f32, noise_wave: f32) -> f32 {
    let decay = (1.0 - t).clamp(0.0, 1.0).powi(4);
    if step.is_multiple_of(4) {
        noise_wave * 0.055 * decay
    } else if step % 4 == 2 {
        noise_wave * 0.035 * decay
    } else {
        0.0
    }
}

pub(super) fn music_gate_envelope(t: f32) -> f32 {
    let attack = (t / 0.02).clamp(0.0, 1.0);
    let release = ((1.0 - t) / 0.10).clamp(0.0, 1.0);
    attack.min(release)
}

pub(super) fn append_square_note(samples: &mut Vec<f32>, note: SoundNote) {
    let sample_count = sample_count(note.duration_secs);
    let mut phase = 0.0_f32;
    for index in 0..sample_count {
        let t = index as f32 / sample_count as f32;
        phase = (phase + note.frequency / SOUND_SAMPLE_RATE as f32) % 1.0;
        let wave = if phase < 0.5 { 1.0 } else { -1.0 };
        samples.push(wave * note.volume * decay_envelope(t));
    }
}

pub(super) fn sample_count(duration_secs: f32) -> usize {
    (duration_secs * SOUND_SAMPLE_RATE as f32).round().max(1.0) as usize
}

pub(super) fn decay_envelope(t: f32) -> f32 {
    let attack = (t / 0.08).clamp(0.0, 1.0);
    let release = (1.0 - t).clamp(0.0, 1.0);
    attack * release * release
}

pub(super) fn sound_from_samples(samples: Vec<f32>) -> RetroSound {
    RetroSound {
        samples: samples.into(),
        sample_rate: SOUND_SAMPLE_RATE,
    }
}

pub(super) fn create_terrain_atlas(manifest: GeneratedAtlasManifest) -> Image {
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

pub(super) fn draw_brick(pixels: &mut [u8], width: usize, x_offset: usize) {
    let mortar = [48, 24, 16, 255];
    let brick_dark = [112, 44, 24, 255];
    let brick_mid = [160, 64, 32, 255];
    let brick_light = [200, 96, 48, 255];

    fill_rect(pixels, width, x_offset, 0, 8, 8, mortar);
    fill_rect(pixels, width, x_offset, 0, 3, 2, brick_mid);
    fill_rect(pixels, width, x_offset + 4, 0, 4, 2, brick_mid);
    fill_rect(pixels, width, x_offset, 3, 5, 2, brick_dark);
    fill_rect(pixels, width, x_offset + 6, 3, 2, 2, brick_mid);
    fill_rect(pixels, width, x_offset, 6, 3, 2, brick_mid);
    fill_rect(pixels, width, x_offset + 4, 6, 4, 2, brick_dark);
    fill_rect(pixels, width, x_offset, 0, 3, 1, brick_light);
    fill_rect(pixels, width, x_offset + 4, 0, 4, 1, brick_light);
    fill_rect(pixels, width, x_offset, 3, 5, 1, brick_light);
    fill_rect(pixels, width, x_offset + 6, 3, 2, 1, brick_light);
    fill_rect(pixels, width, x_offset, 6, 3, 1, brick_light);
}

pub(super) fn draw_steel(pixels: &mut [u8], width: usize, x_offset: usize) {
    let mid = [104, 112, 120, 255];
    let light = [208, 216, 216, 255];
    let shadow = [40, 48, 56, 255];
    let rivet = [72, 80, 88, 255];

    fill_rect(pixels, width, x_offset, 0, 8, 8, mid);
    fill_rect(pixels, width, x_offset, 0, 8, 1, light);
    fill_rect(pixels, width, x_offset, 0, 1, 8, light);
    fill_rect(pixels, width, x_offset + 7, 0, 1, 8, shadow);
    fill_rect(pixels, width, x_offset, 7, 8, 1, shadow);
    fill_rect(pixels, width, x_offset + 2, 2, 4, 1, [144, 152, 160, 255]);
    fill_rect(pixels, width, x_offset + 2, 5, 4, 1, shadow);
    for (x, y) in [(2, 2), (5, 2), (2, 5), (5, 5)] {
        set_pixel(pixels, width, x_offset + x, y, rivet);
    }
}

pub(super) fn draw_water(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let deep = [16, 48, 120, 255];
    let mid = [32, 88, 168, 255];
    let foam = [104, 176, 232, 255];
    let shadow = [8, 32, 88, 255];

    fill_rect(pixels, width, x_offset, 0, 8, 8, deep);
    for y in [1, 4, 6] {
        let offset = (frame + y) % 3;
        for x in 0..8 {
            if (x + offset).is_multiple_of(3) {
                set_pixel(pixels, width, x_offset + x, y, foam);
            } else if (x + offset).is_multiple_of(2) {
                set_pixel(pixels, width, x_offset + x, y, mid);
            }
        }
    }
    fill_rect(pixels, width, x_offset, 7, 8, 1, shadow);
}

pub(super) fn draw_forest(pixels: &mut [u8], width: usize, x_offset: usize) {
    let dark = [16, 72, 32, 225];
    let mid = [32, 120, 48, 235];
    let light = [88, 176, 80, 240];

    fill_rect(pixels, width, x_offset, 0, 8, 8, dark);
    for (x, y) in [(1, 0), (4, 0), (6, 1), (0, 3), (3, 3), (5, 5), (2, 6)] {
        fill_rect(pixels, width, x_offset + x, y, 2, 2, mid);
    }
    for (x, y) in [(1, 1), (5, 1), (3, 4), (6, 6)] {
        set_pixel(pixels, width, x_offset + x, y, light);
    }
}

pub(super) fn draw_ice(pixels: &mut [u8], width: usize, x_offset: usize) {
    let base = [120, 184, 216, 255];
    let light = [224, 248, 255, 255];
    let mid = [160, 216, 232, 255];
    let crack = [64, 128, 176, 255];

    fill_rect(pixels, width, x_offset, 0, 8, 8, base);
    fill_rect(pixels, width, x_offset, 0, 8, 1, light);
    fill_rect(pixels, width, x_offset, 0, 1, 8, light);
    for (x, y) in [(2, 2), (3, 2), (4, 3), (5, 4), (2, 5), (1, 6)] {
        set_pixel(pixels, width, x_offset + x, y, crack);
    }
    for (x, y) in [(5, 1), (6, 2), (1, 4), (4, 6)] {
        set_pixel(pixels, width, x_offset + x, y, mid);
    }
}

pub(super) fn create_tank_atlas(manifest: GeneratedAtlasManifest) -> Image {
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
pub(super) struct TankPalette {
    dark: [u8; 4],
    body: [u8; 4],
    light: [u8; 4],
    tread: [u8; 4],
}

pub(super) fn draw_tank(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    direction: Direction,
    palette: TankPalette,
    frame: usize,
) {
    match direction {
        Direction::Up | Direction::Down => {
            draw_vertical_tank(pixels, width, x_offset, direction, palette, frame)
        }
        Direction::Left | Direction::Right => {
            draw_horizontal_tank(pixels, width, x_offset, direction, palette, frame)
        }
    }
}

pub(super) fn draw_vertical_tank(
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
        Direction::Left | Direction::Right => {}
    }
}

pub(super) fn draw_horizontal_tank(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    direction: Direction,
    palette: TankPalette,
    frame: usize,
) {
    fill_rect(pixels, width, x_offset + 2, 2, 12, 4, palette.tread);
    fill_rect(pixels, width, x_offset + 2, 10, 12, 4, palette.tread);
    for x in [3 + frame % 2, 7 + frame % 2, 11 + frame % 2] {
        fill_rect(pixels, width, x_offset + x, 2, 1, 4, palette.dark);
        fill_rect(pixels, width, x_offset + x, 10, 1, 4, palette.dark);
    }
    fill_rect(pixels, width, x_offset + 4, 4, 8, 8, palette.body);
    fill_rect(pixels, width, x_offset + 6, 6, 4, 4, palette.light);

    match direction {
        Direction::Left => {
            fill_rect(pixels, width, x_offset, 7, 7, 2, palette.light);
            fill_rect(pixels, width, x_offset + 11, 4, 1, 8, palette.dark);
        }
        Direction::Right => {
            fill_rect(pixels, width, x_offset + 9, 7, 7, 2, palette.light);
            fill_rect(pixels, width, x_offset + 4, 4, 1, 8, palette.dark);
        }
        Direction::Up | Direction::Down => {}
    }
}

pub(super) fn draw_tank_group(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    palette: TankPalette,
) {
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

pub(super) fn create_bullet_atlas(manifest: GeneratedAtlasManifest) -> Image {
    let atlas_width = manifest.tile_width * manifest.tiles;
    let mut pixels = vec![0; atlas_width * manifest.tile_height * 4];
    draw_bullet(&mut pixels, atlas_width, 0, Direction::Up);
    draw_bullet(
        &mut pixels,
        atlas_width,
        manifest.tile_width,
        Direction::Down,
    );
    draw_bullet(
        &mut pixels,
        atlas_width,
        manifest.tile_width * 2,
        Direction::Left,
    );
    draw_bullet(
        &mut pixels,
        atlas_width,
        manifest.tile_width * 3,
        Direction::Right,
    );
    image_from_pixels(atlas_width, manifest.tile_height, pixels)
}

pub(super) fn draw_bullet(pixels: &mut [u8], width: usize, x_offset: usize, direction: Direction) {
    let light = [248, 248, 216, 255];
    let mid = [208, 184, 96, 255];
    let dark = [128, 112, 64, 255];

    match direction {
        Direction::Up => {
            fill_rect(pixels, width, x_offset + 1, 0, 2, 1, light);
            fill_rect(pixels, width, x_offset + 1, 1, 2, 2, mid);
            fill_rect(pixels, width, x_offset + 1, 3, 2, 1, dark);
        }
        Direction::Down => {
            fill_rect(pixels, width, x_offset + 1, 0, 2, 1, dark);
            fill_rect(pixels, width, x_offset + 1, 1, 2, 2, mid);
            fill_rect(pixels, width, x_offset + 1, 3, 2, 1, light);
        }
        Direction::Left => {
            fill_rect(pixels, width, x_offset, 1, 1, 2, light);
            fill_rect(pixels, width, x_offset + 1, 1, 2, 2, mid);
            fill_rect(pixels, width, x_offset + 3, 1, 1, 2, dark);
        }
        Direction::Right => {
            fill_rect(pixels, width, x_offset, 1, 1, 2, dark);
            fill_rect(pixels, width, x_offset + 1, 1, 2, 2, mid);
            fill_rect(pixels, width, x_offset + 3, 1, 1, 2, light);
        }
    }
}

pub(super) fn create_effect_atlas(manifest: GeneratedAtlasManifest) -> Image {
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

pub(super) fn draw_explosion_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let core = [255, 248, 184, 255];
    let flame = [248, 184, 64, 255];
    let ember = [232, 80, 40, 240];
    let smoke = [88, 72, 64, 190];

    match frame {
        0 => {
            fill_rect(pixels, width, x_offset + 7, 7, 2, 2, core);
            for (x, y) in [(8, 4), (4, 8), (11, 8), (8, 11)] {
                set_pixel(pixels, width, x_offset + x, y, flame);
            }
        }
        1 => {
            fill_rect(pixels, width, x_offset + 6, 6, 4, 4, core);
            fill_rect(pixels, width, x_offset + 5, 7, 6, 2, flame);
            fill_rect(pixels, width, x_offset + 7, 5, 2, 6, flame);
            for (x, y) in [(4, 6), (11, 6), (4, 10), (11, 10), (8, 3), (8, 12)] {
                set_pixel(pixels, width, x_offset + x, y, ember);
            }
        }
        2 => {
            fill_rect(pixels, width, x_offset + 4, 6, 8, 5, flame);
            fill_rect(pixels, width, x_offset + 6, 4, 4, 9, ember);
            fill_rect(pixels, width, x_offset + 7, 7, 2, 2, core);
            for (x, y) in [(3, 8), (12, 8), (8, 3), (8, 13), (5, 5), (11, 11)] {
                set_pixel(pixels, width, x_offset + x, y, smoke);
            }
        }
        _ => {
            for (x, y) in [(4, 6), (8, 4), (11, 6), (3, 10), (7, 11), (12, 10), (9, 13)] {
                set_pixel(pixels, width, x_offset + x, y, smoke);
            }
            fill_rect(pixels, width, x_offset + 6, 8, 4, 3, [104, 80, 56, 170]);
            set_pixel(pixels, width, x_offset + 8, 8, [184, 72, 40, 180]);
        }
    }
}

pub(super) fn draw_spawn_frame(pixels: &mut [u8], width: usize, x_offset: usize, frame: usize) {
    let color = if frame.is_multiple_of(2) {
        [144, 224, 255, 235]
    } else {
        [80, 176, 240, 225]
    };
    let accent = [232, 248, 255, 245];
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
    set_pixel(pixels, width, x_offset + 7, 7, accent);
    set_pixel(pixels, width, x_offset + 8, 8, accent);
    set_pixel(pixels, width, x_offset + inset, inset, accent);
    set_pixel(pixels, width, x_offset + 15 - inset, 15 - inset, accent);
}

pub(super) fn draw_base_destruction_frame(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    frame: usize,
) {
    match frame {
        0 => {
            fill_rect(pixels, width, x_offset + 4, 5, 8, 7, [224, 160, 72, 255]);
            fill_rect(pixels, width, x_offset + 6, 3, 4, 6, [248, 232, 128, 255]);
            fill_rect(pixels, width, x_offset + 3, 12, 10, 2, [80, 56, 40, 255]);
            set_pixel(pixels, width, x_offset + 7, 2, [255, 248, 184, 255]);
        }
        1 => {
            fill_rect(pixels, width, x_offset + 2, 6, 12, 6, [232, 96, 40, 255]);
            fill_rect(pixels, width, x_offset + 5, 2, 6, 9, [248, 200, 72, 255]);
            fill_rect(pixels, width, x_offset + 1, 12, 14, 3, [72, 56, 48, 255]);
            set_pixel(pixels, width, x_offset + 3, 4, [255, 248, 184, 230]);
            set_pixel(pixels, width, x_offset + 12, 5, [184, 72, 40, 230]);
        }
        2 => {
            fill_rect(pixels, width, x_offset + 3, 4, 10, 8, [104, 88, 80, 220]);
            fill_rect(pixels, width, x_offset + 5, 7, 7, 5, [184, 72, 40, 240]);
            fill_rect(pixels, width, x_offset + 2, 12, 12, 3, [48, 40, 32, 255]);
            set_pixel(pixels, width, x_offset + 4, 3, [88, 72, 64, 180]);
            set_pixel(pixels, width, x_offset + 11, 4, [88, 72, 64, 180]);
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

pub(super) fn draw_powerup_sparkle_frame(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    frame: usize,
) {
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
    if frame.is_multiple_of(2) {
        fill_rect(pixels, width, x_offset + 7, 7, 2, 2, color);
    } else {
        set_pixel(pixels, width, x_offset + 8, 8, color);
    }
}

pub(super) fn draw_bullet_impact_frame(
    pixels: &mut [u8],
    width: usize,
    x_offset: usize,
    frame: usize,
) {
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
            set_pixel(pixels, width, x_offset + 12, 8, [248, 216, 96, 220]);
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

pub(super) fn create_powerup_atlas(manifest: GeneratedAtlasManifest) -> Image {
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

pub(super) fn draw_star_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let shadow = [128, 88, 24, 255];
    let gold = [248, 216, 72, 255];
    let light = [255, 248, 160, 255];

    for (x, y, w) in [(8, 2, 1), (7, 4, 3), (4, 6, 9), (6, 8, 5)] {
        fill_rect(pixels, width, x_offset + x, y + 1, w, 1, shadow);
        fill_rect(pixels, width, x_offset + x, y, w, 1, gold);
    }
    fill_rect(pixels, width, x_offset + 5, 10, 3, 2, gold);
    fill_rect(pixels, width, x_offset + 10, 10, 3, 2, shadow);
    fill_rect(pixels, width, x_offset + 7, 4, 2, 1, light);
    fill_rect(pixels, width, x_offset + 6, 6, 3, 1, light);
    set_pixel(pixels, width, x_offset + 8, 2, light);
}

pub(super) fn draw_helmet_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let dark = [32, 96, 144, 255];
    let mid = [80, 184, 216, 255];
    let light = [176, 240, 248, 255];
    let visor = [216, 248, 248, 255];

    fill_rect(pixels, width, x_offset + 5, 3, 6, 2, light);
    fill_rect(pixels, width, x_offset + 4, 5, 8, 5, mid);
    fill_rect(pixels, width, x_offset + 3, 9, 10, 2, dark);
    fill_rect(pixels, width, x_offset + 5, 6, 6, 2, visor);
    fill_rect(pixels, width, x_offset + 7, 4, 2, 5, light);
    set_pixel(pixels, width, x_offset + 4, 4, dark);
    set_pixel(pixels, width, x_offset + 11, 4, dark);
}

pub(super) fn draw_clock_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let rim_light = [216, 232, 248, 255];
    let rim_dark = [72, 120, 184, 255];
    let face = [248, 248, 208, 255];
    let hand = [48, 64, 96, 255];

    fill_rect(pixels, width, x_offset + 5, 2, 6, 1, rim_light);
    fill_rect(pixels, width, x_offset + 3, 4, 2, 8, rim_light);
    fill_rect(pixels, width, x_offset + 11, 4, 2, 8, rim_dark);
    fill_rect(pixels, width, x_offset + 5, 12, 6, 1, rim_dark);
    fill_rect(pixels, width, x_offset + 5, 4, 6, 8, face);
    fill_rect(pixels, width, x_offset + 7, 6, 2, 4, hand);
    fill_rect(pixels, width, x_offset + 8, 9, 4, 2, hand);
    set_pixel(pixels, width, x_offset + 8, 8, rim_dark);
}

pub(super) fn draw_grenade_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let dark = [40, 72, 32, 255];
    let mid = [80, 128, 48, 255];
    let light = [136, 184, 72, 255];
    let metal = [200, 184, 80, 255];
    let spark = [248, 232, 128, 255];

    fill_rect(pixels, width, x_offset + 6, 3, 4, 2, metal);
    fill_rect(pixels, width, x_offset + 10, 2, 2, 3, metal);
    fill_rect(pixels, width, x_offset + 12, 1, 3, 1, spark);
    fill_rect(pixels, width, x_offset + 5, 5, 8, 8, mid);
    fill_rect(pixels, width, x_offset + 4, 8, 10, 4, mid);
    fill_rect(pixels, width, x_offset + 5, 11, 8, 2, dark);
    fill_rect(pixels, width, x_offset + 6, 6, 2, 2, light);
    fill_rect(pixels, width, x_offset + 9, 6, 1, 7, dark);
    fill_rect(pixels, width, x_offset + 5, 9, 8, 1, dark);
}

pub(super) fn draw_shovel_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let handle_dark = [120, 72, 32, 255];
    let handle = [184, 112, 48, 255];
    let blade = [160, 168, 176, 255];
    let shine = [232, 240, 240, 255];
    let shadow = [72, 88, 96, 255];

    fill_rect(pixels, width, x_offset + 7, 2, 2, 8, handle);
    fill_rect(pixels, width, x_offset + 8, 2, 1, 8, handle_dark);
    fill_rect(pixels, width, x_offset + 5, 9, 6, 2, handle);
    fill_rect(pixels, width, x_offset + 4, 11, 10, 3, blade);
    fill_rect(pixels, width, x_offset + 5, 12, 8, 1, shine);
    fill_rect(pixels, width, x_offset + 6, 14, 6, 1, shadow);
}

pub(super) fn draw_tank_powerup(pixels: &mut [u8], width: usize, x_offset: usize) {
    let tread = [56, 56, 56, 255];
    let body = [216, 72, 64, 255];
    let light = [248, 160, 88, 255];
    let shadow = [128, 40, 40, 255];

    fill_rect(pixels, width, x_offset + 3, 5, 4, 8, tread);
    fill_rect(pixels, width, x_offset + 9, 5, 4, 8, tread);
    fill_rect(pixels, width, x_offset + 5, 6, 6, 6, body);
    fill_rect(pixels, width, x_offset + 7, 3, 2, 6, light);
    fill_rect(pixels, width, x_offset + 6, 8, 4, 3, light);
    fill_rect(pixels, width, x_offset + 5, 11, 6, 1, shadow);
    set_pixel(pixels, width, x_offset + 4, 6, [96, 96, 96, 255]);
    set_pixel(pixels, width, x_offset + 12, 11, [32, 32, 32, 255]);
}

pub(super) fn create_glyph_atlas(manifest: &GlyphManifest) -> Image {
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

pub(super) fn glyph_index(ch: char, manifest: &GlyphManifest) -> usize {
    manifest
        .characters
        .chars()
        .position(|glyph| glyph == ch)
        .unwrap_or(0)
}

pub(super) fn glyph_size(manifest: &GlyphManifest) -> Vec2 {
    Vec2::new(manifest.tile_width as f32, manifest.tile_height as f32)
}

pub(super) fn glyph_pattern_has_pixels(pattern: [&str; GENERATED_GLYPH_HEIGHT]) -> bool {
    pattern.iter().any(|row| row.contains('#'))
}

pub(super) fn glyph_pattern(ch: char) -> [&'static str; 7] {
    match ch {
        ' ' => [
            ".....", ".....", ".....", ".....", ".....", ".....", ".....",
        ],
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

pub(super) fn generated_sprite_size(manifest: GeneratedSpriteManifest) -> Vec2 {
    Vec2::new(manifest.width as f32, manifest.height as f32)
}

pub(super) fn create_base_image(manifest: GeneratedSpriteManifest, destroyed: bool) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    let outline = [56, 48, 32, 255];
    let shadow = [72, 56, 40, 255];
    let gold_dark = [152, 104, 40, 255];
    let gold_mid = [208, 152, 56, 255];
    let gold_light = [248, 216, 104, 255];
    let body = [240, 232, 160, 255];

    if destroyed {
        let rubble = [120, 80, 48, 255];
        let ember = [184, 64, 32, 255];
        let flame = [248, 184, 64, 255];
        let smoke = [80, 72, 64, 255];

        fill_rect(&mut pixels, manifest.width, 7, 2, 2, 2, smoke);
        fill_rect(&mut pixels, manifest.width, 6, 5, 2, 3, ember);
        fill_rect(&mut pixels, manifest.width, 9, 6, 2, 3, ember);
        set_pixel(&mut pixels, manifest.width, 6, 5, flame);
        set_pixel(&mut pixels, manifest.width, 10, 6, flame);
        fill_rect(&mut pixels, manifest.width, 4, 10, 4, 2, rubble);
        fill_rect(&mut pixels, manifest.width, 9, 9, 4, 3, rubble);
        fill_rect(&mut pixels, manifest.width, 6, 11, 5, 1, gold_dark);
        fill_rect(&mut pixels, manifest.width, 3, 12, 10, 2, outline);
        fill_rect(&mut pixels, manifest.width, 2, 13, 12, 1, shadow);
    } else {
        fill_rect(&mut pixels, manifest.width, 7, 2, 2, 1, outline);
        fill_rect(&mut pixels, manifest.width, 7, 3, 2, 2, body);
        set_pixel(&mut pixels, manifest.width, 9, 4, [232, 112, 40, 255]);
        fill_rect(&mut pixels, manifest.width, 6, 6, 4, 1, outline);
        fill_rect(&mut pixels, manifest.width, 7, 6, 2, 5, body);
        fill_rect(&mut pixels, manifest.width, 6, 8, 4, 2, gold_light);
        fill_rect(&mut pixels, manifest.width, 3, 7, 3, 1, gold_dark);
        fill_rect(&mut pixels, manifest.width, 10, 7, 3, 1, gold_dark);
        fill_rect(&mut pixels, manifest.width, 2, 8, 5, 1, gold_mid);
        fill_rect(&mut pixels, manifest.width, 9, 8, 5, 1, gold_mid);
        fill_rect(&mut pixels, manifest.width, 4, 9, 3, 1, gold_light);
        fill_rect(&mut pixels, manifest.width, 9, 9, 3, 1, gold_light);
        set_pixel(&mut pixels, manifest.width, 2, 9, outline);
        set_pixel(&mut pixels, manifest.width, 13, 9, outline);
        fill_rect(&mut pixels, manifest.width, 6, 11, 4, 2, gold_dark);
        fill_rect(&mut pixels, manifest.width, 4, 12, 8, 1, shadow);
        fill_rect(&mut pixels, manifest.width, 3, 13, 10, 1, outline);
    }
    image_from_pixels(manifest.width, manifest.height, pixels)
}

pub(super) fn create_score_badge_icon(manifest: GeneratedSpriteManifest) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    let dark = [96, 64, 32, 255];
    let shadow = [136, 88, 40, 255];
    let gold = [224, 160, 56, 255];
    let light = [255, 232, 128, 255];

    fill_rect(&mut pixels, manifest.width, 2, 0, 4, 1, light);
    fill_rect(&mut pixels, manifest.width, 1, 1, 6, 3, gold);
    set_pixel(&mut pixels, manifest.width, 0, 2, shadow);
    set_pixel(&mut pixels, manifest.width, 7, 2, shadow);
    fill_rect(&mut pixels, manifest.width, 2, 4, 4, 1, shadow);
    fill_rect(&mut pixels, manifest.width, 3, 5, 2, 1, dark);
    fill_rect(&mut pixels, manifest.width, 2, 6, 4, 1, gold);
    fill_rect(&mut pixels, manifest.width, 1, 7, 6, 1, dark);
    fill_rect(&mut pixels, manifest.width, 3, 2, 2, 1, light);
    image_from_pixels(manifest.width, manifest.height, pixels)
}

pub(super) fn create_stage_flag_icon(manifest: GeneratedSpriteManifest) -> Image {
    let mut pixels = vec![0; manifest.width * manifest.height * 4];
    let pole = [232, 232, 208, 255];
    let pole_shadow = [120, 120, 96, 255];
    let flag_light = [255, 232, 96, 255];
    let flag_mid = [232, 168, 48, 255];
    let flag_dark = [160, 96, 32, 255];

    fill_rect(&mut pixels, manifest.width, 1, 0, 1, 7, pole);
    set_pixel(&mut pixels, manifest.width, 2, 6, pole_shadow);
    fill_rect(&mut pixels, manifest.width, 2, 1, 5, 1, flag_light);
    fill_rect(&mut pixels, manifest.width, 2, 2, 5, 2, flag_mid);
    fill_rect(&mut pixels, manifest.width, 2, 4, 3, 1, flag_dark);
    set_pixel(&mut pixels, manifest.width, 6, 3, flag_light);
    fill_rect(&mut pixels, manifest.width, 0, 7, 4, 1, pole_shadow);
    image_from_pixels(manifest.width, manifest.height, pixels)
}

pub(super) fn create_shield_image() -> Image {
    let mut pixels = vec![0; GENERATED_SHIELD_SIZE * GENERATED_SHIELD_SIZE * 4];
    let light = [224, 248, 255, 220];
    let mid = [112, 208, 248, 210];
    let dark = [48, 128, 200, 180];

    fill_rect(&mut pixels, GENERATED_SHIELD_SIZE, 4, 1, 8, 1, light);
    fill_rect(&mut pixels, GENERATED_SHIELD_SIZE, 2, 4, 1, 8, light);
    fill_rect(&mut pixels, GENERATED_SHIELD_SIZE, 13, 4, 1, 8, dark);
    fill_rect(&mut pixels, GENERATED_SHIELD_SIZE, 4, 13, 8, 1, dark);
    for (x, y) in [(3, 2), (12, 2), (3, 12), (12, 12)] {
        fill_rect(&mut pixels, GENERATED_SHIELD_SIZE, x, y, 2, 2, mid);
    }
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 7, 0, light);
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 8, 0, light);
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 1, 7, light);
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 14, 8, dark);
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 7, 14, dark);
    set_pixel(&mut pixels, GENERATED_SHIELD_SIZE, 8, 14, dark);

    image_from_pixels(GENERATED_SHIELD_SIZE, GENERATED_SHIELD_SIZE, pixels)
}

pub(super) fn image_from_pixels(width: usize, height: usize, pixels: Vec<u8>) -> Image {
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

pub(super) fn fill_rect(
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

pub(super) fn set_pixel(pixels: &mut [u8], width: usize, x: usize, y: usize, color: [u8; 4]) {
    let index = (y * width + x) * 4;
    pixels[index..index + 4].copy_from_slice(&color);
}
