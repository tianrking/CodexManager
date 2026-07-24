Write-Host "=================================================" -ForegroundColor Cyan
Write-Host "   🚀 Building CodexManager Standalone Release" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

npm install
npm run build
npx tauri build

Write-Host "=================================================" -ForegroundColor Green
Write-Host "🎉 Build Complete! Release packages are saved in:" -ForegroundColor Green
Write-Host "   src-tauri/target/release/bundle/" -ForegroundColor Green
Write-Host "=================================================" -ForegroundColor Green
