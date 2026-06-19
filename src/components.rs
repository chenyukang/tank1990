use crate::*;

#[derive(Component)]
pub(super) struct Player {
    pub(super) id: PlayerId,
}

#[derive(Component)]
pub(super) struct GameEntity;

#[derive(Component)]
pub(super) struct EnemyTank {
    pub(super) kind: EnemyKind,
    pub(super) carried_powerup: Option<PowerUpKind>,
}

#[derive(Component)]
pub(super) struct EnemyAi {
    pub(super) turn_timer: Timer,
    pub(super) fire_timer: Timer,
    pub(super) strategy: EnemyAiStrategy,
    pub(super) difficulty_profile: EnemyDifficultyProfile,
}

#[derive(Component)]
pub(super) struct SpawnProtection {
    pub(super) timer: Timer,
}

impl SpawnProtection {
    pub(super) fn for_spawn_shimmer(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(spawn_shimmer_duration_secs(frames), TimerMode::Once),
        }
    }

    pub(super) fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
pub(super) struct PlayerRespawnDelay {
    pub(super) timer: Timer,
}

impl PlayerRespawnDelay {
    pub(super) fn for_spawn_shimmer(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(spawn_shimmer_duration_secs(frames), TimerMode::Once),
        }
    }

    pub(super) fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
pub(super) struct PlayerRespawnPending {
    pub(super) timer: Timer,
}

impl PlayerRespawnPending {
    pub(super) fn for_explosion(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(explosion_duration_secs(frames), TimerMode::Once),
        }
    }

    pub(super) fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
pub(super) struct Tank {
    pub(super) top_left: Vec2,
    pub(super) facing: Direction,
    pub(super) speed: f32,
}

#[derive(Component)]
pub(super) struct TankSpriteState {
    pub(super) set: TankSpriteSet,
    pub(super) frame: usize,
    pub(super) timer: Timer,
}

impl TankSpriteState {
    pub(super) fn new(set: TankSpriteSet) -> Self {
        Self {
            set,
            frame: 0,
            timer: Timer::from_seconds(0.14, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub(super) struct Bullet {
    pub(super) previous_top_left: Vec2,
    pub(super) top_left: Vec2,
    pub(super) facing: Direction,
    pub(super) owner: Team,
    pub(super) speed: f32,
    pub(super) breaks_steel: bool,
    pub(super) resolved: bool,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub(super) struct BulletImpactDirection {
    pub(super) direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BulletTileHit {
    pub(super) x: usize,
    pub(super) y: usize,
    pub(super) tile: TileKind,
    pub(super) impact_top_left: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct BulletPathClash {
    pub(super) impact_top_left: Vec2,
    pub(super) time: f32,
}

#[derive(Component)]
pub(super) struct EnemyBulletSource {
    pub(super) shooter: Entity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Team {
    Player1,
    Player2,
    Enemy,
}

impl Team {
    pub(super) fn player_id(self) -> Option<PlayerId> {
        match self {
            Self::Player1 => Some(PlayerId::One),
            Self::Player2 => Some(PlayerId::Two),
            Self::Enemy => None,
        }
    }

    pub(super) fn is_player(self) -> bool {
        self.player_id().is_some()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TankSpriteSet {
    Player1,
    Player2,
    EnemyBasic,
    EnemyFast,
    EnemyPower,
    EnemyArmor,
}

impl TankSpriteSet {
    pub(super) fn player(player: PlayerId) -> Self {
        match player {
            PlayerId::One => Self::Player1,
            PlayerId::Two => Self::Player2,
        }
    }

    pub(super) fn enemy(kind: EnemyKind) -> Self {
        match kind {
            EnemyKind::Basic => Self::EnemyBasic,
            EnemyKind::Fast => Self::EnemyFast,
            EnemyKind::Power => Self::EnemyPower,
            EnemyKind::Armor => Self::EnemyArmor,
        }
    }
}

#[derive(Component)]
pub(super) struct Health {
    pub(super) current: i32,
}

#[derive(Component)]
pub(super) struct RespawnPoint {
    pub(super) top_left: Vec2,
    pub(super) facing: Direction,
}

#[derive(Component)]
pub(super) struct PlayerLives {
    pub(super) current: i32,
}

#[derive(Component)]
pub(super) struct PlayerUpgrade {
    pub(super) level: u8,
}

#[derive(Component)]
pub(super) struct Shield {
    pub(super) timer: Timer,
}

#[derive(Component)]
pub(super) struct ShieldVisual {
    pub(super) owner: Entity,
}

#[derive(Component)]
pub(super) struct GridTile {
    pub(super) x: usize,
    pub(super) y: usize,
}

#[derive(Component)]
pub(super) struct BaseSprite {
    pub(super) owner: Option<PlayerId>,
    pub(super) top_left: Vec2,
}

#[derive(Component)]
pub(super) struct SpriteAnimation {
    pub(super) first: usize,
    pub(super) last: usize,
    pub(super) timer: Timer,
    pub(super) despawn_on_finish: bool,
}

#[derive(Component)]
pub(super) struct DestroyedTank {
    pub(super) timer: Timer,
}

impl DestroyedTank {
    pub(super) fn for_explosion(frames: SpriteFrameRange) -> Self {
        Self {
            timer: Timer::from_seconds(explosion_duration_secs(frames), TimerMode::Once),
        }
    }

    pub(super) fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.is_finished()
    }
}

#[derive(Component)]
pub(super) struct PowerUp {
    pub(super) kind: PowerUpKind,
}

#[derive(Component)]
pub(super) struct PowerUpSparkle;
