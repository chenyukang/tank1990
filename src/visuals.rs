use crate::*;

pub(crate) fn explosion_duration_secs(frames: SpriteFrameRange) -> f32 {
    (frames.last - frames.first + 1) as f32 * EXPLOSION_FRAME_SECONDS
}

pub(crate) fn spawn_shimmer_duration_secs(frames: SpriteFrameRange) -> f32 {
    (frames.last - frames.first + 1) as f32 * SPAWN_SHIMMER_FRAME_SECONDS
}

pub(crate) fn spawn_bullet_impact_effect(
    commands: &mut Commands,
    assets: &SpriteAssets,
    bullet_top_left: Vec2,
) {
    spawn_bullet_impact_effect_with_direction(commands, assets, bullet_top_left, None);
}

pub(crate) fn spawn_directed_bullet_impact_effect(
    commands: &mut Commands,
    assets: &SpriteAssets,
    bullet_top_left: Vec2,
    direction: Direction,
) {
    spawn_bullet_impact_effect_with_direction(commands, assets, bullet_top_left, Some(direction));
}

fn spawn_bullet_impact_effect_with_direction(
    commands: &mut Commands,
    assets: &SpriteAssets,
    bullet_top_left: Vec2,
    direction: Option<Direction>,
) {
    let frames = assets.manifest.bullet_impact_frames();
    let mut entity = commands.spawn((
        Sprite::from_atlas_image(
            assets.effect_image.clone(),
            TextureAtlas {
                layout: assets.effect_layout.clone(),
                index: frames.first,
            },
        ),
        Transform::from_translation(board_object_center(
            bullet_top_left.x,
            bullet_top_left.y,
            Vec2::splat(BULLET_SIZE),
            8.1,
        ))
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(BULLET_IMPACT_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
    if let Some(direction) = direction {
        entity.insert(BulletImpactDirection { direction });
    }
}

pub(crate) fn spawn_explosion(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
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
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(EXPLOSION_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

pub(crate) fn mark_enemy_tank_destroyed(
    commands: &mut Commands,
    assets: &SpriteAssets,
    enemy_entity: Entity,
    top_left: Vec2,
    transform: &mut Transform,
) {
    let frames = assets.manifest.explosion_frames();
    spawn_explosion(commands, assets, top_left);
    park_tank_transform(transform);
    commands
        .entity(enemy_entity)
        .remove::<(Tank, Health, EnemyTank, EnemyAi, SpawnProtection)>()
        .insert(DestroyedTank::for_explosion(frames));
}

pub(crate) fn parked_tank_top_left() -> Vec2 {
    Vec2::new(-TANK_SIZE * 4.0, -TANK_SIZE * 4.0)
}

pub(crate) fn parked_tank_translation() -> Vec3 {
    let top_left = parked_tank_top_left();
    board_object_center(top_left.x, top_left.y, Vec2::splat(TANK_SIZE), 6.0)
}

pub(crate) fn park_tank_transform(transform: &mut Transform) {
    transform.translation = parked_tank_translation();
}

fn park_tank(tank: &mut Tank, transform: &mut Transform) {
    tank.top_left = parked_tank_top_left();
    park_tank_transform(transform);
}

pub(crate) fn mark_player_tank_destroyed_for_respawn(
    commands: &mut Commands,
    assets: &SpriteAssets,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
    upgrade: &mut PlayerUpgrade,
) {
    park_tank(tank, transform);
    upgrade.level = 0;
    commands
        .entity(player_entity)
        .remove::<(Tank, Health, Shield)>()
        .insert(PlayerRespawnPending::for_explosion(
            assets.manifest.explosion_frames(),
        ));
}

pub(crate) fn mark_player_tank_destroyed_terminal(
    commands: &mut Commands,
    assets: &SpriteAssets,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
) {
    park_tank(tank, transform);
    commands
        .entity(player_entity)
        .remove::<(
            Tank,
            Health,
            Shield,
            PlayerRespawnDelay,
            PlayerRespawnPending,
        )>()
        .insert(DestroyedTank::for_explosion(
            assets.manifest.explosion_frames(),
        ));
}

pub(crate) fn spawn_spawn_effect(commands: &mut Commands, assets: &SpriteAssets, top_left: Vec2) {
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
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(SPAWN_SHIMMER_FRAME_SECONDS, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

pub(crate) fn spawn_base_destruction_effect(
    commands: &mut Commands,
    assets: &SpriteAssets,
    top_left: Vec2,
) {
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
        .with_scale(Vec3::splat(window_scale())),
        SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.09, TimerMode::Repeating),
            despawn_on_finish: true,
        },
        GameEntity,
    ));
}

pub(crate) fn spawn_powerup(
    commands: &mut Commands,
    assets: &SpriteAssets,
    kind: PowerUpKind,
    top_left: Vec2,
    active_powerups: impl IntoIterator<Item = Entity>,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
) {
    for active_powerup in active_powerups {
        commands.entity(active_powerup).despawn();
    }
    despawn_powerup_sparkles(commands, active_sparkles);

    spawn_powerup_entity(commands, assets, kind, top_left);
}

pub(crate) fn spawn_powerup_entity(
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
        .with_scale(Vec3::splat(window_scale())),
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
        .with_scale(Vec3::splat(window_scale())),
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

pub(crate) fn despawn_powerup_sparkles(
    commands: &mut Commands,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
) {
    for sparkle in active_sparkles {
        commands.entity(sparkle).despawn();
    }
}

pub(crate) fn player_upgrade_visual_color(upgrade_level: u8) -> Color {
    let [r, g, b] = player_upgrade_visual_rgb(upgrade_level);
    Color::srgb_u8(r, g, b)
}

pub(crate) fn player_upgrade_visual_rgb(upgrade_level: u8) -> [u8; 3] {
    match upgrade_level.min(3) {
        0 => [255, 255, 255],
        1 => [184, 248, 184],
        2 => [255, 232, 104],
        _ => [255, 176, 104],
    }
}

pub(crate) fn player_shield_visual_rgb(elapsed_secs: f32, upgrade_level: u8) -> [u8; 3] {
    if elapsed_secs % 0.25 < 0.125 {
        [160, 220, 255]
    } else {
        player_upgrade_visual_rgb(upgrade_level)
    }
}

pub(crate) fn shield_visual_color(elapsed_secs: f32) -> Color {
    let [r, g, b] = shield_visual_rgb(elapsed_secs);
    Color::srgb_u8(r, g, b)
}

pub(crate) fn shield_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.20 < 0.10 {
        [255, 255, 255]
    } else {
        [160, 220, 255]
    }
}

pub(crate) fn shield_visual_translation(top_left: Vec2) -> Vec3 {
    board_object_center(top_left.x, top_left.y, Vec2::splat(TANK_SIZE), 6.7)
}

pub(crate) fn player_frozen_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [136, 216, 255]
    } else {
        [216, 248, 255]
    }
}

pub(crate) fn shovel_warning_visual_color(elapsed_secs: f32) -> Color {
    let [r, g, b] = shovel_warning_visual_rgb(elapsed_secs);
    Color::srgb_u8(r, g, b)
}

pub(crate) fn shovel_warning_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [255, 255, 255]
    } else {
        [248, 232, 96]
    }
}

pub(crate) fn powerup_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.30 < 0.15 {
        [255, 255, 255]
    } else {
        [255, 232, 104]
    }
}

pub(crate) fn animated_tank_sprite_index(
    manifest: &AssetManifest,
    set: TankSpriteSet,
    direction: Direction,
    frame: usize,
) -> usize {
    manifest.tank_index(set, direction, frame)
}

pub(crate) fn set_tank_sprite_direction(
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

pub(crate) fn update_tank_sprite(
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
