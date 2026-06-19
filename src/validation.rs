use crate::*;
use std::collections::HashSet;

pub(crate) fn validate_level_positions(
    level: &LevelDefinition,
    grid: &TileGrid,
) -> Result<(), String> {
    validate_tank_spawn(grid, "player spawn", &level.player_spawn)?;

    for (index, spawn) in level.enemy_spawns.iter().enumerate() {
        let label = format!("enemy spawn {}", index + 1);
        validate_tank_spawn(grid, &label, spawn)?;
    }

    validate_base_position(grid, "base position", &level.base_position)?;
    validate_classic_campaign_base_position(&level.base_position)
}

fn validate_tank_spawn(grid: &TileGrid, label: &str, spawn: &SpawnPoint) -> Result<(), String> {
    let top_left = Vec2::new(spawn.x as f32 * TILE_SIZE, spawn.y as f32 * TILE_SIZE);
    if grid.can_tank_occupy(top_left) {
        Ok(())
    } else {
        Err(format!(
            "{label} ({}, {}) must fit a tank on passable tiles",
            spawn.x, spawn.y
        ))
    }
}

pub(crate) fn validate_arena_spawns(
    grid: &TileGrid,
    arena: &ArenaDefinition,
) -> Result<(), String> {
    validate_tank_spawn(grid, "p1 spawn", &arena.p1_spawn)?;
    validate_tank_spawn(grid, "p2 spawn", &arena.p2_spawn)?;

    let p1_top_left = spawn_point_top_left(&arena.p1_spawn);
    let p2_top_left = spawn_point_top_left(&arena.p2_spawn);
    if tank_rects_overlap(p1_top_left, p2_top_left) {
        return Err(format!(
            "p1 spawn ({}, {}) and p2 spawn ({}, {}) must not overlap",
            arena.p1_spawn.x, arena.p1_spawn.y, arena.p2_spawn.x, arena.p2_spawn.y
        ));
    }

    Ok(())
}

pub(crate) fn validate_base_position(
    grid: &TileGrid,
    label: &str,
    point: &GridPoint,
) -> Result<(), String> {
    if point.x >= BOARD_TILES - 1 || point.y >= BOARD_TILES - 1 {
        return Err(format!(
            "{label} ({}, {}) must fit a 2x2 base inside the battlefield",
            point.x, point.y
        ));
    }

    for y in point.y..=(point.y + 1) {
        for x in point.x..=(point.x + 1) {
            if grid.get(x as i32, y as i32) != Some(TileKind::Base) {
                return Err(format!(
                    "{label} ({}, {}) must cover a 2x2 base tile area",
                    point.x, point.y
                ));
            }
        }
    }

    Ok(())
}

fn validate_classic_campaign_base_position(point: &GridPoint) -> Result<(), String> {
    if point.x == CLASSIC_BASE_X && point.y == CLASSIC_BASE_Y {
        return Ok(());
    }

    Err(format!(
        "base position ({}, {}) must use classic campaign base ({CLASSIC_BASE_X}, {CLASSIC_BASE_Y})",
        point.x, point.y
    ))
}

pub(crate) fn validate_base_positions_do_not_overlap(
    p1_base: GridPoint,
    p2_base: GridPoint,
) -> Result<(), String> {
    if rects_overlap(
        grid_point_top_left(&p1_base),
        Vec2::splat(TANK_SIZE),
        grid_point_top_left(&p2_base),
        Vec2::splat(TANK_SIZE),
    ) {
        return Err(format!(
            "p1 base ({}, {}) and p2 base ({}, {}) must not overlap",
            p1_base.x, p1_base.y, p2_base.x, p2_base.y
        ));
    }

    Ok(())
}

pub(crate) fn validate_powerup_spawns(grid: &TileGrid, points: &[GridPoint]) -> Result<(), String> {
    let mut seen = HashSet::new();

    for (index, point) in points.iter().enumerate() {
        let spawn_index = index + 1;
        validate_powerup_spawn(grid, spawn_index, point)?;

        if !seen.insert((point.x, point.y)) {
            return Err(format!(
                "power-up spawn {spawn_index} ({}, {}) is configured more than once",
                point.x, point.y
            ));
        }
    }

    Ok(())
}

fn validate_powerup_spawn(grid: &TileGrid, index: usize, point: &GridPoint) -> Result<(), String> {
    let top_left = Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE);
    if grid.can_tank_occupy(top_left) {
        Ok(())
    } else {
        Err(format!(
            "power-up spawn {index} ({}, {}) must fit a 16x16 reward on passable tiles",
            point.x, point.y
        ))
    }
}

pub(crate) fn validate_powerup_carriers(level: &LevelDefinition) -> Result<(), String> {
    let mut seen = HashSet::new();
    for carrier in &level.powerup_carriers {
        if carrier.enemy == 0 || carrier.enemy > level.enemies.len() {
            return Err(format!(
                "powerup carrier enemy {} is outside the 1..={} roster",
                carrier.enemy,
                level.enemies.len()
            ));
        }
        if !seen.insert(carrier.enemy) {
            return Err(format!(
                "powerup carrier enemy {} is configured more than once",
                carrier.enemy
            ));
        }
    }

    Ok(())
}

pub(crate) fn validate_classic_enemy_spawns(spawns: &[SpawnPoint]) -> Result<(), String> {
    let expected = [
        (0, 0, Direction::Down),
        (12, 0, Direction::Down),
        (24, 0, Direction::Down),
    ];

    for (index, (spawn, (x, y, facing))) in spawns.iter().zip(expected).enumerate() {
        if spawn.x != x || spawn.y != y || spawn.facing != facing {
            return Err(format!(
                "enemy spawn {} must be classic top spawn ({x}, {y}, {facing:?}), got ({}, {}, {:?})",
                index + 1,
                spawn.x,
                spawn.y,
                spawn.facing
            ));
        }
    }

    Ok(())
}
