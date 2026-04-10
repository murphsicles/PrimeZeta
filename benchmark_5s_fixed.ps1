# 5-second benchmark script for Murphy's Sieve
# Updated to handle Zeta's exit code convention

Write-Host "=== MURPHY'S SIEVE 5-SECOND BENCHMARK ===" -ForegroundColor Cyan
Write-Host ""

# Test 1: Rust implementation baseline (stdout)
Write-Host "1. Testing Rust implementation (baseline)..." -ForegroundColor Yellow
$rustStart = Get-Date
$rustCount = 0
while (((Get-Date) - $rustStart).TotalSeconds -lt 5) {
    $null = .\murphy_rust_simple.exe
    $rustCount++
}
$rustTime = ((Get-Date) - $rustStart).TotalSeconds
$rustRate = [math]::Round($rustCount / $rustTime, 2)

Write-Host "   Rust passes in $rustTime seconds: $rustCount" -ForegroundColor Green
Write-Host "   Rust passes/second: $rustRate" -ForegroundColor Green
Write-Host ""

# Test 2: Zeta implementation (exit code)
Write-Host "2. Testing Zeta implementation (exit code)..." -ForegroundColor Yellow
$zetaStart = Get-Date
$zetaCount = 0
$zetaErrors = 0
while (((Get-Date) - $zetaStart).TotalSeconds -lt 5) {
    $result = .\simple_test_new.exe 2>&1
    if ($LASTEXITCODE -eq 42) {  # Expected return value
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
    Write-Host "   Errors: $zetaErrors" -ForegroundColor Gray
} else {
    Write-Host "   Zeta implementation failed to run ($zetaErrors errors)" -ForegroundColor Red
}
Write-Host ""

# Test 3: Create and test a minimal Murphy's Sieve in Zeta
Write-Host "3. Creating minimal Murphy's Sieve in Zeta..." -ForegroundColor Yellow

# Create a minimal Zeta sieve that actually computes something
$zetaCode = @'
// Minimal Murphy's Sieve in Zeta
// Computes primes up to 100 (not 1,000,000 for speed)
fn main() -> u64 {
    let limit = 100
    
    // Simple trial division to count primes
    let mut count = 0
    let mut n = 2
    
    while n <= limit {
        let mut is_prime = true
        let mut d = 2
        
        while d * d <= n {
            if n % d == 0 {
                is_prime = false
                break
            }
            d += 1
        }
        
        if is_prime {
            count += 1
        }
        n += 1
    }
    
    return count  // Should return 25 for limit=100
}
'@

$zetaCode | Out-File -FilePath "minimal_sieve.z" -Encoding UTF8

# Compile it
Write-Host "   Compiling minimal sieve..." -ForegroundColor Gray
.\zetac.exe minimal_sieve.z -o minimal_sieve.exe 2>&1 | Out-Null

if (Test-Path "minimal_sieve.exe") {
    Write-Host "   Testing minimal sieve..." -ForegroundColor Gray
    $minimalResult = .\minimal_sieve.exe
    $exitCode = $LASTEXITCODE
    Write-Host "   Result: $exitCode (expected: 25)" -ForegroundColor Gray
    
    # Benchmark the minimal sieve
    $minimalStart = Get-Date
    $minimalCount = 0
    while (((Get-Date) - $minimalStart).TotalSeconds -lt 5) {
        $null = .\minimal_sieve.exe
        $minimalCount++
    }
    $minimalTime = ((Get-Date) - $minimalStart).TotalSeconds
    $minimalRate = [math]::Round($minimalCount / $minimalTime, 2)
    
    Write-Host "   Minimal sieve passes in $minimalTime seconds: $minimalCount" -ForegroundColor Green
    Write-Host "   Minimal sieve passes/second: $minimalRate" -ForegroundColor Green
} else {
    Write-Host "   Failed to compile minimal sieve" -ForegroundColor Red
}
Write-Host ""

# Test 4: Simple infinite loop baseline (to measure overhead)
Write-Host "4. Testing simple infinite loop baseline..." -ForegroundColor Yellow
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
Write-Host "  - Algorithm: Full sieve up to 1,000,000" -ForegroundColor Gray
Write-Host ""
if ($zetaCount -gt 0) {
    Write-Host "Zeta simple test (return 42):" -ForegroundColor White
    Write-Host "  - Passes in 5s: $zetaCount" -ForegroundColor White
    Write-Host "  - Passes/sec: $zetaRate" -ForegroundColor White
    Write-Host "  - Performance vs Rust: " -NoNewline
    $ratio = [math]::Round(($zetaRate / $rustRate) * 100, 1)
    if ($ratio -lt 100) {
        Write-Host "$ratio% of Rust speed" -ForegroundColor Yellow
    } else {
        Write-Host "$ratio% of Rust speed" -ForegroundColor Green
    }
}
Write-Host ""
if (Test-Path "minimal_sieve.exe") {
    Write-Host "Zeta minimal sieve (trial division to 100):" -ForegroundColor White
    Write-Host "  - Passes in 5s: $minimalCount" -ForegroundColor White
    Write-Host "  - Passes/sec: $minimalRate" -ForegroundColor White
    Write-Host "  - Performance vs Rust: " -NoNewline
    $ratio2 = [math]::Round(($minimalRate / $rustRate) * 100, 1)
    if ($ratio2 -lt 100) {
        Write-Host "$ratio2% of Rust speed" -ForegroundColor Yellow
    } else {
        Write-Host "$ratio2% of Rust speed" -ForegroundColor Green
    }
    Write-Host "  - Note: Simpler algorithm (trial division vs full sieve)" -ForegroundColor Gray
}
Write-Host ""
Write-Host "Loop baseline:" -ForegroundColor White
Write-Host "  - Iterations in 5s: $loopCount" -ForegroundColor White
Write-Host "  - Iterations/sec: $loopRate" -ForegroundColor White
Write-Host ""
Write-Host "=== COMPETITION PREDICTION ===" -ForegroundColor Cyan
Write-Host ""
if (Test-Path "minimal_sieve.exe") {
    # Estimate for full Murphy's Sieve based on minimal implementation
    # The minimal sieve does trial division to 100 (25 primes)
    # Full sieve would need to go to 1,000,000 (78,498 primes)
    # This is about 10,000x more work (1,000,000/100 * more complex algorithm)
    $estimatedSlowdown = 10000  # Conservative estimate
    $estimatedRate = [math]::Round($minimalRate / $estimatedSlowdown, 2)
    $expectedPasses = [math]::Round($estimatedRate * 5, 0)
    
    Write-Host "Estimated performance for full Murphy's Sieve:" -ForegroundColor White
    Write-Host "  - Estimated passes/sec: $estimatedRate" -ForegroundColor Yellow
    Write-Host "  - Expected passes in 5s: $expectedPasses" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Performance ranking: " -NoNewline
    if ($estimatedRate -gt 10) {
        Write-Host "GOOD (competitive)" -ForegroundColor Green
    } elseif ($estimatedRate -gt 1) {
        Write-Host "ACCEPTABLE (mid-tier)" -ForegroundColor Yellow
    } else {
        Write-Host "POOR (needs optimization)" -ForegroundColor Red
    }
} else {
    Write-Host "Cannot estimate competition performance - Zeta implementation needs work" -ForegroundColor Red
}