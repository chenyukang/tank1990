# Tank 1990 Bevy Remake

A Rust + Bevy desktop remake inspired by Tank 1990 / Battle City. The project
uses a 256x240 virtual screen, nearest-neighbor sprites, 26x26 small-tile maps,
campaign stages, local versus arenas, power-ups, retro generated sounds, and
gitignored personal asset overrides for private playtesting.

## Run

```bash
cargo run
```

The default window scale is 3x. Music, sound, and scale are controlled from the
main menu settings, not environment variables. For capture or small displays,
choose a crisp integer scale from the main menu `SCALE` setting: `2X`, `3X`, or
`4X`.

## Controls

Mode select:

- `W` / `S` or arrow up/down: switch Campaign/Battle/Music/Sound/Scale
- `A` / `D` or arrow left/right: choose stage while Campaign is selected, or
  arena while Battle is selected; toggle `MUSIC`, `SOUND`, or `SCALE` settings
- `Space`, `Enter`, or `RightShift`: start selected mode, or toggle the selected
  setting

In game:

- P1 move: `W` `A` `S` `D`
- P1 fire: `Space`
- P2 move: arrow keys
- P2 fire: `Enter` or `RightShift`
- Pause/resume: `P`, `Esc`, or `Pause`
- Restart current stage or round: `R`
- Return to mode select: `M`

## Current Content

- Campaign: 50 authored stages in `assets/levels/`.
- Versus: 8 authored arenas in `assets/arenas/`.
- Arenas 5, 6, and 8 are `BaseBattle`; the others are `Deathmatch`.
- Generated placeholder sprite atlases and sounds are used when no personal
  override exists.

## Distribution

Build the release executable with:

```bash
cargo build --release
```

Then run the binary directly:

```bash
./target/release/tank
```

The default asset manifest, campaign stages, versus arenas, generated sprites,
and generated sounds are built into the executable, so the default game can be
distributed as one platform-specific binary without an `assets/` directory.
Optional private overrides still work when `assets/personal/` is present next
to the working directory used to launch the game. This is asset-free
distribution, not a fully static Linux build; target systems may still need the
usual graphics and audio runtime libraries.

Tag-based GitHub Releases are published by `.github/workflows/release.yml`.
Pushing a version tag such as `v0.1.0` builds Linux, macOS, and Windows release
archives that each contain only the executable:

```bash
git tag v0.1.0
git push origin v0.1.0
```

## Personal Assets

Do not commit original Tank 1990, Battle City, ROM-extracted, or otherwise
commercial assets to this repository. For private local playtesting, place
copied or converted files under the gitignored `assets/personal/` tree.

Supported personal sprite/image override paths:

```text
assets/personal/manifest.ron
assets/personal/tanks.png
assets/personal/terrain.png
assets/personal/bullets.png
assets/personal/effects.png
assets/personal/powerups.png
assets/personal/base_intact.png
assets/personal/base_destroyed.png
assets/personal/score_badge.png
assets/personal/stage_flag.png
assets/personal/shield.png
assets/personal/glyphs.png
```

The default atlas dimensions are:

- `tanks.png`: 48 horizontal frames, `16x16` each
- `terrain.png`: 6 horizontal frames, `8x8` each
- `bullets.png`: 4 horizontal frames, `4x4` each
- `effects.png`: 20 horizontal frames, `16x16` each
- `powerups.png`: 6 horizontal frames, `16x16` each
- standalone base/UI/shield images: see `assets/manifest.ron`; `shield.png`
  is `16x16`

Supported personal sound override paths:

```text
assets/personal/sounds/fire.ogg
assets/personal/sounds/brick_hit.ogg
assets/personal/sounds/steel_hit.ogg
assets/personal/sounds/tank_explosion.ogg
assets/personal/sounds/base_destroyed.ogg
assets/personal/sounds/powerup_pickup.ogg
assets/personal/sounds/stage_start.ogg
assets/personal/sounds/level_clear.ogg
assets/personal/sounds/game_over.ogg
assets/personal/sounds/background.ogg
```

Use the main menu `MUSIC` setting to choose `BGM` for the generated background
loop, `CUSTOM` for `assets/personal/sounds/background.ogg` when that file is
present, or `CLASSIC` for original-style play with no continuous background
loop. Use `SOUND` to enable or mute one-shot sound effects and short jingles. To
use an original soundtrack for private local play, provide your own lawful copy
converted to OGG at `assets/personal/sounds/background.ogg`; this repository
intentionally does not include or distribute original game music.

Supported personal map override paths:

```text
assets/personal/levels/001.level.ron
assets/personal/arenas/arena_01.ron
```

Use the same naming pattern for other stages and arenas. Personal maps use the
same RON schema and validation rules as committed maps.

## Checks

```bash
cargo fmt --all
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
git diff --check
```
