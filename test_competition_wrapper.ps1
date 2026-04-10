# Test the competition infinite loop wrapper
$exePath = "competition_infinite.exe"
$duration = 2  # seconds (shorter for testing)

Write-Host "Testing competition wrapper for $duration seconds..."
Write-Host "This simulates the competition harness counting passes"

$startTime = Get-Date
$passes = 0

do {
    $process = Start-Process -FilePath $exePath -NoNewWindow -PassThru -Wait
    $passes++
    
    # Show progress
    $elapsed = ((Get-Date) - $startTime).TotalSeconds
    if ($passes % 1000 -eq 0) {
        Write-Host "  Pass $passes - Elapsed: $elapsed seconds"
    }
} while ($elapsed -lt $duration)

$endTime = Get-Date
$totalTime = ($endTime - $startTime).TotalSeconds
$passesPerSecond = $passes / $totalTime

Write-Host "`n=== TEST RESULTS ==="
Write-Host "Total passes: $passes"
Write-Host "Total time: $totalTime seconds"
Write-Host "Passes per second: $passesPerSecond"
Write-Host "`nExpected competition output:"
Write-Host "zeta;$passes;$totalTime;1;algorithm=wrapper;faithful=no;bits=0;parallel=no"