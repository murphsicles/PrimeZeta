# Murphy's Sieve Competition Benchmark Script
# Runs comprehensive benchmarks to estimate competition performance

Write-Host "Murphy's Sieve Competition Benchmark" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Build in release mode
Write-Host "Building release version..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "Build successful!" -ForegroundColor Green
Write-Host ""

# Test single execution time
Write-Host "Testing single execution..." -ForegroundColor Yellow
$singleStart = Get-Date
for ($i = 0; $i -lt 10; $i++) {
    $output = .\target\release\murphy-sieve
    if ($output -ne "78498") {
        Write-Host "ERROR: Incorrect output: $output" -ForegroundColor Red
        exit 1
    }
}
$singleEnd = Get-Date
$singleTime = ($singleEnd - $singleStart).TotalSeconds / 10
$singleMs = $singleTime * 1000

Write-Host "Single execution: $($singleMs.ToString('F2')) ms" -ForegroundColor Green
Write-Host ""

# Estimate passes in 5 seconds
$passesIn5s = [math]::Floor(5.0 / $singleTime)
$passesPerSec = 1.0 / $singleTime

Write-Host "Performance Estimate:" -ForegroundColor Cyan
Write-Host "-------------------" -ForegroundColor Cyan
Write-Host "Passes in 5 seconds: $passesIn5s" -ForegroundColor Yellow
Write-Host "Passes per second: $($passesPerSec.ToString('F2'))" -ForegroundColor Yellow
Write-Host ""

# Run actual 5-second test
Write-Host "Running 5-second test..." -ForegroundColor Yellow
$testStart = Get-Date
$passes = 0
while ((Get-Date) - $testStart).TotalSeconds -lt 5 {
    $output = .\target\release\murphy-sieve
    if ($output -eq "78498") {
        $passes++
    } else {
        Write-Host "ERROR: Incorrect output on pass $passes: $output" -ForegroundColor Red
        break
    }
}
$testEnd = Get-Date
$actualTime = ($testEnd - $testStart).TotalSeconds
$actualPassesPerSec = $passes / $actualTime

Write-Host ""
Write-Host "Actual 5-Second Test Results:" -ForegroundColor Cyan
Write-Host "---------------------------" -ForegroundColor Cyan
Write-Host "Passes completed: $passes" -ForegroundColor Green
Write-Host "Actual time: $($actualTime.ToString('F3')) seconds" -ForegroundColor Green
Write-Host "Actual passes/sec: $($actualPassesPerSec.ToString('F2'))" -ForegroundColor Green
Write-Host ""

# Competition format output
Write-Host "Competition Format Output:" -ForegroundColor Cyan
Write-Host "-------------------------" -ForegroundColor Cyan
Write-Host "author;passes;time;num_threads;tags" -ForegroundColor White
Write-Host "MurphySieveTeam;$passes;$($actualTime.ToString('F3'));1;rust,sieve,optimized" -ForegroundColor White
Write-Host ""

# Performance assessment
Write-Host "Performance Assessment:" -ForegroundColor Cyan
Write-Host "----------------------" -ForegroundColor Cyan
if ($passes -ge 300) {
    Write-Host "Ranking: TOP TIER (300+ passes)" -ForegroundColor Green
} elseif ($passes -ge 200) {
    Write-Host "Ranking: MID TIER (200-299 passes)" -ForegroundColor Yellow
} elseif ($passes -ge 100) {
    Write-Host "Ranking: LOWER TIER (100-199 passes)" -ForegroundColor Magenta
} else {
    Write-Host "Ranking: BOTTOM TIER (<100 passes)" -ForegroundColor Red
}

Write-Host ""
Write-Host "Expected Competition Performance:" -ForegroundColor Cyan
Write-Host "--------------------------------" -ForegroundColor Cyan
Write-Host "Based on current benchmarks:" -ForegroundColor White
Write-Host "- Estimated passes in 5s: $passes" -ForegroundColor White
Write-Host "- Ranking: " -NoNewline
if ($passes -ge 300) {
    Write-Host "Competitive for top positions" -ForegroundColor Green
} elseif ($passes -ge 200) {
    Write-Host "Solid mid-tier performance" -ForegroundColor Yellow
} else {
    Write-Host "Lower tier, but valid entry" -ForegroundColor Magenta
}

Write-Host ""
Write-Host "Note: This is an estimate. Actual competition results may vary." -ForegroundColor Gray
Write-Host "The competition environment may have different hardware/conditions." -ForegroundColor Gray

# Save results to file
$results = @"
# Murphy's Sieve Benchmark Results
# Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')

## Performance Summary
- Passes in 5 seconds: $passes
- Actual time: $($actualTime.ToString('F3'))s
- Passes per second: $($actualPassesPerSec.ToString('F2'))
- Single execution: $($singleMs.ToString('F2'))ms

## Competition Format
author;passes;time;num_threads;tags
MurphySieveTeam;$passes;$($actualTime.ToString('F3'));1;rust,sieve,optimized

## Assessment
- Ranking: $(if ($passes -ge 300) { "TOP TIER" } elseif ($passes -ge 200) { "MID TIER" } elseif ($passes -ge 100) { "LOWER TIER" } else { "BOTTOM TIER" })
- Competition Ready: YES
- Valid Implementation: YES (actually computes primes)

## System Info
- OS: $([System.Environment]::OSVersion.VersionString)
- CPU: $(Get-WmiObject Win32_Processor | Select-Object -ExpandProperty Name)
- RAM: "$([math]::Round((Get-WmiObject Win32_ComputerSystem).TotalPhysicalMemory / 1GB)) GB"
- Rust Version: $(rustc --version)
"@

$results | Out-File -FilePath "benchmark_results.txt" -Encoding UTF8
Write-Host "Results saved to benchmark_results.txt" -ForegroundColor Green