#!/bin/bash
# PrimeZeta competition runner
# Prints prime count for limit=1,000,000 repeatedly

set -e

# Compile if needed
if [ ! -f "prime" ]; then
    echo "Compiling prime.z with Zeta compiler..."
    zetac prime.z -o prime
fi

echo "PrimeZeta competition entry - Murphy's Sieve with bit array"
echo "Algorithm: wheel (30030), Faithful: yes, Bits: 1, Parallel: no"
echo ""

# Infinite loop printing prime count (competition harness times 5 seconds)
while true; do
    ./prime
done