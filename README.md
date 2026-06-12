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


