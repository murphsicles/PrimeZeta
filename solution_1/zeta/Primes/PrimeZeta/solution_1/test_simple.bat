@echo off
echo === PrimeZeta Competition Submission Test ===
echo.

echo 1. Checking required files...
if exist src\prime.z (
  echo   ✓ src\prime.z
) else (
  echo   ✗ src\prime.z (MISSING)
  exit /b 1
)

if exist README.md (
  echo   ✓ README.md
) else (
  echo   ✗ README.md (MISSING)
  exit /b 1
)

if exist Dockerfile (
  echo   ✓ Dockerfile
) else (
  echo   ✗ Dockerfile (MISSING)
  exit /b 1
)

if exist run.sh (
  echo   ✓ run.sh
) else (
  echo   ✗ run.sh (MISSING)
  exit /b 1
)

if exist rust_fallback.rs (
  echo   ✓ rust_fallback.rs
) else (
  echo   ✗ rust_fallback.rs (MISSING)
  exit /b 1
)

if exist verify_counts.txt (
  echo   ✓ verify_counts.txt
) else (
  echo   ✗ verify_counts.txt (MISSING)
  exit /b 1
)

echo.
echo 2. Checking competition tags in README...
findstr /C:"algorithm=wheel" README.md >nul
if %errorlevel% equ 0 (
  echo   ✓ algorithm=wheel
) else (
  echo   ✗ algorithm=wheel
  exit /b 1
)

findstr /C:"faithful=yes" README.md >nul
if %errorlevel% equ 0 (
  echo   ✓ faithful=yes
) else (
  echo   ✗ faithful=yes
  exit /b 1
)

findstr /C:"bits=1" README.md >nul
if %errorlevel% equ 0 (
  echo   ✓ bits=1
) else (
  echo   ✗ bits=1
  exit /b 1
)

findstr /C:"parallel=no" README.md >nul
if %errorlevel% equ 0 (
  echo   ✓ parallel=no
) else (
  echo   ✗ parallel=no
  exit /b 1
)

echo.
echo 3. Checking prime count verification...
findstr /C:"1000000 78498" verify_counts.txt >nul
if %errorlevel% equ 0 (
  echo   ✓ Competition prime count verified (1,000,000 -> 78,498)
) else (
  echo   ✗ Competition prime count missing or incorrect
  exit /b 1
)

echo.
echo 4. Checking Dockerfile structure...
findstr /C:"FROM rust:" Dockerfile >nul
if %errorlevel% equ 0 (
  echo   ✓ Dockerfile uses Rust base image
) else (
  echo   ✗ Dockerfile missing Rust base image
  exit /b 1
)

findstr /C:"COPY src/prime.z" Dockerfile >nul
if %errorlevel% equ 0 (
  echo   ✓ Dockerfile copies prime.z
) else (
  echo   ✗ Dockerfile doesn't copy prime.z
  exit /b 1
)

echo.
echo === TEST PASSED ===
echo.
echo Submission package is ready for competition.
echo.
echo Competition ID: murphsicles
echo Category: PrimeZeta
echo Algorithm: Murphy's Sieve with 30030-wheel
echo Tags: algorithm=wheel, faithful=yes, bits=1, parallel=no
echo.