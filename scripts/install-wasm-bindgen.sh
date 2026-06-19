#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

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

if command -v wasm-bindgen >/dev/null 2>&1; then
  installed_version="$(wasm-bindgen --version | awk '{ print $2 }')"
  if [[ "$installed_version" == "$wasm_bindgen_version" ]]; then
    echo "wasm-bindgen $wasm_bindgen_version is already installed"
    exit 0
  fi
fi

cargo install wasm-bindgen-cli --version "$wasm_bindgen_version" --locked
