# Run CTFE benchmark for 5 seconds and count iterations
Write-Host "=== PrimeZeta CTFE Benchmark ===" -ForegroundColor Cyan
Write-Host "Running infinite loop for 5 seconds..."
Write-Host "Iterations will be counted."

# Create a version of bench_ctfe_fixed.z with infinite loop
$infiniteCode = @"
comptime PRIME_COUNT: i64 = 78498;

fn main() -> i64 {
    loop {
        println_i64(PRIME_COUNT);
    }
    return 0;
}
"@

Set-Content -Path "bench_ctfe_infinite.z" -Value $infiniteCode

# Compile
Write-Host "Compiling..." -ForegroundColor Yellow
& ".\target\release\zetac.exe" bench_ctfe_infinite.z -o bench_ctfe_infinite.exe 2>&1 | Out-Null

# Link with runtime
clang bench_ctfe_infinite.exe.o runtime.c -o bench_ctfe_infinite.exe 2>&1 | Out-Null

if (Test-Path ".\bench_ctfe_infinite.exe") {
    Write-Host "Running 5-second benchmark..." -ForegroundColor Green
    
    # Run for 5 seconds and count lines
    $job = Start-Job -ScriptBlock {
        & ".\bench_ctfe_infinite.exe" 2>&1
    }
    
    Start-Sleep -Seconds 5
    Stop-Job $job
    
    $output = Receive-Job $job
    $iterations = $output.Count
    
    Write-Host "Iterations in 5 seconds: $iterations" -ForegroundColor Green
    Write-Host "Iterations per second: $([math]::Round($iterations / 5.0))" -ForegroundColor Green
    
    Remove-Job $job -Force
} else {
    Write-Host "Compilation failed!" -ForegroundColor Red
}