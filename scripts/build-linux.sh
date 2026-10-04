#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../apps/desktop"
npm ci
npm run tauri build
echo "Sign checksums/SBOM before publishing official artifacts."
