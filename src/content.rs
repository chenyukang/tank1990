use super::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const EMBEDDED_ASSET_MANIFEST: &str = include_str!("../assets/manifest.ron");
const EMBEDDED_CUSTOM_LEVELS: [&str; CUSTOM_LEVEL_COUNT] = [
    include_str!("../assets/levels/001.level.ron"),
    include_str!("../assets/levels/002.level.ron"),
    include_str!("../assets/levels/003.level.ron"),
    include_str!("../assets/levels/004.level.ron"),
    include_str!("../assets/levels/005.level.ron"),
    include_str!("../assets/levels/006.level.ron"),
    include_str!("../assets/levels/007.level.ron"),
    include_str!("../assets/levels/008.level.ron"),
    include_str!("../assets/levels/009.level.ron"),
    include_str!("../assets/levels/010.level.ron"),
    include_str!("../assets/levels/011.level.ron"),
    include_str!("../assets/levels/012.level.ron"),
    include_str!("../assets/levels/013.level.ron"),
    include_str!("../assets/levels/014.level.ron"),
    include_str!("../assets/levels/015.level.ron"),
    include_str!("../assets/levels/016.level.ron"),
    include_str!("../assets/levels/017.level.ron"),
    include_str!("../assets/levels/018.level.ron"),
    include_str!("../assets/levels/019.level.ron"),
    include_str!("../assets/levels/020.level.ron"),
    include_str!("../assets/levels/021.level.ron"),
    include_str!("../assets/levels/022.level.ron"),
    include_str!("../assets/levels/023.level.ron"),
    include_str!("../assets/levels/024.level.ron"),
    include_str!("../assets/levels/025.level.ron"),
    include_str!("../assets/levels/026.level.ron"),
    include_str!("../assets/levels/027.level.ron"),
    include_str!("../assets/levels/028.level.ron"),
    include_str!("../assets/levels/029.level.ron"),
    include_str!("../assets/levels/030.level.ron"),
    include_str!("../assets/levels/031.level.ron"),
    include_str!("../assets/levels/032.level.ron"),
    include_str!("../assets/levels/033.level.ron"),
    include_str!("../assets/levels/034.level.ron"),
    include_str!("../assets/levels/035.level.ron"),
    include_str!("../assets/levels/036.level.ron"),
    include_str!("../assets/levels/037.level.ron"),
    include_str!("../assets/levels/038.level.ron"),
    include_str!("../assets/levels/039.level.ron"),
    include_str!("../assets/levels/040.level.ron"),
    include_str!("../assets/levels/041.level.ron"),
    include_str!("../assets/levels/042.level.ron"),
    include_str!("../assets/levels/043.level.ron"),
    include_str!("../assets/levels/044.level.ron"),
    include_str!("../assets/levels/045.level.ron"),
    include_str!("../assets/levels/046.level.ron"),
    include_str!("../assets/levels/047.level.ron"),
    include_str!("../assets/levels/048.level.ron"),
    include_str!("../assets/levels/049.level.ron"),
    include_str!("../assets/levels/050.level.ron"),
];
const EMBEDDED_ORIGINAL_LEVELS: [&str; ORIGINAL_LEVEL_COUNT] = [
    include_str!("../assets/levels_original/001.level.ron"),
    include_str!("../assets/levels_original/002.level.ron"),
    include_str!("../assets/levels_original/003.level.ron"),
    include_str!("../assets/levels_original/004.level.ron"),
    include_str!("../assets/levels_original/005.level.ron"),
    include_str!("../assets/levels_original/006.level.ron"),
    include_str!("../assets/levels_original/007.level.ron"),
    include_str!("../assets/levels_original/008.level.ron"),
    include_str!("../assets/levels_original/009.level.ron"),
    include_str!("../assets/levels_original/010.level.ron"),
    include_str!("../assets/levels_original/011.level.ron"),
    include_str!("../assets/levels_original/012.level.ron"),
    include_str!("../assets/levels_original/013.level.ron"),
    include_str!("../assets/levels_original/014.level.ron"),
    include_str!("../assets/levels_original/015.level.ron"),
    include_str!("../assets/levels_original/016.level.ron"),
    include_str!("../assets/levels_original/017.level.ron"),
    include_str!("../assets/levels_original/018.level.ron"),
    include_str!("../assets/levels_original/019.level.ron"),
    include_str!("../assets/levels_original/020.level.ron"),
    include_str!("../assets/levels_original/021.level.ron"),
    include_str!("../assets/levels_original/022.level.ron"),
    include_str!("../assets/levels_original/023.level.ron"),
    include_str!("../assets/levels_original/024.level.ron"),
    include_str!("../assets/levels_original/025.level.ron"),
    include_str!("../assets/levels_original/026.level.ron"),
    include_str!("../assets/levels_original/027.level.ron"),
    include_str!("../assets/levels_original/028.level.ron"),
    include_str!("../assets/levels_original/029.level.ron"),
    include_str!("../assets/levels_original/030.level.ron"),
    include_str!("../assets/levels_original/031.level.ron"),
    include_str!("../assets/levels_original/032.level.ron"),
    include_str!("../assets/levels_original/033.level.ron"),
    include_str!("../assets/levels_original/034.level.ron"),
    include_str!("../assets/levels_original/035.level.ron"),
];
const EMBEDDED_ARENAS: [&str; 8] = [
    include_str!("../assets/arenas/arena_01.ron"),
    include_str!("../assets/arenas/arena_02.ron"),
    include_str!("../assets/arenas/arena_03.ron"),
    include_str!("../assets/arenas/arena_04.ron"),
    include_str!("../assets/arenas/arena_05.ron"),
    include_str!("../assets/arenas/arena_06.ron"),
    include_str!("../assets/arenas/arena_07.ron"),
    include_str!("../assets/arenas/arena_08.ron"),
];

pub(super) fn stage_path(stage: usize) -> String {
    format!("assets/levels/{stage:03}.level.ron")
}

pub(super) fn personal_stage_path(stage: usize) -> String {
    format!("assets/personal/levels/{stage:03}.level.ron")
}

pub(super) fn campaign_stage_path(pack: CampaignMapPack, stage: usize) -> String {
    match pack {
        CampaignMapPack::Original => format!("assets/levels_original/{stage:03}.level.ron"),
        CampaignMapPack::Custom => stage_path(stage),
    }
}

pub(super) fn personal_campaign_stage_path(pack: CampaignMapPack, stage: usize) -> String {
    match pack {
        CampaignMapPack::Original => {
            format!("assets/personal/levels_original/{stage:03}.level.ron")
        }
        CampaignMapPack::Custom => personal_stage_path(stage),
    }
}

pub(super) fn runtime_campaign_stage_path(pack: CampaignMapPack, stage: usize) -> String {
    preferred_existing_path(
        &personal_campaign_stage_path(pack, stage),
        &campaign_stage_path(pack, stage),
        |path| Path::new(path).is_file(),
    )
}

pub(super) fn preferred_existing_path(
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

#[cfg(test)]
pub(super) fn load_stage_bundle(stage: usize) -> Result<(LevelDefinition, TileGrid), String> {
    load_campaign_stage_bundle(CampaignMapPack::Custom, stage)
}

pub(super) fn load_campaign_stage_bundle(
    pack: CampaignMapPack,
    stage: usize,
) -> Result<(LevelDefinition, TileGrid), String> {
    let personal_path = personal_campaign_stage_path(pack, stage);
    let authored_path = campaign_stage_path(pack, stage);
    let (path, contents) = load_runtime_text(
        &personal_path,
        &authored_path,
        embedded_campaign_stage_contents(pack, stage),
    )?;
    let level =
        parse_level(&contents).map_err(|err| format!("failed to load level {path}: {err}"))?;
    let grid = TileGrid::from_level(&level)
        .map_err(|err| format!("failed to build level grid {path}: {err}"))?;
    Ok((level, grid))
}

#[cfg(test)]
pub(super) fn load_stage_bundle_or_panic(stage: usize) -> (LevelDefinition, TileGrid) {
    load_campaign_stage_bundle_or_panic(CampaignMapPack::Custom, stage)
}

pub(super) fn load_campaign_stage_bundle_or_panic(
    pack: CampaignMapPack,
    stage: usize,
) -> (LevelDefinition, TileGrid) {
    let path = runtime_campaign_stage_path(pack, stage);
    load_campaign_stage_bundle(pack, stage).unwrap_or_else(|err| {
        panic!("{}", campaign_stage_load_error(pack, stage, &path, &err));
    })
}

pub(super) fn campaign_stage_load_error(
    pack: CampaignMapPack,
    stage: usize,
    path: &str,
    err: &str,
) -> String {
    format!(
        "failed to load {} campaign stage {stage} from {path}: {err}",
        campaign_map_pack_label(pack)
    )
}

pub(super) fn arena_path(arena: usize) -> String {
    format!("assets/arenas/arena_{arena:02}.ron")
}

pub(super) fn personal_arena_path(arena: usize) -> String {
    format!("assets/personal/arenas/arena_{arena:02}.ron")
}

pub(super) fn runtime_arena_path(arena: usize) -> String {
    preferred_existing_path(&personal_arena_path(arena), &arena_path(arena), |path| {
        Path::new(path).is_file()
    })
}

fn load_runtime_arena_definition(arena: usize) -> Result<(String, ArenaDefinition), String> {
    let personal_path = personal_arena_path(arena);
    let authored_path = arena_path(arena);
    let (path, contents) = load_runtime_text(
        &personal_path,
        &authored_path,
        embedded_arena_contents(arena),
    )?;
    let arena_definition =
        parse_arena(&contents).map_err(|err| format!("failed to load arena {path}: {err}"))?;

    Ok((path, arena_definition))
}

pub(super) fn load_arena_definition(arena: usize) -> Result<ArenaDefinition, String> {
    load_runtime_arena_definition(arena).map(|(_, arena)| arena)
}

pub(super) fn load_arena_bundle(arena: usize) -> Result<(ArenaDefinition, TileGrid), String> {
    let (path, arena_definition) = load_runtime_arena_definition(arena)?;
    let grid = TileGrid::from_arena(&arena_definition)
        .map_err(|err| format!("failed to build arena grid {path}: {err}"))?;
    Ok((arena_definition, grid))
}

pub(super) fn load_arena_bundle_or_panic(arena: usize) -> (ArenaDefinition, TileGrid) {
    let path = runtime_arena_path(arena);
    load_arena_bundle(arena).unwrap_or_else(|err| {
        panic!("{}", versus_arena_load_error(arena, &path, &err));
    })
}

pub(super) fn versus_arena_load_error(arena: usize, path: &str, err: &str) -> String {
    format!("failed to load versus arena {arena} from {path}: {err}")
}

pub(super) fn runtime_asset_manifest_path() -> String {
    preferred_existing_path(PERSONAL_ASSET_MANIFEST_PATH, ASSET_MANIFEST_PATH, |path| {
        Path::new(path).is_file()
    })
}

#[cfg(test)]
pub(super) fn load_level(path: &str) -> Result<LevelDefinition, String> {
    let contents = read_text(path)?;
    parse_level(&contents).map_err(|err| format!("failed to load level {path}: {err}"))
}

#[cfg(test)]
pub(super) fn load_arena(path: &str) -> Result<ArenaDefinition, String> {
    let contents = read_text(path)?;
    parse_arena(&contents).map_err(|err| format!("failed to load arena {path}: {err}"))
}

pub(super) fn battle_kind_label(rules: BattleRules) -> &'static str {
    match rules {
        BattleRules::Deathmatch { .. } => "DUEL",
        BattleRules::BaseBattle { .. } => "BASE",
    }
}

pub(super) fn battle_kind_label_for_arena(arena: usize) -> &'static str {
    load_arena_definition(arena)
        .map(|arena| battle_kind_label(arena.battle_rules))
        .unwrap_or("DUEL")
}

pub(super) fn load_asset_manifest(path: &str) -> Result<AssetManifest, String> {
    let contents = load_text_or_embedded_with(
        path,
        (path == ASSET_MANIFEST_PATH).then_some(embedded_asset_manifest_contents()),
        |path| Path::new(path).is_file(),
        read_text,
    )?;
    parse_asset_manifest(&contents)
}

pub(super) fn embedded_asset_manifest_contents() -> &'static str {
    EMBEDDED_ASSET_MANIFEST
}

#[cfg(test)]
pub(super) fn embedded_stage_contents(stage: usize) -> Option<&'static str> {
    embedded_campaign_stage_contents(CampaignMapPack::Custom, stage)
}

pub(super) fn embedded_campaign_stage_contents(
    pack: CampaignMapPack,
    stage: usize,
) -> Option<&'static str> {
    let levels = match pack {
        CampaignMapPack::Original => EMBEDDED_ORIGINAL_LEVELS.as_slice(),
        CampaignMapPack::Custom => EMBEDDED_CUSTOM_LEVELS.as_slice(),
    };

    stage
        .checked_sub(1)
        .and_then(|index| levels.get(index))
        .copied()
}

pub(super) fn embedded_arena_contents(arena: usize) -> Option<&'static str> {
    arena
        .checked_sub(1)
        .and_then(|index| EMBEDDED_ARENAS.get(index))
        .copied()
}

fn read_text(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))
}

pub(super) fn load_runtime_text_with(
    personal_path: &str,
    authored_path: &str,
    embedded: Option<&'static str>,
    exists: impl Fn(&str) -> bool,
    read_to_string: impl Fn(&str) -> Result<String, String>,
) -> Result<(String, Cow<'static, str>), String> {
    if exists(personal_path) {
        return read_to_string(personal_path)
            .map(|contents| (personal_path.to_string(), Cow::Owned(contents)));
    }

    if exists(authored_path) {
        return read_to_string(authored_path)
            .map(|contents| (authored_path.to_string(), Cow::Owned(contents)));
    }

    if let Some(contents) = embedded {
        return Ok((authored_path.to_string(), Cow::Borrowed(contents)));
    }

    read_to_string(authored_path).map(|contents| (authored_path.to_string(), Cow::Owned(contents)))
}

fn load_runtime_text(
    personal_path: &str,
    authored_path: &str,
    embedded: Option<&'static str>,
) -> Result<(String, Cow<'static, str>), String> {
    load_runtime_text_with(
        personal_path,
        authored_path,
        embedded,
        |path| Path::new(path).is_file(),
        read_text,
    )
}

pub(super) fn load_text_or_embedded_with(
    path: &str,
    embedded: Option<&'static str>,
    exists: impl Fn(&str) -> bool,
    read_to_string: impl Fn(&str) -> Result<String, String>,
) -> Result<Cow<'static, str>, String> {
    if exists(path) {
        return read_to_string(path).map(Cow::Owned);
    }

    if let Some(contents) = embedded {
        return Ok(Cow::Borrowed(contents));
    }

    read_to_string(path).map(Cow::Owned)
}

pub(super) fn parse_asset_manifest(contents: &str) -> Result<AssetManifest, String> {
    let manifest: AssetManifest =
        ron::from_str(contents).map_err(|err| format!("failed to parse asset manifest: {err}"))?;
    validate_asset_manifest(&manifest)?;
    Ok(manifest)
}

pub(super) fn validate_asset_manifest(manifest: &AssetManifest) -> Result<(), String> {
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

pub(super) fn validate_generated_atlases(
    manifest: &GeneratedAtlasesManifest,
) -> Result<(), String> {
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

pub(super) fn validate_generated_atlas(
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

pub(super) fn validate_generated_sprite(
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

pub(super) fn validate_bullet_manifest(manifest: DirectionalSpriteManifest) -> Result<(), String> {
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

pub(super) fn validate_glyph_manifest(manifest: &GlyphManifest) -> Result<(), String> {
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
        if ch != ' ' && !glyph_pattern_has_pixels(pattern) {
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

pub(super) fn validate_sound_manifest(manifest: &SoundManifest) -> Result<(), String> {
    for (name, spec) in sound_manifest_specs(manifest) {
        validate_sound_spec(name, spec)?;
    }
    Ok(())
}

pub(super) fn sound_manifest_specs(
    manifest: &SoundManifest,
) -> [(&'static str, &RetroSoundSpec); 9] {
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

pub(super) fn validate_sound_spec(name: &str, spec: &RetroSoundSpec) -> Result<(), String> {
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

pub(super) fn validate_sound_duration(name: &str, duration_secs: f32) -> Result<(), String> {
    if duration_secs <= 0.0 || duration_secs > MAX_RETRO_SOUND_SECONDS {
        return Err(format!(
            "{name} duration {duration_secs} must be in 0..={MAX_RETRO_SOUND_SECONDS} seconds"
        ));
    }
    Ok(())
}

pub(super) fn validate_sound_frequency(
    name: &str,
    field: &str,
    frequency: f32,
) -> Result<(), String> {
    if frequency <= 0.0 || frequency > MAX_RETRO_SOUND_FREQUENCY {
        return Err(format!(
            "{name} {field} {frequency} must be in 0..={MAX_RETRO_SOUND_FREQUENCY} Hz"
        ));
    }
    Ok(())
}

pub(super) fn validate_sound_volume(name: &str, volume: f32) -> Result<(), String> {
    if volume <= 0.0 || volume > MAX_RETRO_SOUND_VOLUME {
        return Err(format!(
            "{name} volume {volume} must be in 0..={MAX_RETRO_SOUND_VOLUME}"
        ));
    }
    Ok(())
}

pub(super) fn validate_tank_frames(manifest: &TankSpriteManifest) -> Result<(), String> {
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

pub(super) fn validate_frame_range(
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

pub(super) fn parse_level(contents: &str) -> Result<LevelDefinition, String> {
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

pub(super) fn parse_arena(contents: &str) -> Result<ArenaDefinition, String> {
    let arena: ArenaDefinition =
        ron::from_str(contents).map_err(|err| format!("failed to parse arena: {err}"))?;

    let grid = TileGrid::from_arena(&arena)?;
    validate_arena_spawns(&grid, &arena)?;
    validate_battle_rules(&grid, arena.battle_rules)?;
    validate_powerup_spawns(&grid, &arena.powerup_spawns)?;

    Ok(arena)
}

pub(super) fn validate_battle_rules(grid: &TileGrid, rules: BattleRules) -> Result<(), String> {
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
