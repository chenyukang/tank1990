# Original Campaign Level Layouts

This directory contains 35 campaign map layouts converted from the
`battle-city-tanks` Google Code archive:

- Google Code archive: https://code.google.com/archive/p/battle-city-tanks
- GitHub mirror used for inspection: https://github.com/ovidiubute/googlecode-battle-city-tanks
- Source files: `levels/1` through `levels/35`

The Google Code archive lists `battle-city-tanks` under GNU GPL v3. These files
therefore carry that source's GPLv3 licensing expectations.

Conversion notes:

- `#` -> `B` brick
- `@` -> `S` steel
- `~` -> `W` water
- `%` -> `F` forest
- `-` -> `I` ice
- `.` -> `.` empty

The original source stores the eagle/base separately from the level files. This
project writes the base into the converted maps at `(12, 24)` and preserves the
one-tile fortress wall shape used by the original game.
