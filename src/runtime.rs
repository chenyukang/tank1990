use crate::*;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub(super) struct EnemyFreeze {
    pub(super) timer: Option<Timer>,
}

impl EnemyFreeze {
    pub(super) fn start(&mut self) {
        self.timer = Some(Timer::from_seconds(CLOCK_SECONDS, TimerMode::Once));
    }

    pub(super) fn reset(&mut self) {
        self.timer = None;
    }

    pub(super) fn is_active(&self) -> bool {
        self.timer
            .as_ref()
            .is_some_and(|timer| !timer.is_finished())
    }

    pub(super) fn tick(&mut self, delta: Duration) {
        let Some(timer) = &mut self.timer else {
            return;
        };
        timer.tick(delta);
        if timer.is_finished() {
            self.timer = None;
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct VersusPlayerFreeze {
    pub(super) frozen_player: Option<PlayerId>,
    pub(super) timer: Option<Timer>,
}

impl VersusPlayerFreeze {
    pub(super) fn start(&mut self, player: PlayerId) {
        self.frozen_player = Some(player);
        self.timer = Some(Timer::from_seconds(CLOCK_SECONDS, TimerMode::Once));
    }

    pub(super) fn reset(&mut self) {
        self.frozen_player = None;
        self.timer = None;
    }

    pub(super) fn is_player_frozen(&self, player: PlayerId) -> bool {
        self.frozen_player == Some(player)
            && self
                .timer
                .as_ref()
                .is_some_and(|timer| !timer.is_finished())
    }

    pub(super) fn tick(&mut self, delta: Duration) {
        let Some(timer) = &mut self.timer else {
            return;
        };
        timer.tick(delta);
        if timer.is_finished() {
            self.reset();
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct BaseReinforcement {
    pub(super) timer: Option<Timer>,
    pub(super) saved_tiles: Vec<(usize, usize, TileKind)>,
}

impl BaseReinforcement {
    pub(super) fn start(&mut self) {
        self.timer = Some(Timer::from_seconds(SHOVEL_SECONDS, TimerMode::Once));
    }

    pub(super) fn reset(&mut self) {
        self.timer = None;
        self.saved_tiles.clear();
    }

    pub(super) fn tick(&mut self, delta: Duration) -> bool {
        let Some(timer) = &mut self.timer else {
            return false;
        };
        timer.tick(delta);
        timer.is_finished()
    }

    pub(super) fn warning_elapsed_secs(&self) -> Option<f32> {
        let timer = self.timer.as_ref()?;
        (timer.remaining_secs() <= SHOVEL_WARNING_SECONDS).then_some(timer.elapsed_secs())
    }

    pub(super) fn contains_position(&self, x: usize, y: usize) -> bool {
        self.saved_tiles
            .iter()
            .any(|(tile_x, tile_y, _)| *tile_x == x && *tile_y == y)
    }
}

#[derive(Resource)]
pub(super) struct EnemyDirector {
    pub(super) roster: VecDeque<EnemyRosterEntry>,
    pub(super) spawns: Vec<SpawnPoint>,
    pub(super) spawn_timer: Timer,
    pub(super) max_active: usize,
    pub(super) spawn_cursor: usize,
    pub(super) spawned_count: usize,
    pub(super) ai_strategy: EnemyAiStrategy,
    pub(super) difficulty_profile: EnemyDifficultyProfile,
}

impl EnemyDirector {
    #[cfg(test)]
    pub(super) fn from_level(level: &LevelDefinition) -> Self {
        Self::from_level_with_ai(level, level.enemy_ai_strategy, level.difficulty_profile)
    }

    pub(super) fn from_level_with_ai(
        level: &LevelDefinition,
        ai_strategy: EnemyAiStrategy,
        difficulty_profile: EnemyDifficultyProfile,
    ) -> Self {
        Self {
            roster: level
                .enemies
                .iter()
                .enumerate()
                .map(|(index, kind)| EnemyRosterEntry {
                    kind: *kind,
                    carried_powerup: carrier_powerup_for_spawn(index + 1, &level.powerup_carriers),
                })
                .collect(),
            spawns: level.enemy_spawns.clone(),
            spawn_timer: Timer::from_seconds(
                enemy_spawn_interval_for_profile(level.spawn_interval_secs, difficulty_profile),
                TimerMode::Repeating,
            ),
            max_active: level.max_enemies_on_screen,
            spawn_cursor: 0,
            spawned_count: 0,
            ai_strategy,
            difficulty_profile,
        }
    }

    pub(super) fn inactive() -> Self {
        Self {
            roster: VecDeque::new(),
            spawns: Vec::new(),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_active: 0,
            spawn_cursor: 0,
            spawned_count: 0,
            ai_strategy: EnemyAiStrategy::default(),
            difficulty_profile: EnemyDifficultyProfile::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct EnemyRosterEntry {
    pub(super) kind: EnemyKind,
    pub(super) carried_powerup: Option<PowerUpKind>,
}

#[derive(Resource)]
pub(super) struct VersusPowerUpDirector {
    pub(super) spawn_points: Vec<Vec2>,
    pub(super) spawn_timer: Timer,
    pub(super) spawn_cursor: usize,
    pub(super) kind_cursor: usize,
    pub(super) spawn_immediately: bool,
}

impl VersusPowerUpDirector {
    pub(super) fn from_arena(arena: &ArenaDefinition) -> Self {
        Self {
            spawn_points: arena
                .powerup_spawns
                .iter()
                .map(|point| Vec2::new(point.x as f32 * TILE_SIZE, point.y as f32 * TILE_SIZE))
                .collect(),
            spawn_timer: Timer::from_seconds(VERSUS_POWERUP_INTERVAL_SECONDS, TimerMode::Repeating),
            spawn_cursor: 0,
            kind_cursor: 0,
            spawn_immediately: true,
        }
    }

    pub(super) fn inactive() -> Self {
        Self {
            spawn_points: Vec::new(),
            spawn_timer: Timer::from_seconds(VERSUS_POWERUP_INTERVAL_SECONDS, TimerMode::Repeating),
            spawn_cursor: 0,
            kind_cursor: 0,
            spawn_immediately: false,
        }
    }

    pub(super) fn next_spawn(&mut self) -> Option<(Vec2, PowerUpKind)> {
        if self.spawn_points.is_empty() {
            return None;
        }

        let top_left = self.spawn_points[self.spawn_cursor];
        self.spawn_cursor = (self.spawn_cursor + 1) % self.spawn_points.len();
        let kind = powerup_for_cycle(self.kind_cursor);
        self.kind_cursor += 1;
        self.spawn_immediately = false;
        self.spawn_timer.reset();
        Some((top_left, kind))
    }
}
