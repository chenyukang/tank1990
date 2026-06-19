use crate::*;

#[derive(Resource, Clone)]
pub(super) struct TileGrid {
    pub(super) tiles: Vec<TileKind>,
}

impl TileGrid {
    pub(super) fn empty() -> Self {
        Self {
            tiles: vec![TileKind::Empty; BOARD_TILES * BOARD_TILES],
        }
    }

    pub(super) fn from_level(level: &LevelDefinition) -> Result<Self, String> {
        Self::from_map(&level.map)
    }

    pub(super) fn from_arena(arena: &ArenaDefinition) -> Result<Self, String> {
        Self::from_map(&arena.map)
    }

    pub(super) fn from_map(map: &[String]) -> Result<Self, String> {
        if map.len() != BOARD_TILES {
            return Err(format!(
                "expected {BOARD_TILES} map rows, got {}",
                map.len()
            ));
        }

        let mut tiles = Vec::with_capacity(BOARD_TILES * BOARD_TILES);
        for (y, row) in map.iter().enumerate() {
            let chars: Vec<char> = row.chars().collect();
            if chars.len() != BOARD_TILES {
                return Err(format!(
                    "expected row {y} to have {BOARD_TILES} columns, got {}",
                    chars.len()
                ));
            }
            for ch in chars {
                tiles.push(TileKind::from_symbol(ch)?);
            }
        }

        Ok(Self { tiles })
    }

    pub(super) fn get(&self, x: i32, y: i32) -> Option<TileKind> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= BOARD_TILES || y >= BOARD_TILES {
            return None;
        }
        Some(self.tiles[y * BOARD_TILES + x])
    }

    pub(super) fn set(&mut self, x: usize, y: usize, tile: TileKind) {
        self.tiles[y * BOARD_TILES + x] = tile;
    }

    pub(super) fn can_tank_occupy(&self, top_left: Vec2) -> bool {
        if top_left.x < 0.0
            || top_left.y < 0.0
            || top_left.x + TANK_SIZE > board_size()
            || top_left.y + TANK_SIZE > board_size()
        {
            return false;
        }

        let left = (top_left.x / TILE_SIZE).floor() as i32;
        let right = ((top_left.x + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;
        let top = (top_left.y / TILE_SIZE).floor() as i32;
        let bottom = ((top_left.y + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;

        for y in top..=bottom {
            for x in left..=right {
                if !self.get(x, y).is_some_and(TileKind::tank_passable) {
                    return false;
                }
            }
        }

        true
    }

    pub(super) fn tank_overlaps_tile(&self, top_left: Vec2, tile: TileKind) -> bool {
        let left = (top_left.x / TILE_SIZE).floor() as i32;
        let right = ((top_left.x + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;
        let top = (top_left.y / TILE_SIZE).floor() as i32;
        let bottom = ((top_left.y + TANK_SIZE - 0.1) / TILE_SIZE).floor() as i32;

        for y in top..=bottom {
            for x in left..=right {
                if self.get(x, y) == Some(tile) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TileKind {
    Empty,
    Brick,
    Steel,
    Water,
    Forest,
    Ice,
    Base,
}

impl TileKind {
    pub(super) fn from_symbol(symbol: char) -> Result<Self, String> {
        match symbol {
            '.' => Ok(Self::Empty),
            'B' => Ok(Self::Brick),
            'S' => Ok(Self::Steel),
            'W' => Ok(Self::Water),
            'F' => Ok(Self::Forest),
            'I' => Ok(Self::Ice),
            'E' => Ok(Self::Base),
            other => Err(format!("unknown tile symbol {other:?}")),
        }
    }

    pub(super) fn tank_passable(self) -> bool {
        matches!(self, Self::Empty | Self::Forest | Self::Ice)
    }

    pub(super) fn bullet_blocks(self) -> bool {
        matches!(self, Self::Brick | Self::Steel | Self::Base)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub(super) enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(super) fn movement(self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0.0, -1.0),
            Self::Down => Vec2::new(0.0, 1.0),
            Self::Left => Vec2::new(-1.0, 0.0),
            Self::Right => Vec2::new(1.0, 0.0),
        }
    }

    pub(super) fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub(super) fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub(super) fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Deserialize)]
pub(super) struct LevelDefinition {
    pub(super) name: String,
    pub(super) map: Vec<String>,
    pub(super) player_spawn: SpawnPoint,
    pub(super) base_position: GridPoint,
    pub(super) enemy_spawns: Vec<SpawnPoint>,
    pub(super) enemies: Vec<EnemyKind>,
    pub(super) powerup_carriers: Vec<PowerUpCarrier>,
    pub(super) spawn_interval_secs: f32,
    pub(super) max_enemies_on_screen: usize,
    #[serde(default)]
    pub(super) player_steel_destruction: bool,
    #[serde(default)]
    pub(super) enemy_ai_strategy: EnemyAiStrategy,
    #[serde(default)]
    pub(super) difficulty_profile: EnemyDifficultyProfile,
}

#[derive(Deserialize)]
pub(super) struct ArenaDefinition {
    pub(super) name: String,
    pub(super) map: Vec<String>,
    pub(super) p1_spawn: SpawnPoint,
    pub(super) p2_spawn: SpawnPoint,
    pub(super) battle_rules: BattleRules,
    pub(super) powerup_spawns: Vec<GridPoint>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub(super) enum BattleRules {
    Deathmatch {
        target_score: u32,
        lives: i32,
        respawn_invulnerability_secs: f32,
    },
    BaseBattle {
        p1_base: GridPoint,
        p2_base: GridPoint,
        lives: i32,
        respawn_invulnerability_secs: f32,
    },
}

#[derive(Clone, Deserialize)]
pub(super) struct SpawnPoint {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) facing: Direction,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GridPoint {
    pub(super) x: usize,
    pub(super) y: usize,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(super) enum PowerUpKind {
    Star,
    Helmet,
    Clock,
    Grenade,
    Shovel,
    Tank,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct PowerUpCarrier {
    pub(super) enemy: usize,
    pub(super) kind: PowerUpKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(super) enum EnemyKind {
    Basic,
    Fast,
    Power,
    Armor,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub(super) enum EnemyAiStrategy {
    #[default]
    Classic,
    PathToObjective,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub(super) enum EnemyDifficultyProfile {
    Easy,
    #[default]
    Normal,
    Hard,
}
