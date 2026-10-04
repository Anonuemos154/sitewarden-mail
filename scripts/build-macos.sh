#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../apps/desktop"
npm ci
npm run tauri build
echo "Production distribution requires Developer ID signing and notarization."
