#!/usr/bin/env bash
# Fails when tracked generated code differs from a fresh regeneration.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

"$root/scripts/regen.sh" "$tmp"
diff -ru "$root/crates/providers/cloudflare/src/generated" "$tmp/src/generated"
