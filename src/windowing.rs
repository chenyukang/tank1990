use super::*;

pub(super) fn handle_fullscreen_toggle(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    assets: Res<SpriteAssets>,
    game_status: Res<GameStatus>,
    mut mode_select: ResMut<ModeSelect>,
    mut fullscreen_queries: ParamSet<(
        Query<Entity, With<GameEntity>>,
        Query<&mut Window, With<PrimaryWindow>>,
        Query<&mut Transform, With<GameEntity>>,
    )>,
) {
    if !keys.just_pressed(KeyCode::KeyF) {
        return;
    }

    let toggle_result = {
        let mut windows = fullscreen_queries.p1();
        toggle_primary_window_fullscreen(&mut windows, &mut mode_select)
    };

    if let Some((old_scale, new_scale)) = toggle_result {
        if game_status.phase == GamePhase::ModeSelect {
            respawn_mode_select_screen(
                &mut commands,
                &assets,
                &mode_select,
                &fullscreen_queries.p0(),
            );
        } else {
            rescale_game_entity_transforms(&mut fullscreen_queries.p2(), old_scale, new_scale);
        }
    }
}

pub(super) fn change_mode_select_window_scale(mode_select: &mut ModeSelect, scale: u32) {
    mode_select.window_scale = clamp_window_scale(scale);
    set_window_scale(mode_select.window_scale);
}

pub(super) fn resize_primary_window(
    primary_window: &mut Query<&mut Window, With<PrimaryWindow>>,
    scale: u32,
) {
    if browser_controls_primary_window_resolution() {
        return;
    }

    if let Ok(mut window) = primary_window.single_mut() {
        let (width, height) = virtual_window_size(scale as f32);
        window.resolution.set(width as f32, height as f32);
    }
}

pub(super) fn toggle_window_mode(mode: WindowMode) -> WindowMode {
    match mode {
        WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
        WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen(_, _) => WindowMode::Windowed,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FullscreenScalePolicy {
    DesktopFixedScale,
    PreserveCurrentScale,
}

pub(super) fn fullscreen_scale_policy() -> FullscreenScalePolicy {
    if cfg!(target_arch = "wasm32") {
        FullscreenScalePolicy::PreserveCurrentScale
    } else {
        FullscreenScalePolicy::DesktopFixedScale
    }
}

fn browser_controls_primary_window_resolution() -> bool {
    cfg!(target_arch = "wasm32")
}

pub(super) fn toggle_window_fullscreen(
    window: &mut Window,
    mode_select: &mut ModeSelect,
) -> (u32, u32) {
    toggle_window_fullscreen_with_policy(window, mode_select, fullscreen_scale_policy())
}

pub(super) fn toggle_window_fullscreen_with_policy(
    window: &mut Window,
    mode_select: &mut ModeSelect,
    policy: FullscreenScalePolicy,
) -> (u32, u32) {
    let old_scale = window_scale() as u32;
    window.mode = toggle_window_mode(window.mode);
    match (window.mode, policy) {
        (WindowMode::Windowed, FullscreenScalePolicy::DesktopFixedScale) => {
            restore_windowed_scale(window, mode_select);
        }
        (WindowMode::Windowed, FullscreenScalePolicy::PreserveCurrentScale) => {
            keep_current_window_scale(mode_select);
        }
        (
            WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen(_, _),
            FullscreenScalePolicy::DesktopFixedScale,
        ) => {
            store_windowed_scale_setting(clamp_window_scale(mode_select.window_scale));
            mode_select.window_scale = MAX_WINDOW_SCALE;
            set_window_scale(MAX_WINDOW_SCALE);
        }
        (
            WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen(_, _),
            FullscreenScalePolicy::PreserveCurrentScale,
        ) => {
            keep_current_window_scale(mode_select);
        }
    }
    (old_scale, window_scale() as u32)
}

fn restore_windowed_scale(window: &mut Window, mode_select: &mut ModeSelect) {
    let scale = clamp_window_scale(load_windowed_scale_setting());
    mode_select.window_scale = scale;
    set_window_scale(scale);
    let (width, height) = virtual_window_size(scale as f32);
    window.resolution.set(width as f32, height as f32);
}

fn keep_current_window_scale(mode_select: &mut ModeSelect) {
    let scale = clamp_window_scale(mode_select.window_scale);
    mode_select.window_scale = scale;
    set_window_scale(scale);
}

pub(super) fn toggle_primary_window_fullscreen(
    primary_window: &mut Query<&mut Window, With<PrimaryWindow>>,
    mode_select: &mut ModeSelect,
) -> Option<(u32, u32)> {
    if let Ok(mut window) = primary_window.single_mut() {
        Some(toggle_window_fullscreen(&mut window, mode_select))
    } else {
        None
    }
}

pub(super) fn rescale_game_entity_transforms(
    transforms: &mut Query<&mut Transform, With<GameEntity>>,
    old_scale: u32,
    new_scale: u32,
) {
    if old_scale == 0 || old_scale == new_scale {
        return;
    }

    let ratio = new_scale as f32 / old_scale as f32;
    for mut transform in transforms.iter_mut() {
        transform.translation.x *= ratio;
        transform.translation.y *= ratio;
        transform.scale *= ratio;
    }
}

pub(super) fn sync_2d_camera_projection(
    mut projections: Query<&mut Projection, Or<(With<Main2dCamera>, With<View3dHudCamera>)>>,
) {
    for mut projection in &mut projections {
        *projection = Projection::Orthographic(game_2d_projection());
    }
}
