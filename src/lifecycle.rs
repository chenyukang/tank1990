use crate::*;

pub(crate) fn animate_sprites(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut animations: Query<(Entity, &mut Sprite, &mut SpriteAnimation)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (entity, mut sprite, mut animation) in &mut animations {
        animation.timer.tick(time.delta());
        if !animation.timer.just_finished() {
            continue;
        }

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if atlas.index >= animation.last {
            if animation.despawn_on_finish {
                commands.entity(entity).despawn();
            } else {
                atlas.index = animation.first;
            }
        } else {
            atlas.index += 1;
        }
    }
}

pub(crate) fn tick_destroyed_tanks(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut destroyed_tanks: Query<(Entity, &mut DestroyedTank)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (entity, mut destroyed_tank) in &mut destroyed_tanks {
        if destroyed_tank.tick(time.delta()) {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn tick_spawn_protections(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut protected_tanks: Query<(Entity, &mut SpawnProtection)>,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut protection) in &mut protected_tanks {
        if protection.tick(time.delta()) {
            commands.entity(entity).remove::<SpawnProtection>();
        }
    }
}

pub(crate) fn tick_player_respawns(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<SpriteAssets>,
    game_status: Res<GameStatus>,
    score_board: Res<ScoreBoard>,
    mut pending_players: Query<(
        Entity,
        &mut PlayerRespawnPending,
        &RespawnPoint,
        &mut PlayerUpgrade,
        &mut Transform,
        &mut Sprite,
        &mut TankSpriteState,
    )>,
    active_tanks: Query<&Tank>,
    mut respawning_players: Query<(Entity, &mut PlayerRespawnDelay)>,
) {
    let can_process_pending_respawns = game_status.is_playing();
    let can_tick_spawn_delay = player_spawn_delay_can_tick(game_status.phase);

    if !can_process_pending_respawns && !can_tick_spawn_delay {
        return;
    }

    if can_process_pending_respawns {
        let mut occupied_positions: Vec<Vec2> =
            active_tanks.iter().map(|tank| tank.top_left).collect();

        for (
            entity,
            mut pending_respawn,
            respawn,
            mut upgrade,
            mut transform,
            mut sprite,
            mut tank_sprite,
        ) in &mut pending_players
        {
            if !pending_respawn.tick(time.delta()) {
                continue;
            }
            if !tank_spawn_position_free(respawn.top_left, &occupied_positions) {
                continue;
            }

            tank_sprite.frame = 0;
            tank_sprite.timer.reset();
            reset_player_upgrade(&mut upgrade, &mut sprite);
            set_tank_sprite_direction(&mut sprite, &tank_sprite, respawn.facing, &assets.manifest);
            transform.translation = board_object_center(
                respawn.top_left.x,
                respawn.top_left.y,
                Vec2::splat(TANK_SIZE),
                6.0,
            );

            commands
                .entity(entity)
                .remove::<PlayerRespawnPending>()
                .insert((
                    Tank {
                        top_left: respawn.top_left,
                        facing: respawn.facing,
                        speed: PLAYER_SPEED,
                    },
                    Health { current: 1 },
                    Shield {
                        timer: Timer::from_seconds(
                            score_board.respawn_invulnerability_secs,
                            TimerMode::Once,
                        ),
                    },
                    PlayerRespawnDelay::for_spawn_shimmer(assets.manifest.spawn_shimmer_frames()),
                ));
            occupied_positions.push(respawn.top_left);
            spawn_spawn_effect(&mut commands, &assets, respawn.top_left);
        }
    }

    if can_tick_spawn_delay {
        for (entity, mut respawn_delay) in &mut respawning_players {
            if respawn_delay.tick(time.delta()) {
                commands.entity(entity).remove::<PlayerRespawnDelay>();
            }
        }
    }
}

pub(crate) fn tick_shields(
    mut commands: Commands,
    time: Res<Time>,
    game_status: Res<GameStatus>,
    mut shielded: Query<
        (Entity, &mut Shield, &PlayerUpgrade, &mut Sprite),
        (
            With<Player>,
            Without<PlayerRespawnDelay>,
            Without<PlayerRespawnPending>,
            Without<DestroyedTank>,
        ),
    >,
) {
    if !game_status.is_playing() {
        return;
    }

    for (entity, mut shield, upgrade, mut sprite) in &mut shielded {
        shield.timer.tick(time.delta());
        let [r, g, b] = player_shield_visual_rgb(shield.timer.elapsed_secs(), upgrade.level);
        sprite.color = Color::srgb_u8(r, g, b);

        if shield.timer.is_finished() {
            sprite.color = player_upgrade_visual_color(upgrade.level);
            commands.entity(entity).remove::<Shield>();
        }
    }
}

pub(crate) fn sync_shield_visuals(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    game_status: Res<GameStatus>,
    shielded_players: Query<
        (Entity, &Tank, &Shield),
        (
            With<Player>,
            Without<PlayerRespawnDelay>,
            Without<PlayerRespawnPending>,
            Without<DestroyedTank>,
            Without<ShieldVisual>,
        ),
    >,
    mut visuals: Query<(Entity, &ShieldVisual, &mut Transform, &mut Sprite), Without<Player>>,
) {
    if !shield_visuals_can_render(game_status.phase) {
        for (visual_entity, _, _, _) in &mut visuals {
            commands.entity(visual_entity).despawn();
        }
        return;
    }

    let shielded: Vec<(Entity, Vec2, f32)> = shielded_players
        .iter()
        .map(|(entity, tank, shield)| (entity, tank.top_left, shield.timer.elapsed_secs()))
        .collect();
    let mut matched_owners = Vec::new();

    for (visual_entity, visual, mut transform, mut sprite) in &mut visuals {
        let Some((_, top_left, elapsed_secs)) =
            shielded.iter().find(|(owner, _, _)| *owner == visual.owner)
        else {
            commands.entity(visual_entity).despawn();
            continue;
        };

        transform.translation = shield_visual_translation(*top_left);
        sprite.color = shield_visual_color(*elapsed_secs);
        matched_owners.push(visual.owner);
    }

    for (owner, top_left, elapsed_secs) in shielded {
        if matched_owners.contains(&owner) {
            continue;
        }

        commands.spawn((
            Sprite {
                image: assets.shield_image.clone(),
                color: shield_visual_color(elapsed_secs),
                ..default()
            },
            Transform::from_translation(shield_visual_translation(top_left))
                .with_scale(Vec3::splat(window_scale())),
            ShieldVisual { owner },
            GameEntity,
        ));
    }
}

fn shield_visuals_can_render(phase: GamePhase) -> bool {
    matches!(phase, GamePhase::Playing | GamePhase::Paused)
}

pub(crate) fn update_versus_frozen_player_visuals(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    versus_freeze: Res<VersusPlayerFreeze>,
    mut players: Query<
        (&Player, &PlayerUpgrade, Option<&Shield>, &mut Sprite),
        (
            With<Player>,
            Without<PlayerRespawnDelay>,
            Without<PlayerRespawnPending>,
            Without<DestroyedTank>,
        ),
    >,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    for (player, upgrade, shield, mut sprite) in &mut players {
        if versus_freeze.is_player_frozen(player.id) {
            let [r, g, b] = player_frozen_visual_rgb(time.elapsed_secs());
            sprite.color = Color::srgb_u8(r, g, b);
        } else if shield.is_none() {
            sprite.color = player_upgrade_visual_color(upgrade.level);
        }
    }
}

pub(crate) fn update_enemy_visual_feedback(
    time: Res<Time>,
    game_status: Res<GameStatus>,
    enemy_freeze: Res<EnemyFreeze>,
    mut enemies: Query<(&EnemyTank, &Health, Option<&SpawnProtection>, &mut Sprite)>,
) {
    if !visual_effects_can_advance(game_status.phase) {
        return;
    }

    let frozen = enemy_freeze.is_active();
    for (enemy, health, spawn_protection, mut sprite) in &mut enemies {
        sprite.color = enemy_visual_color(
            enemy.kind,
            enemy.carried_powerup,
            health.current,
            time.elapsed_secs(),
            spawn_protection.is_some(),
            frozen,
        );
    }
}
