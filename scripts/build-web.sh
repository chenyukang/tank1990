#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-"$ROOT_DIR/dist/web"}"
TARGET="wasm32-unknown-unknown"
WASM_INPUT="$ROOT_DIR/target/$TARGET/release/tank.wasm"

wasm_bindgen_version="$(
  awk '
    $0 == "[[package]]" { found = 0 }
    $0 == "name = \"wasm-bindgen\"" { found = 1 }
    found && /^version = / {
      gsub(/"/, "", $3)
      print $3
      exit
    }
  ' "$ROOT_DIR/Cargo.lock"
)"

if [[ -z "$wasm_bindgen_version" ]]; then
  echo "failed to find wasm-bindgen version in Cargo.lock" >&2
  exit 1
fi

if ! rustup target list --installed | grep -qx "$TARGET"; then
  echo "missing Rust target: $TARGET" >&2
  echo "install it with: rustup target add $TARGET" >&2
  exit 1
fi

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  echo "missing wasm-bindgen $wasm_bindgen_version" >&2
  echo "install it with: cargo install wasm-bindgen-cli --version $wasm_bindgen_version --locked" >&2
  exit 1
fi

installed_wasm_bindgen_version="$(wasm-bindgen --version | awk '{ print $2 }')"
if [[ "$installed_wasm_bindgen_version" != "$wasm_bindgen_version" ]]; then
  echo "wasm-bindgen version mismatch: found $installed_wasm_bindgen_version, need $wasm_bindgen_version" >&2
  echo "install the matching version with: cargo install wasm-bindgen-cli --version $wasm_bindgen_version --locked" >&2
  exit 1
fi

cd "$ROOT_DIR"
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS="${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:-1}"
export CARGO_PROFILE_RELEASE_LTO="${CARGO_PROFILE_RELEASE_LTO:-thin}"
export CARGO_PROFILE_RELEASE_OPT_LEVEL="${CARGO_PROFILE_RELEASE_OPT_LEVEL:-z}"
export CARGO_PROFILE_RELEASE_PANIC="${CARGO_PROFILE_RELEASE_PANIC:-abort}"
export CARGO_PROFILE_RELEASE_STRIP="${CARGO_PROFILE_RELEASE_STRIP:-symbols}"
cargo build --locked --release --target "$TARGET"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR/pkg" "$OUT_DIR/assets"

wasm-bindgen \
  --target web \
  --no-typescript \
  --out-dir "$OUT_DIR/pkg" \
  "$WASM_INPUT"

if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt \
    -Oz \
    --enable-bulk-memory \
    --enable-sign-ext \
    -o "$OUT_DIR/pkg/tank_bg.wasm.optimized" \
    "$OUT_DIR/pkg/tank_bg.wasm"
  mv "$OUT_DIR/pkg/tank_bg.wasm.optimized" "$OUT_DIR/pkg/tank_bg.wasm"
else
  echo "warning: wasm-opt not found; skipping wasm size optimization" >&2
fi

cp "$ROOT_DIR/web/index.html" "$OUT_DIR/index.html"
cp "$ROOT_DIR/assets/manifest.ron" "$OUT_DIR/assets/manifest.ron"
cp -R "$ROOT_DIR/assets/arenas" "$OUT_DIR/assets/arenas"
cp -R "$ROOT_DIR/assets/levels" "$OUT_DIR/assets/levels"
cp -R "$ROOT_DIR/assets/levels_original" "$OUT_DIR/assets/levels_original"
touch "$OUT_DIR/.nojekyll"

echo "web build written to $OUT_DIR"
