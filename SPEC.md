# Tank 1990 Bevy Remake Spec

## 1. Product Goal

Build a Rust + Bevy remake inspired by Tank 1990 / Battle City.

The game should preserve the classic FC/NES-era feel: top-down tile maps, sprite-based tanks, destructible bricks, enemy waves, base defense, power-ups, stage-based progression, a right-side status panel, and fast local play. It should also support both a one-player campaign mode and a local player-vs-player battle mode.

The first milestone is not a byte-for-byte clone. The goal is a faithful-feeling foundation whose core systems are data-driven enough to support many stages, arenas, sprites, and rule variants later.

## 2. Fidelity Target

The intended feel is "as if this could have been a late FC/Dendy tank game," not a modern smooth twin-stick shooter.

Hard fidelity rules:

- Use a low-resolution virtual canvas and integer scaling.
- Use nearest-neighbor texture sampling only.
- Use sprites, not rotated vector shapes.
- Tanks face only up, down, left, and right.
- Movement is axis-aligned.
- Bullets travel in straight cardinal directions.
- The battlefield is built from small square tiles.
- The background outside visible tiles is black.
- UI is compact and score-panel-like, not a modern HUD overlay.
- Effects should be short sprite animations: spawn shimmer, explosion, base destruction.

Soft fidelity rules:

- Prefer limited palettes per sprite.
- Prefer simple two-frame movement animation.
- Prefer short, sharp sound effects.
- Prefer fixed-step game logic over purely variable-time movement.
- Let the game feel slightly mechanical and tile-snapped.

Legal and asset boundary:

- Do not copy original Tank 1990, Battle City, or ROM-extracted sprites, sounds, maps, or music into this repository.
- Original or permissively licensed assets may imitate the era and constraints.
- Public repositories without a clear license may be used only as gameplay or structure references, not as asset sources.

## 3. Target Platform

- Language: Rust
- Engine: Bevy
- Rendering: 2D sprites with a texture atlas
- Initial platform: desktop macOS
- Future platform: web/WASM if the Bevy version and asset pipeline make it practical

## 4. Screen And Rendering

### 4.1 Virtual Resolution

Use a virtual FC-like resolution:

- Virtual canvas: `256x240`
- Battlefield: `208x208`
- Status panel: `48x208`, placed on the right
- Top/bottom margin: reserved for centering or future transitions
- Logical battlefield: `26x26` small tiles
- Small tile size: `8x8`
- Classic large cell size: `16x16`, represented as `2x2` small tiles

This produces the classic 13x13 stage layout while still allowing quarter-brick destruction.

Render the virtual canvas to the actual window using integer scaling:

- `2x`: `512x480`
- `3x`: `768x720`
- `4x`: `1024x960`

Default desktop window:

- Use `3x` scale if it fits the display.
- Fall back to `2x` if needed.
- Never stretch non-uniformly.

### 4.2 Camera And Pixels

- Camera coordinates should map cleanly to integer virtual pixels.
- Sprites should be placed on integer pixel boundaries.
- Texture filtering must be nearest-neighbor.
- Avoid subpixel movement in final transforms.
- Camera should not pan during normal gameplay.

### 4.3 Draw Order

Suggested draw layers:

1. Background black fill
2. Ground tiles: empty, ice
3. Water
4. Brick, steel, base walls
5. Base
6. Power-ups
7. Tanks
8. Bullets
9. Forest overlay
10. Explosions and spawn effects
11. Status panel and UI text/icons

Forest should visually cover tanks and bullets while remaining passable.

## 5. Game Modes

### 5.1 Solo Campaign

Classic one-player Tank 1990 mode.

Player objective:

- Destroy all enemy tanks in the stage.
- Protect the base.
- Survive with remaining lives.

Lose conditions:

- Player lives reach zero.
- The base is destroyed.

Win condition:

- All enemies in the stage roster are destroyed.

Progression:

- Load the next level after clearing the current level.
- If the last authored level is cleared, show a victory screen or loop with harder enemy rosters.

Stage count target:

- MVP: 3 authored levels.
- First complete campaign: 35 levels.
- Optional Tank 1990-style extended campaign: 50 levels.

### 5.2 Local Versus

Two-player local battle mode using the same tank, bullet, tile, collision, and power-up systems.

Supported battle variants:

- `Deathmatch`: P1 and P2 fight until one reaches the target score or the other runs out of lives.
- `BaseBattle`: P1 and P2 each defend a base while trying to destroy the opponent's base.

Initial implementation target:

- Implement `Deathmatch` first.
- Add `BaseBattle` after campaign base logic is stable.
- Mode select should allow choosing among authored deathmatch arenas before starting a battle.

### 5.3 Optional Co-op

Two-player co-op is not required for MVP, but the architecture should not block it. If added, co-op uses campaign levels with:

- P1 and P2 both on the player team.
- Shared base.
- Separate lives or shared lives, to be decided.
- Enemy roster and stage clear rules unchanged.

## 6. Controls

### 6.1 Player 1

- Move up: `W`
- Move down: `S`
- Move left: `A`
- Move right: `D`
- Fire: `Space`

### 6.2 Player 2

- Move up: `ArrowUp`
- Move down: `ArrowDown`
- Move left: `ArrowLeft`
- Move right: `ArrowRight`
- Fire: `Enter` or `RightShift`

### 6.3 Shared Controls

- Pause: `Esc`
- Restart current stage/round: `R`
- Return to mode select: `M`

Gamepad support is optional for a later milestone.

### 6.4 Input Priority

To mimic classic digital controls:

- Only one movement axis is active at a time.
- If multiple direction keys are held, use the most recently pressed direction.
- Fire is edge-triggered by default: one shot request per press.
- Holding fire may be supported later, but should still respect bullet limits.

## 7. Core Gameplay Rules

### 7.1 Movement

- Tanks move in four cardinal directions.
- Tanks do not move diagonally.
- A moving tank keeps its last facing direction.
- A tank can only enter passable tiles.
- Tanks collide with walls, water, base tiles, and other tanks.
- Tank movement should feel grid-aligned but not strictly turn-by-turn.
- Tanks are `16x16` virtual pixels.
- Tank movement should align to the `8x8` small-tile grid when turning into corridors.
- Turning while slightly off-grid may gently snap the tank to the nearest valid lane.
- Movement speed should be tuned in virtual pixels per fixed update, not window pixels.

Suggested speed targets:

- Player basic speed: roughly 1 virtual pixel per frame at 60 FPS.
- Fast enemy speed: higher than basic enemy, but still readable.
- Bullet speed: several times faster than tank speed.

### 7.2 Shooting

- Tanks fire bullets in their facing direction.
- Bullets travel in straight lines.
- Bullets are destroyed when they hit a solid tile, tank, base, or another bullet.
- Player bullet count is limited by tank upgrade level.
- Enemy bullet count is limited per enemy tank.
- Bullets spawn from the center-front of the firing tank.
- Bullets should be small sprites, visually closer to `4x4` or `8x8` than to a full tile.
- Bullet-vs-bullet collision should cancel both bullets.

Player upgrade behavior:

- Level 0: one bullet on screen.
- Level 1: faster bullet.
- Level 2: two bullets on screen.
- Level 3: bullets can destroy steel if that rule is enabled for the stage.

### 7.3 Damage

- Basic tanks are destroyed by one hit.
- Armored tanks require multiple hits.
- Player tanks lose one life when destroyed.
- Destroyed player tanks respawn with short invulnerability if lives remain.
- Bases are destroyed by a direct bullet hit.
- A destroyed tank plays an explosion animation before disappearing.
- A respawning player tank plays a spawn animation before control resumes.
- Invulnerable tanks should flicker or show a shield sprite.

### 7.4 Terrain

Tiles:

- `Empty`: passable, bullets pass through.
- `Brick`: blocks tanks and bullets; destructible.
- `Steel`: blocks tanks and bullets; indestructible by default.
- `Water`: blocks tanks; bullets pass through.
- `Forest`: tanks and bullets pass through; visually hides tanks underneath.
- `Ice`: passable; lightly modifies tank handling with a low-friction speed boost.
- `Base`: blocks tanks; destroyed by bullet impact.

### 7.5 Destructible Bricks

Use a 26x26 logical tile grid. This preserves the classic 13x13 stage shape while allowing each large cell to be split into smaller destructible pieces.

Brick destruction rule:

- A bullet destroys the brick sub-tile it hits.
- The logical map is updated first.
- Sprite entities are synchronized from the logical map.

The map data is authoritative. Sprites are visual output, not the source of collision truth.

### 7.6 Base Area

The base should occupy the classic bottom-center position in campaign mode.

Campaign base rules:

- The base itself is a `16x16` sprite occupying a `2x2` small-tile area.
- The base is normally surrounded by brick tiles.
- If the base is hit, switch to destroyed-base sprite and enter Game Over.
- Power-ups such as `Shovel` may temporarily replace surrounding bricks with steel.

## 8. Enemy And Stage Rules

### 8.1 Enemy Roster

Each campaign stage has a fixed enemy roster.

MVP roster size:

- 20 enemies per stage.

Classic active enemy rule:

- At most 4 enemies on screen at once.
- Enemies spawn from three top spawn points.
- Spawn should be blocked or delayed if another tank occupies the spawn area.

Enemy types:

- `Basic`: normal speed, normal bullet, one hit.
- `Fast`: faster movement, one hit.
- `Power`: stronger/faster bullet, one hit.
- `Armor`: multiple hit points.

### 8.2 Enemy Visual Feedback

- Some enemies may flash to indicate they carry a power-up reward.
- Armored enemies should visually change color or palette as health decreases.
- Spawn animation should make new enemies temporarily non-colliding or invulnerable until the animation completes.

### 8.3 Enemy AI Fidelity

AI should feel simple and old-school:

- It should be dangerous because of pressure and numbers, not because of perfect planning.
- Enemies should sometimes drift toward the base.
- Enemies should sometimes shoot immediately when aligned with the player or base.
- Randomness is acceptable and desirable.

## 9. Level And Arena Data

The game should be data-driven.

Solo campaign levels live in:

```text
assets/levels/001.level.ron
assets/levels/002.level.ron
assets/levels/003.level.ron
```

Versus arenas live in:

```text
assets/arenas/arena_01.ron
assets/arenas/arena_02.ron
```

### 9.1 Solo Level Definition

Example:

```ron
(
  name: "Stage 1",
  map: [
    "..........................",
    "..........................",
    "...BB..BB........BB..BB...",
    "...BB..BB........BB..BB...",
    "..........................",
    "WWWW....FFFF....WWWW......",
    "..........................",
    "...BB..BB........BB..BB...",
    "...BB..BB........BB..BB...",
    "............BB............",
    "..........BBBBBB..........",
    "..........BEEB............",
    "..........BBBB............",
    "..........................",
    "..........................",
    "...BB..BB........BB..BB...",
    "...BB..BB........BB..BB...",
    "..........................",
    "WWWW....FFFF....WWWW......",
    "..........................",
    "...BB..BB........BB..BB...",
    "...BB..BB........BB..BB...",
    "............BB............",
    "..........BBBBBB..........",
    "..........BEEB............",
    "..........BBBB............",
  ],
  player_spawn: (x: 8, y: 24, facing: Up),
  base_position: (x: 12, y: 24),
  enemy_spawns: [
    (x: 0, y: 0, facing: Down),
    (x: 12, y: 0, facing: Down),
    (x: 24, y: 0, facing: Down),
  ],
  enemies: [
    Basic, Basic, Basic, Fast, Basic,
    Power, Basic, Armor, Basic, Fast,
    Basic, Basic, Power, Armor, Fast,
    Basic, Basic, Basic, Armor, Power,
  ],
  powerup_carriers: [
    (enemy: 5, kind: Star),
    (enemy: 10, kind: Helmet),
    (enemy: 15, kind: Clock),
    (enemy: 20, kind: Grenade),
  ],
  spawn_interval_secs: 3.0,
  max_enemies_on_screen: 4,
)
```

### 9.2 Versus Arena Definition

Example:

```ron
(
  name: "Arena 1",
  map: [
    "..........................",
    "....BBBB..........BBBB....",
    "....BSSB..........BSSB....",
    "..........................",
    "........WWWWWWWW..........",
    "........WWWWWWWW..........",
    "..........................",
    "...FFFF............FFFF...",
    "...FFFF............FFFF...",
    "..........................",
    "....BBBB..........BBBB....",
    "....BSSB..........BSSB....",
    "..........................",
    "..........................",
    "....BBBB..........BBBB....",
    "....BSSB..........BSSB....",
    "..........................",
    "...FFFF............FFFF...",
    "...FFFF............FFFF...",
    "..........................",
    "........WWWWWWWW..........",
    "........WWWWWWWW..........",
    "..........................",
    "....BSSB..........BSSB....",
    "....BBBB..........BBBB....",
    "..........................",
  ],
  p1_spawn: (x: 4, y: 24, facing: Up),
  p2_spawn: (x: 21, y: 1, facing: Down),
  battle_rules: Deathmatch(
    target_score: 5,
    lives: 3,
    respawn_invulnerability_secs: 2.0,
  ),
  powerup_spawns: [
    (x: 12, y: 12),
  ],
)
```

For `BaseBattle`, the same arena schema is used, but `battle_rules` declares two
2x2 base positions that must cover `E` tiles in the map:

```ron
battle_rules: BaseBattle(
  p1_base: (x: 24, y: 24),
  p2_base: (x: 0, y: 0),
  lives: 3,
  respawn_invulnerability_secs: 2.0,
)
```

### 9.3 Tile Symbols

- `.` = Empty
- `B` = Brick
- `S` = Steel
- `W` = Water
- `F` = Forest
- `I` = Ice
- `E` = Base

### 9.4 Map Authoring Rules

- Campaign and arena maps must be exactly 26 rows.
- Each row must contain exactly 26 tile symbols.
- Coordinates are small-tile coordinates.
- `(0, 0)` is the top-left battlefield tile.
- Full-size tanks occupy a `2x2` small-tile footprint.
- Spawn points should be aligned to even coordinates when possible.

## 10. Sprites And Assets

Use pixel-art sprites through a texture atlas.

Sprite dimensions should follow FC-like sizes:

- Tank: `16x16`
- Base: `16x16`
- Large terrain cell: `16x16`, implemented as four `8x8` tiles where needed.
- Brick sub-tile: `8x8`
- Steel sub-tile: `8x8`
- Bullet: `4x4` or `8x8`
- Power-up: `16x16`
- Explosion: `16x16` or `32x32`

Initial asset groups:

- Player tank sprites: four directions, optional animation frames.
- Enemy tank sprites: basic, fast, power, armor.
- Bullet sprites: four directions.
- Terrain sprites: brick, steel, water, forest, ice.
- Base sprites: intact and destroyed.
- Effects: explosion, spawn shimmer, power-up sparkle.
- UI icons: lives, score, stage number.

Asset layout:

```text
assets/sprites/tanks.png
assets/sprites/terrain.png
assets/sprites/effects.png
assets/sprites/ui.png
assets/levels/
assets/arenas/
```

Use placeholder sprites first if final art is unavailable. Placeholders should still respect the final sprite dimensions and palette style, so gameplay tuning does not change when final art lands.

### 10.1 Asset Manifest

Use an explicit asset manifest instead of hard-coding every atlas index in gameplay systems.

Suggested path:

```text
assets/manifest.ron
```

Suggested contents:

- Texture paths
- Atlas tile size
- Sprite names
- Animation frame ranges
- Audio clip paths
- Palette metadata if needed

Current implementation note:

- `assets/manifest.ron` maps generated tank, terrain, power-up, effect, glyph, and sound entries to semantic names, frame ranges, or generated-asset parameters.
- Tank manifest entries include separate two-frame directional groups for P1, P2, Basic, Fast, Power, and Armor tanks.
- Terrain manifest entries include a two-frame water animation range plus static brick, steel, forest, and ice entries.
- Effect manifest entries include explosion, spawn shimmer, base destruction, and power-up sparkle frame ranges.
- Glyphs use generated placeholder pixels, but their atlas character order and tile dimensions live in `assets/manifest.ron`.
- Sounds use generated placeholder waveforms, but their retro sweep/noise/note definitions now live in `assets/manifest.ron`.
- `assets/arenas/arena_05.ron` is the first playable `BaseBattle` arena; destroying a player's base ends the round for the opponent.

### 10.2 Audio Style

Audio should be simple and era-appropriate:

- Short fire sound.
- Short brick-hit sound.
- Steel ricochet sound.
- Tank explosion sound.
- Base destroyed sound.
- Power-up pickup sound.
- Stage start and game over jingles.

No original Tank 1990/Battle City audio should be copied.

## 11. ECS Model

### 11.1 Resources

- `GameMode`: current mode and variant.
- `GamePhase`: mode select, loading, playing, paused, level clear, game over.
- `LevelIndex`: current campaign level.
- `TileGrid`: authoritative logical map.
- `EnemySpawnQueue`: remaining solo enemies.
- `ScoreBoard`: scores, lives, destroyed enemy counts.
- `AssetManifest`: handles and atlas layout metadata.
- `VirtualScreen`: base resolution and integer scale.
- `FixedStepClock`: gameplay tick accumulator if not using Bevy's fixed schedule directly.

### 11.2 Components

- `Tank`
- `PlayerControlled`
- `AiControlled`
- `Team`
- `Facing`
- `Velocity`
- `Bullet`
- `Tile`
- `Base`
- `PowerUp`
- `LevelScoped`
- `RoundScoped`
- `Health`
- `Invulnerability`
- `Explosion`
- `GridAligned`
- `SpriteAnimation`
- `CarriedPowerUp`

### 11.3 Teams

```rust
enum Team {
    Player1,
    Player2,
    Enemy,
    Neutral,
}
```

### 11.4 Controllers

```rust
enum TankController {
    Human(PlayerId),
    Ai,
}
```

Solo campaign:

- Player tank: `Human(Player1)` + `Team::Player1`
- Enemy tanks: `Ai` + `Team::Enemy`

Versus:

- P1 tank: `Human(Player1)` + `Team::Player1`
- P2 tank: `Human(Player2)` + `Team::Player2`

## 12. System Flow

### 12.1 App States

```text
Boot
  -> LoadingAssets
  -> ModeSelect
  -> LoadingLevel
  -> Playing
  -> Paused
  -> LevelClear
  -> GameOver
```

### 12.2 Level Load Flow

1. Despawn all entities with `LevelScoped`.
2. Load the selected level or arena definition.
3. Build `TileGrid`.
4. Spawn terrain sprites from the grid.
5. Spawn bases.
6. Spawn player tanks.
7. Initialize enemy queue or battle rules.
8. Enter `Playing`.

### 12.3 Gameplay Update Flow

1. Read player input.
2. Update AI intentions.
3. Resolve tank movement.
4. Spawn bullets from fire requests.
5. Move bullets.
6. Resolve bullet collisions.
7. Apply damage and tile destruction.
8. Spawn explosions and power-ups.
9. Update score and lives.
10. Check win/lose conditions.
11. Render sprites and UI.

### 12.4 Fixed Update

Gameplay logic should run at a fixed rate, ideally 60 updates per second.

Fixed-update systems:

- Input intent resolution
- AI intent resolution
- Tank movement
- Bullet movement
- Collision
- Damage
- Spawn timers
- Win/lose checks

Render-only systems:

- Sprite animation frame selection
- Flicker effects
- UI interpolation if needed

## 13. Enemy AI

MVP AI:

- Move in current direction until blocked or timer expires.
- Randomly choose a new direction when blocked.
- Prefer moving toward the player or base some of the time.
- Fire when aligned with the player or base.
- Fire randomly at a low rate otherwise.

Later AI improvements:

- Pathfinding toward the base.
- Difficulty profiles per level.
- Enemy coordination to lure player away from base.
- Distinct behavior per enemy type.

## 14. Power-Ups

Initial power-ups:

- `Star`: upgrade player firepower.
- `Helmet`: temporary shield.
- `Clock`: freeze enemies.
- `Grenade`: destroy all visible enemies.
- `Shovel`: temporarily reinforce base walls.
- `Tank`: extra life.

MVP can include only `Star` and `Helmet`; the rest can be added once core gameplay is stable.

Power-up spawn rule:

- Some enemies in the roster are marked as power-up carriers.
- Campaign level files mark carriers with 1-based enemy roster indexes.
- Destroying a carrier enemy spawns one power-up at a valid battlefield position.
- Only one power-up may be active at a time in campaign mode unless a stage explicitly overrides this.

## 15. Scoring

Solo scoring:

- Basic enemy: 100
- Fast enemy: 200
- Power enemy: 300
- Armor enemy: 400
- Stage clear bonus: optional later

Versus scoring:

- Destroy opponent tank: +1
- Destroy opponent base: immediate win in `BaseBattle`

## 16. Milestones

### Milestone 1: Bevy Window And Sprite Prototype

Acceptance:

- `cargo run` opens a Bevy window.
- The game renders into a `256x240` virtual canvas with integer scaling.
- Texture filtering is nearest-neighbor.
- A `26x26` tile map is rendered with NES-like placeholder sprites.
- The right-side status panel is visible.
- A player tank sprite can move in four directions.
- Tank movement is axis-aligned and does not rotate the sprite.

### Milestone 2: Collision And Bullets

Acceptance:

- Tank movement respects wall and water collisions.
- Player can fire bullets.
- Bullets destroy brick tiles.
- Bullets disappear on steel.
- Bullet-vs-bullet cancellation works.
- Brick destruction updates the logical tile grid before visuals.

### Milestone 3: Solo Campaign MVP

Acceptance:

- Load one `.level.ron` file.
- Spawn enemies from a roster.
- Enemy tanks move and fire.
- Player can destroy enemies.
- Base destruction causes Game Over.
- Destroying all enemies causes Level Clear.
- Max 4 enemies are active on screen.
- Enemy spawn points use top-left, top-center, and top-right positions.

### Milestone 4: Multi-Level Campaign

Acceptance:

- Load multiple level files.
- Stage clear advances to the next level.
- Stage number and lives are shown in UI.
- Missing or invalid level files produce a clear error.
- At least 3 authored stages are included.

### Milestone 5: Local Versus Deathmatch

Acceptance:

- Load one `.arena.ron` file.
- P1 and P2 can move and fire with separate controls.
- Players can destroy each other.
- Respawn and invulnerability work.
- Score target ends the round.

### Milestone 6: Polish

Acceptance:

- Add final or near-final pixel sprites.
- Add explosion animation.
- Add sound effects.
- Add pause/menu UI.
- Add more campaign stages and arenas.
- Tune speeds, timing, and sprites against the retro fidelity rules.

## 17. Testing Strategy

Unit-test pure logic where possible:

- Level file parsing.
- Tile symbol decoding.
- Passability and bullet collision rules.
- Enemy spawn queue behavior.
- Win/lose condition evaluation.
- Fixed-step movement snapping.
- Bullet-vs-tile hit positions.

Manual playtest checklist:

- Player cannot pass through walls, steel, water, base, or tanks.
- Bullets hit the expected tile.
- Brick destruction updates collision immediately.
- Enemy bullets can destroy the base.
- Level clear and game over do not leave old entities behind.
- Versus controls do not conflict.
- Window scaling is crisp at 2x, 3x, and 4x.
- Sprites never blur.
- Tanks do not drift off tile lanes after repeated turns.

## 18. Non-Goals For MVP

- Online multiplayer.
- NES/FC ROM emulation.
- Exact CPU/PPU timing behavior.
- Perfect original maps for every Tank 1990 variant.
- Full level editor.
- Gamepad support.
- WASM release.
- Advanced enemy pathfinding.
- Copied original commercial sprites, sounds, maps, or music.

## 19. Open Questions

- Should the campaign use 35 Battle City-style stages, 50 Tank 1990-style stages, or an original set?
- Should sprites be original art, generated pixel art, or references recreated from scratch?
- Should friendly fire exist in local versus?
- Should solo mode eventually support two-player co-op?
- Should the game include a built-in level editor after the core campaign works?
- Should the first playable build include the right-side panel immediately, or should it start with the battlefield only?
- Should P1 controls prioritize `WASD`, or should the first build use arrow keys to match common emulator muscle memory?
