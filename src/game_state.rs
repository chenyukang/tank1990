use crate::*;

#[derive(Resource, Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GameMode {
    Campaign,
    CoopCampaign,
    VersusDeathmatch,
    VersusBaseBattle,
}

impl GameMode {
    pub(super) fn is_campaign(self) -> bool {
        matches!(self, Self::Campaign | Self::CoopCampaign)
    }

    pub(super) fn is_coop_campaign(self) -> bool {
        matches!(self, Self::CoopCampaign)
    }

    pub(super) fn is_versus(self) -> bool {
        matches!(self, Self::VersusDeathmatch | Self::VersusBaseBattle)
    }

    pub(super) fn has_two_player_targets(self) -> bool {
        self.is_coop_campaign() || self.is_versus()
    }

    pub(super) fn mode_select_option(self) -> ModeSelectOption {
        match self {
            Self::Campaign => ModeSelectOption::Campaign,
            Self::CoopCampaign => ModeSelectOption::CoopCampaign,
            Self::VersusDeathmatch | Self::VersusBaseBattle => ModeSelectOption::Battle,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CampaignMapPack {
    Original,
    Custom,
}

impl CampaignMapPack {
    pub(super) fn stage_count(self) -> usize {
        match self {
            Self::Original => ORIGINAL_LEVEL_COUNT,
            Self::Custom => CUSTOM_LEVEL_COUNT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModeSelectOption {
    Campaign,
    CoopCampaign,
    Battle,
    MapPack,
    ViewMode,
    ViewAssist,
    AiStrategy,
    Difficulty,
    Music,
    Sound,
    Scale,
    Stage,
    Arena,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModeSelectAiStrategy {
    Auto,
    Classic,
    PathToObjective,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModeSelectDifficultyProfile {
    Easy,
    Auto,
    Normal,
    Hard,
}

#[derive(Resource)]
pub(super) struct ModeSelect {
    pub(super) selected: ModeSelectOption,
    pub(super) map_pack: CampaignMapPack,
    pub(super) stage: usize,
    pub(super) arena: usize,
    pub(super) view_mode: TankViewMode,
    pub(super) view_assist: bool,
    pub(super) view_target: PlayerId,
    pub(super) ai_strategy: ModeSelectAiStrategy,
    pub(super) difficulty_profile: ModeSelectDifficultyProfile,
    pub(super) audio_mode: AudioMode,
    pub(super) sound_enabled: bool,
    pub(super) window_scale: u32,
}

impl Default for ModeSelect {
    fn default() -> Self {
        Self {
            selected: ModeSelectOption::Campaign,
            map_pack: CampaignMapPack::Original,
            stage: 1,
            arena: DEFAULT_VERSUS_ARENA,
            view_mode: TankViewMode::TwoD,
            view_assist: true,
            view_target: PlayerId::One,
            ai_strategy: ModeSelectAiStrategy::Auto,
            difficulty_profile: ModeSelectDifficultyProfile::Easy,
            audio_mode: AudioMode::Bgm,
            sound_enabled: true,
            window_scale: DEFAULT_WINDOW_SCALE,
        }
    }
}

pub(crate) fn next_arena(current: usize) -> usize {
    if current >= ARENA_COUNT {
        1
    } else {
        current + 1
    }
}

pub(crate) fn previous_arena(current: usize) -> usize {
    if current <= 1 {
        ARENA_COUNT
    } else {
        current - 1
    }
}

pub(crate) fn next_stage(current: usize, map_pack: CampaignMapPack) -> usize {
    let stage_count = map_pack.stage_count();
    if current >= stage_count {
        1
    } else {
        current + 1
    }
}

pub(crate) fn previous_stage(current: usize, map_pack: CampaignMapPack) -> usize {
    if current <= 1 {
        map_pack.stage_count()
    } else {
        current - 1
    }
}

pub(crate) fn selected_campaign_stage(mode_select: &ModeSelect) -> usize {
    mode_select
        .stage
        .clamp(1, mode_select.map_pack.stage_count())
}

pub(crate) fn next_campaign_map_pack(pack: CampaignMapPack) -> CampaignMapPack {
    match pack {
        CampaignMapPack::Original => CampaignMapPack::Custom,
        CampaignMapPack::Custom => CampaignMapPack::Original,
    }
}

pub(crate) fn previous_campaign_map_pack(pack: CampaignMapPack) -> CampaignMapPack {
    match pack {
        CampaignMapPack::Original => CampaignMapPack::Custom,
        CampaignMapPack::Custom => CampaignMapPack::Original,
    }
}

pub(crate) fn campaign_map_pack_label(pack: CampaignMapPack) -> &'static str {
    match pack {
        CampaignMapPack::Original => "ORIGINAL",
        CampaignMapPack::Custom => "CUSTOM",
    }
}

pub(crate) fn selected_enemy_ai_strategy(
    mode_select: &ModeSelect,
    level: &LevelDefinition,
) -> EnemyAiStrategy {
    match mode_select.ai_strategy {
        ModeSelectAiStrategy::Auto => level.enemy_ai_strategy,
        ModeSelectAiStrategy::Classic => EnemyAiStrategy::Classic,
        ModeSelectAiStrategy::PathToObjective => EnemyAiStrategy::PathToObjective,
    }
}

pub(crate) fn selected_enemy_difficulty_profile(
    mode_select: &ModeSelect,
    level: &LevelDefinition,
) -> EnemyDifficultyProfile {
    match mode_select.difficulty_profile {
        ModeSelectDifficultyProfile::Easy => EnemyDifficultyProfile::Easy,
        ModeSelectDifficultyProfile::Auto => level.difficulty_profile,
        ModeSelectDifficultyProfile::Normal => EnemyDifficultyProfile::Normal,
        ModeSelectDifficultyProfile::Hard => EnemyDifficultyProfile::Hard,
    }
}

pub(crate) fn next_mode_select_ai_strategy(strategy: ModeSelectAiStrategy) -> ModeSelectAiStrategy {
    match strategy {
        ModeSelectAiStrategy::Auto => ModeSelectAiStrategy::Classic,
        ModeSelectAiStrategy::Classic => ModeSelectAiStrategy::PathToObjective,
        ModeSelectAiStrategy::PathToObjective => ModeSelectAiStrategy::Auto,
    }
}

pub(crate) fn previous_mode_select_ai_strategy(
    strategy: ModeSelectAiStrategy,
) -> ModeSelectAiStrategy {
    match strategy {
        ModeSelectAiStrategy::Auto => ModeSelectAiStrategy::PathToObjective,
        ModeSelectAiStrategy::Classic => ModeSelectAiStrategy::Auto,
        ModeSelectAiStrategy::PathToObjective => ModeSelectAiStrategy::Classic,
    }
}

pub(crate) fn next_mode_select_difficulty_profile(
    profile: ModeSelectDifficultyProfile,
) -> ModeSelectDifficultyProfile {
    match profile {
        ModeSelectDifficultyProfile::Easy => ModeSelectDifficultyProfile::Normal,
        ModeSelectDifficultyProfile::Auto => ModeSelectDifficultyProfile::Easy,
        ModeSelectDifficultyProfile::Normal => ModeSelectDifficultyProfile::Hard,
        ModeSelectDifficultyProfile::Hard => ModeSelectDifficultyProfile::Auto,
    }
}

pub(crate) fn previous_mode_select_difficulty_profile(
    profile: ModeSelectDifficultyProfile,
) -> ModeSelectDifficultyProfile {
    match profile {
        ModeSelectDifficultyProfile::Easy => ModeSelectDifficultyProfile::Auto,
        ModeSelectDifficultyProfile::Auto => ModeSelectDifficultyProfile::Hard,
        ModeSelectDifficultyProfile::Normal => ModeSelectDifficultyProfile::Easy,
        ModeSelectDifficultyProfile::Hard => ModeSelectDifficultyProfile::Normal,
    }
}

pub(crate) fn next_mode_select_option(option: ModeSelectOption) -> ModeSelectOption {
    match option {
        ModeSelectOption::Campaign => ModeSelectOption::CoopCampaign,
        ModeSelectOption::CoopCampaign => ModeSelectOption::Battle,
        ModeSelectOption::Battle => ModeSelectOption::MapPack,
        ModeSelectOption::MapPack => ModeSelectOption::ViewMode,
        ModeSelectOption::ViewMode => ModeSelectOption::ViewAssist,
        ModeSelectOption::ViewAssist => ModeSelectOption::AiStrategy,
        ModeSelectOption::AiStrategy => ModeSelectOption::Difficulty,
        ModeSelectOption::Difficulty => ModeSelectOption::Music,
        ModeSelectOption::Music => ModeSelectOption::Sound,
        ModeSelectOption::Sound => ModeSelectOption::Scale,
        ModeSelectOption::Scale => ModeSelectOption::Stage,
        ModeSelectOption::Stage => ModeSelectOption::Arena,
        ModeSelectOption::Arena => ModeSelectOption::Campaign,
    }
}

pub(crate) fn previous_mode_select_option(option: ModeSelectOption) -> ModeSelectOption {
    match option {
        ModeSelectOption::Campaign => ModeSelectOption::Arena,
        ModeSelectOption::CoopCampaign => ModeSelectOption::Campaign,
        ModeSelectOption::Battle => ModeSelectOption::CoopCampaign,
        ModeSelectOption::MapPack => ModeSelectOption::Battle,
        ModeSelectOption::ViewMode => ModeSelectOption::MapPack,
        ModeSelectOption::ViewAssist => ModeSelectOption::ViewMode,
        ModeSelectOption::AiStrategy => ModeSelectOption::ViewAssist,
        ModeSelectOption::Difficulty => ModeSelectOption::AiStrategy,
        ModeSelectOption::Music => ModeSelectOption::Difficulty,
        ModeSelectOption::Sound => ModeSelectOption::Music,
        ModeSelectOption::Scale => ModeSelectOption::Sound,
        ModeSelectOption::Stage => ModeSelectOption::Scale,
        ModeSelectOption::Arena => ModeSelectOption::Stage,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PlayerId {
    One,
    Two,
}

impl PlayerId {
    pub(super) fn team(self) -> Team {
        match self {
            Self::One => Team::Player1,
            Self::Two => Team::Player2,
        }
    }

    pub(super) fn opponent(self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
        }
    }
}

#[derive(Resource)]
pub(super) struct GameStatus {
    pub(super) phase: GamePhase,
    pub(super) map_pack: CampaignMapPack,
    pub(super) stage: usize,
    pub(super) arena: usize,
    pub(super) winner: Option<PlayerId>,
    pub(super) transition_timer: Timer,
}

impl Default for GameStatus {
    fn default() -> Self {
        Self {
            phase: GamePhase::ModeSelect,
            map_pack: CampaignMapPack::Original,
            stage: 1,
            arena: DEFAULT_VERSUS_ARENA,
            winner: None,
            transition_timer: Timer::from_seconds(LEVEL_CLEAR_DELAY_SECONDS, TimerMode::Once),
        }
    }
}

impl GameStatus {
    pub(super) fn is_playing(&self) -> bool {
        self.phase == GamePhase::Playing
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GamePhase {
    ModeSelect,
    StageIntro,
    Playing,
    Paused,
    GameOver,
    LevelClear,
    RoundOver,
    Victory,
}

#[derive(Resource)]
pub(super) struct ScoreBoard {
    pub(super) score: u32,
    pub(super) lives: i32,
    pub(super) enemies_destroyed: usize,
    pub(super) total_enemies: usize,
    pub(super) enemy_kills: EnemyKillCounts,
    pub(super) p1_score: u32,
    pub(super) p2_score: u32,
    pub(super) p1_lives: i32,
    pub(super) p2_lives: i32,
    pub(super) target_score: u32,
    pub(super) respawn_invulnerability_secs: f32,
}

impl ScoreBoard {
    pub(super) fn campaign(total_enemies: usize) -> Self {
        Self {
            score: 0,
            lives: 3,
            enemies_destroyed: 0,
            total_enemies,
            enemy_kills: EnemyKillCounts::default(),
            p1_score: 0,
            p2_score: 0,
            p1_lives: 3,
            p2_lives: 0,
            target_score: 0,
            respawn_invulnerability_secs: DEFAULT_RESPAWN_INVULNERABILITY_SECONDS,
        }
    }

    pub(super) fn coop_campaign(total_enemies: usize) -> Self {
        Self {
            p2_lives: 3,
            lives: 6,
            ..Self::campaign(total_enemies)
        }
    }

    pub(super) fn versus(lives: i32, target_score: u32, respawn_invulnerability_secs: f32) -> Self {
        Self {
            score: 0,
            lives,
            enemies_destroyed: 0,
            total_enemies: 0,
            enemy_kills: EnemyKillCounts::default(),
            p1_score: 0,
            p2_score: 0,
            p1_lives: lives,
            p2_lives: lives,
            target_score,
            respawn_invulnerability_secs,
        }
    }

    pub(super) fn player_score(&self, player: PlayerId) -> u32 {
        match player {
            PlayerId::One => self.p1_score,
            PlayerId::Two => self.p2_score,
        }
    }

    pub(super) fn add_player_score(&mut self, player: PlayerId) {
        match player {
            PlayerId::One => self.p1_score += 1,
            PlayerId::Two => self.p2_score += 1,
        }
    }

    pub(super) fn record_enemy_destroyed(&mut self, kind: EnemyKind) {
        self.score += enemy_score(kind);
        self.enemies_destroyed += 1;
        self.enemy_kills.add(kind);
    }

    pub(super) fn set_player_lives(&mut self, player: PlayerId, lives: i32) {
        match player {
            PlayerId::One => self.p1_lives = lives,
            PlayerId::Two => self.p2_lives = lives,
        }
    }

    pub(super) fn set_coop_player_lives(&mut self, player: PlayerId, lives: i32) {
        self.set_player_lives(player, lives);
        self.lives = self.p1_lives.max(0) + self.p2_lives.max(0);
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct EnemyKillCounts {
    pub(super) basic: usize,
    pub(super) fast: usize,
    pub(super) power: usize,
    pub(super) armor: usize,
}

impl EnemyKillCounts {
    pub(super) fn add(&mut self, kind: EnemyKind) {
        *self.count_mut(kind) += 1;
    }

    pub(super) fn count(self, kind: EnemyKind) -> usize {
        match kind {
            EnemyKind::Basic => self.basic,
            EnemyKind::Fast => self.fast,
            EnemyKind::Power => self.power,
            EnemyKind::Armor => self.armor,
        }
    }

    pub(super) fn total(self) -> usize {
        self.basic + self.fast + self.power + self.armor
    }

    fn count_mut(&mut self, kind: EnemyKind) -> &mut usize {
        match kind {
            EnemyKind::Basic => &mut self.basic,
            EnemyKind::Fast => &mut self.fast,
            EnemyKind::Power => &mut self.power,
            EnemyKind::Armor => &mut self.armor,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Resource)]
pub(super) struct StageRules {
    pub(super) player_steel_destruction: bool,
}

impl StageRules {
    pub(super) fn from_level(level: &LevelDefinition) -> Self {
        Self {
            player_steel_destruction: level.player_steel_destruction,
        }
    }
}
