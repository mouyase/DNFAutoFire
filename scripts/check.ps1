# 代码质量检查脚本
# 用法: .\scripts\check.ps1

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "  DNF AutoFire - 代码质量检查" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# 1. 格式化检查
Write-Host "[1/4] 检查代码格式..." -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 格式化检查失败！运行 'cargo fmt' 修复" -ForegroundColor Red
    exit 1
}
Write-Host "✅ 格式化检查通过" -ForegroundColor Green
Write-Host ""

# 2. Clippy 检查
Write-Host "[2/4] 运行 Clippy 检查..." -ForegroundColor Yellow
cargo clippy --all-targets --all-features -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Clippy 检查失败！" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Clippy 检查通过" -ForegroundColor Green
Write-Host ""

# 3. 测试
Write-Host "[3/4] 运行测试..." -ForegroundColor Yellow
cargo test --all
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 测试失败！" -ForegroundColor Red
    exit 1
}
Write-Host "✅ 测试通过" -ForegroundColor Green
Write-Host ""

# 4. 编译检查
Write-Host "[4/4] 编译检查..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ 编译失败！" -ForegroundColor Red
    exit 1
}
Write-Host "✅ 编译成功" -ForegroundColor Green
Write-Host ""

Write-Host "=====================================" -ForegroundColor Green
Write-Host "  ✅ 所有检查通过！" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green
