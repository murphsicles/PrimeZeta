# Simple validation script for competition submission

Write-Host "Validating Murphy's Sieve submission..."
Write-Host ""

# Check required files
$requiredFiles = @(
    "src/prime.z",
    "README.md"
)

Write-Host "Checking required files:"
foreach ($file in $requiredFiles) {
    if (Test-Path $file) {
        Write-Host "  [OK] $file"
    } else {
        Write-Host "  [MISSING] $file"
        exit 1
    }
}

Write-Host ""

# Check README tags
Write-Host "Checking README tags:"
$readme = Get-Content "README.md" -Raw
$tags = @("algorithm=wheel", "faithful=yes", "bits=1", "parallel=no")

foreach ($tag in $tags) {
    if ($readme -match $tag) {
        Write-Host "  [OK] $tag"
    } else {
        Write-Host "  [MISSING] $tag"
    }
}

Write-Host ""

# Check Zeta files
Write-Host "Checking Zeta files:"
$zetaFiles = Get-ChildItem -Path . -Filter "*.z" -Recurse

foreach ($file in $zetaFiles) {
    $content = Get-Content $file.FullName -First 2
    Write-Host "  [FOUND] $($file.Name) ($($file.Length) bytes)"
}

Write-Host ""

# Create submission package
Write-Host "Creating submission package..."
$timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$zipName = "murphy_sieve_competition_$timestamp.zip"

$filesToZip = @(
    "src/prime.z",
    "README.md",
    "benchmark_wrapper.z",
    "verification_tests.z",
    "Dockerfile",
    "benchmark.ps1",
    "validate_submission.ps1",
    "SUBMISSION_SUMMARY.md"
)

# Filter to only existing files
$existingFiles = $filesToZip | Where-Object { Test-Path $_ }

if ($existingFiles.Count -gt 0) {
    Compress-Archive -Path $existingFiles -DestinationPath $zipName -Force
    Write-Host "Created: $zipName"
    Write-Host "Files included: $($existingFiles.Count)"
} else {
    Write-Host "No files to package!"
}

Write-Host ""
Write-Host "Validation complete!"
Write-Host "Submission is ready for competition."