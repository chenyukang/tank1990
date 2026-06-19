use bevy::camera::ScalingMode;
use bevy::prelude::{OrthographicProjection, Vec2, Vec3};
use std::sync::atomic::{AtomicU32, Ordering};

pub(crate) const VIRTUAL_HEIGHT: f32 = 240.0;
pub(crate) const DEFAULT_WINDOW_SCALE: u32 = 3;
pub(crate) const MIN_WINDOW_SCALE: u32 = 2;
pub(crate) const MAX_WINDOW_SCALE: u32 = 4;

pub(crate) const BOARD_ORIGIN_X: f32 = 24.0;
pub(crate) const BOARD_ORIGIN_Y: f32 = 16.0;
pub(crate) const BOARD_TILES: usize = 26;
pub(crate) const TILE_SIZE: f32 = 8.0;
pub(crate) const BOARD_SIZE: f32 = 208.0;

pub(crate) const STATUS_PANEL_GAP: f32 = 24.0;
pub(crate) const STATUS_PANEL_WIDTH: f32 = 48.0;
pub(crate) const VIRTUAL_WIDTH: f32 =
    BOARD_ORIGIN_X + BOARD_SIZE + STATUS_PANEL_GAP + STATUS_PANEL_WIDTH;
pub(crate) const STATUS_PANEL_HEIGHT: f32 = BOARD_SIZE;
pub(crate) const STATUS_PANEL_LEFT: f32 = BOARD_ORIGIN_X + BOARD_SIZE + STATUS_PANEL_GAP;
pub(crate) const STATUS_PANEL_TOP: f32 = BOARD_ORIGIN_Y;
pub(crate) const STATUS_PANEL_INNER_LEFT: f32 = STATUS_PANEL_LEFT + 4.0;
pub(crate) const STATUS_PANEL_INNER_TOP: f32 = STATUS_PANEL_TOP + 8.0;
pub(crate) const STATUS_PANEL_INNER_WIDTH: f32 = 40.0;
pub(crate) const STATUS_PANEL_INNER_HEIGHT: f32 = 192.0;

pub(crate) const TANK_SIZE: f32 = 16.0;
pub(crate) const BULLET_SIZE: f32 = 4.0;

pub(crate) const ENEMY_MARKER_COUNT: usize = 20;
pub(crate) const ENEMY_MARKER_COLUMNS: usize = 4;
pub(crate) const ENEMY_MARKER_SIZE: f32 = 8.0;
pub(crate) const PLAYER_LIFE_ICON_SIZE: f32 = 8.0;
pub(crate) const ENEMY_MARKER_LEFT: f32 = STATUS_PANEL_LEFT + 8.0;
pub(crate) const ENEMY_MARKER_TOP: f32 = 159.0;
pub(crate) const ENEMY_MARKER_CELL_X: f32 = 9.0;
pub(crate) const ENEMY_MARKER_CELL_Y: f32 = 9.0;

pub(crate) const GLYPH_ADVANCE: f32 = 6.0;
pub(crate) const MODE_SELECT_WIDTH: f32 = 208.0;
pub(crate) const MODE_SELECT_LEFT: f32 = (VIRTUAL_WIDTH - MODE_SELECT_WIDTH) / 2.0;
pub(crate) const MODE_SELECT_TITLE_Y: f32 = 35.0;
pub(crate) const MODE_SELECT_CURSOR_GAP: f32 = 22.0;
pub(crate) const MODE_SELECT_HINT_TOP: f32 = 198.0;

static WINDOW_SCALE_SETTING: AtomicU32 = AtomicU32::new(DEFAULT_WINDOW_SCALE);
static WINDOWED_SCALE_SETTING: AtomicU32 = AtomicU32::new(DEFAULT_WINDOW_SCALE);

fn load_window_scale_setting() -> u32 {
    WINDOW_SCALE_SETTING.load(Ordering::Relaxed)
}

fn store_window_scale_setting(scale: u32) {
    WINDOW_SCALE_SETTING.store(scale, Ordering::Relaxed);
}

pub(crate) fn load_windowed_scale_setting() -> u32 {
    WINDOWED_SCALE_SETTING.load(Ordering::Relaxed)
}

pub(crate) fn store_windowed_scale_setting(scale: u32) {
    WINDOWED_SCALE_SETTING.store(scale, Ordering::Relaxed);
}

#[cfg(test)]
pub(crate) fn reset_window_scale_settings_for_test() {
    store_window_scale_setting(DEFAULT_WINDOW_SCALE);
    store_windowed_scale_setting(DEFAULT_WINDOW_SCALE);
}

pub(crate) fn window_scale() -> f32 {
    load_window_scale_setting() as f32
}

pub(crate) fn set_window_scale(scale: u32) {
    store_window_scale_setting(clamp_window_scale(scale));
}

pub(crate) fn clamp_window_scale(scale: u32) -> u32 {
    scale.clamp(MIN_WINDOW_SCALE, MAX_WINDOW_SCALE)
}

pub(crate) fn next_window_scale(scale: u32) -> u32 {
    match clamp_window_scale(scale) {
        MAX_WINDOW_SCALE => MIN_WINDOW_SCALE,
        scale => scale + 1,
    }
}

pub(crate) fn previous_window_scale(scale: u32) -> u32 {
    match clamp_window_scale(scale) {
        MIN_WINDOW_SCALE => MAX_WINDOW_SCALE,
        scale => scale - 1,
    }
}

pub(crate) fn window_scale_label(scale: u32) -> String {
    format!("{}X", clamp_window_scale(scale))
}

pub(crate) fn virtual_window_size(scale: f32) -> (u32, u32) {
    (
        (VIRTUAL_WIDTH * scale).round() as u32,
        (VIRTUAL_HEIGHT * scale).round() as u32,
    )
}

pub(crate) fn game_2d_projection() -> OrthographicProjection {
    game_2d_projection_for_scale(window_scale())
}

pub(crate) fn game_2d_projection_for_scale(scale: f32) -> OrthographicProjection {
    let (width, height) = virtual_window_size(scale);

    OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
            min_width: width as f32,
            min_height: height as f32,
        },
        ..OrthographicProjection::default_2d()
    }
}

pub(crate) fn game_2d_projection_visible_size(window_size: Vec2, scale: f32) -> Vec2 {
    let (min_width, min_height) = virtual_window_size(scale);
    let min_width = min_width as f32;
    let min_height = min_height as f32;
    let window_width = window_size.x.max(1.0);
    let window_height = window_size.y.max(1.0);
    let window_aspect = window_width / window_height;
    let min_aspect = min_width / min_height;

    if window_aspect > min_aspect {
        Vec2::new(min_height * window_aspect, min_height)
    } else {
        Vec2::new(min_width, min_width / window_aspect)
    }
}

pub(crate) fn board_size() -> f32 {
    BOARD_SIZE
}

pub(crate) fn board_tile_center(x: usize, y: usize, z: f32) -> Vec3 {
    board_object_center(
        x as f32 * TILE_SIZE,
        y as f32 * TILE_SIZE,
        Vec2::splat(TILE_SIZE),
        z,
    )
}

pub(crate) fn board_object_center(local_x: f32, local_y: f32, size: Vec2, z: f32) -> Vec3 {
    virtual_center_scaled(
        Vec2::new(BOARD_ORIGIN_X + local_x, BOARD_ORIGIN_Y + local_y),
        size,
        z,
    )
}

pub(crate) fn board_top_left_from_translation(translation: Vec3, object_size: f32) -> Vec2 {
    let center_x = translation.x / window_scale() + VIRTUAL_WIDTH / 2.0;
    let center_y = VIRTUAL_HEIGHT / 2.0 - translation.y / window_scale();
    Vec2::new(
        center_x - object_size / 2.0 - BOARD_ORIGIN_X,
        center_y - object_size / 2.0 - BOARD_ORIGIN_Y,
    )
}

pub(crate) fn virtual_center_scaled(top_left: Vec2, size: Vec2, z: f32) -> Vec3 {
    let center = top_left + size / 2.0;
    Vec3::new(
        (center.x - VIRTUAL_WIDTH / 2.0) * window_scale(),
        (VIRTUAL_HEIGHT / 2.0 - center.y) * window_scale(),
        z,
    )
}

#[cfg(test)]
pub(crate) fn virtual_top_left_from_translation(translation: Vec3, object_size: f32) -> Vec2 {
    let center_x = translation.x / window_scale() + VIRTUAL_WIDTH / 2.0;
    let center_y = VIRTUAL_HEIGHT / 2.0 - translation.y / window_scale();
    Vec2::new(center_x - object_size / 2.0, center_y - object_size / 2.0)
}

pub(crate) fn status_panel_top_left(offset_x: f32, y: f32) -> Vec2 {
    Vec2::new(STATUS_PANEL_LEFT + offset_x, y)
}

pub(crate) fn enemy_marker_top_left(index: usize) -> Vec2 {
    let col = index % ENEMY_MARKER_COLUMNS;
    let row = index / ENEMY_MARKER_COLUMNS;
    Vec2::new(
        ENEMY_MARKER_LEFT + col as f32 * ENEMY_MARKER_CELL_X,
        ENEMY_MARKER_TOP + row as f32 * ENEMY_MARKER_CELL_Y,
    )
}

pub(crate) fn phase_text_width(text: &str) -> f32 {
    text.chars().count() as f32 * GLYPH_ADVANCE - 1.0
}

pub(crate) fn mode_select_centered_x(text: &str) -> f32 {
    MODE_SELECT_LEFT + (MODE_SELECT_WIDTH - phase_text_width(text)) / 2.0
}

pub(crate) fn mode_select_centered_top_left(text: &str, y: f32) -> Vec2 {
    Vec2::new(mode_select_centered_x(text), y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_window_size_uses_integer_scale() {
        assert_eq!(virtual_window_size(2.0), (608, 480));
        assert_eq!(virtual_window_size(3.0), (912, 720));
        assert_eq!(virtual_window_size(4.0), (1216, 960));
    }

    #[test]
    fn game_2d_projection_keeps_full_virtual_window_visible() {
        let projection = game_2d_projection_for_scale(3.0);

        match projection.scaling_mode {
            ScalingMode::AutoMin {
                min_width,
                min_height,
            } => {
                assert_eq!(min_width, 912.0);
                assert_eq!(min_height, 720.0);
            }
            mode => panic!("unexpected scaling mode: {mode:?}"),
        }
    }

    #[test]
    fn game_2d_projection_visible_size_matches_virtual_window_at_same_aspect() {
        let size = game_2d_projection_visible_size(Vec2::new(912.0, 720.0), 3.0);

        assert_eq!(size, Vec2::new(912.0, 720.0));
    }

    #[test]
    fn game_2d_projection_visible_size_expands_wide_windows() {
        let size = game_2d_projection_visible_size(Vec2::new(2048.0, 1280.0), 4.0);

        assert_eq!(size, Vec2::new(1536.0, 960.0));
    }

    #[test]
    fn game_2d_projection_visible_size_expands_tall_windows() {
        let size = game_2d_projection_visible_size(Vec2::new(390.0, 844.0), 3.0);
        let expected_height = 912.0 * 844.0 / 390.0;

        assert_eq!(size.x, 912.0);
        assert!((size.y - expected_height).abs() < 0.001);
    }

    #[test]
    fn status_panel_gap_is_reserved_in_virtual_screen_width() {
        assert_eq!(
            VIRTUAL_WIDTH,
            BOARD_ORIGIN_X + board_size() + STATUS_PANEL_GAP + STATUS_PANEL_WIDTH
        );
        assert_eq!(STATUS_PANEL_LEFT - (BOARD_ORIGIN_X + board_size()), 24.0);
    }

    #[test]
    fn battlefield_has_balanced_horizontal_gutters() {
        assert_eq!(BOARD_ORIGIN_X, STATUS_PANEL_GAP);
        assert_eq!(
            STATUS_PANEL_LEFT - (BOARD_ORIGIN_X + board_size()),
            BOARD_ORIGIN_X
        );
    }

    #[test]
    fn right_enemy_spawn_keeps_a_clear_gap_from_status_panel() {
        let right_spawn_top_left = Vec2::new(24.0 * TILE_SIZE, 0.0);

        assert_eq!(
            STATUS_PANEL_LEFT - (BOARD_ORIGIN_X + right_spawn_top_left.x + TANK_SIZE),
            STATUS_PANEL_GAP
        );
    }
}
