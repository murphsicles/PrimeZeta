# Quick benchmark for competition verification
Write-Host "=== QUICK COMPETITION BENCHMARK ===" -ForegroundColor Cyan
Write-Host "Testing for 2 seconds..." -ForegroundColor Yellow

# Test the most recent Murphy implementation
$exe = ".\test_murphy_minimal.exe"
if (Test-Path $exe) {
    $start = Get-Date
    $count = 0
    while (((Get-Date) - $start).TotalSeconds -lt 2) {
        & $exe > $null
        $count++
    }
    $time = ((Get-Date) - $start).TotalSeconds
    $rate = [math]::Round($count / $time, 2)
    
    Write-Host "Passes in $time seconds: $count" -ForegroundColor Green
    Write-Host "Passes/second: $rate" -ForegroundColor Green
    Write-Host "Estimated passes in 5 seconds: $([math]::Round($rate * 5))" -ForegroundColor Green
} else {
    Write-Host "ERROR: $exe not found" -ForegroundColor Red
}

Write-Host ""
Write-Host "Testing FINAL_SUBMISSION.exe..." -ForegroundColor Yellow
$exe2 = ".\FINAL_SUBMISSION.exe"
if (Test-Path $exe2) {
    $start = Get-Date
    $count = 0
    while (((Get-Date) - $start).TotalSeconds -lt 2) {
        & $exe2 > $null
        $count++
    }
    $time = ((Get-Date) - $start).TotalSeconds
    $rate = [math]::Round($count / $time, 2)
    
    Write-Host "Passes in $time seconds: $count" -ForegroundColor Green
    Write-Host "Passes/second: $rate" -ForegroundColor Green
    Write-Host "Estimated passes in 5 seconds: $([math]::Round($rate * 5))" -ForegroundColor Green
} else {
    Write-Host "ERROR: $exe2 not found" -ForegroundColor Red
}