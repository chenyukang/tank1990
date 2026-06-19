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

pub(super) fn toggle_window_fullscreen(
    window: &mut Window,
    mode_select: &mut ModeSelect,
) -> (u32, u32) {
    let old_scale = window_scale() as u32;
    window.mode = toggle_window_mode(window.mode);
    match window.mode {
        WindowMode::Windowed => {
            let scale = clamp_window_scale(load_windowed_scale_setting());
            mode_select.window_scale = scale;
            set_window_scale(scale);
            let (width, height) = virtual_window_size(scale as f32);
            window.resolution.set(width as f32, height as f32);
        }
        WindowMode::BorderlessFullscreen(_) | WindowMode::Fullscreen(_, _) => {
            store_windowed_scale_setting(clamp_window_scale(mode_select.window_scale));
            mode_select.window_scale = MAX_WINDOW_SCALE;
            set_window_scale(MAX_WINDOW_SCALE);
        }
    }
    (old_scale, window_scale() as u32)
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
