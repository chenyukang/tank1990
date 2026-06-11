use super::*;
use std::collections::{HashSet, VecDeque};

pub(super) fn tank_center(top_left: Vec2) -> Vec2 {
    top_left + Vec2::splat(TANK_SIZE / 2.0)
}

pub(super) fn closest_player_center(from_center: Vec2, player_top_lefts: &[Vec2]) -> Option<Vec2> {
    let mut closest = None;
    let mut closest_distance = f32::MAX;

    for player_top_left in player_top_lefts {
        let player_center = tank_center(*player_top_left);
        let distance = from_center.distance_squared(player_center);
        if distance < closest_distance {
            closest = Some(player_center);
            closest_distance = distance;
        }
    }

    closest
}

pub(super) fn axis_direction_toward(from_center: Vec2, target_center: Vec2) -> Direction {
    let delta = target_center - from_center;
    if delta.x.abs() > delta.y.abs() {
        if delta.x < 0.0 {
            Direction::Left
        } else {
            Direction::Right
        }
    } else if delta.y < 0.0 {
        Direction::Up
    } else {
        Direction::Down
    }
}

pub(super) fn aligned_fire_direction(from_center: Vec2, target_center: Vec2) -> Option<Direction> {
    let delta = target_center - from_center;
    if delta.x.abs() <= TILE_SIZE / 2.0 && delta.y.abs() >= TILE_SIZE {
        Some(if delta.y < 0.0 {
            Direction::Up
        } else {
            Direction::Down
        })
    } else if delta.y.abs() <= TILE_SIZE / 2.0 && delta.x.abs() >= TILE_SIZE {
        Some(if delta.x < 0.0 {
            Direction::Left
        } else {
            Direction::Right
        })
    } else {
        None
    }
}

pub(super) fn enemy_aim_direction(
    enemy_top_left: Vec2,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
) -> Option<Direction> {
    let enemy_center = tank_center(enemy_top_left);
    for player_top_left in player_top_lefts {
        if let Some(direction) = aligned_fire_direction(enemy_center, tank_center(*player_top_left))
        {
            return Some(direction);
        }
    }

    base_center.and_then(|base| aligned_fire_direction(enemy_center, base))
}

pub(super) fn select_enemy_direction(
    strategy: EnemyAiStrategy,
    difficulty_profile: EnemyDifficultyProfile,
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
    grid: &TileGrid,
) -> Direction {
    match strategy {
        EnemyAiStrategy::Classic => classic_enemy_direction(
            difficulty_profile,
            kind,
            top_left,
            current,
            player_top_lefts,
            base_center,
        ),
        EnemyAiStrategy::PathToObjective => path_to_objective_enemy_direction(
            difficulty_profile,
            kind,
            top_left,
            current,
            player_top_lefts,
            base_center,
            grid,
        )
        .unwrap_or_else(|| {
            classic_enemy_direction(
                difficulty_profile,
                kind,
                top_left,
                current,
                player_top_lefts,
                base_center,
            )
        }),
    }
}

#[cfg(test)]
pub(super) fn preferred_enemy_direction(
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
) -> Direction {
    classic_enemy_direction(
        EnemyDifficultyProfile::Normal,
        kind,
        top_left,
        current,
        player_top_lefts,
        base_center,
    )
}

pub(super) fn classic_enemy_direction(
    difficulty_profile: EnemyDifficultyProfile,
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
) -> Direction {
    if let Some(direction) = enemy_aim_direction(top_left, player_top_lefts, base_center) {
        return direction;
    }

    let own_center = tank_center(top_left);
    if let Some(player_center) = closest_player_center(own_center, player_top_lefts) {
        let delta = player_center - own_center;
        if delta.x.abs() > delta.y.abs() && delta.x.abs() > TANK_SIZE {
            return if delta.x < 0.0 {
                Direction::Left
            } else {
                Direction::Right
            };
        }
    }

    if let Some(base_center) = base_center {
        let delta = base_center - own_center;
        if delta.length_squared() > TANK_SIZE * TANK_SIZE {
            if enemy_should_roam_for_profile(kind, top_left, current, difficulty_profile) {
                return enemy_patrol_direction(kind, top_left, current);
            }
            return axis_direction_toward(own_center, base_center);
        }
    }

    enemy_patrol_direction(kind, top_left, current)
}

pub(super) fn path_to_objective_enemy_direction(
    difficulty_profile: EnemyDifficultyProfile,
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
    player_top_lefts: &[Vec2],
    base_center: Option<Vec2>,
    grid: &TileGrid,
) -> Option<Direction> {
    if let Some(direction) = enemy_aim_direction(top_left, player_top_lefts, base_center) {
        return Some(direction);
    }
    if enemy_should_roam_for_profile(kind, top_left, current, difficulty_profile) {
        return Some(enemy_patrol_direction(kind, top_left, current));
    }

    let targets = enemy_objective_targets(grid, player_top_lefts);
    path_direction_to_targets(grid, top_left, &targets)
}

pub(super) fn enemy_objective_targets(
    grid: &TileGrid,
    player_top_lefts: &[Vec2],
) -> Vec<GridPoint> {
    let mut targets = Vec::new();
    for player_top_left in player_top_lefts {
        if let Some(point) = tank_grid_position(*player_top_left) {
            targets.push(point);
        }
    }
    if let Some(base_top_left) = base_top_left_from_grid(grid) {
        targets.extend(base_approach_positions(grid, base_top_left));
    }
    targets
}

pub(super) fn base_approach_positions(grid: &TileGrid, base_top_left: Vec2) -> Vec<GridPoint> {
    let Some(base) = tank_grid_position(base_top_left) else {
        return Vec::new();
    };
    let mut positions = Vec::new();
    let min_x = base.x.saturating_sub(1);
    let max_x = (base.x + 2).min(BOARD_TILES - 2);
    for x in min_x..=max_x {
        if base.y >= 2 {
            positions.push(GridPoint { x, y: base.y - 2 });
        }
        if base.y + 2 <= BOARD_TILES - 2 {
            positions.push(GridPoint { x, y: base.y + 2 });
        }
    }

    let min_y = base.y.saturating_sub(1);
    let max_y = (base.y + 2).min(BOARD_TILES - 2);
    for y in min_y..=max_y {
        if base.x >= 2 {
            positions.push(GridPoint { x: base.x - 2, y });
        }
        if base.x + 2 <= BOARD_TILES - 2 {
            positions.push(GridPoint { x: base.x + 2, y });
        }
    }

    positions
        .into_iter()
        .filter(|point| grid.can_tank_occupy(grid_point_top_left(point)))
        .collect()
}

pub(super) fn path_direction_to_targets(
    grid: &TileGrid,
    top_left: Vec2,
    targets: &[GridPoint],
) -> Option<Direction> {
    let start = tank_grid_position(top_left)?;
    if targets.iter().any(|target| *target == start) {
        return None;
    }
    let target_set: HashSet<(usize, usize)> =
        targets.iter().map(|target| (target.x, target.y)).collect();
    if target_set.is_empty() {
        return None;
    }

    let mut visited = vec![false; BOARD_TILES * BOARD_TILES];
    let mut queue = VecDeque::new();
    visited[grid_point_index(start)] = true;
    queue.push_back((start, None));

    while let Some((point, first_direction)) = queue.pop_front() {
        for direction in path_neighbor_directions(first_direction) {
            let Some(next) = neighbor_grid_point(point, direction) else {
                continue;
            };
            let index = grid_point_index(next);
            if visited[index] || !grid.can_tank_occupy(grid_point_top_left(&next)) {
                continue;
            }
            let first_direction = first_direction.unwrap_or(direction);
            if target_set.contains(&(next.x, next.y)) {
                return Some(first_direction);
            }
            visited[index] = true;
            queue.push_back((next, Some(first_direction)));
        }
    }

    None
}

pub(super) fn path_neighbor_directions(first_direction: Option<Direction>) -> [Direction; 4] {
    match first_direction {
        Some(direction) => [
            direction,
            next_direction(direction),
            opposite_direction(direction),
            previous_direction(direction),
        ],
        None => [
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Up,
        ],
    }
}

pub(super) fn tank_grid_position(top_left: Vec2) -> Option<GridPoint> {
    let x = (top_left.x / TILE_SIZE).round() as i32;
    let y = (top_left.y / TILE_SIZE).round() as i32;
    if x < 0 || y < 0 || x as usize > BOARD_TILES - 2 || y as usize > BOARD_TILES - 2 {
        return None;
    }
    Some(GridPoint {
        x: x as usize,
        y: y as usize,
    })
}

pub(super) fn grid_point_index(point: GridPoint) -> usize {
    point.y * BOARD_TILES + point.x
}

pub(super) fn neighbor_grid_point(point: GridPoint, direction: Direction) -> Option<GridPoint> {
    match direction {
        Direction::Up => point.y.checked_sub(1).map(|y| GridPoint { y, ..point }),
        Direction::Down if point.y < BOARD_TILES - 2 => Some(GridPoint {
            y: point.y + 1,
            ..point
        }),
        Direction::Left => point.x.checked_sub(1).map(|x| GridPoint { x, ..point }),
        Direction::Right if point.x < BOARD_TILES - 2 => Some(GridPoint {
            x: point.x + 1,
            ..point
        }),
        Direction::Down | Direction::Right => None,
    }
}

pub(super) fn enemy_patrol_direction(
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
) -> Direction {
    if top_left.y < 20.0 {
        Direction::Down
    } else {
        enemy_roam_direction(kind, top_left, current)
    }
}

#[cfg(test)]
pub(super) fn enemy_should_roam(kind: EnemyKind, top_left: Vec2, current: Direction) -> bool {
    enemy_should_roam_for_profile(kind, top_left, current, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_should_roam_for_profile(
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
    difficulty_profile: EnemyDifficultyProfile,
) -> bool {
    enemy_roam_seed(kind, top_left, current)
        .is_multiple_of(enemy_roam_rate_for_profile(kind, difficulty_profile))
}

pub(super) fn enemy_roam_direction(
    kind: EnemyKind,
    top_left: Vec2,
    current: Direction,
) -> Direction {
    direction_from_index((enemy_roam_seed(kind, top_left, current) / enemy_roam_rate(kind)) % 4)
}

pub(super) fn enemy_roam_seed(kind: EnemyKind, top_left: Vec2, current: Direction) -> u32 {
    let tile_x = (top_left.x / TILE_SIZE).round() as u32;
    let tile_y = (top_left.y / TILE_SIZE).round() as u32;
    tile_x.wrapping_mul(31)
        ^ tile_y.wrapping_mul(17)
        ^ direction_index(current).wrapping_mul(13)
        ^ enemy_kind_index(kind).wrapping_mul(23)
}

pub(super) fn direction_index(direction: Direction) -> u32 {
    match direction {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    }
}

pub(super) fn direction_from_index(index: u32) -> Direction {
    match index % 4 {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        _ => Direction::Left,
    }
}

#[cfg(test)]
pub(super) fn enemy_alignment_fire_ready(kind: EnemyKind, elapsed_secs: f32) -> bool {
    enemy_alignment_fire_ready_for_profile(kind, elapsed_secs, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_alignment_fire_ready_for_profile(
    kind: EnemyKind,
    elapsed_secs: f32,
    difficulty_profile: EnemyDifficultyProfile,
) -> bool {
    elapsed_secs
        >= enemy_fire_interval_for_profile(kind, difficulty_profile) * ENEMY_ALIGNMENT_FIRE_FRACTION
}

pub(super) fn enemy_fire_slot_available(
    active_enemy_bullets: usize,
    active_for_tank: usize,
) -> bool {
    active_enemy_bullets < ENEMY_BULLET_LIMIT && active_for_tank < ENEMY_BULLET_LIMIT_PER_TANK
}

#[cfg(test)]
pub(super) fn enemy_random_fire_ready(top_left: Vec2, facing: Direction, kind: EnemyKind) -> bool {
    enemy_random_fire_ready_for_profile(top_left, facing, kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_random_fire_ready_for_profile(
    top_left: Vec2,
    facing: Direction,
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> bool {
    enemy_fire_seed(top_left, facing, kind)
        .is_multiple_of(enemy_random_fire_rate_for_profile(kind, difficulty_profile))
}

#[cfg(test)]
pub(super) fn enemy_random_fire_rate(kind: EnemyKind) -> u32 {
    enemy_random_fire_rate_for_profile(kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_random_fire_rate_for_profile(
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> u32 {
    let normal = match kind {
        EnemyKind::Power => 2,
        EnemyKind::Fast => 3,
        EnemyKind::Basic | EnemyKind::Armor => 4,
    };
    match difficulty_profile {
        EnemyDifficultyProfile::Normal => normal,
        EnemyDifficultyProfile::Hard => normal.saturating_sub(1).max(1),
    }
}

pub(super) fn enemy_fire_seed(top_left: Vec2, facing: Direction, kind: EnemyKind) -> u32 {
    let tile_x = (top_left.x / TILE_SIZE).round() as u32;
    let tile_y = (top_left.y / TILE_SIZE).round() as u32;
    tile_x.wrapping_mul(29)
        ^ tile_y.wrapping_mul(37)
        ^ direction_index(facing).wrapping_mul(11)
        ^ enemy_kind_index(kind).wrapping_mul(19)
}

pub(super) fn enemy_kind_index(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Basic => 0,
        EnemyKind::Fast => 1,
        EnemyKind::Power => 2,
        EnemyKind::Armor => 3,
    }
}

pub(super) fn enemy_roam_rate(kind: EnemyKind) -> u32 {
    enemy_roam_rate_for_profile(kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_roam_rate_for_profile(
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> u32 {
    let normal = match kind {
        EnemyKind::Fast => 2,
        EnemyKind::Power => 3,
        EnemyKind::Basic => 4,
        EnemyKind::Armor => 6,
    };
    match difficulty_profile {
        EnemyDifficultyProfile::Normal => normal,
        EnemyDifficultyProfile::Hard => normal + 2,
    }
}

pub(super) fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

pub(super) fn previous_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

pub(super) fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

pub(super) fn tank_move_speed(base_speed: f32, grid: &TileGrid, top_left: Vec2) -> f32 {
    if grid.tank_overlaps_tile(top_left, TileKind::Ice) {
        base_speed * ICE_SPEED_MULTIPLIER
    } else {
        base_speed
    }
}

#[cfg(test)]
pub(super) fn enemy_speed(kind: EnemyKind) -> f32 {
    enemy_speed_for_profile(kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_speed_for_profile(
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> f32 {
    let normal = match kind {
        EnemyKind::Fast => 72.0,
        EnemyKind::Power => 56.0,
        EnemyKind::Armor => 48.0,
        EnemyKind::Basic => 52.0,
    };
    normal * enemy_ai_tuning(difficulty_profile).speed_multiplier
}

pub(super) fn enemy_spawn_interval_for_profile(
    spawn_interval_secs: f32,
    difficulty_profile: EnemyDifficultyProfile,
) -> f32 {
    spawn_interval_secs * enemy_ai_tuning(difficulty_profile).spawn_interval_multiplier
}

pub(super) fn enemy_ai_tuning(profile: EnemyDifficultyProfile) -> EnemyAiTuning {
    match profile {
        EnemyDifficultyProfile::Normal => EnemyAiTuning {
            speed_multiplier: 1.0,
            turn_interval_multiplier: 1.0,
            fire_interval_multiplier: 1.0,
            spawn_interval_multiplier: 1.0,
        },
        EnemyDifficultyProfile::Hard => EnemyAiTuning {
            speed_multiplier: 1.08,
            turn_interval_multiplier: 0.75,
            fire_interval_multiplier: 0.78,
            spawn_interval_multiplier: 0.75,
        },
    }
}

#[derive(Clone, Copy)]
pub(super) struct EnemyAiTuning {
    speed_multiplier: f32,
    turn_interval_multiplier: f32,
    fire_interval_multiplier: f32,
    spawn_interval_multiplier: f32,
}

pub(super) fn enemy_health(kind: EnemyKind) -> i32 {
    match kind {
        EnemyKind::Armor => 3,
        _ => 1,
    }
}

#[cfg(test)]
pub(super) fn enemy_turn_interval(kind: EnemyKind) -> f32 {
    enemy_turn_interval_for_profile(kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_turn_interval_for_profile(
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> f32 {
    base_enemy_turn_interval(kind) * enemy_ai_tuning(difficulty_profile).turn_interval_multiplier
}

pub(super) fn base_enemy_turn_interval(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Fast => 0.8,
        EnemyKind::Power => 1.0,
        EnemyKind::Basic => 1.2,
        EnemyKind::Armor => 1.5,
    }
}

#[cfg(test)]
pub(super) fn enemy_fire_interval(kind: EnemyKind) -> f32 {
    enemy_fire_interval_for_profile(kind, EnemyDifficultyProfile::Normal)
}

pub(super) fn enemy_fire_interval_for_profile(
    kind: EnemyKind,
    difficulty_profile: EnemyDifficultyProfile,
) -> f32 {
    base_enemy_fire_interval(kind) * enemy_ai_tuning(difficulty_profile).fire_interval_multiplier
}

pub(super) fn base_enemy_fire_interval(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Power => 1.0,
        EnemyKind::Fast => 1.5,
        EnemyKind::Armor => 1.8,
        EnemyKind::Basic => 1.6,
    }
}

pub(super) fn enemy_bullet_speed(kind: EnemyKind) -> f32 {
    match kind {
        EnemyKind::Power => POWER_ENEMY_BULLET_SPEED,
        EnemyKind::Basic | EnemyKind::Fast | EnemyKind::Armor => BULLET_SPEED,
    }
}

pub(super) fn enemy_score(kind: EnemyKind) -> u32 {
    match kind {
        EnemyKind::Basic => 100,
        EnemyKind::Fast => 200,
        EnemyKind::Power => 300,
        EnemyKind::Armor => 400,
    }
}

pub(super) fn enemy_hit_sound(health_after_hit: i32) -> SoundKind {
    if health_after_hit <= 0 {
        SoundKind::TankExplosion
    } else {
        SoundKind::SteelHit
    }
}

pub(super) fn enemy_visual_color(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
    frozen: bool,
) -> Color {
    let [r, g, b] = enemy_display_rgb(
        kind,
        carried_powerup,
        health,
        elapsed_secs,
        spawn_protected,
        frozen,
    );
    Color::srgb_u8(r, g, b)
}

pub(super) fn enemy_display_rgb(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
    spawn_protected: bool,
    frozen: bool,
) -> [u8; 3] {
    if spawn_protected && elapsed_secs % 0.16 < 0.08 {
        return [160, 220, 255];
    }
    if frozen {
        return enemy_frozen_visual_rgb(elapsed_secs);
    }

    enemy_visual_rgb(kind, carried_powerup, health, elapsed_secs)
}

pub(super) fn enemy_frozen_visual_rgb(elapsed_secs: f32) -> [u8; 3] {
    if elapsed_secs % 0.24 < 0.12 {
        [136, 216, 255]
    } else {
        [216, 248, 255]
    }
}

pub(super) fn enemy_visual_rgb(
    kind: EnemyKind,
    carried_powerup: Option<PowerUpKind>,
    health: i32,
    elapsed_secs: f32,
) -> [u8; 3] {
    if carried_powerup.is_some() && elapsed_secs % 0.25 < 0.125 {
        return [248, 232, 96];
    }

    match (kind, health) {
        (EnemyKind::Armor, 1) => [248, 168, 88],
        (EnemyKind::Armor, 2) => [216, 96, 72],
        (EnemyKind::Armor, _) => [168, 184, 216],
        (EnemyKind::Power, _) => [248, 112, 112],
        (EnemyKind::Fast, _) => [112, 216, 128],
        (EnemyKind::Basic, _) => [255, 255, 255],
    }
}
