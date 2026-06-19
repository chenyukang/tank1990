use crate::*;

pub(crate) fn move_player_tank(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut control: ResMut<PlayerControl>,
    assets: Res<SpriteAssets>,
    grid: Res<TileGrid>,
    game_status: Res<GameStatus>,
    mode_select: Option<Res<ModeSelect>>,
    versus_freeze: Res<VersusPlayerFreeze>,
    mut tank_queries: ParamSet<(
        Query<&Tank>,
        Query<
            (
                &mut Tank,
                &mut Sprite,
                &mut Transform,
                &mut TankSpriteState,
                &Player,
            ),
            (With<Player>, Without<PlayerRespawnDelay>),
        >,
    )>,
) {
    if !game_status.is_playing() {
        return;
    }

    let occupied: Vec<Vec2> = tank_queries.p0().iter().map(|tank| tank.top_left).collect();
    let use_3d_controls = mode_select
        .as_deref()
        .is_some_and(|mode_select| view_3d_should_render(mode_select, &game_status));

    for (mut tank, mut sprite, mut transform, mut tank_sprite, player) in &mut tank_queries.p1() {
        if versus_freeze.is_player_frozen(player.id) {
            update_tank_sprite(
                &mut sprite,
                &mut tank_sprite,
                tank.facing,
                false,
                time.delta(),
                &assets.manifest,
            );
            continue;
        }

        let Some(motion) = control.tank_motion(&keys, player.id, tank.facing, use_3d_controls)
        else {
            update_tank_sprite(
                &mut sprite,
                &mut tank_sprite,
                tank.facing,
                false,
                time.delta(),
                &assets.manifest,
            );
            continue;
        };

        tank.facing = motion.facing;

        let mut moved = false;
        if let Some(direction) = motion.movement
            && let Some(next) = tank_move_candidate(
                tank.top_left,
                direction,
                tank_move_speed(tank.speed, &grid, tank.top_left) * time.delta_secs(),
                &grid,
                &occupied,
            )
        {
            tank.top_left = next;
            transform.translation =
                board_object_center(next.x, next.y, Vec2::splat(TANK_SIZE), 6.0);
            moved = true;
        }
        update_tank_sprite(
            &mut sprite,
            &mut tank_sprite,
            tank.facing,
            moved,
            time.delta(),
            &assets.manifest,
        );
    }
}
