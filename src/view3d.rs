use super::*;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{PerspectiveProjection, Projection};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::render::view::Msaa;

const VIEW_3D_LAYER: usize = 1;
const VIEW_3D_HUD_LAYER: usize = 2;
const VIEW_3D_GROUND_THICKNESS: f32 = 0.2;
const VIEW_3D_BRICK_HEIGHT: f32 = 3.4;
const VIEW_3D_STEEL_HEIGHT: f32 = 4.8;
const VIEW_3D_FOREST_HEIGHT: f32 = 5.2;
const VIEW_3D_TILE_VISUAL_FOOTPRINT: f32 = TILE_SIZE - 0.8;
const VIEW_3D_WATER_SURFACE_HEIGHT: f32 = 0.07;
const VIEW_3D_WATER_BASIN_HEIGHT: f32 = 0.04;
const VIEW_3D_WATER_EDGE_HEIGHT: f32 = 0.55;
const VIEW_3D_WATER_EDGE_THICKNESS: f32 = 0.42;
const VIEW_3D_ICE_SURFACE_HEIGHT: f32 = 0.1;
const VIEW_3D_TANK_HEIGHT: f32 = 3.4;
const VIEW_3D_TANK_TRACK_Y: f32 = 0.55;
const VIEW_3D_TANK_WHEEL_Y: f32 = 0.95;
const VIEW_3D_TANK_BARREL_CENTER_OFFSET: f32 = 7.0;
const VIEW_3D_TANK_BARREL_HALF_LENGTH: f32 = 6.8;
const VIEW_3D_TANK_BARREL_Y: f32 = VIEW_3D_TANK_HEIGHT + 0.8;
const VIEW_3D_TANK_FRONT_PLATE_Y: f32 = VIEW_3D_TANK_HEIGHT + 0.12;
const VIEW_3D_TANK_HATCH_Y: f32 = VIEW_3D_TANK_HEIGHT + 1.72;
const VIEW_3D_TANK_SIDE_ARMOR_Y: f32 = VIEW_3D_TANK_HEIGHT * 0.56;
const VIEW_3D_TANK_TRACK_PAD_Y: f32 = 1.35;
const VIEW_3D_TANK_EXHAUST_Y: f32 = VIEW_3D_TANK_HEIGHT + 0.25;
const VIEW_3D_TANK_HEADLIGHT_Y: f32 = VIEW_3D_TANK_HEIGHT + 0.42;
const VIEW_3D_TANK_ANTENNA_Y: f32 = VIEW_3D_TANK_HEIGHT + 3.1;
const VIEW_3D_BULLET_Y: f32 = 2.0;
const VIEW_3D_BULLET_TRAIL_Y: f32 = 1.55;
const VIEW_3D_BULLET_TRAIL_LENGTH: f32 = 5.6;
const VIEW_3D_BULLET_TRAIL_MIN_LENGTH: f32 = 1.6;
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
const VIEW_3D_MOVEMENT_BLOCK_MAX_LENGTH: f32 = TILE_SIZE * 3.0;
const VIEW_3D_MOVEMENT_BLOCK_SAMPLE_STEP: f32 = 1.0;
const VIEW_3D_MOVEMENT_BLOCK_WIDTH: f32 = TANK_SIZE + 1.2;
const VIEW_3D_MOVEMENT_BLOCK_HEIGHT: f32 = 5.4;
const VIEW_3D_MOVEMENT_BLOCK_THICKNESS: f32 = 0.5;
const VIEW_3D_CAMERA_DISTANCE: f32 = 38.0;
const VIEW_3D_CAMERA_HEIGHT: f32 = 22.0;
const VIEW_3D_CAMERA_LOOK_AHEAD: f32 = 36.0;
const VIEW_3D_CAMERA_LOOK_HEIGHT: f32 = 3.6;
const VIEW_3D_CAMERA_FOV_DEGREES: f32 = 38.0;
const VIEW_3D_CAMERA_TURN_RATE: f32 = std::f32::consts::PI * 7.0;
const VIEW_3D_CAMERA_POSITION_RESPONSE: f32 = 24.0;
const VIEW_3D_CAMERA_HEIGHT_MODE_SETTLED_DOT: f32 = 0.995;
const VIEW_3D_OCCLUDED_CAMERA_DISTANCE: f32 = 30.0;
const VIEW_3D_OCCLUDED_CAMERA_HEIGHT: f32 = VIEW_3D_CAMERA_HEIGHT;
const VIEW_3D_OCCLUDED_LOOK_HEIGHT: f32 = VIEW_3D_CAMERA_LOOK_HEIGHT;
const VIEW_3D_HUD_LINE_STEP: f32 = 9.0;
const VIEW_3D_HUD_TEXT_Z: f32 = 21.0;
const VIEW_3D_HUD_PANEL_Z: f32 = 20.0;
const VIEW_3D_STATUS_PANEL_WIDTH: f32 = 116.0;
const VIEW_3D_ENEMY_RESERVE_LEFT: f32 = 0.0;
const VIEW_3D_ENEMY_RESERVE_TOP: f32 = 72.0;
const VIEW_3D_ENEMY_RESERVE_COLUMNS: usize = 10;
const VIEW_3D_ENEMY_RESERVE_ICON_SIZE: f32 = 5.0;
const VIEW_3D_ENEMY_RESERVE_CELL_X: f32 = 6.0;
const VIEW_3D_ENEMY_RESERVE_CELL_Y: f32 = 6.0;
const VIEW_3D_ENEMY_DIRECTION_ARROW_SIZE: usize = 9;
const VIEW_3D_ENEMY_DIRECTION_LABEL_GAP: f32 = 3.0;
const VIEW_3D_ENEMY_DIRECTION_CENTER_OFFSET: f32 = 68.0;
pub(super) const VIEW_3D_MINIMAP_CELL_PIXELS: usize = 4;
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
const MINIMAP_OBJECT_OUTLINE_COLOR: [u8; 4] = [0, 0, 0, 255];
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum View3dEnemyDirection {
    Front,
    Right,
    Back,
    Left,
}

impl View3dEnemyDirection {
    fn index(self) -> usize {
        match self {
            View3dEnemyDirection::Front => 0,
            View3dEnemyDirection::Right => 1,
            View3dEnemyDirection::Back => 2,
            View3dEnemyDirection::Left => 3,
        }
    }

    fn arrow_rotation(self) -> f32 {
        match self {
            View3dEnemyDirection::Front => 0.0,
            View3dEnemyDirection::Right => -std::f32::consts::FRAC_PI_2,
            View3dEnemyDirection::Back => std::f32::consts::PI,
            View3dEnemyDirection::Left => std::f32::consts::FRAC_PI_2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct View3dEnemyDirectionIndicator {
    pub(super) direction: View3dEnemyDirection,
    pub(super) distance_tiles: u8,
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
    enemy_direction_arrow_image: Handle<Image>,
    ground_mesh: Handle<Mesh>,
    floor_grid_x_mesh: Handle<Mesh>,
    floor_grid_z_mesh: Handle<Mesh>,
    floor_tile_mesh: Handle<Mesh>,
    beveled_block_mesh: Handle<Mesh>,
    round_detail_mesh: Handle<Mesh>,
    canopy_mesh: Handle<Mesh>,
    capsule_strip_mesh: Handle<Mesh>,
    tank_body_mesh: Handle<Mesh>,
    tank_front_plate_mesh: Handle<Mesh>,
    tank_turret_mesh: Handle<Mesh>,
    tank_barrel_mesh: Handle<Mesh>,
    tank_track_mesh: Handle<Mesh>,
    tank_wheel_mesh: Handle<Mesh>,
    tank_hatch_mesh: Handle<Mesh>,
    tank_muzzle_mesh: Handle<Mesh>,
    protection_x_mesh: Handle<Mesh>,
    protection_z_mesh: Handle<Mesh>,
    effect_mesh: Handle<Mesh>,
    effect_sphere_mesh: Handle<Mesh>,
    effect_spark_mesh: Handle<Mesh>,
    base_mesh: Handle<Mesh>,
    base_cap_mesh: Handle<Mesh>,
    bullet_mesh: Handle<Mesh>,
    bullet_trail_mesh: Handle<Mesh>,
    marker_mesh: Handle<Mesh>,
    powerup_mesh: Handle<Mesh>,
    powerup_ring_mesh: Handle<Mesh>,
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
    water_basin_material: Handle<StandardMaterial>,
    water_edge_material: Handle<StandardMaterial>,
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
    tank_detail_material: Handle<StandardMaterial>,
    tank_headlight_material: Handle<StandardMaterial>,
    base_material: Handle<StandardMaterial>,
    player_one_base_material: Handle<StandardMaterial>,
    player_two_base_material: Handle<StandardMaterial>,
    enemy_marker_material: Handle<StandardMaterial>,
    base_marker_material: Handle<StandardMaterial>,
    player_one_bullet_material: Handle<StandardMaterial>,
    player_two_bullet_material: Handle<StandardMaterial>,
    enemy_bullet_material: Handle<StandardMaterial>,
    bullet_trail_material: Handle<StandardMaterial>,
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
    movement_block_material: Handle<StandardMaterial>,
    explosion_material: Handle<StandardMaterial>,
    base_destruction_material: Handle<StandardMaterial>,
    bullet_impact_material: Handle<StandardMaterial>,
    effect_flash_material: Handle<StandardMaterial>,
    effect_smoke_material: Handle<StandardMaterial>,
    effect_debris_material: Handle<StandardMaterial>,
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
        enemy_direction_arrow_image: images.add(create_3d_enemy_direction_arrow_image()),
        ground_mesh: meshes.add(Cuboid::new(
            board_size(),
            VIEW_3D_GROUND_THICKNESS,
            board_size(),
        )),
        floor_grid_x_mesh: meshes.add(Cuboid::new(board_size(), 0.05, 0.16)),
        floor_grid_z_mesh: meshes.add(Cuboid::new(0.16, 0.05, board_size())),
        floor_tile_mesh: meshes.add(Cuboid::new(TILE_SIZE, 0.08, TILE_SIZE)),
        beveled_block_mesh: meshes.add(sloped_box_mesh(Vec2::splat(1.0), Vec2::splat(0.84), 1.0)),
        round_detail_mesh: meshes.add(Cylinder {
            radius: 1.0,
            half_height: 0.5,
        }),
        canopy_mesh: meshes.add(Sphere { radius: 1.0 }),
        capsule_strip_mesh: meshes.add(Capsule3d {
            radius: 0.5,
            half_length: 0.5,
        }),
        tank_body_mesh: meshes.add(sloped_box_mesh(
            Vec2::new(13.4, 12.6),
            Vec2::new(8.8, 8.2),
            VIEW_3D_TANK_HEIGHT,
        )),
        tank_front_plate_mesh: meshes.add(sloped_box_mesh(
            Vec2::new(6.8, 4.8),
            Vec2::new(4.8, 2.8),
            0.65,
        )),
        tank_turret_mesh: meshes.add(Cylinder {
            radius: 3.7,
            half_height: 0.75,
        }),
        tank_barrel_mesh: meshes.add(Cylinder {
            radius: 0.65,
            half_height: VIEW_3D_TANK_BARREL_HALF_LENGTH,
        }),
        tank_track_mesh: meshes.add(Capsule3d {
            radius: 0.76,
            half_length: 5.7,
        }),
        tank_wheel_mesh: meshes.add(Cylinder {
            radius: 0.62,
            half_height: 0.22,
        }),
        tank_hatch_mesh: meshes.add(Cylinder {
            radius: 1.45,
            half_height: 0.24,
        }),
        tank_muzzle_mesh: meshes.add(Sphere { radius: 0.86 }),
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
        effect_sphere_mesh: meshes.add(Sphere { radius: 1.0 }),
        effect_spark_mesh: meshes.add(Capsule3d {
            radius: 0.5,
            half_length: 0.5,
        }),
        base_mesh: meshes.add(sloped_box_mesh(
            Vec2::splat(TANK_SIZE),
            Vec2::splat(TANK_SIZE * 0.72),
            VIEW_3D_BASE_HEIGHT,
        )),
        base_cap_mesh: meshes.add(Cylinder {
            radius: 4.2,
            half_height: 0.55,
        }),
        bullet_mesh: meshes.add(Capsule3d {
            radius: 0.95,
            half_length: 1.7,
        }),
        bullet_trail_mesh: meshes.add(Capsule3d {
            radius: 0.5,
            half_length: 0.5,
        }),
        marker_mesh: meshes.add(Sphere { radius: 1.5 }),
        powerup_mesh: meshes.add(Sphere { radius: 1.0 }),
        powerup_ring_mesh: meshes.add(Torus {
            major_radius: 2.4,
            minor_radius: 0.18,
        }),
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
        water_basin_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(10, 26, 44),
            perceptual_roughness: 0.82,
            ..default()
        }),
        water_edge_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(36, 52, 56),
            perceptual_roughness: 0.84,
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
        tank_detail_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(24, 30, 28),
            perceptual_roughness: 0.72,
            metallic: 0.15,
            ..default()
        }),
        tank_headlight_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 232, 128, 235),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
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
        bullet_trail_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 232, 128, 150),
            alpha_mode: AlphaMode::Blend,
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
        movement_block_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 72, 56, 190),
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
        effect_flash_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(255, 248, 184, 235),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }),
        effect_smoke_material: materials.add(StandardMaterial {
            base_color: Color::srgba_u8(48, 44, 40, 170),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        }),
        effect_debris_material: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(96, 68, 48),
            perceptual_roughness: 0.95,
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

pub(super) fn sloped_box_mesh(bottom_size: Vec2, top_size: Vec2, height: f32) -> Mesh {
    let bx = bottom_size.x / 2.0;
    let bz = bottom_size.y / 2.0;
    let tx = top_size.x / 2.0;
    let tz = top_size.y / 2.0;
    let bottom_y = -height / 2.0;
    let top_y = height / 2.0;

    let bfl = Vec3::new(-bx, bottom_y, -bz);
    let bfr = Vec3::new(bx, bottom_y, -bz);
    let bbr = Vec3::new(bx, bottom_y, bz);
    let bbl = Vec3::new(-bx, bottom_y, bz);
    let tfl = Vec3::new(-tx, top_y, -tz);
    let tfr = Vec3::new(tx, top_y, -tz);
    let tbr = Vec3::new(tx, top_y, tz);
    let tbl = Vec3::new(-tx, top_y, tz);

    let mut positions = Vec::with_capacity(24);
    let mut normals = Vec::with_capacity(24);
    let mut uvs = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        tfl,
        tbl,
        tbr,
        tfr,
    );
    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        bfl,
        bfr,
        bbr,
        bbl,
    );
    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        bfl,
        tfl,
        tfr,
        bfr,
    );
    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        bfr,
        tfr,
        tbr,
        bbr,
    );
    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        bbr,
        tbr,
        tbl,
        bbl,
    );
    push_sloped_box_quad(
        &mut positions,
        &mut normals,
        &mut uvs,
        &mut indices,
        bbl,
        tbl,
        tfl,
        bfl,
    );

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn push_sloped_box_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
) {
    let base = positions.len() as u32;
    let normal = (b - a).cross(c - a).normalize_or_zero().to_array();
    positions.extend([a.to_array(), b.to_array(), c.to_array(), d.to_array()]);
    normals.extend([normal; 4]);
    uvs.extend([[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]);
    indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
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
    effects: Query<(
        Entity,
        &Transform,
        &SpriteAnimation,
        Option<&BulletImpactDirection>,
    )>,
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
        for (entity, transform, animation, impact_direction) in &effects {
            let Some(kind) = view_3d_effect_kind(animation, &sprite_assets.manifest) else {
                continue;
            };
            let size = view_3d_effect_size(kind);
            let top_left = board_top_left_from_translation(transform.translation, size);
            if top_left_is_on_board(top_left) {
                spawn_3d_effect(
                    &mut commands,
                    &assets,
                    entity,
                    kind,
                    top_left,
                    impact_direction.map(|impact| impact.direction),
                );
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
    let hud_window_size = view_3d_hud_window_size(&primary_window);
    spawn_3d_status_hud(&mut commands, &assets, &status_lines, hud_window_size);

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
        spawn_3d_minimap(&mut commands, &view_assets, hud_window_size);
        let enemy_direction_indicators =
            view_3d_enemy_direction_indicators(*game_mode, tanks.iter(), view_target);
        spawn_3d_enemy_direction_indicators(
            &mut commands,
            &assets,
            &view_assets,
            &enemy_direction_indicators,
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

pub(super) fn create_3d_enemy_direction_arrow_image() -> Image {
    let size = VIEW_3D_ENEMY_DIRECTION_ARROW_SIZE;
    let mut pixels = vec![0; size * size * 4];
    let fill = [248, 216, 72, 245];
    let highlight = [255, 248, 160, 255];
    let shadow = [112, 80, 24, 220];

    for y in 0..=4 {
        let half_width = y;
        for x in (4 - half_width)..=(4 + half_width) {
            set_pixel(&mut pixels, size, x, y, fill);
        }
    }
    fill_rect(&mut pixels, size, 3, 4, 3, 5, fill);
    for y in 1..=4 {
        set_pixel(&mut pixels, size, 4, y, highlight);
    }
    for y in 4..size {
        set_pixel(&mut pixels, size, 6, y, shadow);
    }

    image_from_pixels(size, size, pixels)
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
            fill_minimap_tile_cell(&mut pixels, x, y, minimap_tile_color(tile));
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

pub(super) fn view_3d_enemy_direction_indicators<'a>(
    mode: GameMode,
    tanks: impl IntoIterator<Item = (&'a Tank, Option<&'a Player>, Option<&'a EnemyTank>)>,
    view_target: PlayerId,
) -> Vec<View3dEnemyDirectionIndicator> {
    let mut target = None;
    let mut threats = Vec::new();

    for (tank, player, enemy) in tanks {
        if player.is_some_and(|player| player.id == view_target) {
            target = Some((tank.top_left, tank.facing));
        }
        if view_3d_tank_threatens_target(mode, view_target, player, enemy) {
            threats.push(tank.top_left);
        }
    }

    let Some((target_top_left, target_facing)) = target else {
        return Vec::new();
    };

    let mut nearest: [Option<View3dEnemyDirectionIndicator>; 4] = [None; 4];
    for threat_top_left in threats {
        let Some(indicator) =
            view_3d_enemy_direction_indicator(target_top_left, target_facing, threat_top_left)
        else {
            continue;
        };
        let index = indicator.direction.index();
        if nearest[index].is_none_or(|current| indicator.distance_tiles < current.distance_tiles) {
            nearest[index] = Some(indicator);
        }
    }

    nearest.into_iter().flatten().collect()
}

fn view_3d_tank_threatens_target(
    mode: GameMode,
    view_target: PlayerId,
    player: Option<&Player>,
    enemy: Option<&EnemyTank>,
) -> bool {
    enemy.is_some()
        || (matches!(
            mode,
            GameMode::VersusDeathmatch | GameMode::VersusBaseBattle
        ) && player.is_some_and(|player| player.id != view_target))
}

pub(super) fn view_3d_enemy_direction_indicator(
    target_top_left: Vec2,
    target_facing: Direction,
    threat_top_left: Vec2,
) -> Option<View3dEnemyDirectionIndicator> {
    let target_center = target_top_left + Vec2::splat(TANK_SIZE / 2.0);
    let threat_center = threat_top_left + Vec2::splat(TANK_SIZE / 2.0);
    let delta = threat_center - target_center;
    if delta.length_squared() <= f32::EPSILON {
        return None;
    }

    let forward = target_facing.movement().normalize_or_zero();
    let right = Vec2::new(-forward.y, forward.x);
    let ahead = delta.dot(forward);
    let side = delta.dot(right);
    let direction = if ahead.abs() >= side.abs() {
        if ahead >= 0.0 {
            View3dEnemyDirection::Front
        } else {
            View3dEnemyDirection::Back
        }
    } else if side >= 0.0 {
        View3dEnemyDirection::Right
    } else {
        View3dEnemyDirection::Left
    };
    let distance_tiles = ((delta.length() / TILE_SIZE).ceil() as u8).clamp(1, 99);

    Some(View3dEnemyDirectionIndicator {
        direction,
        distance_tiles,
    })
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

fn spawn_3d_enemy_direction_indicators(
    commands: &mut Commands,
    assets: &SpriteAssets,
    view_assets: &View3dAssets,
    indicators: &[View3dEnemyDirectionIndicator],
) {
    for indicator in indicators {
        spawn_3d_enemy_direction_indicator(commands, assets, view_assets, *indicator);
    }
}

fn spawn_3d_enemy_direction_indicator(
    commands: &mut Commands,
    assets: &SpriteAssets,
    view_assets: &View3dAssets,
    indicator: View3dEnemyDirectionIndicator,
) {
    let arrow_size = Vec2::splat(VIEW_3D_ENEMY_DIRECTION_ARROW_SIZE as f32);
    let label = format!("E{:02}", indicator.distance_tiles);
    let label_size = Vec2::new(phase_text_width(&label), GENERATED_GLYPH_HEIGHT as f32);
    let panel_size = Vec2::new(
        arrow_size.x.max(label_size.x) + 6.0,
        arrow_size.y + VIEW_3D_ENEMY_DIRECTION_LABEL_GAP + label_size.y + 6.0,
    );
    let panel_top_left = view_3d_enemy_direction_panel_top_left(indicator.direction, panel_size);
    let panel_center = panel_top_left + panel_size / 2.0;
    let arrow_top_left = Vec2::new(panel_center.x - arrow_size.x / 2.0, panel_top_left.y + 3.0);
    let label_top_left = Vec2::new(
        panel_center.x - label_size.x / 2.0,
        arrow_top_left.y + arrow_size.y + VIEW_3D_ENEMY_DIRECTION_LABEL_GAP,
    );

    spawn_3d_hud_panel(commands, panel_top_left, panel_size);

    let mut arrow_transform = Transform::from_translation(virtual_center_scaled(
        arrow_top_left,
        arrow_size,
        VIEW_3D_HUD_TEXT_Z,
    ))
    .with_scale(Vec3::splat(window_scale()));
    arrow_transform.rotation = Quat::from_rotation_z(indicator.direction.arrow_rotation());
    commands.spawn((
        Sprite::from_image(view_assets.enemy_direction_arrow_image.clone()),
        arrow_transform,
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHud,
        GameEntity,
        Name::new(format!("3D Enemy Direction {:?}", indicator.direction)),
    ));

    spawn_3d_hud_text(commands, assets, &label, label_top_left);
}

fn view_3d_enemy_direction_panel_top_left(
    direction: View3dEnemyDirection,
    panel_size: Vec2,
) -> Vec2 {
    let center = match direction {
        View3dEnemyDirection::Front => Vec2::new(
            VIRTUAL_WIDTH / 2.0,
            VIRTUAL_HEIGHT / 2.0 - VIEW_3D_ENEMY_DIRECTION_CENTER_OFFSET,
        ),
        View3dEnemyDirection::Right => Vec2::new(
            VIRTUAL_WIDTH / 2.0 + VIEW_3D_ENEMY_DIRECTION_CENTER_OFFSET,
            VIRTUAL_HEIGHT / 2.0,
        ),
        View3dEnemyDirection::Back => Vec2::new(
            VIRTUAL_WIDTH / 2.0,
            VIRTUAL_HEIGHT / 2.0 + VIEW_3D_ENEMY_DIRECTION_CENTER_OFFSET,
        ),
        View3dEnemyDirection::Left => Vec2::new(
            VIRTUAL_WIDTH / 2.0 - VIEW_3D_ENEMY_DIRECTION_CENTER_OFFSET,
            VIRTUAL_HEIGHT / 2.0,
        ),
    };

    center - panel_size / 2.0
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

fn spawn_3d_status_hud(
    commands: &mut Commands,
    assets: &SpriteAssets,
    lines: &[String],
    window_size: Vec2,
) {
    let scale = window_scale();
    let panel_size = Vec2::new(
        VIEW_3D_STATUS_PANEL_WIDTH * scale,
        (lines.len() as f32 * VIEW_3D_HUD_LINE_STEP + 2.0) * scale,
    );
    spawn_3d_window_hud_panel(commands, Vec2::ZERO, panel_size, window_size);

    for (index, line) in lines.iter().enumerate() {
        spawn_3d_window_hud_text(
            commands,
            assets,
            line,
            Vec2::new(0.0, index as f32 * VIEW_3D_HUD_LINE_STEP * scale),
            window_size,
        );
    }
}

fn spawn_3d_window_hud_panel(
    commands: &mut Commands,
    top_left: Vec2,
    size: Vec2,
    window_size: Vec2,
) {
    commands.spawn((
        Sprite::from_color(Color::srgba_u8(0, 0, 0, 190), size),
        Transform::from_translation(window_top_left_center(
            top_left,
            size,
            window_size,
            VIEW_3D_HUD_PANEL_Z,
        )),
        RenderLayers::layer(VIEW_3D_HUD_LAYER),
        View3dHud,
        GameEntity,
    ));
}

fn spawn_3d_window_hud_text(
    commands: &mut Commands,
    assets: &SpriteAssets,
    text: &str,
    top_left: Vec2,
    window_size: Vec2,
) {
    let scale = window_scale();
    let glyph_size = glyph_size(&assets.manifest.glyphs) * scale;
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
            Transform::from_translation(window_top_left_center(
                Vec2::new(
                    top_left.x + index as f32 * GLYPH_ADVANCE * scale,
                    top_left.y,
                ),
                glyph_size,
                window_size,
                VIEW_3D_HUD_TEXT_Z,
            ))
            .with_scale(Vec3::splat(scale)),
            RenderLayers::layer(VIEW_3D_HUD_LAYER),
            View3dHud,
            GameEntity,
        ));
    }
}

pub(super) fn window_top_left_center(
    top_left: Vec2,
    size: Vec2,
    window_size: Vec2,
    z: f32,
) -> Vec3 {
    let center = top_left + size / 2.0;
    Vec3::new(
        center.x - window_size.x / 2.0,
        window_size.y / 2.0 - center.y,
        z,
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

fn fill_minimap_tile_cell(pixels: &mut [u8], grid_x: usize, grid_y: usize, color: [u8; 4]) {
    fill_minimap_cell_rect(
        pixels,
        grid_x,
        grid_y,
        0,
        0,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        MINIMAP_EMPTY_COLOR,
    );
    if color == MINIMAP_EMPTY_COLOR {
        return;
    }
    fill_minimap_cell_rect(
        pixels,
        grid_x,
        grid_y,
        1,
        1,
        VIEW_3D_MINIMAP_CELL_PIXELS.saturating_sub(2),
        VIEW_3D_MINIMAP_CELL_PIXELS.saturating_sub(2),
        color,
    );
}

fn fill_minimap_object_cell(
    pixels: &mut [u8],
    grid_x: usize,
    grid_y: usize,
    color: [u8; 4],
    outline: [u8; 4],
) {
    fill_minimap_cell_rect(
        pixels,
        grid_x,
        grid_y,
        0,
        0,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        VIEW_3D_MINIMAP_CELL_PIXELS,
        outline,
    );
    fill_minimap_cell_rect(
        pixels,
        grid_x,
        grid_y,
        1,
        1,
        VIEW_3D_MINIMAP_CELL_PIXELS.saturating_sub(2),
        VIEW_3D_MINIMAP_CELL_PIXELS.saturating_sub(2),
        color,
    );
}

fn fill_minimap_cell_rect(
    pixels: &mut [u8],
    grid_x: usize,
    grid_y: usize,
    offset_x: usize,
    offset_y: usize,
    width: usize,
    height: usize,
    color: [u8; 4],
) {
    fill_rect(
        pixels,
        VIEW_3D_MINIMAP_SIZE,
        grid_x * VIEW_3D_MINIMAP_CELL_PIXELS + offset_x,
        grid_y * VIEW_3D_MINIMAP_CELL_PIXELS + offset_y,
        width,
        height,
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
        fill_minimap_object_cell(pixels, grid_x, grid_y, color, MINIMAP_TARGET_COLOR);
    } else {
        fill_minimap_object_cell(pixels, grid_x, grid_y, color, MINIMAP_OBJECT_OUTLINE_COLOR);
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
            spawn_3d_tile(commands, assets, tile_grid, tile, x, y);
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
    tile_grid: &TileGrid,
    tile: TileKind,
    x: usize,
    y: usize,
) {
    match tile {
        TileKind::Empty | TileKind::Base => {}
        TileKind::Brick => spawn_3d_brick_tile(commands, assets, x, y),
        TileKind::Steel => spawn_3d_steel_tile(commands, assets, x, y),
        TileKind::Forest => spawn_3d_forest_tile(commands, assets, x, y),
        TileKind::Water => spawn_3d_water_tile(commands, assets, tile_grid, x, y),
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
            spawn_3d_static_beveled_block(
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
    spawn_3d_static_beveled_block(
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
    spawn_3d_static_beveled_block(
        commands,
        assets,
        center,
        seam_y,
        Vec3::new(seam_length, 0.18, 0.3),
        assets.steel_dark_material.clone(),
    );
    spawn_3d_static_beveled_block(
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
        spawn_3d_static_round_detail(
            commands,
            assets,
            center + offset,
            VIEW_3D_STEEL_HEIGHT + 0.38,
            Vec3::new(0.36, 0.34, 0.36),
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
        spawn_3d_static_round_detail(
            commands,
            assets,
            cluster_center,
            1.2,
            Vec3::new(0.35, 2.4, 0.35),
            assets.forest_trunk_material.clone(),
        );
        spawn_3d_static_canopy(
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

fn spawn_3d_water_tile(
    commands: &mut Commands,
    assets: &View3dAssets,
    tile_grid: &TileGrid,
    x: usize,
    y: usize,
) {
    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);
    spawn_3d_static_box(
        commands,
        assets,
        center,
        VIEW_3D_WATER_BASIN_HEIGHT / 2.0,
        Vec3::new(
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
            VIEW_3D_WATER_BASIN_HEIGHT,
            VIEW_3D_TILE_VISUAL_FOOTPRINT,
        ),
        assets.water_basin_material.clone(),
    );
    spawn_3d_static_box(
        commands,
        assets,
        center,
        VIEW_3D_WATER_SURFACE_HEIGHT,
        Vec3::new(
            VIEW_3D_TILE_VISUAL_FOOTPRINT - 1.2,
            0.04,
            VIEW_3D_TILE_VISUAL_FOOTPRINT - 1.2,
        ),
        assets.water_material.clone(),
    );

    for edge in water_tile_exposed_edges(tile_grid, x, y) {
        spawn_3d_water_edge(commands, assets, top_left, edge);
    }

    for (offset, width) in [
        (Vec2::new(-1.3, -2.0), 4.2),
        (Vec2::new(1.2, 0.0), 5.0),
        (Vec2::new(-0.6, 2.1), 3.6),
    ] {
        spawn_3d_static_capsule_strip_with_yaw(
            commands,
            assets,
            top_left + Vec2::splat(TILE_SIZE / 2.0) + offset,
            VIEW_3D_WATER_SURFACE_HEIGHT + 0.08,
            width,
            0.52,
            0.08,
            0.0,
            assets.water_highlight_material.clone(),
        );
    }
}

pub(super) fn water_tile_exposed_edges(tile_grid: &TileGrid, x: usize, y: usize) -> Vec<Direction> {
    [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .into_iter()
    .filter(|direction| !water_neighbor_matches(tile_grid, x, y, *direction))
    .collect()
}

fn water_neighbor_matches(tile_grid: &TileGrid, x: usize, y: usize, direction: Direction) -> bool {
    let (dx, dy) = match direction {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };
    tile_grid
        .get(x as i32 + dx, y as i32 + dy)
        .is_some_and(|tile| tile == TileKind::Water)
}

fn spawn_3d_water_edge(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    edge: Direction,
) {
    let half_tile = TILE_SIZE / 2.0;
    let half_edge = VIEW_3D_WATER_EDGE_THICKNESS / 2.0;
    let edge_length = TILE_SIZE;
    let (center, size) = match edge {
        Direction::Up => (
            top_left + Vec2::new(half_tile, half_edge),
            Vec3::new(
                edge_length,
                VIEW_3D_WATER_EDGE_HEIGHT,
                VIEW_3D_WATER_EDGE_THICKNESS,
            ),
        ),
        Direction::Right => (
            top_left + Vec2::new(TILE_SIZE - half_edge, half_tile),
            Vec3::new(
                VIEW_3D_WATER_EDGE_THICKNESS,
                VIEW_3D_WATER_EDGE_HEIGHT,
                edge_length,
            ),
        ),
        Direction::Down => (
            top_left + Vec2::new(half_tile, TILE_SIZE - half_edge),
            Vec3::new(
                edge_length,
                VIEW_3D_WATER_EDGE_HEIGHT,
                VIEW_3D_WATER_EDGE_THICKNESS,
            ),
        ),
        Direction::Left => (
            top_left + Vec2::new(half_edge, half_tile),
            Vec3::new(
                VIEW_3D_WATER_EDGE_THICKNESS,
                VIEW_3D_WATER_EDGE_HEIGHT,
                edge_length,
            ),
        ),
    };
    spawn_3d_static_beveled_block(
        commands,
        assets,
        center,
        VIEW_3D_WATER_EDGE_HEIGHT / 2.0,
        size,
        assets.water_edge_material.clone(),
    );
}

fn spawn_3d_ice_tile(commands: &mut Commands, assets: &View3dAssets, x: usize, y: usize) {
    spawn_3d_floor_tile(commands, assets, x, y, assets.ice_material.clone());

    let top_left = Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
    let center = top_left + Vec2::splat(TILE_SIZE / 2.0);
    for (offset, length, yaw) in [
        (Vec2::new(-1.2, -0.7), 5.0, 0.72),
        (Vec2::new(1.4, 1.3), 3.7, -0.62),
    ] {
        spawn_3d_static_capsule_strip_with_yaw(
            commands,
            assets,
            center + offset,
            VIEW_3D_ICE_SURFACE_HEIGHT + 0.08,
            length,
            0.38,
            0.08,
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
        assets.beveled_block_mesh.clone(),
        material,
        transform,
        View3dStatic,
    );
}

fn spawn_3d_static_beveled_block(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    size: Vec3,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_static_mesh_with_rotation(
        commands,
        assets.beveled_block_mesh.clone(),
        center,
        y_center,
        size,
        Quat::IDENTITY,
        material,
    );
}

fn spawn_3d_static_round_detail(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    size: Vec3,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_static_mesh_with_rotation(
        commands,
        assets.round_detail_mesh.clone(),
        center,
        y_center,
        size,
        Quat::IDENTITY,
        material,
    );
}

fn spawn_3d_static_canopy(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    size: Vec3,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_static_mesh_with_rotation(
        commands,
        assets.canopy_mesh.clone(),
        center,
        y_center,
        size / 2.0,
        Quat::IDENTITY,
        material,
    );
}

fn spawn_3d_static_capsule_strip_with_yaw(
    commands: &mut Commands,
    assets: &View3dAssets,
    center: Vec2,
    y_center: f32,
    length: f32,
    thickness: f32,
    height: f32,
    yaw: f32,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_static_mesh_with_rotation(
        commands,
        assets.capsule_strip_mesh.clone(),
        center,
        y_center,
        Vec3::new(height, length / 2.0, thickness),
        Quat::from_rotation_y(yaw) * Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
        material,
    );
}

fn spawn_3d_static_mesh_with_rotation(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    center: Vec2,
    y_center: f32,
    scale: Vec3,
    rotation: Quat,
    material: Handle<StandardMaterial>,
) {
    let mut transform = Transform::from_translation(board_3d_point(center, y_center));
    transform.rotation = rotation;
    transform.scale = scale;
    spawn_3d_mesh(commands, mesh, material, transform, View3dStatic);
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
    let mut body_transform = Transform::from_translation(center);
    body_transform.rotation = rotation_from_forward_to_direction(tank.facing);
    let body = spawn_3d_mesh(
        commands,
        assets.tank_body_mesh.clone(),
        material.clone(),
        body_transform,
        View3dDynamic,
    );
    commands.entity(body).insert(Name::new(format!(
        "3D Tank {:?} {:?}",
        material_kind, source
    )));

    spawn_3d_tank_tracks(commands, assets, tank.top_left, tank.facing);
    spawn_3d_tank_body_details(commands, assets, tank.top_left, tank.facing);
    spawn_3d_tank_front_plate(
        commands,
        assets,
        tank.top_left,
        tank.facing,
        material.clone(),
    );
    spawn_3d_tank_turret(commands, assets, tank.top_left, tank.facing, material);

    if let Some(player) = player {
        spawn_3d_player_marker(commands, assets, tank.top_left, player.id);
        if view_target {
            spawn_3d_view_target_marker(commands, assets, tank.top_left, player.id);
        }
    }

    spawn_3d_tank_barrel(commands, assets, tank.top_left, tank.facing);
    spawn_3d_tank_headlights(commands, assets, tank.top_left, tank.facing);

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
    if player.is_some() && view_target {
        spawn_3d_movement_block_indicator(commands, assets, tank, tile_grid);
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

    for offset in [-5.2, 5.2] {
        let mut transform = Transform::from_translation(board_3d_point(
            center + side * offset,
            VIEW_3D_TANK_TRACK_Y,
        ));
        transform.rotation = rotation_from_y_to_direction(facing);
        spawn_3d_mesh(
            commands,
            assets.tank_track_mesh.clone(),
            assets.barrel_material.clone(),
            transform,
            View3dDynamic,
        );

        for forward_offset in [-4.1, 0.0, 4.1] {
            let mut wheel_transform = Transform::from_translation(board_3d_point(
                center + side * offset + forward * forward_offset,
                VIEW_3D_TANK_WHEEL_Y,
            ));
            wheel_transform.rotation = rotation_from_y_to_horizontal_vector(side);
            spawn_3d_mesh(
                commands,
                assets.tank_wheel_mesh.clone(),
                assets.barrel_material.clone(),
                wheel_transform,
                View3dDynamic,
            );
        }
    }
}

fn spawn_3d_tank_body_details(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward = facing.movement();
    let side = Vec2::new(-forward.y, forward.x);

    for side_offset in [-4.85, 4.85] {
        spawn_3d_named_dynamic_mesh(
            commands,
            assets.effect_mesh.clone(),
            assets.tank_detail_material.clone(),
            Transform::from_translation(board_3d_point(
                center + side * side_offset,
                VIEW_3D_TANK_SIDE_ARMOR_Y,
            ))
            .with_scale(oriented_tank_box_scale(facing, 8.4, 0.58, 1.2)),
            "3D Tank Side Armor",
        );

        for forward_offset in [-5.0, -2.5, 0.0, 2.5, 5.0] {
            spawn_3d_named_dynamic_mesh(
                commands,
                assets.effect_mesh.clone(),
                assets.tank_detail_material.clone(),
                Transform::from_translation(board_3d_point(
                    center + side * side_offset + forward * forward_offset,
                    VIEW_3D_TANK_TRACK_PAD_Y,
                ))
                .with_scale(oriented_tank_box_scale(facing, 0.62, 0.24, 1.55)),
                "3D Tank Track Pad",
            );
        }
    }

    for side_offset in [-2.1, 2.1] {
        let mut exhaust_transform = Transform::from_translation(board_3d_point(
            center - forward * 5.8 + side * side_offset,
            VIEW_3D_TANK_EXHAUST_Y,
        ));
        exhaust_transform.rotation = rotation_from_y_to_direction(facing.opposite());
        exhaust_transform.scale = Vec3::new(0.34, 1.15, 0.34);
        spawn_3d_named_dynamic_mesh(
            commands,
            assets.capsule_strip_mesh.clone(),
            assets.tank_detail_material.clone(),
            exhaust_transform,
            "3D Tank Exhaust",
        );
    }
}

fn spawn_3d_tank_turret(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
    material: Handle<StandardMaterial>,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward = facing.movement();
    let side = Vec2::new(-forward.y, forward.x);
    spawn_3d_mesh(
        commands,
        assets.tank_turret_mesh.clone(),
        material.clone(),
        Transform::from_translation(board_3d_point(center, VIEW_3D_TANK_HEIGHT + 0.7)),
        View3dDynamic,
    );
    spawn_3d_named_dynamic_mesh(
        commands,
        assets.powerup_ring_mesh.clone(),
        assets.tank_detail_material.clone(),
        Transform::from_translation(board_3d_point(center, VIEW_3D_TANK_HEIGHT + 1.02))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::new(0.92, 0.92, 0.92)),
        "3D Tank Turret Ring",
    );
    spawn_3d_mesh(
        commands,
        assets.tank_hatch_mesh.clone(),
        assets.barrel_material.clone(),
        Transform::from_translation(board_3d_point(center, VIEW_3D_TANK_HATCH_Y)),
        View3dDynamic,
    );
    spawn_3d_named_dynamic_mesh(
        commands,
        assets.effect_mesh.clone(),
        assets.tank_detail_material.clone(),
        Transform::from_translation(board_3d_point(
            center + forward * 1.35 + side * 1.05,
            VIEW_3D_TANK_HATCH_Y + 0.34,
        ))
        .with_scale(oriented_tank_box_scale(facing, 1.2, 0.44, 0.72)),
        "3D Tank Periscope",
    );
    spawn_3d_named_dynamic_mesh(
        commands,
        assets.capsule_strip_mesh.clone(),
        assets.tank_detail_material.clone(),
        Transform::from_translation(board_3d_point(
            center - forward * 2.8 - side * 2.2,
            VIEW_3D_TANK_ANTENNA_Y,
        ))
        .with_scale(Vec3::new(0.13, 2.35, 0.13)),
        "3D Tank Antenna",
    );
}

fn spawn_3d_tank_front_plate(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
    material: Handle<StandardMaterial>,
) {
    spawn_3d_mesh(
        commands,
        assets.tank_front_plate_mesh.clone(),
        material,
        tank_front_plate_transform(top_left, facing),
        View3dDynamic,
    );
}

fn spawn_3d_tank_barrel(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
) {
    spawn_3d_mesh(
        commands,
        assets.tank_barrel_mesh.clone(),
        assets.barrel_material.clone(),
        tank_barrel_transform(top_left, facing),
        View3dDynamic,
    );
    spawn_3d_mesh(
        commands,
        assets.tank_muzzle_mesh.clone(),
        assets.barrel_material.clone(),
        tank_muzzle_transform(top_left, facing),
        View3dDynamic,
    );
}

fn spawn_3d_tank_headlights(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    facing: Direction,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let forward = facing.movement();
    let side = Vec2::new(-forward.y, forward.x);

    for side_offset in [-2.45, 2.45] {
        spawn_3d_named_dynamic_mesh(
            commands,
            assets.effect_sphere_mesh.clone(),
            assets.tank_headlight_material.clone(),
            Transform::from_translation(board_3d_point(
                center + forward * 6.4 + side * side_offset,
                VIEW_3D_TANK_HEADLIGHT_Y,
            ))
            .with_scale(Vec3::new(0.42, 0.32, 0.42)),
            "3D Tank Headlight",
        );
    }
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
        bullet_3d_transform(bullet),
        View3dDynamic,
    );
    commands.entity(entity).insert(Name::new(format!(
        "3D Bullet {:?} {:?}",
        material_kind, source
    )));

    let trail = spawn_3d_mesh(
        commands,
        assets.bullet_trail_mesh.clone(),
        assets.bullet_trail_material.clone(),
        bullet_trail_3d_transform(bullet),
        View3dDynamic,
    );
    commands.entity(trail).insert(Name::new(format!(
        "3D Bullet Trail {:?} {:?}",
        material_kind, source
    )));

    let glow = spawn_3d_mesh(
        commands,
        assets.effect_sphere_mesh.clone(),
        bullet_3d_material(assets, material_kind),
        bullet_glow_3d_transform(bullet),
        View3dDynamic,
    );
    commands.entity(glow).insert(Name::new(format!(
        "3D Bullet Glow {:?} {:?}",
        material_kind, source
    )));
}

pub(super) fn bullet_3d_transform(bullet: &Bullet) -> Transform {
    let mut transform = Transform::from_translation(board_3d_point(
        bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0),
        VIEW_3D_BULLET_Y,
    ));
    transform.rotation = rotation_from_y_to_direction(bullet.facing);
    transform
}

pub(super) fn bullet_trail_3d_transform(bullet: &Bullet) -> Transform {
    let center = bullet.top_left + Vec2::splat(BULLET_SIZE / 2.0)
        - bullet.facing.movement() * VIEW_3D_BULLET_TRAIL_BACK_OFFSET;
    let mut transform = Transform::from_translation(board_3d_point(center, VIEW_3D_BULLET_TRAIL_Y));
    transform.rotation = rotation_from_y_to_direction(bullet.facing);
    transform.scale = Vec3::new(0.65, VIEW_3D_BULLET_TRAIL_LENGTH / 2.0, 0.65);
    transform
}

fn bullet_glow_3d_transform(bullet: &Bullet) -> Transform {
    let center = bullet.top_left
        + Vec2::splat(BULLET_SIZE / 2.0)
        + bullet.facing.movement() * (BULLET_SIZE * 0.38);
    Transform::from_translation(board_3d_point(center, VIEW_3D_BULLET_Y))
        .with_scale(Vec3::new(1.25, 1.25, 1.25))
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
    let material = base_3d_material(assets, material_kind);
    let entity = spawn_3d_mesh(
        commands,
        assets.base_mesh.clone(),
        material.clone(),
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
    spawn_3d_mesh(
        commands,
        assets.base_cap_mesh.clone(),
        material.clone(),
        Transform::from_translation(board_3d_point(
            base.top_left + Vec2::splat(TANK_SIZE / 2.0),
            VIEW_3D_BASE_HEIGHT + 0.5,
        )),
        View3dDynamic,
    );
    spawn_3d_base_details(commands, assets, base.top_left, material);

    if assist_enabled {
        spawn_3d_marker(
            commands,
            assets,
            base.top_left,
            assets.base_marker_material.clone(),
        );
    }
}

fn spawn_3d_base_details(
    commands: &mut Commands,
    assets: &View3dAssets,
    top_left: Vec2,
    material: Handle<StandardMaterial>,
) {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    spawn_3d_named_dynamic_mesh(
        commands,
        assets.effect_mesh.clone(),
        assets.tank_detail_material.clone(),
        Transform::from_translation(board_3d_point(center, 0.35)).with_scale(Vec3::new(
            TANK_SIZE + 2.0,
            0.7,
            TANK_SIZE + 2.0,
        )),
        "3D Base Plinth",
    );

    spawn_3d_named_dynamic_mesh(
        commands,
        assets.effect_mesh.clone(),
        assets.tank_detail_material.clone(),
        Transform::from_translation(board_3d_point(
            center + Vec2::new(0.0, -TANK_SIZE / 2.0 - 0.35),
            1.35,
        ))
        .with_scale(Vec3::new(TANK_SIZE * 0.72, 1.6, 0.7)),
        "3D Base Front Lip",
    );

    spawn_3d_named_dynamic_mesh(
        commands,
        assets.effect_mesh.clone(),
        material.clone(),
        Transform::from_translation(board_3d_point(center, VIEW_3D_BASE_HEIGHT + 1.32))
            .with_scale(Vec3::new(2.6, 0.55, 4.4)),
        "3D Base Crest Body",
    );

    for side_offset in [-2.65, 2.65] {
        spawn_3d_named_dynamic_mesh(
            commands,
            assets.effect_mesh.clone(),
            material.clone(),
            Transform::from_translation(board_3d_point(
                center + Vec2::new(side_offset, 0.8),
                VIEW_3D_BASE_HEIGHT + 1.18,
            ))
            .with_scale(Vec3::new(2.2, 0.46, 2.0)),
            "3D Base Crest Wing",
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
        Transform::from_translation(board_3d_point(top_left + Vec2::splat(TANK_SIZE / 2.0), 1.8))
            .with_scale(Vec3::new(3.0, 1.25, 3.0)),
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new(format!("3D PowerUp {:?} {:?}", kind, source)));
    let mut ring_transform = Transform::from_translation(board_3d_point(
        top_left + Vec2::splat(TANK_SIZE / 2.0),
        3.15,
    ));
    ring_transform.rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    spawn_3d_mesh(
        commands,
        assets.powerup_ring_mesh.clone(),
        powerup_3d_material(assets, kind),
        ring_transform,
        View3dDynamic,
    );
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
    impact_direction: Option<Direction>,
) {
    match kind {
        View3dEffectKind::Explosion => {
            spawn_3d_explosion_effect(commands, assets, source, top_left)
        }
        View3dEffectKind::BaseDestruction => {
            spawn_3d_base_destruction_effect(commands, assets, source, top_left);
        }
        View3dEffectKind::BulletImpact => {
            spawn_3d_bullet_impact_effect(commands, assets, source, top_left, impact_direction);
        }
        View3dEffectKind::SpawnShimmer => {
            spawn_3d_spawn_shimmer_effect(commands, assets, source, top_left);
        }
        View3dEffectKind::PowerUpSparkle => {
            spawn_3d_powerup_sparkle_effect(commands, assets, source, top_left);
        }
    }
}

fn spawn_3d_explosion_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    top_left: Vec2,
) {
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.effect_flash_material.clone(),
        effect_part_transform(
            View3dEffectKind::Explosion,
            top_left,
            Vec2::ZERO,
            2.7,
            Vec3::new(11.5, 2.4, 11.5),
        ),
        View3dEffectKind::Explosion,
        "Flash",
        source,
    );
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.explosion_material.clone(),
        effect_part_transform(
            View3dEffectKind::Explosion,
            top_left,
            Vec2::ZERO,
            6.4,
            Vec3::new(7.2, 5.2, 7.2),
        ),
        View3dEffectKind::Explosion,
        "Core",
        source,
    );

    for (index, (offset, y, scale)) in [
        (Vec2::new(-4.8, -3.6), 7.0, Vec3::new(4.4, 3.2, 4.4)),
        (Vec2::new(4.4, -2.8), 7.8, Vec3::new(3.8, 3.0, 3.8)),
        (Vec2::new(-2.2, 4.6), 8.5, Vec3::new(4.8, 3.4, 4.8)),
        (Vec2::new(4.6, 4.0), 6.8, Vec3::new(3.4, 2.8, 3.4)),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_part(
            commands,
            assets.effect_sphere_mesh.clone(),
            assets.effect_smoke_material.clone(),
            effect_part_transform(View3dEffectKind::Explosion, top_left, offset, y, scale),
            View3dEffectKind::Explosion,
            effect_indexed_part_name("Smoke", index),
            source,
        );
    }

    for (index, direction) in [
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(0.74, 0.74),
        Vec2::new(-0.74, -0.74),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_spark(
            commands,
            assets,
            View3dEffectKind::Explosion,
            top_left,
            direction * 4.3,
            direction,
            4.2,
            0.28,
            effect_indexed_part_name("Spark", index),
            source,
        );
    }
}

fn spawn_3d_base_destruction_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    top_left: Vec2,
) {
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.base_destruction_material.clone(),
        effect_part_transform(
            View3dEffectKind::BaseDestruction,
            top_left,
            Vec2::ZERO,
            6.8,
            Vec3::new(10.0, 5.5, 10.0),
        ),
        View3dEffectKind::BaseDestruction,
        "Core",
        source,
    );

    for (index, (offset, scale, rotation)) in [
        (Vec2::new(-5.2, -3.8), Vec3::new(3.4, 1.4, 2.2), -0.35),
        (Vec2::new(4.8, -3.2), Vec3::new(2.8, 1.2, 2.8), 0.55),
        (Vec2::new(-3.4, 5.1), Vec3::new(2.4, 1.6, 3.2), 1.25),
        (Vec2::new(5.8, 4.4), Vec3::new(3.0, 1.1, 2.0), -1.0),
    ]
    .into_iter()
    .enumerate()
    {
        let mut transform = effect_part_transform(
            View3dEffectKind::BaseDestruction,
            top_left,
            offset,
            2.2,
            scale,
        );
        transform.rotation = Quat::from_rotation_y(rotation);
        spawn_3d_effect_part(
            commands,
            assets.beveled_block_mesh.clone(),
            assets.effect_debris_material.clone(),
            transform,
            View3dEffectKind::BaseDestruction,
            effect_indexed_part_name("Debris", index),
            source,
        );
    }

    for (index, (offset, scale)) in [
        (Vec2::new(-5.5, 0.5), Vec3::new(4.6, 3.8, 4.6)),
        (Vec2::new(0.5, -5.8), Vec3::new(4.0, 4.0, 4.0)),
        (Vec2::new(5.4, 1.6), Vec3::new(5.0, 3.5, 5.0)),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_part(
            commands,
            assets.effect_sphere_mesh.clone(),
            assets.effect_smoke_material.clone(),
            effect_part_transform(
                View3dEffectKind::BaseDestruction,
                top_left,
                offset,
                8.2,
                scale,
            ),
            View3dEffectKind::BaseDestruction,
            effect_indexed_part_name("Smoke", index),
            source,
        );
    }
}

fn spawn_3d_bullet_impact_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    top_left: Vec2,
) {
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.bullet_impact_material.clone(),
        effect_part_transform(
            View3dEffectKind::BulletImpact,
            top_left,
            Vec2::ZERO,
            2.6,
            Vec3::new(3.2, 1.5, 3.2),
        ),
        View3dEffectKind::BulletImpact,
        "Flash",
        source,
    );

    for (index, direction) in [
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_spark(
            commands,
            assets,
            View3dEffectKind::BulletImpact,
            top_left,
            direction * 1.9,
            direction,
            2.4,
            0.22,
            effect_indexed_part_name("Spark", index),
            source,
        );
    }

    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.effect_smoke_material.clone(),
        effect_part_transform(
            View3dEffectKind::BulletImpact,
            top_left,
            Vec2::new(0.4, 0.2),
            3.4,
            Vec3::new(2.0, 1.3, 2.0),
        ),
        View3dEffectKind::BulletImpact,
        "Smoke",
        source,
    );
}

fn spawn_3d_spawn_shimmer_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    top_left: Vec2,
) {
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.spawn_effect_material.clone(),
        effect_part_transform(
            View3dEffectKind::SpawnShimmer,
            top_left,
            Vec2::ZERO,
            6.0,
            Vec3::new(9.0, 4.0, 9.0),
        ),
        View3dEffectKind::SpawnShimmer,
        "Glow",
        source,
    );

    let mut ring_transform = effect_part_transform(
        View3dEffectKind::SpawnShimmer,
        top_left,
        Vec2::ZERO,
        2.6,
        Vec3::new(1.8, 1.8, 1.8),
    );
    ring_transform.rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    spawn_3d_effect_part(
        commands,
        assets.powerup_ring_mesh.clone(),
        assets.spawn_effect_material.clone(),
        ring_transform,
        View3dEffectKind::SpawnShimmer,
        "Ring",
        source,
    );

    for (index, offset) in [
        Vec2::new(-5.5, -5.5),
        Vec2::new(5.5, -5.5),
        Vec2::new(-5.5, 5.5),
        Vec2::new(5.5, 5.5),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_part(
            commands,
            assets.effect_spark_mesh.clone(),
            assets.spawn_effect_material.clone(),
            effect_part_transform(
                View3dEffectKind::SpawnShimmer,
                top_left,
                offset,
                5.2,
                Vec3::new(0.34, 5.8, 0.34),
            ),
            View3dEffectKind::SpawnShimmer,
            effect_indexed_part_name("Column", index),
            source,
        );
    }
}

fn spawn_3d_powerup_sparkle_effect(
    commands: &mut Commands,
    assets: &View3dAssets,
    source: Entity,
    top_left: Vec2,
) {
    spawn_3d_effect_part(
        commands,
        assets.effect_sphere_mesh.clone(),
        assets.powerup_sparkle_material.clone(),
        effect_part_transform(
            View3dEffectKind::PowerUpSparkle,
            top_left,
            Vec2::ZERO,
            4.8,
            Vec3::new(4.2, 2.4, 4.2),
        ),
        View3dEffectKind::PowerUpSparkle,
        "Flash",
        source,
    );

    let mut ring_transform = effect_part_transform(
        View3dEffectKind::PowerUpSparkle,
        top_left,
        Vec2::ZERO,
        4.1,
        Vec3::new(1.25, 1.25, 1.25),
    );
    ring_transform.rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    spawn_3d_effect_part(
        commands,
        assets.powerup_ring_mesh.clone(),
        assets.powerup_sparkle_material.clone(),
        ring_transform,
        View3dEffectKind::PowerUpSparkle,
        "Ring",
        source,
    );

    for (index, direction) in [
        Vec2::new(1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(0.7, -0.7),
        Vec2::new(-0.7, 0.7),
    ]
    .into_iter()
    .enumerate()
    {
        spawn_3d_effect_spark(
            commands,
            assets,
            View3dEffectKind::PowerUpSparkle,
            top_left,
            direction * 3.2,
            direction,
            2.8,
            0.2,
            effect_indexed_part_name("Ray", index),
            source,
        );
    }
}

fn spawn_3d_effect_spark(
    commands: &mut Commands,
    assets: &View3dAssets,
    kind: View3dEffectKind,
    top_left: Vec2,
    offset: Vec2,
    direction: Vec2,
    length: f32,
    width: f32,
    part: impl Into<String>,
    source: Entity,
) {
    let mut transform = effect_part_transform(
        kind,
        top_left,
        offset,
        3.7,
        Vec3::new(width, length / 2.0, width),
    );
    transform.rotation = rotation_from_y_to_horizontal_vector(direction);
    spawn_3d_effect_part(
        commands,
        assets.effect_spark_mesh.clone(),
        assets.bullet_impact_material.clone(),
        transform,
        kind,
        part,
        source,
    );
}

fn spawn_3d_effect_part(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    kind: View3dEffectKind,
    part: impl Into<String>,
    source: Entity,
) {
    let entity = spawn_3d_mesh(commands, mesh, material, transform, View3dDynamic);
    commands.entity(entity).insert(Name::new(format!(
        "3D Effect {:?} {} {:?}",
        kind,
        part.into(),
        source
    )));
}

fn effect_part_transform(
    kind: View3dEffectKind,
    top_left: Vec2,
    offset: Vec2,
    y: f32,
    scale: Vec3,
) -> Transform {
    Transform::from_translation(effect_part_translation(kind, top_left, offset, y))
        .with_scale(scale)
}

fn effect_part_translation(kind: View3dEffectKind, top_left: Vec2, offset: Vec2, y: f32) -> Vec3 {
    board_3d_point(
        top_left + Vec2::splat(view_3d_effect_size(kind) / 2.0) + offset,
        y,
    )
}

fn effect_indexed_part_name(label: &'static str, index: usize) -> String {
    format!("{label}{index}")
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

fn spawn_3d_movement_block_indicator(
    commands: &mut Commands,
    assets: &View3dAssets,
    tank: &Tank,
    tile_grid: &TileGrid,
) {
    let Some(clear_distance) =
        movement_block_contact_distance(tile_grid, tank, VIEW_3D_MOVEMENT_BLOCK_MAX_LENGTH)
    else {
        return;
    };

    let movement = tank.facing.movement();
    let center = tank.top_left
        + Vec2::splat(TANK_SIZE / 2.0)
        + movement * (TANK_SIZE / 2.0 + clear_distance);
    let mut transform =
        Transform::from_translation(board_3d_point(center, VIEW_3D_MOVEMENT_BLOCK_HEIGHT / 2.0));
    transform.scale = movement_block_indicator_scale(tank.facing);
    let entity = spawn_3d_mesh(
        commands,
        assets.effect_mesh.clone(),
        assets.movement_block_material.clone(),
        transform,
        View3dDynamic,
    );
    commands
        .entity(entity)
        .insert(Name::new("3D Movement Block"));
}

pub(super) fn movement_block_contact_distance(
    tile_grid: &TileGrid,
    tank: &Tank,
    max_length: f32,
) -> Option<f32> {
    let movement = tank.facing.movement();
    let samples = (max_length / VIEW_3D_MOVEMENT_BLOCK_SAMPLE_STEP).ceil() as usize;
    let mut clear_distance = 0.0;

    for sample in 1..=samples {
        let distance =
            (sample as f32 * VIEW_3D_MOVEMENT_BLOCK_SAMPLE_STEP).min(max_length.max(0.0));
        let top_left = tank.top_left + movement * distance;
        if !tile_grid.can_tank_occupy(top_left) {
            return Some(clear_distance);
        }
        clear_distance = distance;
    }

    None
}

fn movement_block_indicator_scale(direction: Direction) -> Vec3 {
    match direction {
        Direction::Left | Direction::Right => Vec3::new(
            VIEW_3D_MOVEMENT_BLOCK_THICKNESS,
            VIEW_3D_MOVEMENT_BLOCK_HEIGHT,
            VIEW_3D_MOVEMENT_BLOCK_WIDTH,
        ),
        Direction::Up | Direction::Down => Vec3::new(
            VIEW_3D_MOVEMENT_BLOCK_WIDTH,
            VIEW_3D_MOVEMENT_BLOCK_HEIGHT,
            VIEW_3D_MOVEMENT_BLOCK_THICKNESS,
        ),
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

fn spawn_3d_named_dynamic_mesh(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,
    name: &'static str,
) -> Entity {
    let entity = spawn_3d_mesh(commands, mesh, material, transform, View3dDynamic);
    commands.entity(entity).insert(Name::new(name));
    entity
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

fn oriented_tank_box_scale(
    facing: Direction,
    forward_length: f32,
    height: f32,
    width: f32,
) -> Vec3 {
    match facing {
        Direction::Left | Direction::Right => Vec3::new(forward_length, height, width),
        Direction::Up | Direction::Down => Vec3::new(width, height, forward_length),
    }
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

pub(super) fn chase_camera_transform_with_forward_and_height_mode(
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

pub(super) fn tank_barrel_transform(top_left: Vec2, direction: Direction) -> Transform {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let barrel_center = center + direction.movement() * VIEW_3D_TANK_BARREL_CENTER_OFFSET;
    let mut transform =
        Transform::from_translation(board_3d_point(barrel_center, VIEW_3D_TANK_BARREL_Y));
    transform.rotation = rotation_from_y_to_direction(direction);
    transform
}

pub(super) fn tank_front_plate_transform(top_left: Vec2, direction: Direction) -> Transform {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let plate_center = center + direction.movement() * 3.7;
    let mut transform =
        Transform::from_translation(board_3d_point(plate_center, VIEW_3D_TANK_FRONT_PLATE_Y));
    transform.rotation = rotation_from_forward_to_direction(direction);
    transform
}

pub(super) fn tank_muzzle_transform(top_left: Vec2, direction: Direction) -> Transform {
    let center = top_left + Vec2::splat(TANK_SIZE / 2.0);
    let muzzle_center = center
        + direction.movement()
            * (VIEW_3D_TANK_BARREL_CENTER_OFFSET + VIEW_3D_TANK_BARREL_HALF_LENGTH + 0.15);
    Transform::from_translation(board_3d_point(muzzle_center, VIEW_3D_TANK_BARREL_Y))
}

fn rotation_from_y_to_direction(direction: Direction) -> Quat {
    match direction {
        Direction::Up => Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        Direction::Down => Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
        Direction::Left => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
        Direction::Right => Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
    }
}

fn rotation_from_forward_to_direction(direction: Direction) -> Quat {
    match direction {
        Direction::Up => Quat::IDENTITY,
        Direction::Down => Quat::from_rotation_y(std::f32::consts::PI),
        Direction::Left => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        Direction::Right => Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
    }
}

fn rotation_from_y_to_horizontal_vector(direction: Vec2) -> Quat {
    let target = Vec3::new(direction.x, 0.0, direction.y).normalize_or_zero();
    if target.length_squared() <= f32::EPSILON {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(Vec3::Y, target)
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
