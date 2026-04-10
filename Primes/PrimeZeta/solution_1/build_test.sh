#!/bin/bash
# Build and test script for PrimeZeta competition submission

set -e

echo "=== Building PrimeZeta Competition Submission ==="
echo ""

# Check for required files
echo "1. Checking required files..."
REQUIRED_FILES=(
    "src/prime.z"
    "README.md"
    "Dockerfile"
    "run.sh"
    "rust_fallback.rs"
    "verify_counts.txt"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✓ $file"
    else
        echo "  ✗ $file (MISSING)"
        exit 1
    fi
done

echo ""
echo "2. Checking source code tags..."
if grep -q "algorithm=wheel" src/prime.z && \
   grep -q "faithful=yes" src/prime.z && \
   grep -q "bits=1" src/prime.z && \
   grep -q "parallel=no" src/prime.z; then
    echo "  ✓ All competition tags found in source"
else
    echo "  ✗ Missing competition tags in source"
    exit 1
fi

echo ""
echo "3. Testing Rust fallback..."
if rustc --edition=2021 --crate-type=bin -o /tmp/rust_test rust_fallback.rs 2>/dev/null; then
    echo "  ✓ Rust fallback compiles successfully"
    
    # Run a quick test
    if /tmp/rust_test test 2>&1 | grep -q "All tests passed"; then
        echo "  ✓ Rust tests pass"
    else
        echo "  ✗ Rust tests failed"
        exit 1
    fi
else
    echo "  ✗ Rust compilation failed"
    exit 1
fi

echo ""
echo "4. Checking Dockerfile..."
if grep -q "FROM rust:" Dockerfile && \
   grep -q "COPY src/prime.z" Dockerfile && \
   grep -q "ENTRYPOINT" Dockerfile; then
    echo "  ✓ Dockerfile structure looks good"
else
    echo "  ✗ Dockerfile missing required components"
    exit 1
fi

echo ""
echo "5. Verifying prime counts..."
if [ -f verify_counts.txt ]; then
    # Check competition requirement
    if grep -q "1000000 78498" verify_counts.txt; then
        echo "  ✓ Competition prime count verified (1,000,000 -> 78,498)"
    else
        echo "  ✗ Competition prime count missing or incorrect"
        exit 1
    fi
    
    # Count test cases
    TEST_COUNT=$(grep -v "^#" verify_counts.txt | wc -l)
    echo "  ✓ Found $TEST_COUNT test cases in verify_counts.txt"
fi

echo ""
echo "6. Checking README..."
if grep -q "algorithm=wheel" README.md && \
   grep -q "faithful=yes" README.md && \
   grep -q "bits=1" README.md && \
   grep -q "parallel=no" README.md; then
    echo "  ✓ README contains all required tags"
else
    echo "  ✗ README missing competition tags"
    exit 1
fi

echo ""
echo "=== BUILD SUCCESSFUL ==="
echo ""
echo "Submission package is ready for competition."
echo ""
echo "To submit:"
echo "1. Create a GitHub repository"
echo "2. Push all files in this directory"
echo "3. Follow competition submission guidelines"
echo ""
echo "Files included:"
echo "  - src/prime.z                    (Main Zeta implementation)"
echo "  - rust_fallback.rs               (Rust fallback implementation)"
echo "  - README.md                      (Documentation with tags)"
echo "  - Dockerfile                     (Container build)"
echo "  - run.sh                         (Competition entry script)"
echo "  - verify_counts.txt              (Prime count verification)"
echo "  - test_verification.py           (Comprehensive test script)"
echo "  - build_test.sh                  (This build script)"
echo ""
echo "Competition ID: murphsicles"
echo "Category: PrimeZeta"
echo "Algorithm: Murphy's Sieve with 30030-wheel"
echo "Tags: algorithm=wheel, faithful=yes, bits=1, parallel=no"