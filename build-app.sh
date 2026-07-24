#!/bin/bash
set -e

echo "================================================="
echo "   🚀 Building CodexManager Standalone Release"
echo "================================================="

# 1. Install & Build Frontend
npm install
npm run build

# 2. Compile Tauri Native Binary & Bundle Packages
npx tauri build

echo "================================================="
echo "🎉 Build Complete! Release packages are saved in:"
echo "   src-tauri/target/release/bundle/"
echo "================================================="
