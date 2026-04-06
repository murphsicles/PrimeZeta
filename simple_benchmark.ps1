# Simple benchmark runner for Zeta
# Measures actual compilation and execution time

$benchmarkFile = "test_simple.z"
$iterations = 5

Write-Host "Running REAL Zeta benchmark: $benchmarkFile"
Write-Host "Iterations: $iterations"
Write-Host ""

$times = @()
$successful = 0

for ($i = 1; $i -le $iterations; $i++) {
    Write-Host "Iteration $i..."
    
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    $output = cargo run --bin zetac -- $benchmarkFile 2>&1
    $stopwatch.Stop()
    
    $resultLine = $output | Select-String -Pattern '^Result: (\d+)$'
    
    if ($resultLine) {
        $result = [int]$resultLine.Matches.Groups[1].Value
        $time = $stopwatch.Elapsed.TotalMilliseconds
        $times += $time
        $successful++
        
        Write-Host "  Result: $result, Time: ${time}ms"
    } else {
        Write-Host "  Failed - no result found"
    }
    
    # Small delay
    if ($i -lt $iterations) {
        Start-Sleep -Milliseconds 200
    }
}

Write-Host ""
Write-Host "=== BENCHMARK RESULTS ==="
Write-Host "Successful runs: $successful/$iterations"

if ($times.Count -gt 0) {
    $min = ($times | Measure-Object -Minimum).Minimum
    $max = ($times | Measure-Object -Maximum).Maximum
    $avg = ($times | Measure-Object -Average).Average
    $total = ($times | Measure-Object -Sum).Sum
    
    Write-Host "Min time: {0:F2}ms" -f $min
    Write-Host "Max time: {0:F2}ms" -f $max
    Write-Host "Avg time: {0:F2}ms" -f $avg
    Write-Host "Total time: {0:F2}ms" -f $total
    Write-Host "Throughput: {0:F2} runs/sec" -f (1000 / $avg)
    
    # Save results
    $results = @{
        timestamp = Get-Date -Format "yyyy-MM-ddTHH:mm:ss"
        benchmark = $benchmarkFile
        iterations = $iterations
        successful = $successful
        min_time_ms = $min
        max_time_ms = $max
        avg_time_ms = $avg
        total_time_ms = $total
        throughput_rps = 1000 / $avg
    }
    
    $results | ConvertTo-Json | Out-File -FilePath "real_benchmark_simple.json" -Encoding UTF8
    Write-Host "Results saved to: real_benchmark_simple.json"
}