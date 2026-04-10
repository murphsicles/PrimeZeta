# 5-second benchmark script for Murphy's Sieve
# Counts passes in 5 seconds for competition prediction

Write-Host "=== MURPHY'S SIEVE 5-SECOND BENCHMARK ===" -ForegroundColor Cyan
Write-Host ""

# Test 1: Rust implementation baseline
Write-Host "1. Testing Rust implementation (baseline)..." -ForegroundColor Yellow
$rustStart = Get-Date
$rustCount = 0
while (((Get-Date) - $rustStart).TotalSeconds -lt 5) {
    .\murphy_rust_simple.exe > $null
    $rustCount++
}
$rustTime = ((Get-Date) - $rustStart).TotalSeconds
$rustRate = [math]::Round($rustCount / $rustTime, 2)

Write-Host "   Rust passes in $rustTime seconds: $rustCount" -ForegroundColor Green
Write-Host "   Rust passes/second: $rustRate" -ForegroundColor Green
Write-Host ""

# Test 2: Zeta implementation (if it works)
Write-Host "2. Testing Zeta implementation..." -ForegroundColor Yellow
$zetaStart = Get-Date
$zetaCount = 0
$zetaErrors = 0
while (((Get-Date) - $zetaStart).TotalSeconds -lt 5) {
    $result = .\murphy_sieve_final_working.exe 2>&1
    if ($LASTEXITCODE -eq 0) {
        $zetaCount++
    } else {
        $zetaErrors++
    }
}
$zetaTime = ((Get-Date) - $zetaStart).TotalSeconds
if ($zetaCount -gt 0) {
    $zetaRate = [math]::Round($zetaCount / $zetaTime, 2)
    Write-Host "   Zeta passes in $zetaTime seconds: $zetaCount" -ForegroundColor Green
    Write-Host "   Zeta passes/second: $zetaRate" -ForegroundColor Green
} else {
    Write-Host "   Zeta implementation failed to run ($zetaErrors errors)" -ForegroundColor Red
}
Write-Host ""

# Test 3: Simple infinite loop baseline (to measure overhead)
Write-Host "3. Testing simple infinite loop baseline..." -ForegroundColor Yellow
$loopStart = Get-Date
$loopCount = 0
while (((Get-Date) - $loopStart).TotalSeconds -lt 5) {
    $loopCount++
}
$loopTime = ((Get-Date) - $loopStart).TotalSeconds
$loopRate = [math]::Round($loopCount / $loopTime, 0)

Write-Host "   Loop iterations in $loopTime seconds: $loopCount" -ForegroundColor Green
Write-Host "   Loop iterations/second: $loopRate" -ForegroundColor Green
Write-Host ""

# Summary
Write-Host "=== BENCHMARK SUMMARY ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Rust implementation (baseline):" -ForegroundColor White
Write-Host "  - Passes in 5s: $rustCount" -ForegroundColor White
Write-Host "  - Passes/sec: $rustRate" -ForegroundColor White
Write-Host ""
if ($zetaCount -gt 0) {
    Write-Host "Zeta implementation:" -ForegroundColor White
    Write-Host "  - Passes in 5s: $zetaCount" -ForegroundColor White
    Write-Host "  - Passes/sec: $zetaRate" -ForegroundColor White
    Write-Host "  - Performance vs Rust: " -NoNewline
    $ratio = [math]::Round(($zetaRate / $rustRate) * 100, 1)
    if ($ratio -lt 100) {
        Write-Host "$ratio% of Rust speed" -ForegroundColor Yellow
    } else {
        Write-Host "$ratio% of Rust speed" -ForegroundColor Green
    }
} else {
    Write-Host "Zeta implementation: FAILED" -ForegroundColor Red
}
Write-Host ""
Write-Host "Loop baseline:" -ForegroundColor White
Write-Host "  - Iterations in 5s: $loopCount" -ForegroundColor White
Write-Host "  - Iterations/sec: $loopRate" -ForegroundColor White
Write-Host ""
Write-Host "=== COMPETITION PREDICTION ===" -ForegroundColor Cyan
Write-Host ""
if ($zetaCount -gt 0) {
    $expectedPasses = [math]::Round($zetaRate * 5, 0)
    Write-Host "Expected passes in competition (5s): $expectedPasses" -ForegroundColor Green
    Write-Host "Performance ranking: " -NoNewline
    if ($zetaRate -gt 1000) {
        Write-Host "EXCELLENT (top tier)" -ForegroundColor Green
    } elseif ($zetaRate -gt 500) {
        Write-Host "GOOD (competitive)" -ForegroundColor Yellow
    } elseif ($zetaRate -gt 100) {
        Write-Host "ACCEPTABLE (mid-tier)" -ForegroundColor Magenta
    } else {
        Write-Host "POOR (needs optimization)" -ForegroundColor Red
    }
} else {
    Write-Host "Zeta implementation needs fixing before competition" -ForegroundColor Red
}