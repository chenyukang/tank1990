use crate::*;

pub(crate) static PAUSED_BANNER_LINES: [&str; 4] =
    ["PAUSED", "P ESC RESUME", "R RESTART", "M MENU"];
pub(crate) static GAME_OVER_BANNER_LINES: [&str; 2] = ["GAME OVER", "PRESS R OR M"];
pub(crate) static LEVEL_CLEAR_BANNER_LINES: [&str; 1] = ["LEVEL CLEAR"];
pub(crate) static P1_WIN_BANNER_LINES: [&str; 2] = ["P1 WIN", "PRESS R OR M"];
pub(crate) static P2_WIN_BANNER_LINES: [&str; 2] = ["P2 WIN", "PRESS R OR M"];
pub(crate) static VICTORY_BANNER_LINES: [&str; 3] = ["VICTORY", "ALL STAGES CLEAR", "PRESS R OR M"];

pub(crate) fn phase_banner_lines(
    phase: GamePhase,
    winner: Option<PlayerId>,
) -> Option<&'static [&'static str]> {
    match phase {
        GamePhase::ModeSelect | GamePhase::Playing => None,
        GamePhase::StageIntro => None,
        GamePhase::Paused => Some(&PAUSED_BANNER_LINES),
        GamePhase::GameOver => Some(&GAME_OVER_BANNER_LINES),
        GamePhase::LevelClear => Some(&LEVEL_CLEAR_BANNER_LINES),
        GamePhase::RoundOver => match winner {
            Some(PlayerId::One) => Some(&P1_WIN_BANNER_LINES),
            Some(PlayerId::Two) => Some(&P2_WIN_BANNER_LINES),
            None => Some(&GAME_OVER_BANNER_LINES),
        },
        GamePhase::Victory => Some(&VICTORY_BANNER_LINES),
    }
}

pub(crate) fn stage_intro_banner_text(stage: usize) -> Vec<String> {
    vec![format!("STAGE {:02}", stage.min(99)), "READY".to_string()]
}

pub(crate) fn level_clear_banner_text(stage: usize, score_board: &ScoreBoard) -> Vec<String> {
    vec![
        format!("STAGE {:02}", stage.min(99)),
        "LEVEL CLEAR".to_string(),
        stage_clear_kill_line(
            EnemyKind::Basic,
            score_board.enemy_kills.count(EnemyKind::Basic),
            EnemyKind::Fast,
            score_board.enemy_kills.count(EnemyKind::Fast),
        ),
        stage_clear_kill_line(
            EnemyKind::Power,
            score_board.enemy_kills.count(EnemyKind::Power),
            EnemyKind::Armor,
            score_board.enemy_kills.count(EnemyKind::Armor),
        ),
        format!("TOTAL {:02}", score_board.enemy_kills.total().min(99)),
        format!("BONUS {}", stage_clear_bonus(score_board.lives)),
    ]
}

fn stage_clear_kill_line(
    left: EnemyKind,
    left_count: usize,
    right: EnemyKind,
    right_count: usize,
) -> String {
    format!(
        "{}X{:02} {}X{:02}",
        enemy_score(left),
        left_count.min(99),
        enemy_score(right),
        right_count.min(99)
    )
}

pub(crate) fn arena_intro_banner_text(arena: usize, mode: GameMode) -> Vec<String> {
    vec![
        format!("ARENA {:02}", arena.min(99)),
        arena_intro_kind_label(mode).to_string(),
        "READY".to_string(),
    ]
}

pub(crate) fn arena_intro_kind_label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::VersusDeathmatch => "DUEL",
        GameMode::VersusBaseBattle => "BASE BATTLE",
        GameMode::Campaign | GameMode::CoopCampaign => "READY",
    }
}

pub(crate) fn phase_banner_text(
    status: &GameStatus,
    mode: GameMode,
    score_board: &ScoreBoard,
) -> Option<Vec<String>> {
    if status.phase == GamePhase::StageIntro {
        return Some(match mode {
            GameMode::Campaign | GameMode::CoopCampaign => stage_intro_banner_text(status.stage),
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle => {
                arena_intro_banner_text(status.arena, mode)
            }
        });
    }
    if status.phase == GamePhase::LevelClear {
        return Some(level_clear_banner_text(status.stage, score_board));
    }

    phase_banner_lines(status.phase, status.winner)
        .map(|lines| lines.iter().map(|line| (*line).to_string()).collect())
}

pub(crate) fn campaign_phase(
    lives: i32,
    total_enemies: usize,
    enemies_destroyed: usize,
    roster_remaining: usize,
    active_enemies: usize,
) -> GamePhase {
    if lives <= 0 {
        return GamePhase::GameOver;
    }

    if enemies_destroyed >= total_enemies || (roster_remaining == 0 && active_enemies == 0) {
        GamePhase::LevelClear
    } else {
        GamePhase::Playing
    }
}

pub(crate) fn campaign_phase_transition_seconds(phase: GamePhase) -> f32 {
    if phase == GamePhase::LevelClear {
        LEVEL_CLEAR_SCORECARD_SECONDS
    } else {
        LEVEL_CLEAR_DELAY_SECONDS
    }
}

pub(crate) fn stage_clear_bonus(lives: i32) -> u32 {
    lives.max(0) as u32 * STAGE_CLEAR_LIFE_BONUS
}

pub(crate) fn visual_effects_can_advance(phase: GamePhase) -> bool {
    phase != GamePhase::Paused
}

pub(crate) fn player_spawn_delay_can_tick(phase: GamePhase) -> bool {
    matches!(phase, GamePhase::StageIntro | GamePhase::Playing)
}

pub(crate) fn terminal_phase_clears_transients(phase: GamePhase) -> bool {
    matches!(
        phase,
        GamePhase::GameOver | GamePhase::LevelClear | GamePhase::RoundOver | GamePhase::Victory
    )
}
