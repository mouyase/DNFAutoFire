# 代码格式化脚本
# 用法: .\scripts\format.ps1

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  DNF AutoFire - 代码格式化" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "格式化 Rust 代码..." -ForegroundColor Yellow
cargo fmt --all

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ 格式化完成！" -ForegroundColor Green
} else {
    Write-Host "❌ 格式化失败！" -ForegroundColor Red
    exit 1
}
