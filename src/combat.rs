use crate::*;
use std::collections::HashSet;

pub(crate) fn fire_enemy_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    grid: Res<TileGrid>,
    players: Query<&Tank, With<Player>>,
    enemy_bullets: Query<&EnemyBulletSource>,
    mut enemies: Query<
        (
            Entity,
            &mut Tank,
            &EnemyTank,
            &mut EnemyAi,
            &mut Sprite,
            &TankSpriteState,
        ),
        (With<EnemyTank>, Without<Player>, Without<SpawnProtection>),
    >,
) {
    if !game_status.is_playing() || enemy_freeze.is_active() {
        return;
    }

    let player_top_lefts: Vec<Vec2> = players.iter().map(|tank| tank.top_left).collect();
    let base_center = base_center_from_grid(&grid);
    let mut active_enemy_bullet_shooters: Vec<Entity> =
        enemy_bullets.iter().map(|source| source.shooter).collect();
    if active_enemy_bullet_shooters.len() >= ENEMY_BULLET_LIMIT {
        return;
    }

    for (enemy_entity, mut tank, enemy, mut ai, mut sprite, tank_sprite) in &mut enemies {
        ai.fire_timer.tick(time.delta());
        let active_for_tank = active_enemy_bullet_shooters
            .iter()
            .filter(|shooter| **shooter == enemy_entity)
            .count();
        if !enemy_fire_slot_available(active_enemy_bullet_shooters.len(), active_for_tank) {
            continue;
        }

        let aim_direction = enemy_aim_direction(tank.top_left, &player_top_lefts, base_center);
        let snap_fire_ready = aim_direction.is_some()
            && enemy_alignment_fire_ready_for_profile(
                enemy.kind,
                ai.fire_timer.elapsed_secs(),
                ai.difficulty_profile,
            );
        if let Some(direction) = aim_direction {
            if !ai.fire_timer.just_finished() && !snap_fire_ready {
                continue;
            }
            tank.facing = direction;
            set_tank_sprite_direction(&mut sprite, tank_sprite, tank.facing, &assets.manifest);
        } else if !ai.fire_timer.just_finished()
            || !enemy_random_fire_ready_for_profile(
                tank.top_left,
                tank.facing,
                enemy.kind,
                ai.difficulty_profile,
            )
        {
            continue;
        }

        let bullet_top_left = spawn_bullet_position(tank.top_left, tank.facing);
        commands.spawn((
            Sprite::from_atlas_image(
                assets.bullet_image.clone(),
                TextureAtlas {
                    layout: assets.bullet_layout.clone(),
                    index: assets.manifest.bullet_index(tank.facing),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(window_scale())),
            Bullet {
                previous_top_left: bullet_top_left,
                top_left: bullet_top_left,
                facing: tank.facing,
                owner: Team::Enemy,
                speed: enemy_bullet_speed(enemy.kind),
                breaks_steel: false,
                resolved: false,
            },
            EnemyBulletSource {
                shooter: enemy_entity,
            },
            GameEntity,
        ));
        play_sound(&mut commands, &sounds, SoundKind::Fire);
        ai.fire_timer.reset();
        active_enemy_bullet_shooters.push(enemy_entity);

        if enemy.kind == EnemyKind::Power
            || active_enemy_bullet_shooters.len() >= ENEMY_BULLET_LIMIT
        {
            break;
        }
    }
}

pub(crate) fn fire_player_bullet(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    stage_rules: Res<StageRules>,
    versus_freeze: Res<VersusPlayerFreeze>,
    players: Query<(&Tank, &PlayerUpgrade, &Player), (With<Player>, Without<PlayerRespawnDelay>)>,
    bullets: Query<&Bullet>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (tank, upgrade, player) in &players {
        if versus_freeze.is_player_frozen(player.id) {
            continue;
        }

        if !player_fire_pressed(&keys, player.id) {
            continue;
        }

        let owner = player.id.team();
        let active_player_bullets = bullets
            .iter()
            .filter(|bullet| bullet.owner == owner)
            .count();
        if active_player_bullets >= player_bullet_limit(upgrade.level) {
            continue;
        }

        let bullet_top_left = spawn_bullet_position(tank.top_left, tank.facing);
        commands.spawn((
            Sprite::from_atlas_image(
                assets.bullet_image.clone(),
                TextureAtlas {
                    layout: assets.bullet_layout.clone(),
                    index: assets.manifest.bullet_index(tank.facing),
                },
            ),
            Transform::from_translation(board_object_center(
                bullet_top_left.x,
                bullet_top_left.y,
                Vec2::splat(BULLET_SIZE),
                7.0,
            ))
            .with_scale(Vec3::splat(window_scale())),
            Bullet {
                previous_top_left: bullet_top_left,
                top_left: bullet_top_left,
                facing: tank.facing,
                owner,
                speed: player_bullet_speed(upgrade.level),
                breaks_steel: player_bullets_break_steel(upgrade.level, *stage_rules),
                resolved: false,
            },
            GameEntity,
        ));
        play_sound(&mut commands, &sounds, SoundKind::Fire);
    }
}

pub(crate) fn move_bullets(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_mode: Res<GameMode>,
    mut grid: ResMut<TileGrid>,
    mut game_status: ResMut<GameStatus>,
    mut score_board: ResMut<ScoreBoard>,
    mut bullets: Query<(Entity, &mut Bullet, &mut Transform), With<Bullet>>,
    tile_sprites: Query<(Entity, &GridTile)>,
    active_powerups: Query<Entity, With<PowerUp>>,
    active_sparkles: Query<Entity, With<PowerUpSparkle>>,
    mut base_sprites: Query<(&BaseSprite, &mut Sprite), (Without<Player>, Without<Bullet>)>,
    mut enemy_tanks: Query<
        (
            Entity,
            &Tank,
            &mut Transform,
            &EnemyTank,
            &mut Health,
            Option<&SpawnProtection>,
        ),
        (With<EnemyTank>, Without<Player>, Without<Bullet>),
    >,
    mut player_tanks: Query<
        (
            Entity,
            &mut Tank,
            &mut Transform,
            &mut PlayerLives,
            &mut Health,
            &mut PlayerUpgrade,
            Option<&Shield>,
            &Player,
        ),
        (
            With<Player>,
            Without<BaseSprite>,
            Without<EnemyTank>,
            Without<Bullet>,
            Without<PowerUp>,
        ),
    >,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut bullet, mut transform) in &mut bullets {
        let facing = bullet.facing;
        let speed = bullet.speed;
        let previous_top_left = bullet.top_left;
        bullet.previous_top_left = previous_top_left;
        bullet.top_left += facing.movement() * speed * time.delta_secs();
        bullet.top_left = round_vec2(bullet.top_left);

        let center = bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0);
        if center.x < 0.0 || center.y < 0.0 || center.x >= board_size() || center.y >= board_size()
        {
            resolve_bullet(&mut commands, entity, &mut bullet);
            continue;
        }

        let tile_hit = bullet_blocking_tile_hit(&grid, previous_top_left, bullet.top_left);

        if game_mode.is_campaign() && bullet.owner.is_player() {
            let mut hit_enemy = false;
            for (
                enemy_entity,
                enemy_tank,
                mut enemy_transform,
                enemy,
                mut health,
                spawn_protection,
            ) in &mut enemy_tanks
            {
                if let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, enemy_tank.top_left)
                {
                    if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                        continue;
                    }

                    if spawn_protection.is_some() {
                        spawn_directed_bullet_impact_effect(
                            &mut commands,
                            &assets,
                            impact_top_left,
                            facing,
                        );
                        resolve_bullet(&mut commands, entity, &mut bullet);
                        play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                        hit_enemy = true;
                        break;
                    }

                    health.current -= 1;
                    let hit_sound = enemy_hit_sound(health.current);
                    if health.current <= 0 {
                        let enemy_top_left = enemy_tank.top_left;
                        score_board.record_enemy_destroyed(enemy.kind);
                        mark_enemy_tank_destroyed(
                            &mut commands,
                            &assets,
                            enemy_entity,
                            enemy_top_left,
                            &mut enemy_transform,
                        );
                        if let Some(powerup_kind) = enemy.carried_powerup {
                            spawn_powerup(
                                &mut commands,
                                &assets,
                                powerup_kind,
                                enemy_top_left,
                                active_powerups.iter(),
                                &active_sparkles,
                            );
                        }
                    } else {
                        spawn_directed_bullet_impact_effect(
                            &mut commands,
                            &assets,
                            impact_top_left,
                            facing,
                        );
                    }
                    play_sound(&mut commands, &sounds, hit_sound);
                    resolve_bullet(&mut commands, entity, &mut bullet);
                    hit_enemy = true;
                    break;
                }
            }
            if hit_enemy {
                continue;
            }
        }

        if game_mode.is_versus()
            && let Some(shooter) = bullet.owner.player_id()
        {
            let mut hit_player = false;
            for (
                player_entity,
                mut player_tank,
                mut player_transform,
                mut lives,
                mut player_health,
                mut upgrade,
                shield,
                player,
            ) in &mut player_tanks
            {
                if player.id == shooter {
                    continue;
                }
                let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, player_tank.top_left)
                else {
                    continue;
                };
                if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                    continue;
                }

                if shield.is_none() {
                    spawn_explosion(&mut commands, &assets, player_tank.top_left);
                    play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                    resolve_player_destroyed(
                        &mut commands,
                        &assets,
                        &sounds,
                        &mut game_status,
                        &mut score_board,
                        player_entity,
                        &mut player_tank,
                        &mut player_transform,
                        &mut lives,
                        &mut player_health,
                        &mut upgrade,
                        player.id,
                        Some(shooter),
                        *game_mode,
                    );
                } else {
                    spawn_directed_bullet_impact_effect(
                        &mut commands,
                        &assets,
                        impact_top_left,
                        facing,
                    );
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                }
                resolve_bullet(&mut commands, entity, &mut bullet);
                hit_player = true;
                break;
            }
            if hit_player {
                continue;
            }
        }

        if bullet.owner == Team::Enemy {
            let mut hit_player = false;
            for (
                player_entity,
                mut player_tank,
                mut player_transform,
                mut lives,
                mut player_health,
                mut upgrade,
                shield,
                player,
            ) in &mut player_tanks
            {
                let Some(impact_top_left) =
                    bullet_tank_hit(previous_top_left, bullet.top_left, player_tank.top_left)
                else {
                    continue;
                };
                if !bullet_hit_is_before_tile(previous_top_left, impact_top_left, tile_hit) {
                    continue;
                }

                if shield.is_some() {
                    spawn_directed_bullet_impact_effect(
                        &mut commands,
                        &assets,
                        impact_top_left,
                        facing,
                    );
                    resolve_bullet(&mut commands, entity, &mut bullet);
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                    hit_player = true;
                    break;
                }

                spawn_explosion(&mut commands, &assets, player_tank.top_left);
                play_sound(&mut commands, &sounds, SoundKind::TankExplosion);
                resolve_player_destroyed(
                    &mut commands,
                    &assets,
                    &sounds,
                    &mut game_status,
                    &mut score_board,
                    player_entity,
                    &mut player_tank,
                    &mut player_transform,
                    &mut lives,
                    &mut player_health,
                    &mut upgrade,
                    player.id,
                    None,
                    *game_mode,
                );
                resolve_bullet(&mut commands, entity, &mut bullet);
                hit_player = true;
                break;
            }
            if hit_player {
                continue;
            }
        }

        if let Some(tile_hit) = tile_hit {
            spawn_directed_bullet_impact_effect(
                &mut commands,
                &assets,
                tile_hit.impact_top_left,
                facing,
            );
            if bullet_destroys_tile(tile_hit.tile, bullet.breaks_steel) {
                grid.set(tile_hit.x, tile_hit.y, TileKind::Empty);
                play_sound(
                    &mut commands,
                    &sounds,
                    if tile_hit.tile == TileKind::Steel {
                        SoundKind::SteelHit
                    } else {
                        SoundKind::BrickHit
                    },
                );
                for (tile_entity, grid_tile) in &tile_sprites {
                    if grid_tile.x == tile_hit.x && grid_tile.y == tile_hit.y {
                        commands.entity(tile_entity).despawn();
                        break;
                    }
                }
            }

            if tile_hit.tile == TileKind::Base && game_status.is_playing() {
                let mut hit_base = None;
                for (base, mut sprite) in &mut base_sprites {
                    if base_contains_tile(base.top_left, tile_hit.x, tile_hit.y) {
                        let can_destroy =
                            base_can_be_destroyed_by_bullet(*game_mode, bullet.owner, base.owner);
                        if can_destroy {
                            sprite.image = assets.base_destroyed.clone();
                        }
                        hit_base = Some((base.owner, base.top_left, can_destroy));
                        break;
                    }
                }

                let (base_owner, base_top_left, can_destroy_base) = hit_base.unwrap_or((
                    None,
                    base_top_left_from_grid(&grid).unwrap_or(Vec2::new(
                        tile_hit.x as f32 * TILE_SIZE,
                        tile_hit.y as f32 * TILE_SIZE,
                    )),
                    base_can_be_destroyed_by_bullet(*game_mode, bullet.owner, None),
                ));

                if can_destroy_base {
                    let sound_sequence = base_destroyed_sounds(*game_mode, base_owner);
                    match *game_mode {
                        GameMode::Campaign | GameMode::CoopCampaign => {
                            game_status.phase = GamePhase::GameOver;
                        }
                        GameMode::VersusBaseBattle => {
                            if let Some(owner) = base_owner {
                                game_status.phase = GamePhase::RoundOver;
                                game_status.winner = Some(base_battle_winner_for_base(owner));
                            }
                        }
                        GameMode::VersusDeathmatch => {}
                    }

                    for sound in sound_sequence {
                        play_sound(&mut commands, &sounds, *sound);
                    }
                    spawn_base_destruction_effect(&mut commands, &assets, base_top_left);
                } else {
                    play_sound(&mut commands, &sounds, SoundKind::SteelHit);
                }
            } else if tile_hit.tile == TileKind::Steel && !bullet.breaks_steel {
                play_sound(&mut commands, &sounds, SoundKind::SteelHit);
            }

            resolve_bullet(&mut commands, entity, &mut bullet);
            continue;
        }

        transform.translation = board_object_center(
            bullet.top_left.x,
            bullet.top_left.y,
            Vec2::splat(BULLET_SIZE),
            7.0,
        );
    }
}

fn resolve_bullet(commands: &mut Commands, entity: Entity, bullet: &mut Bullet) {
    bullet.resolved = true;
    commands.entity(entity).despawn();
}

pub(crate) fn resolve_player_destroyed(
    commands: &mut Commands,
    assets: &SpriteAssets,
    sounds: &SoundAssets,
    game_status: &mut GameStatus,
    score_board: &mut ScoreBoard,
    player_entity: Entity,
    tank: &mut Tank,
    transform: &mut Transform,
    lives: &mut PlayerLives,
    health: &mut Health,
    upgrade: &mut PlayerUpgrade,
    target: PlayerId,
    shooter: Option<PlayerId>,
    mode: GameMode,
) {
    lives.current -= 1;
    health.current = 1;

    match mode {
        GameMode::Campaign => {
            score_board.lives = lives.current;
            if lives.current <= 0 {
                game_status.phase = GamePhase::GameOver;
                play_sound(commands, sounds, SoundKind::GameOver);
                mark_player_tank_destroyed_terminal(
                    commands,
                    assets,
                    player_entity,
                    tank,
                    transform,
                );
                return;
            }
        }
        GameMode::CoopCampaign => {
            score_board.set_coop_player_lives(target, lives.current);
            if lives.current <= 0 {
                if score_board.lives <= 0 {
                    game_status.phase = GamePhase::GameOver;
                    play_sound(commands, sounds, SoundKind::GameOver);
                }
                mark_player_tank_destroyed_terminal(
                    commands,
                    assets,
                    player_entity,
                    tank,
                    transform,
                );
                return;
            }
        }
        GameMode::VersusDeathmatch => {
            score_board.set_player_lives(target, lives.current);
            if let Some(shooter) = shooter {
                score_board.add_player_score(shooter);
                if let Some(winner) = deathmatch_winner_after_hit(
                    score_board.player_score(shooter),
                    lives.current,
                    score_board.target_score,
                    shooter,
                ) {
                    game_status.phase = GamePhase::RoundOver;
                    game_status.winner = Some(winner);
                    play_sound(commands, sounds, SoundKind::LevelClear);
                    mark_player_tank_destroyed_terminal(
                        commands,
                        assets,
                        player_entity,
                        tank,
                        transform,
                    );
                    return;
                }
            }
        }
        GameMode::VersusBaseBattle => {
            score_board.set_player_lives(target, lives.current);
            if let Some(shooter) = shooter {
                score_board.add_player_score(shooter);
                if lives.current <= 0 {
                    game_status.phase = GamePhase::RoundOver;
                    game_status.winner = Some(shooter);
                    play_sound(commands, sounds, SoundKind::LevelClear);
                    mark_player_tank_destroyed_terminal(
                        commands,
                        assets,
                        player_entity,
                        tank,
                        transform,
                    );
                    return;
                }
            }
        }
    }

    mark_player_tank_destroyed_for_respawn(
        commands,
        assets,
        player_entity,
        tank,
        transform,
        upgrade,
    );
}

pub(crate) fn reset_player_upgrade(upgrade: &mut PlayerUpgrade, sprite: &mut Sprite) {
    upgrade.level = 0;
    sprite.color = player_upgrade_visual_color(upgrade.level);
}

pub(crate) fn deathmatch_winner_after_hit(
    shooter_score: u32,
    target_lives: i32,
    target_score: u32,
    shooter: PlayerId,
) -> Option<PlayerId> {
    if shooter_score >= target_score || target_lives <= 0 {
        Some(shooter)
    } else {
        None
    }
}

pub(crate) fn base_battle_winner_for_base(base_owner: PlayerId) -> PlayerId {
    base_owner.opponent()
}

pub(crate) fn base_can_be_destroyed_by_bullet(
    game_mode: GameMode,
    bullet_owner: Team,
    base_owner: Option<PlayerId>,
) -> bool {
    match game_mode {
        GameMode::Campaign | GameMode::CoopCampaign => true,
        GameMode::VersusBaseBattle => {
            matches!((bullet_owner.player_id(), base_owner), (Some(shooter), Some(owner)) if shooter != owner)
        }
        GameMode::VersusDeathmatch => false,
    }
}

pub(crate) fn base_destroyed_sounds(
    game_mode: GameMode,
    base_owner: Option<PlayerId>,
) -> &'static [SoundKind] {
    match game_mode {
        GameMode::Campaign | GameMode::CoopCampaign => &CAMPAIGN_BASE_DESTROYED_SOUNDS,
        GameMode::VersusBaseBattle if base_owner.is_some() => &VERSUS_BASE_DESTROYED_SOUNDS,
        GameMode::VersusBaseBattle | GameMode::VersusDeathmatch => &NO_BASE_DESTROYED_SOUNDS,
    }
}

pub(crate) fn cancel_colliding_bullets(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    sounds: Res<SoundAssets>,
    game_status: Res<GameStatus>,
    bullets: Query<(Entity, &Bullet)>,
) {
    if !bullet_clashes_can_resolve(game_status.phase) {
        return;
    }

    let bullets: Vec<(Entity, Vec2, Vec2)> = bullets
        .iter()
        .filter(|(_, bullet)| !bullet.resolved)
        .map(|(entity, bullet)| (entity, bullet.previous_top_left, bullet.top_left))
        .collect();
    let mut destroyed = HashSet::new();
    let mut clashes = Vec::new();

    for i in 0..bullets.len() {
        for j in (i + 1)..bullets.len() {
            if let Some(clash) =
                bullet_paths_clash(bullets[i].1, bullets[i].2, bullets[j].1, bullets[j].2)
            {
                clashes.push((
                    clash.time,
                    bullets[i].0,
                    bullets[j].0,
                    clash.impact_top_left,
                ));
            }
        }
    }

    clashes.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut index = 0;
    while index < clashes.len() {
        let group_time = clashes[index].0;
        let mut group_destroyed = HashSet::new();
        let mut impact_top_lefts = Vec::new();

        while index < clashes.len()
            && (clashes[index].0 - group_time).abs() <= BULLET_CLASH_TIME_EPSILON
        {
            let (_, first, second, impact_top_left) = clashes[index];
            if !destroyed.contains(&first) && !destroyed.contains(&second) {
                group_destroyed.insert(first);
                group_destroyed.insert(second);
                if !impact_top_lefts.contains(&impact_top_left) {
                    impact_top_lefts.push(impact_top_left);
                }
            }
            index += 1;
        }

        if !group_destroyed.is_empty() {
            destroyed.extend(group_destroyed);
            for impact_top_left in impact_top_lefts {
                spawn_bullet_impact_effect(&mut commands, &assets, impact_top_left);
                play_sound(&mut commands, &sounds, SoundKind::SteelHit);
            }
        }
    }

    for entity in destroyed {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn bullet_clashes_can_resolve(phase: GamePhase) -> bool {
    phase == GamePhase::Playing
}
