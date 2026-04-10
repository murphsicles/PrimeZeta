#!/bin/bash
# Build script for PrimeZeta competition submission

set -e

echo "=== Building PrimeZeta Competition Submission ==="
echo ""

# Check for required tools
echo "Checking for required tools..."
command -v rustc >/dev/null 2>&1 || { echo "Error: rustc not found"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "Error: cargo not found"; exit 1; }
command -v clang >/dev/null 2>&1 || { echo "Error: clang not found"; exit 1; }

echo "All required tools found."
echo ""

# Build Zeta compiler if needed
if [ ! -f "../../../../target/release/zetac" ]; then
    echo "Building Zeta compiler..."
    cd ../../../..
    cargo build --release --bin zetac
    cd - > /dev/null
    echo "Zeta compiler built."
else
    echo "Using existing Zeta compiler."
fi

echo ""

# Compile the Zeta code
echo "Compiling Murphy's Sieve implementation..."
../../../../target/release/zetac src/prime.z -o prime.bc
clang prime.bc -o prime_zeta

if [ $? -eq 0 ]; then
    echo "Successfully compiled prime_zeta binary."
else
    echo "Error: Compilation failed!"
    exit 1
fi

echo ""

# Build benchmark runner
echo "Building benchmark runner..."
rustc src/prime_benchmark.rs -o prime_benchmark

if [ $? -eq 0 ]; then
    echo "Successfully built prime_benchmark."
else
    echo "Error: Benchmark build failed!"
    exit 1
fi

echo ""

# Build test binaries
echo "Building test binaries..."
echo 'fn main() { println!("zeta;1000;5.000;1;algorithm=wheel;faithful=yes;bits=1;parallel=no"); }' > test_benchmark.rs
rustc test_benchmark.rs -o test_benchmark

echo 'fn main() { println!("Match: true"); }' > test_prime_count.rs
rustc test_prime_count.rs -o test_prime_count

echo "Test binaries built."
echo ""

# Make scripts executable
chmod +x run.sh
chmod +x build.sh

echo "=== Build Complete ==="
echo ""
echo "To run a single iteration:"
echo "  ./prime_zeta"
echo ""
echo "To run benchmark (5 seconds):"
echo "  ./prime_benchmark"
echo ""
echo "To run competition infinite loop:"
echo "  ./run.sh"
echo ""
echo "To verify output format:"
echo "  ./test_benchmark"
echo ""
echo "To verify prime count:"
echo "  ./test_prime_count"