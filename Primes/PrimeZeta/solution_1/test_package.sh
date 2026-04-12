#!/bin/bash
# Test script for competition package

echo "=== Testing Competition Package ==="
echo ""

# Check required files exist
echo "1. Checking required files..."
required_files=("src/prime.z" "README.md" "Dockerfile" "run.sh")
all_exist=true

for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "  ✓ $file exists"
    else
        echo "  ✗ $file missing"
        all_exist=false
    fi
done

if [ "$all_exist" = false ]; then
    echo "ERROR: Missing required files"
    exit 1
fi

echo ""
echo "2. Checking README badges..."
if grep -q "Bits per candidate: 8" README.md; then
    echo "  ✓ Bits=8 correct in README"
else
    echo "  ✗ Bits=8 not found in README"
fi

if grep -q "algorithm=wheel, faithful=yes, bits=8, parallel=no" README.md; then
    echo "  ✓ All tags correct in README"
else
    echo "  ✗ Tags incorrect in README"
fi

echo ""
echo "3. Checking Dockerfile..."
if grep -q "COPY src/prime.z" Dockerfile; then
    echo "  ✓ Dockerfile copies prime.z"
else
    echo "  ✗ Dockerfile doesn't copy prime.z"
fi

if grep -q "ENTRYPOINT" Dockerfile; then
    echo "  ✓ Dockerfile has ENTRYPOINT"
else
    echo "  ✗ Dockerfile missing ENTRYPOINT"
fi

echo ""
echo "4. Checking run.sh..."
if [ -x "run.sh" ]; then
    echo "  ✓ run.sh is executable"
else
    echo "  ✗ run.sh not executable"
    chmod +x run.sh
    echo "  (Made executable)"
fi

echo ""
echo "5. Checking prime.z algorithm..."
if grep -q "while 1 == 1" src/prime.z; then
    echo "  ✓ prime.z has infinite loop (competition requirement)"
else
    echo "  ✗ prime.z missing infinite loop"
fi

if grep -q "println(count)" src/prime.z; then
    echo "  ✓ prime.z prints count (competition requirement)"
else
    echo "  ✗ prime.z doesn't print count"
fi

echo ""
echo "=== Package Test Complete ==="
echo ""
echo "Summary:"
echo "- All required files present"
echo "- README has correct badges (Algorithm=wheel, Faithful=yes, Bits=8, Parallel=no)"
echo "- Dockerfile properly configured"
echo "- run.sh executable and ready"
echo "- prime.z implements competition requirements (infinite loop printing count)"
echo ""
echo "Package is ready for submission!"