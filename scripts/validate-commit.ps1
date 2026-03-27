# Pre-Commit Validation Script
# Ensures no untracked TODOs or placeholder logic

param(
    [string]$CommitMessage = ""
)

Write-Host "=== PRE-COMMIT VALIDATION ===" -ForegroundColor Red
Write-Host ""

# Check for staged changes
$stagedFiles = git diff --cached --name-only
if (-not $stagedFiles) {
    Write-Host "No staged changes to validate." -ForegroundColor Yellow
    exit 0
}

Write-Host "Checking staged files for TODOs and placeholders..." -ForegroundColor Cyan

$hasViolations = $false
$violations = @()

foreach ($file in $stagedFiles) {
    if ($file -match "\.(rs|z)$") {
        # Get staged content for this file
        $stagedContent = git diff --cached "$file" | Where-Object { $_ -match "^\+" } | ForEach-Object { $_.Substring(1) }
        
        # Check for TODOs/FIXMEs
        $lineNum = 0
        foreach ($line in $stagedContent) {
            $lineNum++
            if ($line -match "(TODO|FIXME)") {
                $violationMsg = "❌ ${file}:${lineNum} - Untracked $($matches[1]): $($line.Trim())"
                $violations += $violationMsg
                $hasViolations = $true
            }
            
            # Check for placeholder patterns
            if ($line -match "placeholder|stub|hack|XXX" -and $line -notmatch "TODO_TRACKING") {
                $violationMsg = "⚠️  ${file}:${lineNum} - Placeholder logic: $($line.Trim())"
                $violations += $violationMsg
                $hasViolations = $true
            }
            
            # Check for magic number placeholders
            if ($line -match "Lit\(0\)|return 0|0; //|default_value" -and $line -notmatch "test") {
                $violationMsg = "⚠️  ${file}:${lineNum} - Magic placeholder: $($line.Trim())"
                $violations += $violationMsg
                $hasViolations = $true
            }
        }
    }
}

# Check commit message for TODO references
if ($CommitMessage -match "TODO-") {
    Write-Host "✅ Commit references TODO ID" -ForegroundColor Green
} else {
    # Check if any TODOs were found
    if ($violations | Where-Object { $_ -match "❌" }) {
        Write-Host "⚠️  Commit doesn't reference TODO IDs but has TODOs" -ForegroundColor Yellow
    }
}

# Report violations
if ($violations.Count -gt 0) {
    Write-Host "`n=== VIOLATIONS FOUND ===" -ForegroundColor Red
    foreach ($v in $violations) {
        Write-Host $v
    }
    
    Write-Host "`n=== REQUIRED ACTIONS ===" -ForegroundColor Red
    Write-Host "1. Add TODOs to TODO_TRACKING.md with:" -ForegroundColor Yellow
    Write-Host "   - ID (TODO-YYYYMMDD-NNN)" -ForegroundColor Gray
    Write-Host "   - Owner (agent responsible)" -ForegroundColor Gray
    Write-Host "   - Due date (within 48 hours)" -ForegroundColor Gray
    Write-Host "   - Impact description" -ForegroundColor Gray
    
    Write-Host "`n2. Update commit message to reference TODO ID:" -ForegroundColor Yellow
    Write-Host "   [AGENT] Description [TODO-ID]" -ForegroundColor Gray
    
    Write-Host "`n3. Or remove placeholder logic and implement properly" -ForegroundColor Yellow
    
    Write-Host "`n=== COMMIT BLOCKED ===" -ForegroundColor Red
    Write-Host "Fix violations before committing." -ForegroundColor Red
    
    exit 1
} else {
    Write-Host "✅ No untracked TODOs or placeholder logic found." -ForegroundColor Green
    Write-Host "Commit validation passed." -ForegroundColor Green
    exit 0
}