# Build Script for Murphy's Sieve Competition Submission

Write-Host "Building Murphy's Sieve Competition Submission..." -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Check for Rust installation
Write-Host "Checking Rust installation..." -ForegroundColor Yellow
$rustcVersion = rustc --version 2>$null
if (-not $rustcVersion) {
    Write-Host "ERROR: Rust not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "Rust found: $rustcVersion" -ForegroundColor Green

$cargoVersion = cargo --version 2>$null
if (-not $cargoVersion) {
    Write-Host "ERROR: Cargo not found. Please install Rust with Cargo." -ForegroundColor Red
    exit 1
}
Write-Host "Cargo found: $cargoVersion" -ForegroundColor Green
Write-Host ""

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean
Write-Host "Clean complete." -ForegroundColor Green
Write-Host ""

# Build debug version
Write-Host "Building debug version..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Debug build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Debug build successful!" -ForegroundColor Green
Write-Host ""

# Test debug version
Write-Host "Testing debug version..." -ForegroundColor Yellow
$debugOutput = .\target\debug\murphy-sieve 2>&1
if ($debugOutput -eq "78498") {
    Write-Host "Debug test passed: $debugOutput" -ForegroundColor Green
} else {
    Write-Host "Debug test failed. Output: $debugOutput" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Build release version (optimized)
Write-Host "Building release version (optimized)..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Release build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Release build successful!" -ForegroundColor Green
Write-Host ""

# Test release version
Write-Host "Testing release version..." -ForegroundColor Yellow
$releaseOutput = .\target\release\murphy-sieve 2>&1
if ($releaseOutput -eq "78498") {
    Write-Host "Release test passed: $releaseOutput" -ForegroundColor Green
} else {
    Write-Host "Release test failed. Output: $releaseOutput" -ForegroundColor Red
    exit 1
}
Write-Host ""

# Build competition wrapper
Write-Host "Building competition wrapper..." -ForegroundColor Yellow
cargo build --release --features competition --bin competition-wrapper
if ($LASTEXITCODE -ne 0) {
    Write-Host "Competition wrapper build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Competition wrapper build successful!" -ForegroundColor Green
Write-Host ""

# Run tests
Write-Host "Running tests..." -ForegroundColor Yellow
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "All tests passed!" -ForegroundColor Green
Write-Host ""

# Create binaries directory
Write-Host "Creating distribution..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path "bin" -Force | Out-Null

# Copy binaries
Copy-Item "target\release\murphy-sieve.exe" -Destination "bin\" -Force
Copy-Item "target\release\competition-wrapper.exe" -Destination "bin\" -Force

# Create README for binaries
$binReadme = @"
# Murphy's Sieve Competition Binaries

This directory contains pre-built executables for the competition.

## Files

1. `murphy-sieve.exe` - Standard implementation
   - Computes primes up to 1,000,000
   - Output: 78498
   - Usage: `.\murphy-sieve.exe`

2. `competition-wrapper.exe` - Competition infinite loop
   - Runs infinite loop for competition harness
   - Each iteration computes primes
   - Usage: Run in competition environment

## Source Code
Full source code is available in the parent directory.

## Verification
To verify correctness:
1. Run `.\murphy-sieve.exe`
2. Should output: 78498
3. No error messages

## Competition Submission
Submit the entire project directory, including:
- Source code (src/)
- This README
- Cargo.toml
- Build scripts
- Test files

## License
MIT License
"@

$binReadme | Out-File -FilePath "bin\README.md" -Encoding UTF8

Write-Host "Distribution created in 'bin\' directory" -ForegroundColor Green
Write-Host ""

# Final summary
Write-Host "Build Summary:" -ForegroundColor Cyan
Write-Host "==============" -ForegroundColor Cyan
Write-Host "✅ Rust installation verified" -ForegroundColor Green
Write-Host "✅ Debug build successful" -ForegroundColor Green
Write-Host "✅ Release build successful" -ForegroundColor Green
Write-Host "✅ Competition wrapper built" -ForegroundColor Green
Write-Host "✅ All tests passed" -ForegroundColor Green
Write-Host "✅ Distribution created" -ForegroundColor Green
Write-Host ""
Write-Host "Build completed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Run benchmarks: .\benchmarks\run_benchmarks.ps1" -ForegroundColor White
Write-Host "2. Test competition wrapper: .\bin\competition-wrapper.exe" -ForegroundColor White
Write-Host "3. Package for submission: Compress entire 'final_submission' folder" -ForegroundColor White
Write-Host ""
Write-Host "The submission is READY FOR COMPETITION." -ForegroundColor Green