#!/bin/bash
# Competition entry script for PrimeZeta
# Runs Murphy's Sieve in infinite loop as required by competition harness

set -e

echo "=== PrimeZeta Competition Entry ==="
echo "Starting Murphy's Sieve implementation..."
echo ""

# Check if we have the compiled binary
if [ ! -f "./prime_zeta" ]; then
    echo "Error: prime_zeta binary not found!"
    echo "Please build first:"
    echo "  ./build.sh"
    exit 1
fi

echo "Running infinite loop (Ctrl+C to stop)..."
echo "Each iteration computes primes up to 1,000,000"
echo "Expected output per iteration: 78498"
echo ""

# Infinite loop as required by competition
# The harness will run this for 5 seconds and count iterations
while true; do
    # Run one iteration and print result
    ./prime_zeta
done