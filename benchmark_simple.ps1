# Simple benchmark for working Zeta implementation
Write-Host "=== SIMPLE BENCHMARK ===" -ForegroundColor Cyan
Write-Host ""

# Test 1: Rust baseline
Write-Host "1. Testing Rust baseline..." -ForegroundColor Yellow
$rustStart = Get-Date
$rustCount = 0
while (((Get-Date) - $rustStart).TotalSeconds -lt 2) {
    .\murphy_rust_simple.exe > $null
    $rustCount++
}
$rustTime = ((Get-Date) - $rustStart).TotalSeconds
$rustRate = [math]::Round($rustCount / $rustTime, 2)

Write-Host "   Rust passes in $rustTime seconds: $rustCount" -ForegroundColor Green
Write-Host "   Rust passes/second: $rustRate" -ForegroundColor Green
Write-Host ""

# Test 2: Zeta simple return
Write-Host "2. Testing Zeta simple return..." -ForegroundColor Yellow
$zetaStart = Get-Date
$zetaCount = 0
while (((Get-Date) - $zetaStart).TotalSeconds -lt 2) {
    .\test_return_only.exe > $null
    $zetaCount++
}
$zetaTime = ((Get-Date) - $rustStart).TotalSeconds
$zetaRate = [math]::Round($zetaCount / $zetaTime, 2)

Write-Host "   Zeta passes in $zetaTime seconds: $zetaCount" -ForegroundColor Green
Write-Host "   Zeta passes/second: $zetaRate" -ForegroundColor Green
Write-Host ""

# Test 3: Zeta while true return
Write-Host "3. Testing Zeta while true return..." -ForegroundColor Yellow
$zetaWhileStart = Get-Date
$zetaWhileCount = 0
while (((Get-Date) - $zetaWhileStart).TotalSeconds -lt 2) {
    .\test_while_true.exe > $null
    $zetaWhileCount++
}
$zetaWhileTime = ((Get-Date) - $zetaWhileStart).TotalSeconds
$zetaWhileRate = [math]::Round($zetaWhileCount / $zetaWhileTime, 2)

Write-Host "   Zeta while true passes in $zetaWhileTime seconds: $zetaWhileCount" -ForegroundColor Green
Write-Host "   Zeta while true passes/second: $zetaWhileRate" -ForegroundColor Green
Write-Host ""

# Summary
Write-Host "=== SUMMARY ===" -ForegroundColor Cyan
Write-Host "Rust baseline: $rustRate passes/sec" -ForegroundColor White
Write-Host "Zeta simple return: $zetaRate passes/sec" -ForegroundColor White
Write-Host "Zeta while true: $zetaWhileRate passes/sec" -ForegroundColor White
Write-Host ""
Write-Host "Performance ratio (Zeta/Rust): " -NoNewline
$ratio = [math]::Round(($zetaRate / $rustRate) * 100, 1)
Write-Host "$ratio%" -ForegroundColor Green