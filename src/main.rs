#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::RenderAssetUsages;
use bevy::audio::{AudioPlayer, AudioSource, Decodable, PlaybackSettings, Source, Volume};
#[cfg(test)]
use bevy::image::ImageFilterMode;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};
use serde::Deserialize;
#[cfg(test)]
use std::collections::{HashSet, VecDeque};
#[cfg(test)]
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

mod app;
mod assets;
mod audio;
mod combat;
mod components;
mod content;
mod controls;
mod enemy;
mod game_flow;
mod game_state;
mod layout;
mod lifecycle;
mod phase;
mod physics;
mod player;
mod powerups;
mod runtime;
mod spawning;
mod ui;
mod validation;
mod view3d;
mod visuals;
mod windowing;
mod world;

use app::*;
use assets::*;
use audio::*;
use combat::*;
use components::*;
use content::*;
use controls::*;
use enemy::*;
use game_flow::*;
use game_state::*;
use layout::*;
use lifecycle::*;
use phase::*;
use physics::*;
use player::*;
use powerups::*;
use runtime::*;
use spawning::*;
use ui::*;
use validation::*;
use view3d::*;
use visuals::*;
use windowing::*;
use world::*;

const ASSET_MANIFEST_PATH: &str = "assets/manifest.ron";
const PERSONAL_ASSET_MANIFEST_PATH: &str = "assets/personal/manifest.ron";
const ASSET_ROOT_DIR: &str = "assets";
const PERSONAL_TANK_ATLAS_PATH: &str = "personal/tanks.png";
const PERSONAL_TERRAIN_ATLAS_PATH: &str = "personal/terrain.png";
const PERSONAL_BULLET_ATLAS_PATH: &str = "personal/bullets.png";
const PERSONAL_EFFECT_ATLAS_PATH: &str = "personal/effects.png";
const PERSONAL_POWERUP_ATLAS_PATH: &str = "personal/powerups.png";
const PERSONAL_BASE_INTACT_PATH: &str = "personal/base_intact.png";
const PERSONAL_BASE_DESTROYED_PATH: &str = "personal/base_destroyed.png";
const PERSONAL_SCORE_BADGE_PATH: &str = "personal/score_badge.png";
const PERSONAL_STAGE_FLAG_PATH: &str = "personal/stage_flag.png";
const PERSONAL_SHIELD_PATH: &str = "personal/shield.png";
const PERSONAL_GLYPH_ATLAS_PATH: &str = "personal/glyphs.png";
const PERSONAL_FIRE_SOUND_PATH: &str = "personal/sounds/fire.ogg";
const PERSONAL_BRICK_HIT_SOUND_PATH: &str = "personal/sounds/brick_hit.ogg";
const PERSONAL_STEEL_HIT_SOUND_PATH: &str = "personal/sounds/steel_hit.ogg";
const PERSONAL_TANK_EXPLOSION_SOUND_PATH: &str = "personal/sounds/tank_explosion.ogg";
const PERSONAL_BASE_DESTROYED_SOUND_PATH: &str = "personal/sounds/base_destroyed.ogg";
const PERSONAL_POWERUP_PICKUP_SOUND_PATH: &str = "personal/sounds/powerup_pickup.ogg";
const PERSONAL_STAGE_START_SOUND_PATH: &str = "personal/sounds/stage_start.ogg";
const PERSONAL_LEVEL_CLEAR_SOUND_PATH: &str = "personal/sounds/level_clear.ogg";
const PERSONAL_GAME_OVER_SOUND_PATH: &str = "personal/sounds/game_over.ogg";
const PERSONAL_BACKGROUND_MUSIC_SOUND_PATH: &str = "personal/sounds/background.ogg";
#[cfg(test)]
const PERSONAL_SPRITE_OVERRIDE_PATHS: [&str; 11] = [
    PERSONAL_TANK_ATLAS_PATH,
    PERSONAL_TERRAIN_ATLAS_PATH,
    PERSONAL_BULLET_ATLAS_PATH,
    PERSONAL_EFFECT_ATLAS_PATH,
    PERSONAL_POWERUP_ATLAS_PATH,
    PERSONAL_BASE_INTACT_PATH,
    PERSONAL_BASE_DESTROYED_PATH,
    PERSONAL_SCORE_BADGE_PATH,
    PERSONAL_STAGE_FLAG_PATH,
    PERSONAL_SHIELD_PATH,
    PERSONAL_GLYPH_ATLAS_PATH,
];
#[cfg(test)]
const PERSONAL_SOUND_OVERRIDE_PATHS: [&str; 10] = [
    PERSONAL_FIRE_SOUND_PATH,
    PERSONAL_BRICK_HIT_SOUND_PATH,
    PERSONAL_STEEL_HIT_SOUND_PATH,
    PERSONAL_TANK_EXPLOSION_SOUND_PATH,
    PERSONAL_BASE_DESTROYED_SOUND_PATH,
    PERSONAL_POWERUP_PICKUP_SOUND_PATH,
    PERSONAL_STAGE_START_SOUND_PATH,
    PERSONAL_LEVEL_CLEAR_SOUND_PATH,
    PERSONAL_GAME_OVER_SOUND_PATH,
    PERSONAL_BACKGROUND_MUSIC_SOUND_PATH,
];
const CUSTOM_LEVEL_COUNT: usize = 50;
const ORIGINAL_LEVEL_COUNT: usize = 35;
const LEVEL_CLEAR_DELAY_SECONDS: f32 = 2.0;
const LEVEL_CLEAR_SCORECARD_SECONDS: f32 = 4.0;
const STAGE_INTRO_SECONDS: f32 = 1.2;
const ARENA_COUNT: usize = 8;
const DEFAULT_VERSUS_ARENA: usize = 1;
const TANK_ATLAS_TILES: usize = 48;
const TANK_ANIMATION_FRAMES: usize = 2;
const TANK_ATLAS_TILE_SIZE: usize = 16;
const BULLET_ATLAS_TILES: usize = 4;
const BULLET_ATLAS_TILE_SIZE: usize = 4;
const TERRAIN_ATLAS_TILES: usize = 6;
const TERRAIN_ATLAS_TILE_SIZE: usize = 8;
const EFFECT_ATLAS_TILES: usize = 20;
const EFFECT_ATLAS_TILE_SIZE: usize = 16;
const POWERUP_ATLAS_TILES: usize = 6;
const POWERUP_ATLAS_TILE_SIZE: usize = 16;
const GENERATED_BASE_SIZE: usize = 16;
const GENERATED_SHIELD_SIZE: usize = 16;
const GENERATED_UI_ICON_SIZE: usize = 8;

const PLAYER_SPEED: f32 = 60.0;
const BULLET_SPEED: f32 = 240.0;
const PLAYER_FAST_BULLET_SPEED: f32 = 300.0;
const POWER_ENEMY_BULLET_SPEED: f32 = 300.0;
const ENEMY_BULLET_LIMIT: usize = 4;
const ENEMY_BULLET_LIMIT_PER_TANK: usize = 1;
const CLASSIC_MAX_ACTIVE_ENEMIES: usize = 4;
const CLASSIC_BASE_X: usize = 12;
const CLASSIC_BASE_Y: usize = 24;
const CLASSIC_COOP_P2_SPAWN_X: usize = 16;
const CLASSIC_COOP_P2_SPAWN_Y: usize = 24;
const SNAP_DISTANCE: f32 = 2.0;
const LANE_ASSIST_MAX_DISTANCE: f32 = TILE_SIZE / 2.0;
const REQUIRED_GLYPHS: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const GENERATED_GLYPH_WIDTH: usize = 5;
const GENERATED_GLYPH_HEIGHT: usize = 7;
const GENERATED_GLYPH_ATLAS_PADDING_X: usize = 1;
static MODE_SELECT_HINT_LINES: [&str; 3] = ["", "", "SPACE ENTER START"];
const DEFAULT_RESPAWN_INVULNERABILITY_SECONDS: f32 = 2.0;
const HELMET_SECONDS: f32 = 6.0;
const CLOCK_SECONDS: f32 = 6.0;
const SHOVEL_SECONDS: f32 = 10.0;
const SHOVEL_WARNING_SECONDS: f32 = 2.0;
const EXPLOSION_FRAME_SECONDS: f32 = 0.07;
const BULLET_IMPACT_FRAME_SECONDS: f32 = 0.05;
const STAGE_CLEAR_LIFE_BONUS: u32 = 1000;
const MAX_PLAYER_LIVES: i32 = 9;
const ENEMY_ALIGNMENT_FIRE_FRACTION: f32 = 0.45;
const VERSUS_POWERUP_INTERVAL_SECONDS: f32 = 8.0;
const SOUND_SAMPLE_RATE: u32 = 22_050;
const MAX_RETRO_SOUND_SECONDS: f32 = 1.0;
const MAX_RETRO_SOUND_FREQUENCY: f32 = 4_000.0;
const MAX_RETRO_SOUND_VOLUME: f32 = 1.0;
const BACKGROUND_MUSIC_VOLUME: f32 = 0.18;
const BACKGROUND_MUSIC_STEP_SECONDS: f32 = 0.12;
const ICE_SPEED_MULTIPLIER: f32 = 1.18;
const SPAWN_SHIMMER_FRAME_SECONDS: f32 = 0.08;
const BULLET_CLASH_TIME_EPSILON: f32 = 0.0001;

fn main() {
    run_app();
}

#[cfg(test)]
mod tests;
