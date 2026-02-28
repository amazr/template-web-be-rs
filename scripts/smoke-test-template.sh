#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT_NAME="${1:-template-smoke-app}"
GITHUB_OWNER="${GITHUB_OWNER:-template-owner}"

if ! command -v cargo-generate >/dev/null 2>&1; then
  echo "cargo-generate is required. Install with: cargo install cargo-generate"
  exit 1
fi

TMP_DIR="$(mktemp -d -t template-smoke.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Generating project '$PROJECT_NAME' in $TMP_DIR"
cargo generate \
  --path "$ROOT_DIR" \
  --name "$PROJECT_NAME" \
  --destination "$TMP_DIR" \
  --define "github_owner=$GITHUB_OWNER" \
  --silent \
  --force

GENERATED_DIR="$TMP_DIR/$PROJECT_NAME"

echo "Running cargo check in $GENERATED_DIR"
cargo check --manifest-path "$GENERATED_DIR/Cargo.toml"

echo "Template smoke test passed"
