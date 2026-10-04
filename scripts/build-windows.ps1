$ErrorActionPreference = "Stop"

Write-Host "Foundation build script"
Write-Host "Prerequisites: Rust stable, Node LTS, Tauri system dependencies, signing identity."

Push-Location "$PSScriptRoot\..\apps\desktop"
npm ci
npm run tauri build
Pop-Location

Write-Host "Do not publish unsigned artifacts as official SiteWarden Mail releases."
