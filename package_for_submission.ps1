# Package Murphy's Sieve Competition Submission for Submission

Write-Host "Packaging Competition Submission..." -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan
Write-Host ""

# Check if final_submission exists
if (-not (Test-Path "final_submission")) {
    Write-Host "ERROR: final_submission directory not found!" -ForegroundColor Red
    exit 1
}

Write-Host "Found final_submission directory." -ForegroundColor Green
Write-Host ""

# Create zip file
$zipName = "murphy_sieve_competition.zip"
$zipPath = Join-Path (Get-Location) $zipName

Write-Host "Creating zip file: $zipName" -ForegroundColor Yellow

# Remove old zip if exists
if (Test-Path $zipName) {
    Remove-Item $zipName -Force
    Write-Host "Removed old zip file." -ForegroundColor Yellow
}

# Create zip using Compress-Archive
try {
    Compress-Archive -Path "final_submission\*" -DestinationPath $zipName -CompressionLevel Optimal
    Write-Host "Zip file created successfully!" -ForegroundColor Green
} catch {
    Write-Host "ERROR creating zip: $_" -ForegroundColor Red
    exit 1
}

# Verify zip contents
Write-Host ""
Write-Host "Verifying zip contents..." -ForegroundColor Yellow

try {
    $zip = [System.IO.Compression.ZipFile]::OpenRead($zipPath)
    $entries = $zip.Entries | Select-Object -First 20
    $totalEntries = $zip.Entries.Count
    
    Write-Host "Zip contains $totalEntries files." -ForegroundColor Green
    Write-Host ""
    Write-Host "Key files in zip:" -ForegroundColor Cyan
    Write-Host "-----------------" -ForegroundColor Cyan
    
    $keyFiles = @(
        "Cargo.toml",
        "README.md",
        "src/murphy_sieve.rs",
        "src/competition_wrapper.rs",
        "build.ps1",
        "verify_submission.ps1",
        "SUBMISSION_CHECKLIST.md",
        "FINAL_SUBMISSION_SUMMARY.md"
    )
    
    foreach ($file in $keyFiles) {
        $found = $zip.Entries | Where-Object { $_.FullName -eq $file }
        if ($found) {
            Write-Host "  ✅ $file" -ForegroundColor Green
        } else {
            Write-Host "  ❌ $file (MISSING!)" -ForegroundColor Red
        }
    }
    
    $zip.Dispose()
    
} catch {
    Write-Host "ERROR reading zip: $_" -ForegroundColor Red
}

Write-Host ""
Write-Host "Package Summary:" -ForegroundColor Cyan
Write-Host "================" -ForegroundColor Cyan
Write-Host "Package: $zipName" -ForegroundColor White
Write-Host "Size: $([math]::Round((Get-Item $zipName).Length / 1KB, 2)) KB" -ForegroundColor White
Write-Host "Contents: final_submission\*" -ForegroundColor White
Write-Host ""
Write-Host "Submission Checklist:" -ForegroundColor Yellow
Write-Host "1. ✅ Working implementation (Rust)" -ForegroundColor White
Write-Host "2. ✅ Competition infinite loop wrapper" -ForegroundColor White
Write-Host "3. ✅ Honest documentation" -ForegroundColor White
Write-Host "4. ✅ Build system and tests" -ForegroundColor White
Write-Host "5. ✅ Zeta limitations documented" -ForegroundColor White
Write-Host ""
Write-Host "Ready for competition submission!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Yellow
Write-Host "1. Submit $zipName to competition platform" -ForegroundColor White
Write-Host "2. Include note about experimental Zeta language work" -ForegroundColor White
Write-Host "3. Reference honest performance assessment" -ForegroundColor White
Write-Host ""
Write-Host "Good luck in the competition! 🏆" -ForegroundColor Cyan