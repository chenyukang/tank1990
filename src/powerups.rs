use bevy::ecs::query::QueryFilter;

use crate::*;

pub(crate) fn clock_freeze_target(game_mode: GameMode, collector: PlayerId) -> Option<PlayerId> {
    match game_mode {
        GameMode::Campaign | GameMode::CoopCampaign => None,
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => Some(collector.opponent()),
    }
}

pub(crate) fn grenade_player_target(game_mode: GameMode, collector: PlayerId) -> Option<PlayerId> {
    match game_mode {
        GameMode::Campaign | GameMode::CoopCampaign => None,
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => Some(collector.opponent()),
    }
}

fn grant_extra_life(
    lives: &mut PlayerLives,
    score_board: &mut ScoreBoard,
    game_mode: GameMode,
    player: PlayerId,
) {
    lives.current = lives.current.saturating_add(1).min(MAX_PLAYER_LIVES);
    match game_mode {
        GameMode::Campaign => {
            score_board.lives = lives.current;
        }
        GameMode::CoopCampaign => {
            score_board.set_coop_player_lives(player, lives.current);
        }
        GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
            score_board.set_player_lives(player, lives.current);
        }
    }
}

pub(crate) fn spawn_versus_powerups(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    mut director: ResMut<VersusPowerUpDirector>,
    active_powerups: Query<Entity, With<PowerUp>>,
) {
    if !game_mode.is_versus()
        || !game_status.is_playing()
        || !active_powerups.is_empty()
        || director.spawn_points.is_empty()
    {
        return;
    }

    let should_spawn =
        director.spawn_immediately || director.spawn_timer.tick(time.delta()).just_finished();
    if !should_spawn {
        return;
    }

    if let Some((top_left, kind)) = director.next_spawn() {
        spawn_powerup_entity(&mut commands, &assets, kind, top_left);
    }
}

pub(crate) fn pickup_powerups(
    mut commands: Commands,
    mut game_status: ResMut<GameStatus>,
    game_mode: Res<GameMode>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    mut tile_grid: ResMut<TileGrid>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    powerups: Query<(Entity, &PowerUp, &Transform), (With<PowerUp>, Without<Player>)>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    tile_sprites: Query<(Entity, &GridTile)>,
    bases: Query<&BaseSprite>,
    mut players: Query<
        (
            Entity,
            &mut Tank,
            &Player,
            &mut PlayerUpgrade,
            &mut PlayerLives,
            &mut Health,
            &mut Transform,
            &mut Sprite,
            Option<&Shield>,
        ),
        (With<Player>, Without<EnemyTank>, Without<PowerUp>),
    >,
    mut enemy_tanks: Query<
        (Entity, &Tank, &mut Transform, &EnemyTank),
        (With<EnemyTank>, Without<Player>, Without<PowerUp>),
    >,
    mut score_board: ResMut<ScoreBoard>,
) {
    if !game_status.is_playing() {
        return;
    }

    let active_powerup_entities: Vec<Entity> = powerups
        .iter()
        .map(|(powerup_entity, _, _)| powerup_entity)
        .collect();

    for (powerup_entity, powerup, transform) in &powerups {
        let powerup_top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        let mut grenade_target = None;
        for (
            player_entity,
            tank,
            player,
            mut upgrade,
            mut lives,
            _health,
            _transform,
            mut sprite,
            _shield,
        ) in &mut players
        {
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
                    sprite.color = player_upgrade_visual_color(upgrade.level);
                }
                PowerUpKind::Helmet => {
                    commands.entity(player_entity).insert(Shield {
                        timer: Timer::from_seconds(HELMET_SECONDS, TimerMode::Once),
                    });
                }
                PowerUpKind::Clock => {
                    if let Some(frozen_player) = clock_freeze_target(*game_mode, player.id) {
                        versus_freeze.start(frozen_player);
                    } else {
                        enemy_freeze.start();
                    }
                }
                PowerUpKind::Grenade => {
                    if let Some(target) = grenade_player_target(*game_mode, player.id) {
                        grenade_target = Some((player.id, target));
                    } else {
                        destroy_visible_enemies(
                            &mut commands,
                            &assets,
                            &sounds,
                            &mut score_board,
                            active_powerup_entities
                                .iter()
                                .copied()
                                .filter(|active| *active != powerup_entity),
                            &active_sparkles,
                            &mut enemy_tanks,
                        );
                    }
                }
                PowerUpKind::Shovel => {
                    let positions = shovel_reinforcement_positions(
                        *game_mode,
                        player.id,
                        &tile_grid,
                        bases.iter(),
                    );
                    reinforce_base_walls(
                        &mut commands,
                        &assets,
                        &mut tile_grid,
                        &tile_sprites,
                        &mut base_reinforcement,
                        positions,
                    );
                }
                PowerUpKind::Tank => {
                    grant_extra_life(&mut lives, &mut score_board, *game_mode, player.id);
                }
            }
            commands.entity(powerup_entity).despawn();
            despawn_powerup_sparkles(&mut commands, &active_sparkles);
            play_sound(&mut commands, &sounds, SoundKind::PowerupPickup);
            break;
        }

        if let Some((shooter, target)) = grenade_target {
            for (
                target_entity,
                mut target_tank,
                target_player,
                mut target_upgrade,
                mut target_lives,
                mut target_health,
                mut target_transform,
                _target_sprite,
                target_shield,
            ) in &mut players
            {
                if target_player.id != target {
                    continue;
                }

                if target_shield.is_some() {
                    let impact_top_left =
                        target_tank.top_left + Vec2::splat((TANK_SIZE - BULLET_SIZE) / 2.0);
                    spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                    break;
                }

                spawn_explosion(&mut commands, &assets, target_tank.top_left);
                play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                resolve_player_destroyed(
                    &mut commands,
                    &assets,
                    &sounds,
                    &mut game_status,
                    &mut score_board,
                    target_entity,
                    &mut target_tank,
                    &mut target_transform,
                    &mut target_lives,
                    &mut target_health,
                    &mut target_upgrade,
                    target,
                    Some(shooter),
                    *game_mode,
                );
                break;
            }
        }
    }
}

pub(crate) fn tick_powerup_effects(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    assets: Res<SpriteAssets>,
    mut tile_grid: ResMut<TileGrid>,
    mut enemy_freeze: ResMut<EnemyFreeze>,
    mut versus_freeze: ResMut<VersusPlayerFreeze>,
    mut base_reinforcement: ResMut<BaseReinforcement>,
    tile_sprites: Query<(Entity, &GridTile)>,
) {
    if !game_status.is_playing() {
        return;
    }

    enemy_freeze.tick(time.delta());
    versus_freeze.tick(time.delta());

    if base_reinforcement.tick(time.delta()) {
        restore_base_walls(
            &mut commands,
            &assets,
            &mut tile_grid,
            &tile_sprites,
            &mut base_reinforcement,
        );
    }
}

pub(crate) fn update_base_reinforcement_visuals(
    game_status: Res<GameStatus>,
    base_reinforcement: Res<BaseReinforcement>,
    mut tile_sprites: Query<(&GridTile, &mut Sprite)>,
) {
    if !game_status.is_playing() || base_reinforcement.saved_tiles.is_empty() {
        return;
    }

    let color = base_reinforcement
        .warning_elapsed_secs()
        .map(shovel_warning_visual_color)
        .unwrap_or(Color::WHITE);

    for (grid_tile, mut sprite) in &mut tile_sprites {
        if base_reinforcement.contains_position(grid_tile.x, grid_tile.y) {
            sprite.color = color;
        }
    }
}

pub(crate) fn update_powerup_visuals(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut powerups: Query<&mut Sprite, With<PowerUp>>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    let [r, g, b] = powerup_visual_rgb(time.elapsed_secs());
    for mut sprite in &mut powerups {
        sprite.color = Color::srgb_u8(r, g, b);
    }
}

pub(crate) fn destroy_visible_enemies<F: QueryFilter>(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    score_board: &mut ScoreBoard,
    active_powerups: impl IntoIterator<Item = Entity>,
    active_sparkles: &Query<Entity, With<PowerUpSparkle>>,
    enemy_tanks: &mut Query<(Entity, &Tank, &mut Transform, &EnemyTank), F>,
) {
    let mut destroyed_any = false;
    let mut powerup_drop = None;
    for (enemy_entity, enemy_tank, mut transform, enemy) in enemy_tanks {
        score_board.record_enemy_destroyed(enemy.kind);
        if powerup_drop.is_none()
            && let Some(powerup) = enemy.carried_powerup
        {
            powerup_drop = Some((enemy_tank.top_left, powerup));
        }
        mark_enemy_tank_destroyed(
            commands,
            assets,
            enemy_entity,
            enemy_tank.top_left,
            &mut transform,
        );
        destroyed_any = true;
    }

    if destroyed_any {
        play_sound(commands, sounds, SoundKind::TankExplosion);
    }
    if let Some((top_left, powerup)) = powerup_drop {
        spawn_powerup(
            commands,
            assets,
            powerup,
            top_left,
            active_powerups,
            active_sparkles,
        );
    }
}

pub(crate) fn shovel_reinforcement_positions<'a>(
    mode: GameMode,
    player: PlayerId,
    tile_grid: &TileGrid,
    bases: impl IntoIterator<Item = &'a BaseSprite>,
) -> Vec<(usize, usize)> {
    match mode {
        GameMode::Campaign | GameMode::CoopCampaign => base_wall_positions(tile_grid),
        GameMode::VersusBaseBattle => bases
            .into_iter()
            .find(|base| base.owner == Some(player))
            .map(|base| base_wall_positions_for_top_left(tile_grid, base.top_left))
            .unwrap_or_default(),
        GameMode::VersusDeathmatch => Vec::new(),
    }
}

pub(crate) fn reinforce_base_walls(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    base_reinforcement: &mut BaseReinforcement,
    positions: Vec<(usize, usize)>,
) {
    if !base_reinforcement.saved_tiles.is_empty() {
        if reinforcement_matches_positions(&base_reinforcement.saved_tiles, &positions) {
            base_reinforcement.start();
            return;
        }

        restore_base_walls(
            commands,
            assets,
            tile_grid,
            tile_sprites,
            base_reinforcement,
        );
    }

    if positions.is_empty() {
        return;
    }

    base_reinforcement.saved_tiles = positions
        .iter()
        .map(|(x, y)| (*x, *y, tile_grid.tiles[y * BOARD_TILES + x]))
        .collect();

    for (x, y) in positions {
        sync_tile_sprite(
            commands,
            assets,
            tile_grid,
            tile_sprites,
            x,
            y,
            TileKind::Steel,
        );
    }
    base_reinforcement.start();
}

pub(crate) fn reinforcement_matches_positions(
    saved_tiles: &[(usize, usize, TileKind)],
    positions: &[(usize, usize)],
) -> bool {
    saved_tiles.len() == positions.len()
        && positions
            .iter()
            .all(|position| saved_tiles.iter().any(|(x, y, _)| (*x, *y) == *position))
}

fn restore_base_walls(
    commands: &mut Commands,
    assets: &SpriteAssets,
    tile_grid: &mut TileGrid,
    tile_sprites: &Query<(Entity, &GridTile)>,
    base_reinforcement: &mut BaseReinforcement,
) {
    let saved_tiles = std::mem::take(&mut base_reinforcement.saved_tiles);
    for (x, y, tile) in saved_tiles {
        sync_tile_sprite(commands, assets, tile_grid, tile_sprites, x, y, tile);
    }
    base_reinforcement.reset();
}

pub(crate) fn carrier_powerup_for_spawn(
    spawn_number: usize,
    carriers: &[PowerUpCarrier],
) -> Option<PowerUpKind> {
    carriers
        .iter()
        .find(|carrier| carrier.enemy == spawn_number)
        .map(|carrier| carrier.kind)
}

pub(crate) fn powerup_for_cycle(index: usize) -> PowerUpKind {
    match index % 6 {
        0 => PowerUpKind::Star,
        1 => PowerUpKind::Helmet,
        2 => PowerUpKind::Clock,
        3 => PowerUpKind::Grenade,
        4 => PowerUpKind::Shovel,
        _ => PowerUpKind::Tank,
    }
}
