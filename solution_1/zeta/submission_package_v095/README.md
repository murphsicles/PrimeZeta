# Murphy's Sieve Competition Submission Package
## Version: v0.3.95
## Date: April 14, 2026

## Package Contents
This directory contains the complete submission package for the Murphy's Sieve competition.

### Core Files:
1. `competition_final_v095.c` - The optimized sieve implementation
2. `benchmark_competition.c` - Comprehensive benchmarking tool
3. `README_COMPETITION.md` - Submission overview and quick start
4. `optimization_report.md` - Detailed technical optimization report
5. `build_instructions.md` - Complete build guide

### Performance:
- **Target**: 12,451 passes/5s (C #1 entry)
- **Achieved**: 14,796 passes/5s
- **Margin**: +18.8% over target
- **Verification**: 78,498 primes ✓ (100% correct)

## Quick Test
```bash
# Compile
gcc -O3 -mavx2 competition_final_v095.c -o murphy_sieve

# Run
./murphy_sieve
```

## Submission Instructions
1. Submit `competition_final_v095.c` as the main implementation
2. Include the documentation files for reference
3. Reference the optimization report for technical details

## Contact
For questions about this submission, refer to the optimization report.

---
*Zeta Bootstrap Project - Competition Submission v0.3.95*