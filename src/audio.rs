use crate::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub(super) struct SoundNote {
    pub(super) duration_secs: f32,
    pub(super) frequency: f32,
    pub(super) volume: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(super) enum RetroSoundSpec {
    Sweep {
        duration_secs: f32,
        start_frequency: f32,
        end_frequency: f32,
        volume: f32,
    },
    Noise {
        duration_secs: f32,
        volume: f32,
        seed: u32,
    },
    Layered {
        notes: Vec<SoundNote>,
    },
}

#[derive(Resource)]
pub(super) struct SoundAssets {
    pub(super) sound_enabled: bool,
    pub(super) fire: SoundHandle,
    pub(super) brick_hit: SoundHandle,
    pub(super) steel_hit: SoundHandle,
    pub(super) tank_explosion: SoundHandle,
    pub(super) base_destroyed: SoundHandle,
    pub(super) powerup_pickup: SoundHandle,
    pub(super) stage_start: SoundHandle,
    pub(super) level_clear: SoundHandle,
    pub(super) game_over: SoundHandle,
    pub(super) generated_background_music: SoundHandle,
    pub(super) custom_background_music: Option<SoundHandle>,
}

#[derive(Clone)]
pub(super) enum SoundHandle {
    Retro(Handle<RetroSound>),
    File(Handle<AudioSource>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SoundKind {
    Fire,
    BrickHit,
    SteelHit,
    TankExplosion,
    BaseDestroyed,
    PowerupPickup,
    StageStart,
    LevelClear,
    GameOver,
}

pub(super) const CAMPAIGN_BASE_DESTROYED_SOUNDS: [SoundKind; 2] =
    [SoundKind::BaseDestroyed, SoundKind::GameOver];
pub(super) const VERSUS_BASE_DESTROYED_SOUNDS: [SoundKind; 2] =
    [SoundKind::BaseDestroyed, SoundKind::LevelClear];
pub(super) const NO_BASE_DESTROYED_SOUNDS: [SoundKind; 0] = [];

#[derive(Asset, TypePath)]
pub(super) struct RetroSound {
    pub(super) samples: Arc<[f32]>,
    pub(super) sample_rate: u32,
}

pub(super) struct RetroSoundDecoder {
    pub(super) samples: Arc<[f32]>,
    pub(super) sample_rate: u32,
    pub(super) cursor: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AudioMode {
    Bgm,
    Custom,
    Classic,
}

pub(super) fn next_audio_mode(mode: AudioMode, custom_available: bool) -> AudioMode {
    match (mode, custom_available) {
        (AudioMode::Bgm, true) => AudioMode::Custom,
        (AudioMode::Bgm, false) => AudioMode::Classic,
        (AudioMode::Custom, _) => AudioMode::Classic,
        (AudioMode::Classic, _) => AudioMode::Bgm,
    }
}

pub(super) fn previous_audio_mode(mode: AudioMode, custom_available: bool) -> AudioMode {
    match (mode, custom_available) {
        (AudioMode::Bgm, _) => AudioMode::Classic,
        (AudioMode::Custom, _) => AudioMode::Bgm,
        (AudioMode::Classic, true) => AudioMode::Custom,
        (AudioMode::Classic, false) => AudioMode::Bgm,
    }
}

pub(super) fn custom_background_music_available(sounds: &SoundAssets) -> bool {
    sounds.custom_background_music.is_some()
}

pub(super) fn next_available_audio_mode(mode: AudioMode, sounds: &SoundAssets) -> AudioMode {
    next_audio_mode(mode, custom_background_music_available(sounds))
}

pub(super) fn previous_available_audio_mode(mode: AudioMode, sounds: &SoundAssets) -> AudioMode {
    previous_audio_mode(mode, custom_background_music_available(sounds))
}

pub(super) fn background_music_handle_for_mode(
    sounds: &SoundAssets,
    mode: AudioMode,
) -> Option<&SoundHandle> {
    match mode {
        AudioMode::Bgm => Some(&sounds.generated_background_music),
        AudioMode::Custom => sounds.custom_background_music.as_ref(),
        AudioMode::Classic => None,
    }
}

pub(super) fn audio_mode_label(mode: AudioMode) -> &'static str {
    match mode {
        AudioMode::Bgm => "BGM",
        AudioMode::Custom => "CUSTOM",
        AudioMode::Classic => "CLASSIC",
    }
}

pub(super) fn toggle_sound_enabled(enabled: bool) -> bool {
    !enabled
}

pub(super) fn sound_enabled_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

#[derive(Component)]
pub(super) struct BackgroundMusic {
    pub(super) mode: AudioMode,
}

impl Iterator for RetroSoundDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.samples.get(self.cursor).copied();
        self.cursor += usize::from(sample.is_some());
        sample
    }
}

impl Source for RetroSoundDecoder {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len().saturating_sub(self.cursor))
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32,
        ))
    }
}

impl Decodable for RetroSound {
    type DecoderItem = f32;
    type Decoder = RetroSoundDecoder;

    fn decoder(&self) -> Self::Decoder {
        RetroSoundDecoder {
            samples: self.samples.clone(),
            sample_rate: self.sample_rate,
            cursor: 0,
        }
    }
}
