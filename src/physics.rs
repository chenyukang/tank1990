use crate::*;

pub(crate) fn snap_to_lane(top_left: &mut Vec2, direction: Direction) {
    match direction {
        Direction::Up | Direction::Down => {
            let snapped = (top_left.x / TILE_SIZE).round() * TILE_SIZE;
            if (top_left.x - snapped).abs() <= SNAP_DISTANCE {
                top_left.x = snapped;
            }
        }
        Direction::Left | Direction::Right => {
            let snapped = (top_left.y / TILE_SIZE).round() * TILE_SIZE;
            if (top_left.y - snapped).abs() <= SNAP_DISTANCE {
                top_left.y = snapped;
            }
        }
    }
}

pub(crate) fn tank_move_candidate(
    current: Vec2,
    direction: Direction,
    movement_distance: f32,
    grid: &TileGrid,
    occupied: &[Vec2],
) -> Option<Vec2> {
    if movement_distance <= 0.0 {
        return None;
    }

    let mut forward = current;
    snap_to_lane(&mut forward, direction);
    forward += direction.movement() * movement_distance;
    forward = round_vec2(forward);
    if tank_position_is_available(forward, current, grid, occupied) {
        return Some(forward);
    }

    tank_lane_assist_candidate(current, direction, movement_distance, grid, occupied)
}

fn tank_lane_assist_candidate(
    current: Vec2,
    direction: Direction,
    movement_distance: f32,
    grid: &TileGrid,
    occupied: &[Vec2],
) -> Option<Vec2> {
    let axis_delta = lane_axis_delta(current, direction)?;
    if axis_delta.abs() > LANE_ASSIST_MAX_DISTANCE {
        return None;
    }

    let target_lane = lane_assist_target(current, direction, axis_delta);
    let mut target_forward = target_lane;
    snap_to_lane(&mut target_forward, direction);
    target_forward += direction.movement() * movement_distance.min(TILE_SIZE);
    target_forward = round_vec2(target_forward);
    if !tank_position_is_available(target_lane, current, grid, occupied)
        || !tank_position_is_available(target_forward, current, grid, occupied)
    {
        return None;
    }

    let assist_step = axis_delta.clamp(-movement_distance, movement_distance);
    let candidate = round_vec2(lane_assist_target(current, direction, assist_step));
    tank_position_is_available(candidate, current, grid, occupied).then_some(candidate)
}

fn lane_axis_delta(top_left: Vec2, direction: Direction) -> Option<f32> {
    let value = match direction {
        Direction::Up | Direction::Down => top_left.x,
        Direction::Left | Direction::Right => top_left.y,
    };
    let snapped = (value / TILE_SIZE).round() * TILE_SIZE;
    let delta = snapped - value;
    (delta.abs() > f32::EPSILON).then_some(delta)
}

fn lane_assist_target(top_left: Vec2, direction: Direction, axis_delta: f32) -> Vec2 {
    match direction {
        Direction::Up | Direction::Down => Vec2::new(top_left.x + axis_delta, top_left.y),
        Direction::Left | Direction::Right => Vec2::new(top_left.x, top_left.y + axis_delta),
    }
}

fn tank_position_is_available(
    candidate: Vec2,
    current: Vec2,
    grid: &TileGrid,
    occupied: &[Vec2],
) -> bool {
    grid.can_tank_occupy(candidate) && tank_position_free(candidate, current, occupied)
}

pub(crate) fn spawn_bullet_position(tank_top_left: Vec2, direction: Direction) -> Vec2 {
    let center = tank_top_left + Vec2::splat(TANK_SIZE / 2.0);
    match direction {
        Direction::Up => Vec2::new(center.x - BULLET_SIZE / 2.0, tank_top_left.y - BULLET_SIZE),
        Direction::Down => Vec2::new(center.x - BULLET_SIZE / 2.0, tank_top_left.y + TANK_SIZE),
        Direction::Left => Vec2::new(tank_top_left.x - BULLET_SIZE, center.y - BULLET_SIZE / 2.0),
        Direction::Right => Vec2::new(tank_top_left.x + TANK_SIZE, center.y - BULLET_SIZE / 2.0),
    }
}

pub(crate) fn player_bullet_limit(upgrade_level: u8) -> usize {
    if upgrade_level >= 2 { 2 } else { 1 }
}

pub(crate) fn player_bullet_speed(upgrade_level: u8) -> f32 {
    if upgrade_level >= 1 {
        PLAYER_FAST_BULLET_SPEED
    } else {
        BULLET_SPEED
    }
}

pub(crate) fn player_bullets_break_steel(upgrade_level: u8, stage_rules: StageRules) -> bool {
    upgrade_level >= 3 && stage_rules.player_steel_destruction
}

pub(crate) fn bullet_destroys_tile(tile: TileKind, breaks_steel: bool) -> bool {
    matches!(tile, TileKind::Brick) || (breaks_steel && tile == TileKind::Steel)
}

pub(crate) fn bullet_tank_hit(
    start_top_left: Vec2,
    end_top_left: Vec2,
    tank_top_left: Vec2,
) -> Option<Vec2> {
    let (start, delta, steps) = bullet_sweep(start_top_left, end_top_left);

    for step in 1..=steps {
        let center = start + delta * (step as f32 / steps as f32);
        let impact_top_left = round_vec2(center - Vec2::splat(BULLET_SIZE / 2.0));
        if rects_overlap(
            impact_top_left,
            Vec2::splat(BULLET_SIZE),
            tank_top_left,
            Vec2::splat(TANK_SIZE),
        ) {
            return Some(impact_top_left);
        }
    }

    None
}

pub(crate) fn bullet_hit_is_before_tile(
    start_top_left: Vec2,
    impact_top_left: Vec2,
    tile_hit: Option<BulletTileHit>,
) -> bool {
    let Some(tile_hit) = tile_hit else {
        return true;
    };

    bullet_impact_distance_squared(start_top_left, impact_top_left)
        < bullet_impact_distance_squared(start_top_left, tile_hit.impact_top_left)
}

fn bullet_impact_distance_squared(start_top_left: Vec2, impact_top_left: Vec2) -> f32 {
    let start_center = start_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let impact_center = impact_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    start_center.distance_squared(impact_center)
}

fn bullet_sweep(start_top_left: Vec2, end_top_left: Vec2) -> (Vec2, Vec2, usize) {
    let start = start_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let end = end_top_left + Vec2::splat(BULLET_SIZE / 2.0);
    let delta = end - start;
    let steps = ((delta.length() / (TILE_SIZE / 2.0)).ceil() as usize).max(1);
    (start, delta, steps)
}

fn bullet_overlapped_tile_range(top_left: Vec2) -> Option<(usize, usize, usize, usize)> {
    let board_size = board_size();
    if top_left.x + BULLET_SIZE <= 0.0
        || top_left.y + BULLET_SIZE <= 0.0
        || top_left.x >= board_size
        || top_left.y >= board_size
    {
        return None;
    }

    let left = (top_left.x.max(0.0) / TILE_SIZE).floor() as usize;
    let right_edge = (top_left.x + BULLET_SIZE - 0.1)
        .max(0.0)
        .min(board_size - 0.1);
    let right = (right_edge / TILE_SIZE).floor() as usize;
    let top = (top_left.y.max(0.0) / TILE_SIZE).floor() as usize;
    let bottom_edge = (top_left.y + BULLET_SIZE - 0.1)
        .max(0.0)
        .min(board_size - 0.1);
    let bottom = (bottom_edge / TILE_SIZE).floor() as usize;

    Some((left, right, top, bottom))
}

fn bullet_blocking_tile_key(delta: Vec2, tile_x: usize, tile_y: usize) -> (i32, i32, i32) {
    if delta.x.abs() > delta.y.abs() {
        let primary = if delta.x < 0.0 {
            -(tile_x as i32)
        } else {
            tile_x as i32
        };
        (0, primary, tile_y as i32)
    } else if delta.y.abs() > delta.x.abs() {
        let primary = if delta.y < 0.0 {
            -(tile_y as i32)
        } else {
            tile_y as i32
        };
        (0, primary, tile_x as i32)
    } else {
        (1, tile_y as i32, tile_x as i32)
    }
}

fn first_blocking_tile_overlapped_by_bullet(
    grid: &TileGrid,
    top_left: Vec2,
    delta: Vec2,
) -> Option<(usize, usize, TileKind)> {
    let (left, right, top, bottom) = bullet_overlapped_tile_range(top_left)?;
    let mut best = None;

    for tile_y in top..=bottom {
        for tile_x in left..=right {
            let tile = grid.tiles[tile_y * BOARD_TILES + tile_x];
            if !tile.bullet_blocks() {
                continue;
            }

            let key = bullet_blocking_tile_key(delta, tile_x, tile_y);
            match best {
                Some((_, _, _, best_key)) if best_key <= key => {}
                _ => best = Some((tile_x, tile_y, tile, key)),
            }
        }
    }

    best.map(|(tile_x, tile_y, tile, _)| (tile_x, tile_y, tile))
}

pub(crate) fn bullet_blocking_tile_hit(
    grid: &TileGrid,
    start_top_left: Vec2,
    end_top_left: Vec2,
) -> Option<BulletTileHit> {
    let (start, delta, steps) = bullet_sweep(start_top_left, end_top_left);

    for step in 1..=steps {
        let center = start + delta * (step as f32 / steps as f32);
        let impact_top_left = round_vec2(center - Vec2::splat(BULLET_SIZE / 2.0));
        if let Some((tile_x, tile_y, tile)) =
            first_blocking_tile_overlapped_by_bullet(grid, impact_top_left, delta)
        {
            return Some(BulletTileHit {
                x: tile_x,
                y: tile_y,
                tile,
                impact_top_left,
            });
        }
    }

    None
}

pub(crate) fn tank_rects_overlap(a: Vec2, b: Vec2) -> bool {
    rects_overlap(a, Vec2::splat(TANK_SIZE), b, Vec2::splat(TANK_SIZE))
}

pub(crate) fn tank_position_free(candidate: Vec2, current: Vec2, occupied: &[Vec2]) -> bool {
    occupied
        .iter()
        .filter(|position| **position != current)
        .all(|position| !tank_rects_overlap(candidate, *position))
}

pub(crate) fn tank_spawn_position_free(candidate: Vec2, occupied: &[Vec2]) -> bool {
    occupied
        .iter()
        .all(|position| !tank_rects_overlap(candidate, *position))
}

pub(crate) fn bullet_positions_overlap(a: Vec2, b: Vec2) -> bool {
    rects_overlap(a, Vec2::splat(BULLET_SIZE), b, Vec2::splat(BULLET_SIZE))
}

pub(crate) fn bullet_paths_clash(
    a_start: Vec2,
    a_end: Vec2,
    b_start: Vec2,
    b_end: Vec2,
) -> Option<BulletPathClash> {
    if bullet_positions_overlap(a_start, b_start) {
        return Some(BulletPathClash {
            impact_top_left: bullet_clash_impact_top_left(a_start, b_start),
            time: 0.0,
        });
    }

    let a_delta = a_end - a_start;
    let b_delta = b_end - b_start;
    let relative_delta = b_delta - a_delta;
    let expanded_min = a_start - Vec2::splat(BULLET_SIZE);
    let expanded_max = a_start + Vec2::splat(BULLET_SIZE);

    let (x_entry, x_exit) =
        swept_axis_times(b_start.x, relative_delta.x, expanded_min.x, expanded_max.x)?;
    let (y_entry, y_exit) =
        swept_axis_times(b_start.y, relative_delta.y, expanded_min.y, expanded_max.y)?;

    let entry_time = x_entry.max(y_entry);
    let exit_time = x_exit.min(y_exit);
    if entry_time > exit_time || !(0.0..=1.0).contains(&entry_time) {
        return None;
    }

    Some(BulletPathClash {
        impact_top_left: round_vec2(bullet_clash_impact_top_left(
            a_start + a_delta * entry_time,
            b_start + b_delta * entry_time,
        )),
        time: entry_time,
    })
}

fn swept_axis_times(
    point: f32,
    delta: f32,
    expanded_min: f32,
    expanded_max: f32,
) -> Option<(f32, f32)> {
    if delta == 0.0 {
        return (point >= expanded_min && point <= expanded_max)
            .then_some((f32::NEG_INFINITY, f32::INFINITY));
    }

    let first = (expanded_min - point) / delta;
    let second = (expanded_max - point) / delta;
    Some((first.min(second), first.max(second)))
}

pub(crate) fn bullet_clash_impact_top_left(a: Vec2, b: Vec2) -> Vec2 {
    (a + b) / 2.0
}

pub(crate) fn rects_overlap(a: Vec2, a_size: Vec2, b: Vec2, b_size: Vec2) -> bool {
    a.x < b.x + b_size.x && a.x + a_size.x > b.x && a.y < b.y + b_size.y && a.y + a_size.y > b.y
}

pub(crate) fn round_vec2(value: Vec2) -> Vec2 {
    Vec2::new(value.x.round(), value.y.round())
}
