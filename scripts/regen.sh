#!/usr/bin/env bash
# Regenerates the Cloudflare provider from OpenAPI + overlays.
# Usage: scripts/regen.sh [output-dir]   (default: crates/providers/cloudflare)
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
output="${1:-$root/crates/providers/cloudflare}"

cargo run --quiet --manifest-path "$root/Cargo.toml" -p koof -- \
  --openapi "$root/openapi/cloudflare.yaml" \
  --overlay "$root/openapi/cloudflare.account.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.zone.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.dns-record.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.ruleset.overlay.yaml" \
  --overlay "$root/openapi/cloudflare.pages-project.overlay.yaml" \
  --output "$output"

rustfmt --edition 2024 "$output"/src/generated/*.rs
