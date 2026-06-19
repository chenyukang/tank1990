use bevy::audio::AddAudioSource;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowPlugin};

use crate::*;

pub(crate) fn run_app() {
    let window_size = virtual_window_size(window_scale());

    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(PlayerControl::default())
        .insert_resource(GameMode::Campaign)
        .insert_resource(ModeSelect::default())
        .insert_resource(GameStatus::default())
        .insert_resource(EnemyFreeze::default())
        .insert_resource(VersusPlayerFreeze::default())
        .insert_resource(BaseReinforcement::default())
        .insert_resource(VersusPowerUpDirector::inactive())
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tank 1990 Bevy Remake".into(),
                        resolution: window_size.into(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: cfg!(target_arch = "wasm32"),
                        #[cfg(target_arch = "wasm32")]
                        canvas: Some("#bevy-canvas".into()),
                        #[cfg(target_arch = "wasm32")]
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_audio_source::<RetroSound>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_3d_view.after(setup))
        .add_systems(
            Update,
            (
                handle_fullscreen_toggle,
                handle_shared_controls,
                update_player_control,
                handle_view_hotkeys,
                sync_2d_camera_projection,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            spawn_versus_powerups
                .after(cancel_colliding_bullets)
                .before(pickup_powerups),
        )
        .add_systems(FixedUpdate, advance_after_stage_intro.before(spawn_enemies))
        .add_systems(
            FixedUpdate,
            update_versus_frozen_player_visuals
                .after(tick_shields)
                .before(update_enemy_visual_feedback),
        )
        .add_systems(
            FixedUpdate,
            sync_shield_visuals
                .after(tick_shields)
                .before(update_versus_frozen_player_visuals),
        )
        .add_systems(
            FixedUpdate,
            update_base_reinforcement_visuals
                .after(tick_powerup_effects)
                .before(update_powerup_visuals),
        )
        .add_systems(
            FixedUpdate,
            sync_background_music.after(update_status_panel),
        )
        .add_systems(
            FixedUpdate,
            clear_terminal_transients
                .after(check_game_phase)
                .before(advance_after_level_clear),
        )
        .add_systems(FixedUpdate, tick_destroyed_tanks.after(animate_sprites))
        .add_systems(
            FixedUpdate,
            (
                spawn_enemies,
                move_player_tank,
                move_enemy_tanks,
                fire_player_bullet,
                fire_enemy_bullets,
                move_bullets,
                cancel_colliding_bullets,
                pickup_powerups,
                tick_powerup_effects,
                update_powerup_visuals,
                animate_sprites,
                tick_spawn_protections,
                tick_player_respawns,
                tick_shields,
                update_enemy_visual_feedback,
                check_game_phase,
                advance_after_level_clear,
                update_status_panel,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                sync_view_cameras,
                sync_3d_static_scene,
                sync_3d_dynamic_scene,
                update_3d_chase_camera,
                sync_3d_hud,
            )
                .chain()
                .after(update_status_panel),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mode_select: Res<ModeSelect>,
    mut images: ResMut<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut retro_sounds: ResMut<Assets<RetroSound>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(game_2d_projection()),
        Main2dCamera,
    ));

    let sprite_assets = create_sprite_assets(&asset_server, &mut images, &mut atlas_layouts);
    let sound_assets =
        create_sound_assets(&asset_server, &mut retro_sounds, &sprite_assets.manifest);
    spawn_mode_select_screen(&mut commands, &sprite_assets, &mode_select);

    commands.insert_resource(sprite_assets);
    commands.insert_resource(sound_assets);
    commands.insert_resource(TileGrid::empty());
    commands.insert_resource(StageRules::default());
    commands.insert_resource(EnemyDirector::inactive());
    commands.insert_resource(ScoreBoard::campaign(0));
}
