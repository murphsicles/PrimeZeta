# Simple test script for competition package

Write-Host "=== Testing Competition Package ==="
Write-Host ""

# Check required files
Write-Host "1. Checking required files:"
$files = @("src/prime.z", "README.md", "Dockerfile", "run.sh")
$all_ok = $true

foreach ($f in $files) {
    if (Test-Path $f) {
        Write-Host "  OK: $f"
    } else {
        Write-Host "  MISSING: $f"
        $all_ok = $false
    }
}

if (-not $all_ok) {
    Write-Host "ERROR: Missing files!"
    exit 1
}

Write-Host ""
Write-Host "2. Checking README:"
$readme = Get-Content README.md -Raw
if ($readme -match "Bits per candidate: 8") {
    Write-Host "  OK: Bits=8 in README"
} else {
    Write-Host "  ERROR: Bits=8 not in README"
    $all_ok = $false
}

if ($readme -match "algorithm=wheel, faithful=yes, bits=8, parallel=no") {
    Write-Host "  OK: All tags in README"
} else {
    Write-Host "  ERROR: Tags incorrect in README"
    $all_ok = $false
}

Write-Host ""
Write-Host "3. Checking Dockerfile:"
$docker = Get-Content Dockerfile -Raw
if ($docker -match "COPY src/prime.z") {
    Write-Host "  OK: Dockerfile copies prime.z"
} else {
    Write-Host "  ERROR: Dockerfile doesn't copy prime.z"
    $all_ok = $false
}

if ($docker -match "ENTRYPOINT") {
    Write-Host "  OK: Dockerfile has ENTRYPOINT"
} else {
    Write-Host "  ERROR: Dockerfile missing ENTRYPOINT"
    $all_ok = $false
}

Write-Host ""
Write-Host "4. Checking prime.z:"
$prime = Get-Content src/prime.z -Raw
if ($prime -match "while 1 == 1") {
    Write-Host "  OK: prime.z has infinite loop"
} else {
    Write-Host "  ERROR: prime.z missing infinite loop"
    $all_ok = $false
}

if ($prime -match "println\(count\)") {
    Write-Host "  OK: prime.z prints count"
} else {
    Write-Host "  ERROR: prime.z doesn't print count"
    $all_ok = $false
}

Write-Host ""
Write-Host "=== Test Complete ==="
Write-Host ""

if ($all_ok) {
    Write-Host "SUCCESS: Package is ready for submission!" -ForegroundColor Green
} else {
    Write-Host "FAILURE: Package has issues!" -ForegroundColor Red
}