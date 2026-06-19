use crate::*;

pub(crate) fn spawn_level(
    commands: &mut Commands,
    assets: &SpriteAssets,
    level: &LevelDefinition,
    tile_grid: &TileGrid,
    player_lives: &[(PlayerId, i32)],
    spawn_invulnerability_secs: f32,
) {
    spawn_terrain(commands, assets, tile_grid);

    spawn_base_sprite(commands, assets, &level.base_position, None);

    for (player, lives) in player_lives {
        spawn_player_tank(
            commands,
            assets,
            &campaign_spawn_for_player(level, *player),
            *player,
            *lives,
            spawn_invulnerability_secs,
        );
    }
}

pub(crate) fn campaign_spawn_for_player(level: &LevelDefinition, player: PlayerId) -> SpawnPoint {
    match player {
        PlayerId::One => level.player_spawn.clone(),
        PlayerId::Two => SpawnPoint {
            x: CLASSIC_COOP_P2_SPAWN_X,
            y: CLASSIC_COOP_P2_SPAWN_Y,
            facing: Direction::Up,
        },
    }
}

pub(crate) fn spawn_arena(
    commands: &mut Commands,
    assets: &SpriteAssets,
    arena: &ArenaDefinition,
    tile_grid: &TileGrid,
    player_lives: i32,
    spawn_invulnerability_secs: f32,
) {
    spawn_terrain(commands, assets, tile_grid);
    if let BattleRules::BaseBattle {
        p1_base, p2_base, ..
    } = arena.battle_rules
    {
        spawn_base_sprite(commands, assets, &p1_base, Some(PlayerId::One));
        spawn_base_sprite(commands, assets, &p2_base, Some(PlayerId::Two));
    }
    spawn_player_tank(
        commands,
        assets,
        &arena.p1_spawn,
        PlayerId::One,
        player_lives,
        spawn_invulnerability_secs,
    );
    spawn_player_tank(
        commands,
        assets,
        &arena.p2_spawn,
        PlayerId::Two,
        player_lives,
        spawn_invulnerability_secs,
    );
}

pub(crate) fn spawn_base_sprite(
    commands: &mut Commands,
    assets: &SpriteAssets,
    point: &GridPoint,
    owner: Option<PlayerId>,
) {
    let top_left = grid_point_top_left(point);
    commands.spawn((
        Sprite::from_image(assets.base_intact.clone()),
        Transform::from_translation(board_object_center(
            top_left.x,
            top_left.y,
            generated_sprite_size(assets.manifest.base.intact),
            4.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        BaseSprite { owner, top_left },
        GameEntity,
    ));
}

pub(crate) fn spawn_terrain(commands: &mut Commands, assets: &SpriteAssets, tile_grid: &TileGrid) {
    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            let tile = tile_grid.tiles[y * BOARD_TILES + x];
            spawn_terrain_tile(commands, assets, tile, x, y);
        }
    }
}

fn spawn_terrain_tile(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile: TileKind,
    x: usize,
    y: usize,
) {
    let Some(index) = assets.manifest.terrain_index(tile) else {
        return;
    };

    let mut entity = commands.spawn((
        Sprite::from_atlas_image(
            assets.terrain_image.clone(),
            TextureAtlas {
                layout: assets.terrain_layout.clone(),
                index,
            },
        ),
        Transform::from_translation(board_tile_center(x, y, terrain_z(tile)))
            .with_scale(Vec3::splat(window_scale())),
        GridTile { x, y },
        GameEntity,
    ));

    if let Some(frames) = assets.manifest.terrain_animation_frames(tile) {
        entity.insert(SpriteAnimation {
            first: frames.first,
            last: frames.last,
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
            despawn_on_finish: false,
        });
    }
}

pub(crate) fn sync_tile_sprite(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    x: usize,
    y: usize,
    tile: TileKind,
) {
    // Refresh same-kind tiles too, so temporary tints from shovel warnings cannot stick.
    tile_grid.set(x, y, tile);
    for (tile_entity, grid_tile) in tile_sprites {
        if grid_tile.x == x && grid_tile.y == y {
            commands.entity(tile_entity).despawn();
            break;
        }
    }

    spawn_terrain_tile(commands, assets, tile, x, y);
}

pub(crate) fn base_wall_positions(tile_grid: &TileGrid) -> Vec<(usize, usize)> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found_base = true;
            }
        }
    }

    if !found_base {
        return Vec::new();
    }

    base_wall_positions_around_rect(tile_grid, min_x, min_y, max_x, max_y)
}

pub(crate) fn base_wall_positions_for_top_left(
    tile_grid: &TileGrid,
    top_left: Vec2,
) -> Vec<(usize, usize)> {
    let min_x = (top_left.x / TILE_SIZE).floor().max(0.0) as usize;
    let min_y = (top_left.y / TILE_SIZE).floor().max(0.0) as usize;
    let max_x = (min_x + 1).min(BOARD_TILES - 1);
    let max_y = (min_y + 1).min(BOARD_TILES - 1);

    base_wall_positions_around_rect(tile_grid, min_x, min_y, max_x, max_y)
}

fn base_wall_positions_around_rect(
    tile_grid: &TileGrid,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    let left = min_x.saturating_sub(2);
    let right = (max_x + 2).min(BOARD_TILES - 1);
    let top = min_y.saturating_sub(2);
    let bottom = (max_y + 2).min(BOARD_TILES - 1);
    let mut positions = Vec::new();

    for y in top..=bottom {
        for x in left..=right {
            if tile_grid.tiles[y * BOARD_TILES + x] != TileKind::Base {
                positions.push((x, y));
            }
        }
    }

    positions
}

pub(crate) fn base_center_from_grid(tile_grid: &TileGrid) -> Option<Vec2> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                found_base = true;
            }
        }
    }

    if !found_base {
        return None;
    }

    Some(Vec2::new(
        (min_x + max_x + 1) as f32 * TILE_SIZE / 2.0,
        (min_y + max_y + 1) as f32 * TILE_SIZE / 2.0,
    ))
}

pub(crate) fn base_top_left_from_grid(tile_grid: &TileGrid) -> Option<Vec2> {
    let mut min_x = BOARD_TILES;
    let mut min_y = BOARD_TILES;
    let mut found_base = false;

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            if tile_grid.tiles[y * BOARD_TILES + x] == TileKind::Base {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                found_base = true;
            }
        }
    }

    found_base.then_some(Vec2::new(
        min_x as f32 * TILE_SIZE,
        min_y as f32 * TILE_SIZE,
    ))
}

pub(crate) fn base_contains_tile(base_top_left: Vec2, tile_x: usize, tile_y: usize) -> bool {
    let tile_top_left = Vec2::new(tile_x as f32 * TILE_SIZE, tile_y as f32 * TILE_SIZE);
    rects_overlap(
        base_top_left,
        Vec2::splat(TANK_SIZE),
        tile_top_left,
        Vec2::splat(TILE_SIZE),
    )
}

pub(crate) fn grid_point_top_left(point: &GridPoint) -> Vec2 {
    Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE)
}

pub(crate) fn spawn_point_top_left(spawn: &SpawnPoint) -> Vec2 {
    Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE)
}

pub(crate) fn spawn_player_tank(
    commands: &mut Commands,
    assets: &SpriteAssets,
    spawn: &SpawnPoint,
    player_id: PlayerId,
    player_lives: i32,
    spawn_invulnerability_secs: f32,
) {
    let player_top_left = spawn_point_top_left(spawn);

    commands.spawn((
        Sprite::from_atlas_image(
            assets.tank_image.clone(),
            TextureAtlas {
                layout: assets.tank_layout.clone(),
                index: animated_tank_sprite_index(
                    &assets.manifest,
                    TankSpriteSet::player(player_id),
                    spawn.facing,
                    0,
                ),
            },
        ),
        Transform::from_translation(board_object_center(
            player_top_left.x,
            player_top_left.y,
            Vec2::splat(TANK_SIZE),
            6.0,
        ))
        .with_scale(Vec3::splat(window_scale())),
        Tank {
            top_left: player_top_left,
            facing: spawn.facing,
            speed: PLAYER_SPEED,
        },
        TankSpriteState::new(TankSpriteSet::player(player_id)),
        Health { current: 1 },
        RespawnPoint {
            top_left: player_top_left,
            facing: spawn.facing,
        },
        PlayerLives {
            current: player_lives,
        },
        PlayerUpgrade { level: 0 },
        Shield {
            timer: Timer::from_seconds(spawn_invulnerability_secs, TimerMode::Once),
        },
        PlayerRespawnDelay::for_spawn_shimmer(assets.manifest.spawn_shimmer_frames()),
        Player { id: player_id },
        GameEntity,
    ));
    spawn_spawn_effect(commands, assets, player_top_left);
}

pub(crate) fn terrain_z(tile: TileKind) -> f32 {
    match tile {
        TileKind::Forest => 7.5,
        TileKind::Water => 1.0,
        _ => 2.0,
    }
}
