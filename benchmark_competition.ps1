# Benchmark competition submission for 5 seconds
$exePath = "Primes\PrimeZeta\solution_1\prime_zeta.exe"
$duration = 5  # seconds
$startTime = Get-Date
$passes = 0

Write-Host "Benchmarking competition submission for $duration seconds..."
Write-Host "Starting at: $startTime"

do {
    $process = Start-Process -FilePath $exePath -NoNewWindow -PassThru -Wait
    $passes++
    
    # Quick check every 100 passes
    if ($passes % 100 -eq 0) {
        $elapsed = ((Get-Date) - $startTime).TotalSeconds
        Write-Host "  Pass $passes - Elapsed: $elapsed seconds"
    }
} while (((Get-Date) - $startTime).TotalSeconds -lt $duration)

$endTime = Get-Date
$totalTime = ($endTime - $startTime).TotalSeconds
$passesPerSecond = $passes / $totalTime

Write-Host "`n=== BENCHMARK RESULTS ==="
Write-Host "Total passes: $passes"
Write-Host "Total time: $totalTime seconds"
Write-Host "Passes per second: $passesPerSecond"
Write-Host "Expected competition output:"
Write-Host "zeta;$passes;$totalTime;4;algorithm=wheel;faithful=yes;bits=8;parallel=yes;simd=avx512"