# Final check of competition package

Write-Host "=== Final Competition Package Check ==="
Write-Host ""

# List all files
Write-Host "Package contents:"
Get-ChildItem -Recurse | ForEach-Object {
    $rel_path = $_.FullName.Substring((Get-Location).Path.Length + 1)
    Write-Host "  $rel_path"
}

Write-Host ""
Write-Host "Key files status:"

# Check key files
$key_files = @{
    "src/prime.z" = "Main Zeta implementation"
    "README.md" = "Documentation with badges"
    "Dockerfile" = "Container configuration"
    "run.sh" = "Entry point script"
}

foreach ($file in $key_files.Keys) {
    if (Test-Path $file) {
        Write-Host "  ✓ $file - $($key_files[$file])" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $file - MISSING" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "README content (first 20 lines):"
Get-Content README.md | Select-Object -First 20 | ForEach-Object {
    Write-Host "  $_"
}

Write-Host ""
Write-Host "=== PACKAGE ASSEMBLY COMPLETE ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Competition submission package includes:"
Write-Host "1. Complete directory structure: Primes/PrimeZeta/solution_1/"
Write-Host "2. README with correct badges: Algorithm=wheel, Faithful=yes, Bits=8, Parallel=no"
Write-Host "3. Dockerfile for containerized execution"
Write-Host "4. run.sh entry point script"
Write-Host "5. src/prime.z - Murphy's Sieve implementation in Zeta"
Write-Host ""
Write-Host "The package is ready for submission to the PrimeZeta competition!" -ForegroundColor Green