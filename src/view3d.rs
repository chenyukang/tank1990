use super::*;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{PerspectiveProjection, Projection};
use bevy::render::view::Msaa;

const VIEW_3D_LAYER: usize = 1;
const VIEW_3D_HUD_LAYER: usize = 2;
const VIEW_3D_GROUND_THICKNESS: f32 = 0.2;
const VIEW_3D_BRICK_HEIGHT: f32 = 3.4;
const VIEW_3D_STEEL_HEIGHT: f32 = 4.8;
const VIEW_3D_FOREST_HEIGHT: f32 = 5.2;
const VIEW_3D_TILE_VISUAL_FOOTPRINT: f32 = TILE_SIZE - 0.8;
const VIEW_3D_WATER_SURFACE_HEIGHT: f32 = 0.12;
const VIEW_3D_ICE_SURFACE_HEIGHT: f32 = 0.1;
const VIEW_3D_TANK_HEIGHT: f32 = 3.4;
const VIEW_3D_PROTECTION_BAR_LENGTH: f32 = TANK_SIZE + 2.0;
const VIEW_3D_PROTECTION_BAR_THICKNESS: f32 = 0.38;
const VIEW_3D_PROTECTION_BAR_OFFSET: f32 = TANK_SIZE / 2.0 + 0.65;
const VIEW_3D_PROTECTION_Y: f32 = VIEW_3D_TANK_HEIGHT + 0.7;
const VIEW_3D_BASE_HEIGHT: f32 = 5.8;
const VIEW_3D_PLAYER_MARKER_HEIGHT: f32 = 8.2;
const VIEW_3D_VIEW_TARGET_MARKER_HEIGHT: f32 = 12.4;
const VIEW_3D_MARKER_HEIGHT: f32 = 16.0;
const VIEW_3D_CARRIER_MARKER_HEIGHT: f32 = 10.0;
const VIEW_3D_AIM_GUIDE_MAX_LENGTH: f32 = TILE_SIZE * 4.0;
const VIEW_3D_AIM_GUIDE_START_OFFSET: f32 = TANK_SIZE / 2.0 + 4.0;
const VIEW_3D_AIM_GUIDE_SAMPLE_STEP: f32 = TILE_SIZE / 4.0;
const VIEW_3D_AIM_GUIDE_WIDTH: f32 = 1.4;
const VIEW_3D_AIM_GUIDE_HEIGHT: f32 = 0.24;
const VIEW_3D_AIM_GUIDE_Y: f32 = 0.45;
const VIEW_3D_CAMERA_DISTANCE: f32 = 38.0;
const VIEW_3D_CAMERA_HEIGHT: f32 = 22.0;
const VIEW_3D_CAMERA_LOOK_AHEAD: f32 = 36.0;
const VIEW_3D_CAMERA_LOOK_HEIGHT: f32 = 3.6;
const VIEW_3D_CAMERA_FOV_DEGREES: f32 = 38.0;
const VIEW_3D_CAMERA_TURN_RATE: f32 = std::f32::consts::PI * 7.0;
const VIEW_3D_CAMERA_POSITION_RESPONSE: f32 = 24.0;
const VIEW_3D_CAMERA_HEIGHT_MODE_SETTLED_DOT: f32 = 0.995;
const VIEW_3D_OCCLUDED_CAMERA_DISTANCE: f32 = 30.0;
const VIEW_3D_OCCLUDED_CAMERA_HEIGHT: f32 = 52.0;
const VIEW_3D_OCCLUDED_LOOK_HEIGHT: f32 = 12.0;
const VIEW_3D_HUD_LEFT: f32 = 6.0;
const VIEW_3D_HUD_TOP: f32 = 6.0;
const VIEW_3D_HUD_LINE_STEP: f32 = 9.0;
const VIEW_3D_HUD_TEXT_Z: f32 = 21.0;
const VIEW_3D_HUD_PANEL_Z: f32 = 20.0;
const VIEW_3D_ENEMY_RESERVE_LEFT: f32 = VIEW_3D_HUD_LEFT;
const VIEW_3D_ENEMY_RESERVE_TOP: f32 = 72.0;
const VIEW_3D_ENEMY_RESERVE_COLUMNS: usize = 10;
const VIEW_3D_ENEMY_RESERVE_ICON_SIZE: f32 = 5.0;
const VIEW_3D_ENEMY_RESERVE_CELL_X: f32 = 6.0;
const VIEW_3D_ENEMY_RESERVE_CELL_Y: f32 = 6.0;
pub(super) const VIEW_3D_MINIMAP_CELL_PIXELS: usize = 3;
pub(super) const VIEW_3D_MINIMAP_SIZE: usize = BOARD_TILES * VIEW_3D_MINIMAP_CELL_PIXELS;
const VIEW_3D_MINIMAP_PANEL_PADDING: f32 = 3.0;
const VIEW_3D_MINIMAP_EDGE_MARGIN: f32 = 3.0;

const _: () = {
    assert!(VIEW_3D_TILE_VISUAL_FOOTPRINT < TILE_SIZE);
    assert!(VIEW_3D_TILE_VISUAL_FOOTPRINT <= TILE_SIZE - 0.5);
    assert!(
        VIEW_3D_PROTECTION_BAR_OFFSET - VIEW_3D_PROTECTION_BAR_THICKNESS / 2.0 > TANK_SIZE / 2.0
    );
    assert!(VIEW_3D_PROTECTION_Y > VIEW_3D_TANK_HEIGHT);
};

const MINIMAP_EMPTY_COLOR: [u8; 4] = [8, 8, 8, 210];
const MINIMAP_BRICK_COLOR: [u8; 4] = [152, 64, 36, 255];
const MINIMAP_STEEL_COLOR: [u8; 4] = [144, 152, 160, 255];
const MINIMAP_WATER_COLOR: [u8; 4] = [28, 96, 184, 245];
const MINIMAP_FOREST_COLOR: [u8; 4] = [40, 120, 56, 245];
const MINIMAP_ICE_COLOR: [u8; 4] = [184, 232, 248, 255];
const MINIMAP_BASE_COLOR: [u8; 4] = [248, 216, 96, 255];
const MINIMAP_PLAYER_ONE_BASE_COLOR: [u8; 4] = [144, 248, 152, 255];
const MINIMAP_PLAYER_TWO_BASE_COLOR: [u8; 4] = [112, 184, 255, 255];
const MINIMAP_PLAYER_ONE_COLOR: [u8; 4] = [184, 248, 184, 255];
const MINIMAP_PLAYER_TWO_COLOR: [u8; 4] = [136, 216, 255, 255];
const MINIMAP_ENEMY_COLOR: [u8; 4] = [248, 88, 80, 255];
const MINIMAP_PLAYER_ONE_BULLET_COLOR: [u8; 4] = [184, 248, 184, 255];
const MINIMAP_PLAYER_TWO_BULLET_COLOR: [u8; 4] = [136, 216, 255, 255];
const MINIMAP_ENEMY_BULLET_COLOR: [u8; 4] = [248, 88, 80, 255];
const MINIMAP_POWERUP_COLOR: [u8; 4] = [248, 216, 72, 255];
const MINIMAP_POWERUP_HELMET_COLOR: [u8; 4] = [112, 216, 248, 255];
const MINIMAP_POWERUP_CLOCK_COLOR: [u8; 4] = [176, 176, 248, 255];
const MINIMAP_POWERUP_GRENADE_COLOR: [u8; 4] = [248, 88, 72, 255];
const MINIMAP_POWERUP_SHOVEL_COLOR: [u8; 4] = [184, 232, 160, 255];
const MINIMAP_POWERUP_TANK_COLOR: [u8; 4] = [184, 248, 184, 255];
const MINIMAP_TARGET_COLOR: [u8; 4] = [255, 255, 255, 255];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TankViewMode {
    TwoD,
    ThreeD,
}

#[derive(Component)]
pub(super) struct Main2dCamera;

#[derive(Component)]
pub(super) struct Tank3dCamera;

#[derive(Component)]
pub(super) struct View3dHudCamera;

#[derive(Component)]
pub(super) struct View3dStatic;

#[derive(Component)]
pub(super) struct View3dDynamic;

#[derive(Component)]
pub(super) struct View3dHud;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum View3dCameraHeightMode {
    Chase,
    Tactical,
}

#[derive(Default)]
pub(super) struct View3dCameraState {
    followed_player: Option<PlayerId>,
    forward: Option<Vec2>,
    transform: Option<Transform>,
    height_mode: Option<View3dCameraHeightMode>,
}

impl View3dCameraState {
    pub(super) fn reset(&mut self) {
        self.followed_player = None;
        self.forward = None;
        self.transform = None;
        self.height_mode = None;
    }

    pub(super) fn smoothed_forward(
        &mut self,
        player: PlayerId,
        desired_forward: Vec2,
        delta_secs: f32,
    ) -> Vec2 {
        if self.followed_player != Some(player) {
            self.followed_player = Some(player);
            self.forward = Some(desired_forward);
            self.transform = None;
            self.height_mode = None;
            return desired_forward;
        }

        let Some(current_forward) = self.forward else {
            self.forward = Some(desired_forward);
            return desired_forward;
        };

        let next_forward = rotate_direction_toward(
            current_forward,
            desired_forward,
            VIEW_3D_CAMERA_TURN_RATE * delta_secs.max(0.0),
        );
        self.forward = Some(next_forward);
        next_forward
    }

    pub(super) fn stable_height_mode(
        &mut self,
        camera_forward: Vec2,
        desired_forward: Vec2,
        desired_mode: impl FnOnce() -> View3dCameraHeightMode,
    ) -> View3dCameraHeightMode {
        if self.height_mode.is_none()
            || camera_direction_is_settled(camera_forward, desired_forward)
        {
            let desired_mode = desired_mode();
            self.height_mode = Some(desired_mode);
            return desired_mode;
        }

        self.height_mode
            .expect("height mode should be initialized before turn settling")
    }

    pub(super) fn smoothed_transform(
        &mut self,
        player: PlayerId,
        desired_transform: Transform,
        delta_secs: f32,
    ) -> Transform {
        if self.followed_player != Some(player) {
            self.followed_player = Some(player);
            self.forward = None;
            self.transform = Some(desired_transform);
            self.height_mode = None;
            return desired_transform;
        }

        let Some(current_transform) = self.transform else {
            self.transform = Some(desired_transform);
            return desired_transform;
        };

        let alpha = view_3d_smoothing_alpha(VIEW_3D_CAMERA_POSITION_RESPONSE, delta_secs);
        let next_transform = Transform {
            translation: current_transform
                .translation
                .lerp(desired_transform.translation, alpha),
            rotation: current_transform
                .rotation
                .slerp(desired_transform.rotation, alpha),
            scale: desired_transform.scale,
        };
        self.transform = Some(next_transform);
        next_transform
    }
}

#[derive(Resource)]
pub(super) struct View3dAssets {
    minimap_image: Handle<Image>,
    ground_mesh: Handle<Mesh>,
    floor_grid_x_mesh: Handle<Mesh>,
    floor_grid_z_mesh: Handle<Mesh>,
    floor_tile_mesh: Handle<Mesh>,
    tank_body_mesh: Handle<Mesh>,
    tank_turret_mesh: Handle<Mesh>,
    tank_barrel_x_mesh: Handle<Mesh>,
    tank_barrel_z_mesh: Handle<Mesh>,
    tank_track_x_mesh: Handle<Mesh>,
    tank_track_z_mesh: Handle<Mesh>,
    protection_x_mesh: Handle<Mesh>,
    protection_z_mesh: Handle<Mesh>,
    effect_mesh: Handle<Mesh>,
    base_mesh: Handle<Mesh>,
    bullet_mesh: Handle<Mesh>,
    marker_mesh: Handle<Mesh>,
    powerup_mesh: Handle<Mesh>,
    ground_material: Handle<StandardMaterial>,
    grid_line_material: Handle<StandardMaterial>,
    brick_material: Handle<StandardMaterial>,
    brick_mortar_material: Handle<StandardMaterial>,
    brick_shadow_material: Handle<StandardMaterial>,
    steel_material: Handle<StandardMaterial>,
    steel_dark_material: Handle<StandardMaterial>,
    steel_bolt_material: Handle<StandardMaterial>,
    forest_material: Handle<StandardMaterial>,
    forest_dark_material: Handle<StandardMaterial>,
    forest_trunk_material: Handle<StandardMaterial>,
    water_material: Handle<StandardMaterial>,
    water_highlight_material: Handle<StandardMaterial>,
    ice_material: Handle<StandardMaterial>,
    ice_mark_material: Handle<StandardMaterial>,
    player_one_material: Handle<StandardMaterial>,
    player_two_material: Handle<StandardMaterial>,
    player_upgrade_zero_material: Handle<StandardMaterial>,
    player_upgrade_one_material: Handle<StandardMaterial>,
    player_upgrade_two_material: Handle<StandardMaterial>,
    player_upgrade_three_material: Handle<StandardMaterial>,
    player_one_marker_material: Handle<StandardMaterial>,
    player_two_marker_material: Handle<StandardMaterial>,
    view_target_marker_material: Handle<StandardMaterial>,
    enemy_basic_material: Handle<StandardMaterial>,
    enemy_fast_material: Handle<StandardMaterial>,
    enemy_power_material: Handle<StandardMaterial>,
    enemy_armor_material: Handle<StandardMaterial>,
    barrel_material: Handle<StandardMaterial>,
    base_material: Handle<StandardMaterial>,
    player_one_base_material: Handle<StandardMaterial>,
    player_two_base_material: Handle<StandardMaterial>,
    enemy_marker_material: Handle<StandardMaterial>,
    base_marker_material: Handle<StandardMaterial>,
    player_one_bullet_material: Handle<StandardMaterial>,
    player_two_bullet_material: Handle<StandardMaterial>,
    enemy_bullet_material: Handle<StandardMaterial>,
    powerup_star_material: Handle<StandardMaterial>,
    powerup_helmet_material: Handle<StandardMaterial>,
    powerup_clock_material: Handle<StandardMaterial>,
    powerup_grenade_material: Handle<StandardMaterial>,
    powerup_shovel_material: Handle<StandardMaterial>,
    powerup_tank_material: Handle<StandardMaterial>,
    shield_material: Handle<StandardMaterial>,
    spawn_protection_material: Handle<StandardMaterial>,
    frozen_material: Handle<StandardMaterial>,
    aim_guide_material: Handle<StandardMaterial>,
    explosion_material: Handle<StandardMaterial>,
    base_destruction_material: Handle<StandardMaterial>,
    bullet_impact_material: Handle<StandardMaterial>,
    spawn_effect_material: Handle<StandardMaterial>,
    powerup_sparkle_material: Handle<StandardMaterial>,
}

pub(super) fn next_tank_view_mode(mode: TankViewMode) -> TankViewMode {
    match mode {
        TankViewMode::TwoD => TankViewMode::ThreeD,
        TankViewMode::ThreeD => TankViewMode::TwoD,
    }
}

pub(super) fn previous_tank_view_mode(mode: TankViewMode) -> TankViewMode {
    next_tank_view_mode(mode)
}

pub(super) fn tank_view_mode_label(mode: TankViewMode) -> &'static str {
    match mode {
        TankViewMode::TwoD => "2D",
        TankViewMode::ThreeD => "3D",
    }
}

pub(super) fn toggle_view_assist(enabled: bool) -> bool {
    !enabled
}

pub(super) fn view_assist_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

pub(super) fn active_3d_view_target(mode: GameMode, requested: PlayerId) -> PlayerId {
    if mode.has_two_player_targets() {
        requested
    } else {
        PlayerId::One
    }
}

pub(super) fn resolved_3d_view_target(
    mode: GameMode,
    requested: PlayerId,
    available_players: impl IntoIterator<Item = PlayerId>,
) -> PlayerId {
    let active = active_3d_view_target(mode, requested);
    if !mode.has_two_player_targets() {
        return active;
    }

    let mut has_p1 = false;
    let mut has_p2 = false;
    for player in available_players {
        match player {
            PlayerId::One => has_p1 = true,
            PlayerId::Two => has_p2 = true,
        }
    }

    match active {
        PlayerId::One if has_p1 => PlayerId::One,
        PlayerId::Two if has_p2 => PlayerId::Two,
        PlayerId::One if has_p2 => PlayerId::Two,
        PlayerId::Two if has_p1 => PlayerId::One,
        _ => active,
    }
}

pub(super) fn setup_3d_view(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.insert_resource(create_3d_assets(&mut meshes, &mut materials, &mut images));
    commands.spawn((
        DirectionalLight {
            illuminance: 16_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-90.0, 120.0, -90.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(VIEW_3D_LAYER),
    ));
    commands.spawn((
        PointLight {
            intensity: 1_100_000.0,
            range: 650.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 90.0, -40.0),
        RenderLayers::layer(VIEW_3D_LAYER),
    ));
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: false,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: VIEW_3D_CAMERA_FOV_DEGREES.to_radians(),
            near: 0.5,
            far: 600.0,
            ..default()
        }),
        Msaa::Sample4,
        Transform::from_xyz(0.0, 145.0, 180.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(VIEW_3D_LAYER),
        Tank3dCamera,
    ));
    commands.spawn((
        Camera2d,
        Camera {
            is_active: false,
            order: 2,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHudCamera,
    ));
}

fn create_3d_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) -> View3dAssets {
    View3dAssets {
        minimap_image: images.add(create_3d_minimap_image()),
        ground_mesh: meshes.add(Cuboid::new(
            board_size(),
            VIEW_3D_GROUND_THICKNESS,
            board_size(),
        )),
        floor_grid_x_mesh: meshes.add(Cuboid::new(board_size(), 0.05, 0.16)),
        floor_grid_z_mesh: meshes.add(Cuboid::new(0.16, 0.05, board_size())),
        floor_tile_mesh: meshes.add(Cuboid::new(TILE_SIZE, 0.08, TILE_SIZE)),
        tank_body_mesh: meshes.add(Cuboid::new(12.0, VIEW_3D_TANK_HEIGHT, 12.0)),
        tank_turret_mesh: meshes.add(Cuboid::new(8.0, 1.4, 8.0)),
        tank_barrel_x_mesh: meshes.add(Cuboid::new(13.0, 1.5, 2.0)),
        tank_barrel_z_mesh: meshes.add(Cuboid::new(2.0, 1.5, 13.0)),
        tank_track_x_mesh: meshes.add(Cuboid::new(13.4, 0.5, 1.5)),
        tank_track_z_mesh: meshes.add(Cuboid::new(1.5, 0.5, 13.4)),
        protection_x_mesh: meshes.add(Cuboid::new(
            VIEW_3D_PROTECTION_BAR_LENGTH,
            0.3,
            VIEW_3D_PROTECTION_BAR_THICKNESS,
        )),
        protection_z_mesh: meshes.add(Cuboid::new(
            VIEW_3D_PROTECTION_BAR_THICKNESS,
            0.3,
            VIEW_3D_PROTECTION_BAR_LENGTH,
        )),
        effect_mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        base_mesh: meshes.add(Cuboid::new(TANK_SIZE, VIEW_3D_BASE_HEIGHT, TANK_SIZE)),
        bullet_mesh: meshes.add(Cuboid::new(2.4, 2.4, 2.4)),
        marker_mesh: meshes.add(Cuboid::new(3.0, 3.0, 3.0)),
        powerup_mesh: meshes.add(Cuboid::new(6.0, 1.5, 6.0)),
        ground_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(30, 38, 34),
            perceptual_roughness: 0.96,
            ..default()
        }),
        grid_line_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(0, 0, 0, 72),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        brick_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(172, 82, 50),
            perceptual_roughness: 0.9,
            ..default()
        }),
        brick_mortar_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(72, 40, 32),
            perceptual_roughness: 1.0,
            ..default()
        }),
        brick_shadow_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(120, 48, 34),
            perceptual_roughness: 0.94,
            ..default()
        }),
        steel_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(158, 168, 176),
            perceptual_roughness: 0.42,
            metallic: 0.35,
            ..default()
        }),
        steel_dark_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(72, 80, 88),
            perceptual_roughness: 0.5,
            metallic: 0.4,
            ..default()
        }),
        steel_bolt_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(216, 224, 232),
            perceptual_roughness: 0.34,
            metallic: 0.5,
            ..default()
        }),
        forest_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(56, 140, 72, 220),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.9,
            ..default()
        }),
        forest_dark_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(28, 88, 44, 230),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.94,
            ..default()
        }),
        forest_trunk_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(92, 58, 34),
            perceptual_roughness: 0.86,
            ..default()
        }),
        water_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(28, 96, 184, 210),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.6,
            ..default()
        }),
        water_highlight_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(112, 192, 248, 190),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        ice_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(184, 232, 248),
            perceptual_roughness: 0.28,
            ..default()
        }),
        ice_mark_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(232, 252, 255, 210),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        player_one_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(172, 248, 184),
            perceptual_roughness: 0.55,
            ..default()
        }),
        player_two_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(128, 208, 255),
            perceptual_roughness: 0.55,
            ..default()
        }),
        player_upgrade_zero_material: materials.add(player_upgrade_3d_color(0)),
        player_upgrade_one_material: materials.add(player_upgrade_3d_color(1)),
        player_upgrade_two_material: materials.add(player_upgrade_3d_color(2)),
        player_upgrade_three_material: materials.add(player_upgrade_3d_color(3)),
        player_one_marker_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(80, 248, 112),
            unlit: true,
            ..default()
        }),
        player_two_marker_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(80, 184, 255),
            unlit: true,
            ..default()
        }),
        view_target_marker_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 255, 255, 235),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        enemy_basic_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(232, 232, 216),
            perceptual_roughness: 0.58,
            ..default()
        }),
        enemy_fast_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(104, 216, 128),
            perceptual_roughness: 0.58,
            ..default()
        }),
        enemy_power_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(248, 112, 96),
            perceptual_roughness: 0.58,
            ..default()
        }),
        enemy_armor_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(176, 176, 216),
            perceptual_roughness: 0.44,
            ..default()
        }),
        barrel_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(42, 48, 42),
            perceptual_roughness: 0.6,
            ..default()
        }),
        base_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(232, 196, 96),
            perceptual_roughness: 0.62,
            ..default()
        }),
        player_one_base_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(144, 248, 152),
            perceptual_roughness: 0.62,
            ..default()
        }),
        player_two_base_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(112, 184, 255),
            perceptual_roughness: 0.62,
            ..default()
        }),
        enemy_marker_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(255, 72, 72),
            unlit: true,
            ..default()
        }),
        base_marker_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(255, 232, 96),
            unlit: true,
            ..default()
        }),
        player_one_bullet_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(184, 248, 184),
            unlit: true,
            ..default()
        }),
        player_two_bullet_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(136, 216, 255),
            unlit: true,
            ..default()
        }),
        enemy_bullet_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(248, 88, 80),
            unlit: true,
            ..default()
        }),
        powerup_star_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(248, 216, 72),
            unlit: true,
            ..default()
        }),
        powerup_helmet_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(112, 216, 248),
            unlit: true,
            ..default()
        }),
        powerup_clock_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(176, 176, 248),
            unlit: true,
            ..default()
        }),
        powerup_grenade_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(248, 88, 72),
            unlit: true,
            ..default()
        }),
        powerup_shovel_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(184, 232, 160),
            unlit: true,
            ..default()
        }),
        powerup_tank_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(184, 248, 184),
            unlit: true,
            ..default()
        }),
        shield_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(112, 216, 248, 128),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        spawn_protection_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(232, 248, 255, 132),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        frozen_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(136, 216, 255),
            unlit: true,
            ..default()
        }),
        aim_guide_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 248, 184, 170),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        explosion_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(248, 128, 48, 230),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        base_destruction_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(184, 80, 40, 230),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        bullet_impact_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 232, 96, 230),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        spawn_effect_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(208, 248, 255, 210),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        powerup_sparkle_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 255, 255, 230),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
    }
}

fn player_upgrade_3d_color(upgrade_level: u8) -> Color {
    let [r, g, b] = player_upgrade_visual_rgb(upgrade_level);
    Color::srgb_u8(r, g, b)
}

pub(super) fn handle_view_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    mut mode_select: ResMut<ModeSelect>,
) {
    if view_toggle_just_pressed(&keys) && game_status.phase != GamePhase::ModeSelect {
        mode_select.view_mode = next_tank_view_mode(mode_select.view_mode);
    }

    if keys.just_pressed(KeyCode::Tab)
        && mode_select.view_mode == TankViewMode::ThreeD
        && game_mode.has_two_player_targets()
        && game_status.phase != GamePhase::ModeSelect
    {
        mode_select.view_target = mode_select.view_target.opponent();
    }
}

fn view_toggle_just_pressed(keys: &ButtonInput<KeyCode>) -> bool {
    keys.just_pressed(KeyCode::KeyV)
        || keys.just_pressed(KeyCode::Digit3)
        || keys.just_pressed(KeyCode::Numpad3)
}

pub(super) fn sync_view_cameras(
    mode_select: Res<ModeSelect>,
    game_status: Res<GameStatus>,
    mut cameras: ParamSet<(
        Query<&mut Camera, With<Main2dCamera>>,
        Query<&mut Camera, With<Tank3dCamera>>,
        Query<&mut Camera, With<View3dHudCamera>>,
    )>,
) {
    let render_3d = view_3d_should_render(&mode_select, &game_status);
    for mut camera in &mut cameras.p0() {
        camera.is_active = !render_3d;
    }
    for mut camera in &mut cameras.p1() {
        camera.is_active = render_3d;
    }
    for mut camera in &mut cameras.p2() {
        camera.is_active = render_3d;
    }
}

pub(super) fn sync_3d_static_scene(
    mut commands: Commands,
    assets: Option<Res<View3dAssets>>,
    mode_select: Res<ModeSelect>,
    game_status: Res<GameStatus>,
    tile_grid: Res<TileGrid>,
    static_entities: Query<Entity, With<View3dStatic>>,
) {
    let render_3d = view_3d_should_render(&mode_select, &game_status);
    if !render_3d {
        despawn_entities(&mut commands, static_entities.iter());
        return;
    }

    let static_count = static_entities.iter().count();
    if static_count > 0 && !tile_grid.is_changed() {
        return;
    }

    despawn_entities(&mut commands, static_entities.iter());
    let Some(assets) = assets else {
        return;
    };

    spawn_3d_static_scene(&mut commands, &assets, &tile_grid);
}

pub(super) fn sync_3d_dynamic_scene(
    mut commands: Commands,
    assets: Option<Res<View3dAssets>>,
    sprite_assets: Option<Res<SpriteAssets>>,
    mode_select: Res<ModeSelect>,
    game_mode: Option<Res<GameMode>>,
    game_status: Res<GameStatus>,
    tile_grid: Res<TileGrid>,
    enemy_freeze: Option<Res<EnemyFreeze>>,
    versus_freeze: Option<Res<VersusPlayerFreeze>>,
    dynamic_entities: Query<Entity, With<View3dDynamic>>,
    tanks: Query<(
        Entity,
        &Tank,
        Option<&Player>,
        Option<&PlayerUpgrade>,
        Option<&EnemyTank>,
        Option<&Health>,
        Option<&Shield>,
        Option<&SpawnProtection>,
    )>,
    bullets: Query<(Entity, &Bullet)>,
    bases: Query<(Entity, &BaseSprite)>,
    powerups: Query<(Entity, &PowerUp, &Transform)>,
    effects: Query<(Entity, &Transform, &SpriteAnimation)>,
) {
    despawn_entities(&mut commands, dynamic_entities.iter());

    if !view_3d_should_render(&mode_select, &game_status) {
        return;
    }
    let Some(assets) = assets else {
        return;
    };
    let enemy_frozen = enemy_freeze.as_deref().is_some_and(EnemyFreeze::is_active);
    let frozen_players = versus_freeze.as_deref();
    let view_target = game_mode
        .as_deref()
        .map(|mode| {
            resolved_3d_view_target(
                *mode,
                mode_select.view_target,
                tanks
                    .iter()
                    .filter_map(|(_, _, player, _, _, _, _, _)| player.map(|player| player.id)),
            )
        })
        .unwrap_or(PlayerId::One);

    for (entity, tank, player, player_upgrade, enemy, health, shield, spawn_protection) in &tanks {
        if !top_left_is_on_board(tank.top_left) {
            continue;
        }
        spawn_3d_tank(
            &mut commands,
            &assets,
            entity,
            tank,
            player,
            player_upgrade,
            enemy,
            health,
            shield,
            spawn_protection,
            mode_select.view_assist,
            &tile_grid,
            enemy_frozen,
            frozen_players,
            player.is_some_and(|player| player.id == view_target),
        );
    }

    for (entity, bullet) in &bullets {
        if !top_left_is_on_board(bullet.top_left) || bullet.resolved {
            continue;
        }
        spawn_3d_bullet(&mut commands, &assets, entity, bullet);
    }

    for (entity, base) in &bases {
        spawn_3d_base(
            &mut commands,
            &assets,
            entity,
            base,
            mode_select.view_assist,
        );
    }

    for (entity, powerup, transform) in &powerups {
        let top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        if top_left_is_on_board(top_left) {
            spawn_3d_powerup(&mut commands, &assets, entity, powerup.kind, top_left);
        }
    }

    if let Some(sprite_assets) = sprite_assets {
        for (entity, transform, animation) in &effects {
            let Some(kind) = view_3d_effect_kind(animation, &sprite_assets.manifest) else {
                continue;
            };
            let size = view_3d_effect_size(kind);
            let top_left = board_top_left_from_translation(transform.translation, size);
            if top_left_is_on_board(top_left) {
                spawn_3d_effect(&mut commands, &assets, entity, kind, top_left);
            }
        }
    }
}

pub(super) fn update_3d_chase_camera(
    time: Option<Res<Time>>,
    mode_select: Res<ModeSelect>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    tile_grid: Res<TileGrid>,
    tanks: Query<(&Tank, &Player)>,
    mut cameras: Query<&mut Transform, With<Tank3dCamera>>,
    mut camera_state: Local<View3dCameraState>,
) {
    if !view_3d_should_render(&mode_select, &game_status) {
        camera_state.reset();
        return;
    }

    let view_target = resolved_3d_view_target(
        *game_mode,
        mode_select.view_target,
        tanks.iter().map(|(_, player)| player.id),
    );
    let target_player = tanks
        .iter()
        .find(|(_, player)| player.id == view_target)
        .or_else(|| tanks.iter().next());
    let Some((tank, player)) = target_player else {
        camera_state.reset();
        for mut transform in &mut cameras {
            *transform = overview_camera_transform();
        }
        return;
    };

    let delta_secs = time.as_deref().map(Time::delta_secs).unwrap_or(1.0 / 60.0);
    let desired_forward = tank.facing.movement();
    let camera_forward = camera_state.smoothed_forward(player.id, desired_forward, delta_secs);
    let height_mode = camera_state.stable_height_mode(camera_forward, desired_forward, || {
        chase_camera_height_mode(tank, &tile_grid, desired_forward)
    });
    let desired_camera_transform =
        chase_camera_transform_with_forward_and_height_mode(tank, camera_forward, height_mode);
    let camera_transform =
        camera_state.smoothed_transform(player.id, desired_camera_transform, delta_secs);
    for mut transform in &mut cameras {
        *transform = camera_transform;
    }
}

pub(super) fn sync_3d_hud(
    mut commands: Commands,
    assets: Option<Res<SpriteAssets>>,
    view_assets: Option<Res<View3dAssets>>,
    mut images: Option<ResMut<Assets<Image>>>,
    mode_select: Res<ModeSelect>,
    game_mode: Res<GameMode>,
    game_status: Res<GameStatus>,
    score_board: Res<ScoreBoard>,
    tile_grid: Res<TileGrid>,
    hud_entities: Query<Entity, With<View3dHud>>,
    tanks: Query<(&Tank, Option<&Player>, Option<&EnemyTank>)>,
    bullets: Query<&Bullet>,
    bases: Query<&BaseSprite>,
    powerups: Query<(&PowerUp, &Transform)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    despawn_entities(&mut commands, hud_entities.iter());

    if !view_3d_should_render(&mode_select, &game_status) {
        return;
    }
    let Some(assets) = assets else {
        return;
    };

    let view_target = resolved_3d_view_target(
        *game_mode,
        mode_select.view_target,
        tanks
            .iter()
            .filter_map(|(_, player, _)| player.map(|player| player.id)),
    );
    let status_lines = view_3d_status_lines(
        *game_mode,
        &game_status,
        &score_board,
        &mode_select,
        view_target,
    );
    spawn_3d_hud_panel(
        &mut commands,
        Vec2::new(VIEW_3D_HUD_LEFT - 3.0, VIEW_3D_HUD_TOP - 3.0),
        Vec2::new(
            116.0,
            status_lines.len() as f32 * VIEW_3D_HUD_LINE_STEP + 5.0,
        ),
    );
    for (index, line) in status_lines.iter().enumerate() {
        spawn_3d_hud_text(
            &mut commands,
            &assets,
            line,
            Vec2::new(
                VIEW_3D_HUD_LEFT,
                VIEW_3D_HUD_TOP + index as f32 * VIEW_3D_HUD_LINE_STEP,
            ),
        );
    }

    spawn_3d_enemy_reserve(
        &mut commands,
        &assets,
        view_3d_enemy_reserve_marker_count(*game_mode, &score_board),
    );

    if let Some(lines) = phase_banner_text(&game_status, *game_mode, &score_board) {
        spawn_3d_phase_hud(&mut commands, &assets, &lines);
    }

    if let (Some(view_assets), Some(images)) = (view_assets, images.as_deref_mut()) {
        update_3d_minimap_image(
            images,
            &view_assets.minimap_image,
            &tile_grid,
            &tanks,
            &bullets,
            &bases,
            &powerups,
            view_target,
        );
        spawn_3d_minimap(
            &mut commands,
            &view_assets,
            view_3d_hud_window_size(&primary_window),
        );
    }
}

pub(super) fn view_3d_should_render(mode_select: &ModeSelect, game_status: &GameStatus) -> bool {
    mode_select.view_mode == TankViewMode::ThreeD && game_status.phase != GamePhase::ModeSelect
}

pub(super) fn view_3d_status_lines(
    mode: GameMode,
    game_status: &GameStatus,
    score_board: &ScoreBoard,
    mode_select: &ModeSelect,
    view_target: PlayerId,
) -> Vec<String> {
    let mut lines = match mode {
        GameMode::Campaign => vec![
            format!(
                "P1 SCORE {}",
                status_value_text(StatusValue::Score, mode, game_status, score_board)
            ),
            format!(
                "LIFE {}",
                status_value_text(StatusValue::Lives, mode, game_status, score_board)
            ),
            format!(
                "STAGE {}",
                status_value_text(StatusValue::Stage, mode, game_status, score_board)
            ),
            format!(
                "ENEMY {:02}",
                enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed)
                    .min(99)
            ),
        ],
        GameMode::CoopCampaign => vec![
            format!(
                "P1 LIFE {}",
                status_value_text(StatusValue::Lives, mode, game_status, score_board)
            ),
            format!(
                "P2 LIFE {}",
                status_value_text(StatusValue::P2Lives, mode, game_status, score_board)
            ),
            format!(
                "SCORE {}",
                status_value_text(StatusValue::Score, mode, game_status, score_board)
            ),
            format!(
                "STAGE {}",
                status_value_text(StatusValue::Stage, mode, game_status, score_board)
            ),
            format!(
                "ENEMY {:02}",
                enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed)
                    .min(99)
            ),
        ],
        GameMode::VersusDeathmatch => vec![
            format!(
                "P1 {} LIFE {}",
                status_value_text(StatusValue::Score, mode, game_status, score_board),
                status_value_text(StatusValue::Lives, mode, game_status, score_board)
            ),
            format!(
                "P2 {} LIFE {}",
                status_value_text(StatusValue::P2Score, mode, game_status, score_board),
                status_value_text(StatusValue::P2Lives, mode, game_status, score_board)
            ),
            format!(
                "ARENA {}",
                status_value_text(StatusValue::Arena, mode, game_status, score_board)
            ),
            format!(
                "TARGET {}",
                status_value_text(StatusValue::Target, mode, game_status, score_board)
            ),
        ],
        GameMode::VersusBaseBattle => vec![
            format!(
                "P1 {} LIFE {}",
                status_value_text(StatusValue::Score, mode, game_status, score_board),
                status_value_text(StatusValue::Lives, mode, game_status, score_board)
            ),
            format!(
                "P2 {} LIFE {}",
                status_value_text(StatusValue::P2Score, mode, game_status, score_board),
                status_value_text(StatusValue::P2Lives, mode, game_status, score_board)
            ),
            format!(
                "ARENA {}",
                status_value_text(StatusValue::Arena, mode, game_status, score_board)
            ),
            "BASE BATTLE".to_string(),
        ],
    };

    lines.push(format!(
        "VIEW P{}",
        match view_target {
            PlayerId::One => 1,
            PlayerId::Two => 2,
        }
    ));
    lines.push(format!(
        "ASSIST {}",
        view_assist_label(mode_select.view_assist)
    ));
    lines
}

pub(super) fn view_3d_enemy_reserve_marker_count(
    mode: GameMode,
    score_board: &ScoreBoard,
) -> usize {
    if matches!(mode, GameMode::Campaign | GameMode::CoopCampaign) {
        enemy_markers_remaining(score_board.total_enemies, score_board.enemies_destroyed)
            .min(ENEMY_MARKER_COUNT)
    } else {
        0
    }
}

pub(super) fn create_3d_minimap_image() -> Image {
    image_from_pixels(
        VIEW_3D_MINIMAP_SIZE,
        VIEW_3D_MINIMAP_SIZE,
        vec![0; VIEW_3D_MINIMAP_SIZE * VIEW_3D_MINIMAP_SIZE * 4],
    )
}

fn update_3d_minimap_image(
    images: &mut Assets<Image>,
    image: &Handle<Image>,
    tile_grid: &TileGrid,
    tanks: &Query<(&Tank, Option<&Player>, Option<&EnemyTank>)>,
    bullets: &Query<&Bullet>,
    bases: &Query<&BaseSprite>,
    powerups: &Query<(&PowerUp, &Transform)>,
    view_target: PlayerId,
) {
    let pixels = render_3d_minimap_pixels(
        tile_grid,
        tanks.iter(),
        bullets.iter(),
        bases.iter(),
        powerups.iter(),
        view_target,
    );
    if let Some(image) = images.get_mut(image) {
        image.data = Some(pixels);
    }
}

pub(super) fn render_3d_minimap_pixels<'a>(
    tile_grid: &TileGrid,
    tanks: impl IntoIterator<Item = (&'a Tank, Option<&'a Player>, Option<&'a EnemyTank>)>,
    bullets: impl IntoIterator<Item = &'a Bullet>,
    bases: impl IntoIterator<Item = &'a BaseSprite>,
    powerups: impl IntoIterator<Item = (&'a PowerUp, &'a Transform)>,
    view_target: PlayerId,
) -> Vec<u8> {
    let mut pixels = vec![0; VIEW_3D_MINIMAP_SIZE * VIEW_3D_MINIMAP_SIZE * 4];

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            let tile = tile_grid.get(x as i32, y as i32).unwrap_or(TileKind::Empty);
            fill_minimap_cell(&mut pixels, x, y, minimap_tile_color(tile));
        }
    }

    for base in bases {
        draw_minimap_object(
            &mut pixels,
            base.top_left,
            Vec2::splat(TANK_SIZE),
            minimap_base_color(base.owner),
            false,
        );
    }

    for (powerup, transform) in powerups {
        let top_left = board_top_left_from_translation(transform.translation, TANK_SIZE);
        draw_minimap_object(
            &mut pixels,
            top_left,
            Vec2::splat(TANK_SIZE),
            minimap_powerup_color(powerup.kind),
            false,
        );
    }

    for bullet in bullets {
        if bullet.resolved {
            continue;
        }
        draw_minimap_object(
            &mut pixels,
            bullet.top_left,
            Vec2::splat(BULLET_SIZE),
            minimap_bullet_color(bullet.owner),
            false,
        );
    }

    for (tank, player, enemy) in tanks {
        let Some(color) = minimap_tank_color(player, enemy) else {
            continue;
        };
        draw_minimap_object(
            &mut pixels,
            tank.top_left,
            Vec2::splat(TANK_SIZE),
            color,
            player.is_some_and(|player| player.id == view_target),
        );
    }

    pixels
}

fn spawn_3d_minimap(commands: &mut Commands, assets: &View3dAssets, window_size: Vec2) {
    let scale = window_scale();
    let panel_size = view_3d_minimap_panel_size();
    let center = view_3d_minimap_center(window_size, scale);
    commands.spawn((
        Sprite::from_color(
            Color::srgba_u8(0, 0, 0, 190),
            Vec2::new(panel_size.x * scale, panel_size.y * scale),
        ),
        Transform::from_translation(center.extend(VIEW_3D_HUD_PANEL_Z)),
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHud,
        GameEntity,
    ));
    commands.spawn((
        Sprite::from_image(assets.minimap_image.clone()),
        Transform::from_translation(center.extend(VIEW_3D_HUD_TEXT_Z))
            .with_scale(Vec3::splat(scale)),
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHud,
        GameEntity,
    ));
}

fn view_3d_hud_window_size(primary_window: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
    primary_window
        .single()
        .map(|window| Vec2::new(window.resolution.width(), window.resolution.height()))
        .unwrap_or_else(|_| {
            let (width, height) = virtual_window_size(window_scale());
            Vec2::new(width as f32, height as f32)
        })
}

fn view_3d_minimap_panel_size() -> Vec2 {
    Vec2::splat(VIEW_3D_MINIMAP_SIZE as f32 + VIEW_3D_MINIMAP_PANEL_PADDING * 2.0)
}

pub(super) fn view_3d_minimap_center(window_size: Vec2, scale: f32) -> Vec2 {
    let panel_size = view_3d_minimap_panel_size() * scale;
    let edge_margin = VIEW_3D_MINIMAP_EDGE_MARGIN * scale;
    Vec2::new(
        window_size.x / 2.0 - edge_margin - panel_size.x / 2.0,
        window_size.y / 2.0 - edge_margin - panel_size.y / 2.0,
    )
}

fn spawn_3d_enemy_reserve(commands: &mut Commands, assets: &SpriteAssets, count: usize) {
    if count == 0 {
        return;
    }

    let columns = count.min(VIEW_3D_ENEMY_RESERVE_COLUMNS);
    let rows = count.div_ceil(VIEW_3D_ENEMY_RESERVE_COLUMNS);
    let size = Vec2::new(
        columns as f32 * VIEW_3D_ENEMY_RESERVE_CELL_X
            - (VIEW_3D_ENEMY_RESERVE_CELL_X - VIEW_3D_ENEMY_RESERVE_ICON_SIZE),
        rows as f32 * VIEW_3D_ENEMY_RESERVE_CELL_Y
            - (VIEW_3D_ENEMY_RESERVE_CELL_Y - VIEW_3D_ENEMY_RESERVE_ICON_SIZE),
    );
    spawn_3d_hud_panel(
        commands,
        Vec2::new(
            VIEW_3D_ENEMY_RESERVE_LEFT - 3.0,
            VIEW_3D_ENEMY_RESERVE_TOP - 3.0,
        ),
        size + Vec2::splat(6.0),
    );

    for index in 0..count {
        let col = index % VIEW_3D_ENEMY_RESERVE_COLUMNS;
        let row = index / VIEW_3D_ENEMY_RESERVE_COLUMNS;
        let top_left = Vec2::new(
            VIEW_3D_ENEMY_RESERVE_LEFT + col as f32 * VIEW_3D_ENEMY_RESERVE_CELL_X,
            VIEW_3D_ENEMY_RESERVE_TOP + row as f32 * VIEW_3D_ENEMY_RESERVE_CELL_Y,
        );
        commands.spawn((
            Sprite::from_atlas_image(
                assets.tank_image.clone(),
                TextureAtlas {
                    layout: assets.tank_layout.clone(),
                    index: enemy_marker_tank_index(&assets.manifest),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                top_left,
                Vec2::splat(VIEW_3D_ENEMY_RESERVE_ICON_SIZE),
                VIEW_3D_HUD_TEXT_Z,
            ))
            .with_scale(Vec3::splat(
                window_scale() * VIEW_3D_ENEMY_RESERVE_ICON_SIZE / TANK_SIZE,
            )),
            RenderLayers::layer(VIEW_3D_HUD_LAYER),
            View3dHud,
            GameEntity,
            Name::new(format!("3D Enemy Reserve {}", index)),
        ));
    }
}

fn minimap_tile_color(tile: TileKind) -> [u8; 4] {
    match tile {
        TileKind::Empty => MINIMAP_EMPTY_COLOR,
        TileKind::Brick => MINIMAP_BRICK_COLOR,
        TileKind::Steel => MINIMAP_STEEL_COLOR,
        TileKind::Water => MINIMAP_WATER_COLOR,
        TileKind::Forest => MINIMAP_FOREST_COLOR,
        TileKind::Ice => MINIMAP_ICE_COLOR,
        TileKind::Base => MINIMAP_BASE_COLOR,
    }
}

pub(super) fn minimap_base_color(owner: Option<PlayerId>) -> [u8; 4] {
    match base_3d_material_kind(owner) {
        Base3dMaterialKind::Neutral => MINIMAP_BASE_COLOR,
        Base3dMaterialKind::PlayerOne => MINIMAP_PLAYER_ONE_BASE_COLOR,
        Base3dMaterialKind::PlayerTwo => MINIMAP_PLAYER_TWO_BASE_COLOR,
    }
}

pub(super) fn minimap_powerup_color(kind: PowerUpKind) -> [u8; 4] {
    match powerup_3d_material_kind(kind) {
        PowerUp3dMaterialKind::Star => MINIMAP_POWERUP_COLOR,
        PowerUp3dMaterialKind::Helmet => MINIMAP_POWERUP_HELMET_COLOR,
        PowerUp3dMaterialKind::Clock => MINIMAP_POWERUP_CLOCK_COLOR,
        PowerUp3dMaterialKind::Grenade => MINIMAP_POWERUP_GRENADE_COLOR,
        PowerUp3dMaterialKind::Shovel => MINIMAP_POWERUP_SHOVEL_COLOR,
        PowerUp3dMaterialKind::Tank => MINIMAP_POWERUP_TANK_COLOR,
    }
}

pub(super) fn minimap_bullet_color(owner: Team) -> [u8; 4] {
    match bullet_3d_material_kind(owner) {
        Bullet3dMaterialKind::PlayerOne => MINIMAP_PLAYER_ONE_BULLET_COLOR,
        Bullet3dMaterialKind::PlayerTwo => MINIMAP_PLAYER_TWO_BULLET_COLOR,
        Bullet3dMaterialKind::Enemy => MINIMAP_ENEMY_BULLET_COLOR,
    }
}

fn minimap_tank_color(player: Option<&Player>, enemy: Option<&EnemyTank>) -> Option<[u8; 4]> {
    if let Some(player) = player {
        return Some(match player.id {
            PlayerId::One => MINIMAP_PLAYER_ONE_COLOR,
            PlayerId::Two => MINIMAP_PLAYER_TWO_COLOR,
        });
    }
    enemy.map(minimap_enemy_color)
}

pub(super) fn minimap_enemy_color(enemy: &EnemyTank) -> [u8; 4] {
    enemy
        .carried_powerup
        .map(minimap_powerup_color)
        .unwrap_or(MINIMAP_ENEMY_COLOR)
}

fn fill_minimap_cell(pixels: &mut [u8], grid_x: usize, grid_y: usize, color: [u8; 4]) {
    fill_rect(
        pixels,
        VIEW_3D_MINIMAP_SIZE,
        grid_x * VIEW_3D_MINIMAP_CELL_PIXELS,
        grid_y * VIEW_3D_MINIMAP_CELL_PIXELS,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        color,
    );
}

fn draw_minimap_object(
    pixels: &mut [u8],
    top_left: Vec2,
    size: Vec2,
    color: [u8; 4],
    highlighted: bool,
) {
    let Some((grid_x, grid_y)) = minimap_grid_position(top_left, size) else {
        return;
    };

    if highlighted {
        fill_minimap_cell(pixels, grid_x, grid_y, MINIMAP_TARGET_COLOR);
        let x = grid_x * VIEW_3D_MINIMAP_CELL_PIXELS + 1;
        let y = grid_y * VIEW_3D_MINIMAP_CELL_PIXELS + 1;
        fill_rect(pixels, VIEW_3D_MINIMAP_SIZE, x, y, 1, 1, color);
    } else {
        fill_minimap_cell(pixels, grid_x, grid_y, color);
    }
}

fn minimap_grid_position(top_left: Vec2, size: Vec2) -> Option<(usize, usize)> {
    let center = top_left + size / 2.0;
    if center.x < 0.0 || center.y < 0.0 || center.x >= board_size() || center.y >= board_size() {
        return None;
    }
    Some((
        (center.x / TILE_SIZE).floor() as usize,
        (center.y / TILE_SIZE).floor() as usize,
    ))
}

fn spawn_3d_static_scene(commands: &mut Commands, assets: &View3dAssets, tile_grid: &TileGrid) {
    spawn_3d_mesh(
        commands,
        assets.ground_mesh.clone(),
        assets.ground_material.clone(),
        Transform::from_xyz(0.0, -VIEW_3D_GROUND_THICKNESS / 2.0, 0.0),
        View3dStatic,
    );
    spawn_3d_floor_grid(commands, assets);

    for y in 0..BOARD_TILES {
        for x in 0..BOARD_TILES {
            let Some(tile) = tile_grid.get(x as i32, y as i32) else {
                continue;
            };
            spawn_3d_tile(commands, assets, tile, x, y);
        }
    }
}

fn spawn_3d_floor_grid(commands: &mut Commands, assets: &View3dAssets) {
    for index in 0..=BOARD_TILES {
        let position = index as f32 * TILE_SIZE;
        spawn_3d_mesh(
            commands,
            assets.floor_grid_x_mesh.clone(),
            assets.grid_line_material.clone(),
            Transform::from_translation(board_3d_point(
                Vec2::new(board_size() / 2.0, position),
                0.08,
            )),
            View3dStatic,
        );
        spawn_3d_mesh(
            commands,
            assets.floor_grid_z_mesh.clone(),
            assets.grid_line_material.clone(),
            Transform::from_translation(board_3d_point(
                Vec2::new(position, board_size() / 2.0),
                0.08,
            )),
            View3dStatic,
        );
    }
}

fn spawn_3d_tile(
    commands: &mut Commands,
    assets: &View3dAssets,
    tile: TileKind,
    x: usize,
    y: usize,
) {
    match tile {
        TileKind::Empty | TileKind::Base => {}
        TileKind::Brick => spawn_3d_brick_tile(commands, assets, x, y),
        TileKind::Steel => spawn_3d_steel_tile(commands, assets, x, y),
        TileKind::Forest => spawn_3d_forest_tile(commands, assets, x, y),
        TileKind::Water => spawn_3d_water_tile(commands, assets, x, y),
        TileKind::Ice => spawn_3d_ice_tile(commands, assets, x, y),
    }
}

fn spawn_3d_brick_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);
    let visual_top_left = center - Vec2::splat(VIEW_3D_TILE_VISUAL_FOOTPRINT / 2.0);

    spawn_3d_static_box(
        commands,
        assets,
        center,
        VIEW_3D_BRICK_HEIGHT / 2.0,
        Vec3::new(
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
            VIEW_3D_BRICK_HEIGHT,
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
        ),
        assets.brick_mortar_material.clone(),
    );

    let gap = 0.24;
    let row_depth = (VIEW_3D_TILE_VISUAL_FOOTPRINT - gap * 4.0) / 3.0;
    for row in 0..3 {
        let column_count = if row == 1 { 3 } else { 2 };
        let brick_width = (VIEW_3D_TILE_VISUAL_FOOTPRINT - gap * (column_count as f32 + 1.0))
            / column_count as f32;
        let z = visual_top_left.y + gap + row as f32 * (row_depth + gap) + row_depth / 2.0;
        for column in 0..column_count {
            let center = Vec2::new(
                visual_top_left.x + gap + column as f32 * (brick_width + gap) + brick_width / 2.0,
                z,
            );
            let material = if (row + column) % 2 == 0 {
                assets.brick_material.clone()
            } else {
                assets.brick_shadow_material.clone()
            };
            spawn_3d_static_box(
                commands,
                assets,
                center,
                VIEW_3D_BRICK_HEIGHT / 2.0 + 0.04,
                Vec3::new(brick_width, VIEW_3D_BRICK_HEIGHT + 0.08, row_depth),
                material,
            );
        }
    }
}

fn spawn_3d_steel_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);
    let panel_footprint = VIEW_3D_TILE_VISUAL_FOOTPRINT - 1.0;
    let seam_length = panel_footprint + 0.2;

    spawn_3d_static_box(
        commands,
        assets,
        center,
        VIEW_3D_STEEL_HEIGHT / 2.0,
        Vec3::new(
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
            VIEW_3D_STEEL_HEIGHT,
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
        ),
        assets.steel_dark_material.clone(),
    );
    spawn_3d_static_box(
        commands,
        assets,
        center,
        VIEW_3D_STEEL_HEIGHT / 2.0 + 0.12,
        Vec3::new(
            panel_footprint,
            VIEW_3D_STEEL_HEIGHT + 0.24,
            panel_footprint,
        ),
        assets.steel_material.clone(),
    );

    let seam_y = VIEW_3D_STEEL_HEIGHT + 0.16;
    spawn_3d_static_box(
        commands,
        assets,
        center,
        seam_y,
        Vec3::new(seam_length, 0.18, 0.3),
        assets.steel_dark_material.clone(),
    );
    spawn_3d_static_box(
        commands,
        assets,
        center,
        seam_y,
        Vec3::new(0.3, 0.18, seam_length),
        assets.steel_dark_material.clone(),
    );

    for offset in [
        Vec2::new(-2.35, -2.35),
        Vec2::new(2.35, -2.35),
        Vec2::new(-2.35, 2.35),
        Vec2::new(2.35, 2.35),
    ] {
        spawn_3d_static_box(
            commands,
            assets,
            center + offset,
            VIEW_3D_STEEL_HEIGHT + 0.38,
            Vec3::new(0.72, 0.34, 0.72),
            assets.steel_bolt_material.clone(),
        );
    }
}

fn spawn_3d_forest_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);

    for (index, (offset, canopy_size)) in [
        (Vec2::new(-1.8, -1.7), Vec3::new(2.9, 2.6, 2.8)),
        (Vec2::new(1.8, -1.7), Vec3::new(2.7, 2.4, 2.8)),
        (Vec2::new(-1.7, 1.8), Vec3::new(2.8, 2.5, 2.7)),
        (Vec2::new(1.8, 1.7), Vec3::new(2.8, 2.5, 2.9)),
        (Vec2::new(0.0, 0.0), Vec3::new(3.4, 2.9, 3.4)),
    ]
    .into_iter()
    .enumerate()
    {
        let cluster_center = center + offset;
        spawn_3d_static_box(
            commands,
            assets,
            cluster_center,
            1.2,
            Vec3::new(0.7, 2.4, 0.7),
            assets.forest_trunk_material.clone(),
        );
        spawn_3d_static_box(
            commands,
            assets,
            cluster_center,
            VIEW_3D_FOREST_HEIGHT - canopy_size.y / 2.0,
            canopy_size,
            if index % 2 == 0 {
                assets.forest_material.clone()
            } else {
                assets.forest_dark_material.clone()
            },
        );
    }
}

fn spawn_3d_water_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    spawn_3d_floor_tile(commands, assets, x, y, assets.water_material.clone());

    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    for (offset, width) in [
        (Vec2::new(-1.3, -2.0), 4.2),
        (Vec2::new(1.2, 0.0), 5.0),
        (Vec2::new(-0.6, 2.1), 3.6),
    ] {
        spawn_3d_static_box(
            commands,
            assets,
            top_left + Vec2::splat(TILE_SIZE / 2.0) + offset,
            VIEW_3D_WATER_SURFACE_HEIGHT + 0.08,
            Vec3::new(width, 0.08, 0.52),
            assets.water_highlight_material.clone(),
        );
    }
}

fn spawn_3d_ice_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    spawn_3d_floor_tile(commands, assets, x, y, assets.ice_material.clone());

    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);
    for (offset, length, yaw) in [
        (Vec2::new(-1.2, -0.7), 5.0, 0.72),
        (Vec2::new(1.4, 1.3), 3.7, -0.62),
    ] {
        spawn_3d_static_box_with_yaw(
            commands,
            assets,
            center + offset,
            VIEW_3D_ICE_SURFACE_HEIGHT + 0.08,
            Vec3::new(length, 0.08, 0.38),
            yaw,
            assets.ice_mark_material.clone(),
        );
    }
}

fn spawn_3d_floor_tile(
    commands: &mut Commands,
    assets: &View3dAssets,
    x: usize,
    y: usize,
    material: Handle<StandardMaterial>,
) {
    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    spawn_3d_mesh(
        commands,
        assets.floor_tile_mesh.clone(),
        material,
        Transform::from_translation(board_3d_center(top_left, Vec2::splat(TILE_SIZE), 0.08)),
        View3dStatic,
    );
}

fn spawn_3d_static_box(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    size: Vec3,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_static_box_with_yaw(commands, assets, center, y_center, size, 0.0, material);
}

fn spawn_3d_static_box_with_yaw(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    size: Vec3,
    yaw: f32,
    material: Handle<StandardMaterial>,
) {
    let mut transform = Transform::from_translation(board_3d_point(center, y_center));
    transform.rotation = Quat::from_rotation_y(yaw);
    transform.scale = size;
    spawn_3d_mesh(
        commands,
        assets.effect_mesh.clone(),
        material,
        transform,
        View3dStatic,
    );
}

fn spawn_3d_tank(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    tank: &Tank,
    player: Option<&Player>,
    player_upgrade: Option<&PlayerUpgrade>,
    enemy: Option<&EnemyTank>,
    health: Option<&Health>,
    shield: Option<&Shield>,
    spawn_protection: Option<&SpawnProtection>,
    assist_enabled: bool,
    tile_grid: &TileGrid,
    enemy_frozen: bool,
    frozen_players: Option<&VersusPlayerFreeze>,
    view_target: bool,
) {
    let material_kind = tank_3d_material_kind(
        player.map(|player| player.id),
        player_upgrade.map(|upgrade| upgrade.level),
        enemy.map(|enemy| enemy.kind),
        health.map(|health| health.current).unwrap_or(1),
        enemy.is_some() && enemy_frozen,
        player.is_some_and(|player| {
            frozen_players.is_some_and(|freeze| freeze.is_player_frozen(player.id))
        }),
    );
    let material = tank_3d_material(assets, material_kind);
    let center = board_3d_center(tank.top_left, Vec2::splat(TANK_SIZE), VIEW_3D_TANK_HEIGHT);
    let body = spawn_3d_mesh(
        commands,
        assets.tank_body_mesh.clone(),
        material.clone(),
        Transform::from_translation(center),
        View3dDynamic,
    );
    commands.entity(body).insert(Name::new(format!(
        "3D Tank {:?} {:?}",
        material_kind, source
    )));

    spawn_3d_tank_tracks(commands, assets, tank.top_left, tank.facing);
    spawn_3d_tank_turret(commands, assets, tank.top_left, material);

    if let Some(player) = player {
        spawn_3d_player_marker(commands, assets, tank.top_left, player.id);
        if view_target {
            spawn_3d_view_target_marker(commands, assets, tank.top_left, player.id);
        }
    }

    let (barrel_mesh_kind, barrel_translation) = tank_barrel_transform(tank.top_left, tank.facing);
    spawn_3d_mesh(
        commands,
        barrel_mesh(assets, barrel_mesh_kind),
        assets.barrel_material.clone(),
        Transform::from_translation(barrel_translation),
        View3dDynamic,
    );

    if assist_enabled && enemy.is_some() {
        spawn_3d_marker(
            commands,
            assets,
            tank.top_left,
            assets.enemy_marker_material.clone(),
        );
    }

    if assist_enabled && let Some(powerup_kind) = enemy.and_then(|enemy| enemy.carried_powerup) {
        spawn_3d_carrier_marker(commands, assets, tank.top_left, powerup_kind);
    }

    if assist_enabled && player.is_some() {
        spawn_3d_aim_guide(commands, assets, tank, tile_grid);
    }

    if let Some(kind) = tank_3d_protection_kind(shield.is_some(), spawn_protection.is_some()) {
        spawn_3d_tank_protection(commands, assets, tank.top_left, kind);
    }
}

fn spawn_3d_tank_tracks(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward = facing.movement();
    let side = Vec2::new(-forward.y, forward.x);
    let mesh = match facing {
        Direction::Left | Direction::Right => assets.tank_track_x_mesh.clone(),
        Direction::Up | Direction::Down => assets.tank_track_z_mesh.clone(),
    };

    for offset in [-5.2, 5.2] {
        spawn_3d_mesh(
            commands,
            mesh.clone(),
            assets.barrel_material.clone(),
            Transform::from_translation(board_3d_point(center + side * offset, 0.35)),
            View3dDynamic,
        );
    }
}

fn spawn_3d_tank_turret(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    material: Handle<StandardMaterial>,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    spawn_3d_mesh(
        commands,
        assets.tank_turret_mesh.clone(),
        material,
        Transform::from_translation(board_3d_point(center, VIEW_3D_TANK_HEIGHT + 0.7)),
        View3dDynamic,
    );
}

fn spawn_3d_bullet(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    bullet: &Bullet,
) {
    let material_kind = bullet_3d_material_kind(bullet.owner);
    let entity = spawn_3d_mesh(
        commands,
        assets.bullet_mesh.clone(),
        bullet_3d_material(assets, material_kind),
        Transform::from_translation(board_3d_center(
            bullet.top_left,
            Vec2::splat(BULLET_SIZE),
            1.4,
        )),
        View3dDynamic,
    );
    commands.entity(entity).insert(Name::new(format!(
        "3D Bullet {:?} {:?}",
        material_kind, source
    )));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Bullet3dMaterialKind {
    PlayerOne,
    PlayerTwo,
    Enemy,
}

pub(super) fn bullet_3d_material_kind(owner: Team) -> Bullet3dMaterialKind {
    match owner {
        Team::Player1 => Bullet3dMaterialKind::PlayerOne,
        Team::Player2 => Bullet3dMaterialKind::PlayerTwo,
        Team::Enemy => Bullet3dMaterialKind::Enemy,
    }
}

fn bullet_3d_material(
    assets: &View3dAssets,
    kind: Bullet3dMaterialKind,
) -> Handle<StandardMaterial> {
    match kind {
        Bullet3dMaterialKind::PlayerOne => assets.player_one_bullet_material.clone(),
        Bullet3dMaterialKind::PlayerTwo => assets.player_two_bullet_material.clone(),
        Bullet3dMaterialKind::Enemy => assets.enemy_bullet_material.clone(),
    }
}

fn spawn_3d_base(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    base: &BaseSprite,
    assist_enabled: bool,
) {
    let material_kind = base_3d_material_kind(base.owner);
    let entity = spawn_3d_mesh(
        commands,
        assets.base_mesh.clone(),
        base_3d_material(assets, material_kind),
        Transform::from_translation(board_3d_center(
            base.top_left,
            Vec2::splat(TANK_SIZE),
            VIEW_3D_BASE_HEIGHT,
        )),
        View3dDynamic,
    );
    commands.entity(entity).insert(Name::new(format!(
        "3D Base {:?} {:?}",
        material_kind, source
    )));

    if assist_enabled {
        spawn_3d_marker(
            commands,
            assets,
            base.top_left,
            assets.base_marker_material.clone(),
        );
    }
}

fn spawn_3d_powerup(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    kind: PowerUpKind,
    top_left: Vec2,
) {
    let entity = spawn_3d_mesh(
        commands,
        assets.powerup_mesh.clone(),
        powerup_3d_material(assets, kind),
        Transform::from_translation(board_3d_center(top_left, Vec2::splat(TANK_SIZE), 2.0)),
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D PowerUp {:?} {:?}", kind, source)));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PowerUp3dMaterialKind {
    Star,
    Helmet,
    Clock,
    Grenade,
    Shovel,
    Tank,
}

pub(super) fn powerup_3d_material_kind(kind: PowerUpKind) -> PowerUp3dMaterialKind {
    match kind {
        PowerUpKind::Star => PowerUp3dMaterialKind::Star,
        PowerUpKind::Helmet => PowerUp3dMaterialKind::Helmet,
        PowerUpKind::Clock => PowerUp3dMaterialKind::Clock,
        PowerUpKind::Grenade => PowerUp3dMaterialKind::Grenade,
        PowerUpKind::Shovel => PowerUp3dMaterialKind::Shovel,
        PowerUpKind::Tank => PowerUp3dMaterialKind::Tank,
    }
}

fn powerup_3d_material(assets: &View3dAssets, kind: PowerUpKind) -> Handle<StandardMaterial> {
    match powerup_3d_material_kind(kind) {
        PowerUp3dMaterialKind::Star => assets.powerup_star_material.clone(),
        PowerUp3dMaterialKind::Helmet => assets.powerup_helmet_material.clone(),
        PowerUp3dMaterialKind::Clock => assets.powerup_clock_material.clone(),
        PowerUp3dMaterialKind::Grenade => assets.powerup_grenade_material.clone(),
        PowerUp3dMaterialKind::Shovel => assets.powerup_shovel_material.clone(),
        PowerUp3dMaterialKind::Tank => assets.powerup_tank_material.clone(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum View3dEffectKind {
    Explosion,
    BaseDestruction,
    BulletImpact,
    SpawnShimmer,
    PowerUpSparkle,
}

pub(super) fn view_3d_effect_kind(
    animation: &SpriteAnimation,
    manifest: &AssetManifest,
) -> Option<View3dEffectKind> {
    let frames = SpriteFrameRange {
        first: animation.first,
        last: animation.last,
    };
    if frames == manifest.explosion_frames() {
        Some(View3dEffectKind::Explosion)
    } else if frames == manifest.base_destruction_frames() {
        Some(View3dEffectKind::BaseDestruction)
    } else if frames == manifest.bullet_impact_frames() {
        Some(View3dEffectKind::BulletImpact)
    } else if frames == manifest.spawn_shimmer_frames() {
        Some(View3dEffectKind::SpawnShimmer)
    } else if frames == manifest.powerup_sparkle_frames() {
        Some(View3dEffectKind::PowerUpSparkle)
    } else {
        None
    }
}

pub(super) fn view_3d_effect_size(kind: View3dEffectKind) -> f32 {
    match kind {
        View3dEffectKind::BulletImpact => BULLET_SIZE,
        View3dEffectKind::Explosion
        | View3dEffectKind::BaseDestruction
        | View3dEffectKind::SpawnShimmer
        | View3dEffectKind::PowerUpSparkle => TANK_SIZE,
    }
}

fn spawn_3d_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    kind: View3dEffectKind,
    top_left: Vec2,
) {
    let (material, scale, height) = view_3d_effect_style(assets, kind);
    let mut transform = Transform::from_translation(board_3d_center(
        top_left,
        Vec2::splat(view_3d_effect_size(kind)),
        height,
    ));
    transform.scale = scale;
    let entity = spawn_3d_mesh(
        commands,
        assets.effect_mesh.clone(),
        material,
        transform,
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D Effect {:?} {:?}", kind, source)));
}

fn view_3d_effect_style(
    assets: &View3dAssets,
    kind: View3dEffectKind,
) -> (Handle<StandardMaterial>, Vec3, f32) {
    match kind {
        View3dEffectKind::Explosion => (
            assets.explosion_material.clone(),
            Vec3::new(15.0, 9.0, 15.0),
            9.0,
        ),
        View3dEffectKind::BaseDestruction => (
            assets.base_destruction_material.clone(),
            Vec3::new(18.0, 12.0, 18.0),
            12.0,
        ),
        View3dEffectKind::BulletImpact => (
            assets.bullet_impact_material.clone(),
            Vec3::new(5.5, 3.5, 5.5),
            3.5,
        ),
        View3dEffectKind::SpawnShimmer => (
            assets.spawn_effect_material.clone(),
            Vec3::new(18.0, 8.0, 18.0),
            8.0,
        ),
        View3dEffectKind::PowerUpSparkle => (
            assets.powerup_sparkle_material.clone(),
            Vec3::new(10.0, 5.0, 10.0),
            5.0,
        ),
    }
}

fn spawn_3d_marker(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_mesh(
        commands,
        assets.marker_mesh.clone(),
        material,
        Transform::from_translation(board_3d_center(
            top_left,
            Vec2::splat(TANK_SIZE),
            VIEW_3D_MARKER_HEIGHT,
        )),
        View3dDynamic,
    );
}

fn spawn_3d_carrier_marker(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    kind: PowerUpKind,
) {
    let entity = spawn_3d_mesh(
        commands,
        assets.powerup_mesh.clone(),
        powerup_3d_material(assets, kind),
        Transform::from_translation(board_3d_center(
            top_left,
            Vec2::splat(TANK_SIZE),
            VIEW_3D_CARRIER_MARKER_HEIGHT,
        )),
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D Carrier PowerUp {:?}", kind)));
}

fn spawn_3d_player_marker(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    player: PlayerId,
) {
    let entity = spawn_3d_mesh(
        commands,
        assets.marker_mesh.clone(),
        player_marker_3d_material(assets, player),
        Transform::from_translation(board_3d_center(
            top_left,
            Vec2::splat(TANK_SIZE),
            VIEW_3D_PLAYER_MARKER_HEIGHT,
        )),
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D Player Marker {:?}", player)));
}

fn spawn_3d_view_target_marker(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    player: PlayerId,
) {
    let entity = spawn_3d_mesh(
        commands,
        assets.marker_mesh.clone(),
        assets.view_target_marker_material.clone(),
        Transform::from_translation(board_3d_center(
            top_left,
            Vec2::splat(TANK_SIZE),
            VIEW_3D_VIEW_TARGET_MARKER_HEIGHT,
        )),
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D View Target {:?}", player)));
}

fn spawn_3d_aim_guide(
    commands: &mut Commands,
    assets: &View3dAssets,
    tank: &Tank,
    tile_grid: &TileGrid,
) {
    let length = aim_guide_length(tile_grid, tank);
    if length <= VIEW_3D_AIM_GUIDE_WIDTH {
        return;
    }

    let start = aim_guide_start(tank);
    let center = start + tank.facing.movement() * (length / 2.0);
    let mut transform = Transform::from_translation(board_3d_point(center, VIEW_3D_AIM_GUIDE_Y));
    transform.scale = aim_guide_scale(tank.facing, length);

    let entity = spawn_3d_mesh(
        commands,
        assets.effect_mesh.clone(),
        assets.aim_guide_material.clone(),
        transform,
        View3dDynamic,
    );
    commands.entity(entity).insert(Name::new("3D Aim Guide"));
}

pub(super) fn aim_guide_length(tile_grid: &TileGrid, tank: &Tank) -> f32 {
    let start = aim_guide_start(tank);
    aim_guide_clear_length(tile_grid, start, tank.facing, VIEW_3D_AIM_GUIDE_MAX_LENGTH)
}

fn aim_guide_start(tank: &Tank) -> Vec2 {
    tank.top_left
        + Vec2::splat(TANK_SIZE / 2.0)
        + tank.facing.movement() * VIEW_3D_AIM_GUIDE_START_OFFSET
}

fn aim_guide_clear_length(
    tile_grid: &TileGrid,
    start: Vec2,
    direction: Direction,
    max_length: f32,
) -> f32 {
    let movement = direction.movement();
    let samples = (max_length / VIEW_3D_AIM_GUIDE_SAMPLE_STEP).ceil() as usize;
    let mut clear_length = 0.0;

    for sample in 1..=samples {
        let distance = (sample as f32 * VIEW_3D_AIM_GUIDE_SAMPLE_STEP).min(max_length);
        let point = start + movement * distance;
        if point.x < 0.0 || point.y < 0.0 || point.x >= board_size() || point.y >= board_size() {
            return clear_length;
        }

        let tile_x = (point.x / TILE_SIZE).floor() as i32;
        let tile_y = (point.y / TILE_SIZE).floor() as i32;
        if tile_grid
            .get(tile_x, tile_y)
            .is_some_and(TileKind::bullet_blocks)
        {
            return clear_length;
        }

        clear_length = distance;
    }

    clear_length
}

fn aim_guide_scale(direction: Direction, length: f32) -> Vec3 {
    match direction {
        Direction::Left | Direction::Right => {
            Vec3::new(length, VIEW_3D_AIM_GUIDE_HEIGHT, VIEW_3D_AIM_GUIDE_WIDTH)
        }
        Direction::Up | Direction::Down => {
            Vec3::new(VIEW_3D_AIM_GUIDE_WIDTH, VIEW_3D_AIM_GUIDE_HEIGHT, length)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Tank3dProtectionKind {
    Shield,
    Spawn,
}

pub(super) fn tank_3d_protection_kind(
    shielded: bool,
    spawn_protected: bool,
) -> Option<Tank3dProtectionKind> {
    if spawn_protected {
        Some(Tank3dProtectionKind::Spawn)
    } else if shielded {
        Some(Tank3dProtectionKind::Shield)
    } else {
        None
    }
}

fn spawn_3d_tank_protection(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    kind: Tank3dProtectionKind,
) {
    let material = match kind {
        Tank3dProtectionKind::Shield => assets.shield_material.clone(),
        Tank3dProtectionKind::Spawn => assets.spawn_protection_material.clone(),
    };
    let center = board_3d_point(
        top_left + Vec2::splat(TANK_SIZE / 2.0),
        VIEW_3D_PROTECTION_Y,
    );
    for (mesh, offset) in [
        (
            assets.protection_x_mesh.clone(),
            Vec3::new(0.0, 0.0, -VIEW_3D_PROTECTION_BAR_OFFSET),
        ),
        (
            assets.protection_x_mesh.clone(),
            Vec3::new(0.0, 0.0, VIEW_3D_PROTECTION_BAR_OFFSET),
        ),
        (
            assets.protection_z_mesh.clone(),
            Vec3::new(-VIEW_3D_PROTECTION_BAR_OFFSET, 0.0, 0.0),
        ),
        (
            assets.protection_z_mesh.clone(),
            Vec3::new(VIEW_3D_PROTECTION_BAR_OFFSET, 0.0, 0.0),
        ),
    ] {
        spawn_3d_mesh(
            commands,
            mesh,
            material.clone(),
            Transform::from_translation(center + offset),
            View3dDynamic,
        );
    }
}

fn spawn_3d_phase_hud(commands: &mut Commands, assets: &SpriteAssets, lines: &[String]) {
    let line_gap = 3.0;
    let line_height = GENERATED_GLYPH_HEIGHT as f32;
    let text_step = line_height + line_gap;
    let text_block_height =
        lines.len() as f32 * line_height + lines.len().saturating_sub(1) as f32 * line_gap;
    let text_width = lines
        .iter()
        .map(|line| phase_text_width(line))
        .fold(0.0, f32::max);
    let panel_size = Vec2::new(text_width + 14.0, text_block_height + 10.0);
    let panel_top_left = Vec2::new(
        (VIRTUAL_WIDTH - panel_size.x) / 2.0,
        112.0 - panel_size.y / 2.0,
    );
    let first_line_top = panel_top_left.y + 5.0;

    spawn_3d_hud_panel(commands, panel_top_left, panel_size);
    for (index, line) in lines.iter().enumerate() {
        spawn_3d_hud_text(
            commands,
            assets,
            line,
            Vec2::new(
                (VIRTUAL_WIDTH - phase_text_width(line)) / 2.0,
                first_line_top + index as f32 * text_step,
            ),
        );
    }
}

fn spawn_3d_hud_panel(commands: &mut Commands, top_left: Vec2, size: Vec2) {
    commands.spawn((
        Sprite::from_color(
            Color::srgba_u8(0, 0, 0, 190),
            Vec2::new(size.x * window_scale(), size.y * window_scale()),
        ),
        Transform::from_translation(virtual_center_scaled(top_left, size, VIEW_3D_HUD_PANEL_Z)),
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHud,
        GameEntity,
    ));
}

fn spawn_3d_hud_text(commands: &mut Commands, assets: &SpriteAssets, text: &str, top_left: Vec2) {
    for (index, ch) in text.chars().enumerate() {
        if ch == ' ' {
            continue;
        }

        commands.spawn((
            Sprite::from_atlas_image(
                assets.glyph_image.clone(),
                TextureAtlas {
                    layout: assets.glyph_layout.clone(),
                    index: glyph_index(ch, &assets.manifest.glyphs),
                },
            ),
            Transform::from_translation(virtual_center_scaled(
                Vec2::new(top_left.x + index as f32 * GLYPH_ADVANCE, top_left.y),
                glyph_size(&assets.manifest.glyphs),
                VIEW_3D_HUD_TEXT_Z,
            ))
            .with_scale(Vec3::splat(window_scale())),
            RenderLayers::layer(VIEW_3D_HUD_LAYER),
            View3dHud,
            GameEntity,
        ));
    }
}

fn spawn_3d_mesh<C: Component>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    marker: C,
) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            transform,
            RenderLayers::layer(VIEW_3D_LAYER),
            marker,
        ))
        .id()
}

fn despawn_entities(commands: &mut Commands, entities: impl IntoIterator<Item = Entity>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

fn board_3d_center(top_left: Vec2, size: Vec2, height: f32) -> Vec3 {
    let center = top_left + size / 2.0;
    board_3d_point(center, height / 2.0)
}

fn board_3d_point(point: Vec2, y: f32) -> Vec3 {
    Vec3::new(
        point.x - board_size() / 2.0,
        y,
        point.y - board_size() / 2.0,
    )
}

#[cfg(test)]
pub(super) fn chase_camera_transform(tank: &Tank, tile_grid: &TileGrid) -> Transform {
    chase_camera_transform_with_forward(tank, tile_grid, tank.facing.movement())
}

#[cfg(test)]
fn chase_camera_transform_with_forward(
    tank: &Tank,
    tile_grid: &TileGrid,
    forward_2d: Vec2,
) -> Transform {
    let height_mode = chase_camera_height_mode(tank, tile_grid, forward_2d);
    chase_camera_transform_with_forward_and_height_mode(tank, forward_2d, height_mode)
}

fn chase_camera_height_mode(
    tank: &Tank,
    tile_grid: &TileGrid,
    forward_2d: Vec2,
) -> View3dCameraHeightMode {
    let target_center = tank.top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward_2d = resolved_camera_forward(tank, forward_2d);
    let desired_camera_ground = target_center - forward_2d * VIEW_3D_CAMERA_DISTANCE;
    if camera_path_is_obstructed(tile_grid, target_center, desired_camera_ground) {
        View3dCameraHeightMode::Tactical
    } else {
        View3dCameraHeightMode::Chase
    }
}

fn chase_camera_transform_with_forward_and_height_mode(
    tank: &Tank,
    forward_2d: Vec2,
    height_mode: View3dCameraHeightMode,
) -> Transform {
    let target_center = tank.top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward_2d = resolved_camera_forward(tank, forward_2d);

    let (distance, camera_height, look_height) = match height_mode {
        View3dCameraHeightMode::Tactical => (
            VIEW_3D_OCCLUDED_CAMERA_DISTANCE,
            VIEW_3D_OCCLUDED_CAMERA_HEIGHT,
            VIEW_3D_OCCLUDED_LOOK_HEIGHT,
        ),
        View3dCameraHeightMode::Chase => (
            VIEW_3D_CAMERA_DISTANCE,
            VIEW_3D_CAMERA_HEIGHT,
            VIEW_3D_CAMERA_LOOK_HEIGHT,
        ),
    };

    let camera_ground = target_center - forward_2d * distance;
    let look_at_ground = target_center + forward_2d * VIEW_3D_CAMERA_LOOK_AHEAD;
    let camera_y = VIEW_3D_TANK_HEIGHT / 2.0 + camera_height;
    let look_y = VIEW_3D_TANK_HEIGHT / 2.0 + look_height;

    Transform::from_translation(board_3d_point(camera_ground, camera_y))
        .looking_at(board_3d_point(look_at_ground, look_y), Vec3::Y)
}

fn resolved_camera_forward(tank: &Tank, forward_2d: Vec2) -> Vec2 {
    let forward_2d = forward_2d.normalize_or_zero();
    if forward_2d == Vec2::ZERO {
        tank.facing.movement()
    } else {
        forward_2d
    }
}

pub(super) fn rotate_direction_toward(current: Vec2, desired: Vec2, max_angle_delta: f32) -> Vec2 {
    let current = current.normalize_or_zero();
    let desired = desired.normalize_or_zero();
    if current == Vec2::ZERO || desired == Vec2::ZERO {
        return desired;
    }

    let current_angle = current.y.atan2(current.x);
    let desired_angle = desired.y.atan2(desired.x);
    let delta = shortest_angle_delta(current_angle, desired_angle);
    let clamped_delta = delta.clamp(-max_angle_delta, max_angle_delta);
    Vec2::new(
        (current_angle + clamped_delta).cos(),
        (current_angle + clamped_delta).sin(),
    )
    .normalize_or_zero()
}

fn camera_direction_is_settled(camera_forward: Vec2, desired_forward: Vec2) -> bool {
    let camera_forward = camera_forward.normalize_or_zero();
    let desired_forward = desired_forward.normalize_or_zero();
    camera_forward == Vec2::ZERO
        || desired_forward == Vec2::ZERO
        || camera_forward.dot(desired_forward) >= VIEW_3D_CAMERA_HEIGHT_MODE_SETTLED_DOT
}

fn view_3d_smoothing_alpha(response: f32, delta_secs: f32) -> f32 {
    if response <= 0.0 {
        return 1.0;
    }
    (1.0 - (-response * delta_secs.max(0.0)).exp()).clamp(0.0, 1.0)
}

fn shortest_angle_delta(from: f32, to: f32) -> f32 {
    let delta =
        (to - from + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;
    if (delta + std::f32::consts::PI).abs() < f32::EPSILON {
        std::f32::consts::PI
    } else {
        delta
    }
}

pub(super) fn camera_path_is_obstructed(
    tile_grid: &TileGrid,
    from_board: Vec2,
    to_board: Vec2,
) -> bool {
    let path = to_board - from_board;
    let length = path.length();
    if length <= f32::EPSILON {
        return false;
    }

    let samples = (length / (TILE_SIZE / 2.0)).ceil() as usize;
    for sample in 1..=samples {
        let t = sample as f32 / samples as f32;
        let point = from_board + path * t;
        let tile_x = (point.x / TILE_SIZE).floor() as i32;
        let tile_y = (point.y / TILE_SIZE).floor() as i32;
        if tile_grid
            .get(tile_x, tile_y)
            .is_some_and(tile_blocks_3d_camera)
        {
            return true;
        }
    }

    false
}

fn tile_blocks_3d_camera(tile: TileKind) -> bool {
    matches!(
        tile,
        TileKind::Brick | TileKind::Steel | TileKind::Forest | TileKind::Base
    )
}

fn top_left_is_on_board(top_left: Vec2) -> bool {
    top_left.x >= 0.0
        && top_left.y >= 0.0
        && top_left.x <= board_size()
        && top_left.y <= board_size()
}

fn direction_3d(direction: Direction) -> Vec3 {
    match direction {
        Direction::Up => Vec3::NEG_Z,
        Direction::Down => Vec3::Z,
        Direction::Left => Vec3::NEG_X,
        Direction::Right => Vec3::X,
    }
}

#[derive(Clone, Copy)]
enum BarrelMesh {
    X,
    Z,
}

fn tank_barrel_transform(top_left: Vec2, direction: Direction) -> (BarrelMesh, Vec3) {
    let center = board_3d_center(top_left, Vec2::splat(TANK_SIZE), VIEW_3D_TANK_HEIGHT);
    let forward = direction_3d(direction);
    let barrel_center = center + forward * 7.0 + Vec3::Y * (VIEW_3D_TANK_HEIGHT / 2.0 + 0.8);
    let mesh = match direction {
        Direction::Left | Direction::Right => BarrelMesh::X,
        Direction::Up | Direction::Down => BarrelMesh::Z,
    };
    (mesh, barrel_center)
}

fn barrel_mesh(assets: &View3dAssets, mesh: BarrelMesh) -> Handle<Mesh> {
    match mesh {
        BarrelMesh::X => assets.tank_barrel_x_mesh.clone(),
        BarrelMesh::Z => assets.tank_barrel_z_mesh.clone(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Tank3dMaterialKind {
    PlayerOne,
    PlayerTwo,
    PlayerUpgrade0,
    PlayerUpgrade1,
    PlayerUpgrade2,
    PlayerUpgrade3,
    EnemyBasic,
    EnemyFast,
    EnemyPower,
    EnemyArmor,
    Frozen,
}

pub(super) fn tank_3d_material_kind(
    player: Option<PlayerId>,
    player_upgrade_level: Option<u8>,
    enemy: Option<EnemyKind>,
    health: i32,
    enemy_frozen: bool,
    player_frozen: bool,
) -> Tank3dMaterialKind {
    if player_frozen || enemy_frozen {
        return Tank3dMaterialKind::Frozen;
    }

    if let Some(player) = player {
        if let Some(upgrade_level) = player_upgrade_level {
            return match upgrade_level.min(3) {
                0 => Tank3dMaterialKind::PlayerUpgrade0,
                1 => Tank3dMaterialKind::PlayerUpgrade1,
                2 => Tank3dMaterialKind::PlayerUpgrade2,
                _ => Tank3dMaterialKind::PlayerUpgrade3,
            };
        }

        return match player {
            PlayerId::One => Tank3dMaterialKind::PlayerOne,
            PlayerId::Two => Tank3dMaterialKind::PlayerTwo,
        };
    }

    match enemy.unwrap_or(EnemyKind::Basic) {
        EnemyKind::Armor if health <= 1 => Tank3dMaterialKind::EnemyPower,
        EnemyKind::Armor => Tank3dMaterialKind::EnemyArmor,
        EnemyKind::Power => Tank3dMaterialKind::EnemyPower,
        EnemyKind::Fast => Tank3dMaterialKind::EnemyFast,
        EnemyKind::Basic => Tank3dMaterialKind::EnemyBasic,
    }
}

fn tank_3d_material(assets: &View3dAssets, kind: Tank3dMaterialKind) -> Handle<StandardMaterial> {
    match kind {
        Tank3dMaterialKind::PlayerOne => assets.player_one_material.clone(),
        Tank3dMaterialKind::PlayerTwo => assets.player_two_material.clone(),
        Tank3dMaterialKind::PlayerUpgrade0 => assets.player_upgrade_zero_material.clone(),
        Tank3dMaterialKind::PlayerUpgrade1 => assets.player_upgrade_one_material.clone(),
        Tank3dMaterialKind::PlayerUpgrade2 => assets.player_upgrade_two_material.clone(),
        Tank3dMaterialKind::PlayerUpgrade3 => assets.player_upgrade_three_material.clone(),
        Tank3dMaterialKind::EnemyBasic => assets.enemy_basic_material.clone(),
        Tank3dMaterialKind::EnemyFast => assets.enemy_fast_material.clone(),
        Tank3dMaterialKind::EnemyPower => assets.enemy_power_material.clone(),
        Tank3dMaterialKind::EnemyArmor => assets.enemy_armor_material.clone(),
        Tank3dMaterialKind::Frozen => assets.frozen_material.clone(),
    }
}

fn player_marker_3d_material(assets: &View3dAssets, player: PlayerId) -> Handle<StandardMaterial> {
    match player {
        PlayerId::One => assets.player_one_marker_material.clone(),
        PlayerId::Two => assets.player_two_marker_material.clone(),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Base3dMaterialKind {
    Neutral,
    PlayerOne,
    PlayerTwo,
}

pub(super) fn base_3d_material_kind(owner: Option<PlayerId>) -> Base3dMaterialKind {
    match owner {
        Some(PlayerId::One) => Base3dMaterialKind::PlayerOne,
        Some(PlayerId::Two) => Base3dMaterialKind::PlayerTwo,
        None => Base3dMaterialKind::Neutral,
    }
}

fn base_3d_material(assets: &View3dAssets, kind: Base3dMaterialKind) -> Handle<StandardMaterial> {
    match kind {
        Base3dMaterialKind::Neutral => assets.base_material.clone(),
        Base3dMaterialKind::PlayerOne => assets.player_one_base_material.clone(),
        Base3dMaterialKind::PlayerTwo => assets.player_two_base_material.clone(),
    }
}

fn overview_camera_transform() -> Transform {
    Transform::from_xyz(0.0, 135.0, 165.0).looking_at(Vec3::ZERO, Vec3::Y)
}
