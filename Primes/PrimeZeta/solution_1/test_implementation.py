#!/usr/bin/env python3
"""
Test script for PrimeZeta competition submission
Verifies the implementation meets competition requirements
"""

import subprocess
import time
import sys
import os

def run_command(cmd, timeout=10):
    """Run a command and return output"""
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=timeout)
        return result.returncode, result.stdout.strip(), result.stderr.strip()
    except subprocess.TimeoutExpired:
        return -1, "", "Command timed out"

def test_build():
    """Test that the project builds successfully"""
    print("=== Testing Build ===")
    
    # Check if build.sh exists
    if not os.path.exists("build.sh"):
        print("❌ build.sh not found")
        return False
    
    # Make it executable
    os.chmod("build.sh", 0o755)
    
    # Run build
    print("Running build.sh...")
    retcode, stdout, stderr = run_command("./build.sh", timeout=60)
    
    if retcode != 0:
        print(f"❌ Build failed with code {retcode}")
        if stderr:
            print(f"Stderr: {stderr}")
        return False
    
    print("✅ Build successful")
    
    # Check for required files
    required_files = ["prime_zeta", "prime_benchmark", "run.sh"]
    for f in required_files:
        if not os.path.exists(f):
            print(f"❌ Required file not found: {f}")
            return False
    
    print("✅ All required files present")
    return True

def test_single_iteration():
    """Test a single iteration of the sieve"""
    print("\n=== Testing Single Iteration ===")
    
    if not os.path.exists("prime_zeta"):
        print("❌ prime_zeta binary not found")
        return False
    
    print("Running prime_zeta...")
    retcode, stdout, stderr = run_command("./prime_zeta", timeout=5)
    
    if retcode != 0:
        print(f"❌ Execution failed with code {retcode}")
        if stderr:
            print(f"Stderr: {stderr}")
        return False
    
    output = stdout.strip()
    print(f"Output: {output}")
    
    # Should output 78498
    if output == "78498":
        print("✅ Correct prime count for limit=1,000,000")
        return True
    else:
        print(f"❌ Incorrect output. Expected '78498', got '{output}'")
        return False

def test_benchmark_format():
    """Test benchmark output format"""
    print("\n=== Testing Benchmark Output Format ===")
    
    if not os.path.exists("prime_benchmark"):
        print("❌ prime_benchmark binary not found")
        return False
    
    print("Running prime_benchmark (quick test)...")
    # Run with short timeout just to check format
    retcode, stdout, stderr = run_command("timeout 2 ./prime_benchmark", timeout=3)
    
    if retcode != 0 and retcode != 124:  # 124 is timeout from timeout command
        print(f"❌ Benchmark failed with code {retcode}")
        if stderr:
            print(f"Stderr: {stderr}")
        return False
    
    # Check for competition format in output
    lines = stdout.split('\n')
    format_found = False
    for line in lines:
        if line.startswith("zeta;") and "algorithm=wheel" in line:
            print(f"✅ Found competition format: {line}")
            format_found = True
            break
    
    if not format_found:
        print("❌ Competition format not found in output")
        print(f"Output:\n{stdout}")
        return False
    
    return True

def test_infinite_loop():
    """Test that infinite loop runs (briefly)"""
    print("\n=== Testing Infinite Loop ===")
    
    if not os.path.exists("run.sh"):
        print("❌ run.sh not found")
        return False
    
    # Make it executable
    os.chmod("run.sh", 0o755)
    
    print("Running run.sh for 1 second...")
    # Start the process
    proc = subprocess.Popen(["./run.sh"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    
    # Wait a bit and collect some output
    time.sleep(1)
    
    # Terminate the process
    proc.terminate()
    try:
        proc.wait(timeout=2)
    except subprocess.TimeoutExpired:
        proc.kill()
    
    # Read any output
    stdout, stderr = proc.communicate()
    
    if stderr:
        print(f"Stderr: {stderr}")
    
    # Check if we got any output
    lines = stdout.strip().split('\n')
    valid_outputs = 0
    for line in lines:
        if line.strip() == "78498":
            valid_outputs += 1
    
    if valid_outputs > 0:
        print(f"✅ Infinite loop produced {valid_outputs} valid outputs")
        return True
    else:
        print("❌ Infinite loop didn't produce expected output")
        if stdout:
            print(f"Output:\n{stdout}")
        return False

def test_rust_fallback():
    """Test Rust fallback implementation"""
    print("\n=== Testing Rust Fallback ===")
    
    if not os.path.exists("rust_fallback.rs"):
        print("❌ rust_fallback.rs not found")
        return False
    
    print("Compiling Rust fallback...")
    retcode, stdout, stderr = run_command("rustc rust_fallback.rs -o rust_fallback", timeout=30)
    
    if retcode != 0:
        print(f"❌ Rust compilation failed with code {retcode}")
        if stderr:
            print(f"Stderr: {stderr}")
        return False
    
    print("✅ Rust fallback compiled")
    
    # Run tests
    print("Running Rust tests...")
    retcode, stdout, stderr = run_command("./rust_fallback test", timeout=10)
    
    if retcode != 0:
        print(f"❌ Rust tests failed with code {retcode}")
        if stderr:
            print(f"Stderr: {stderr}")
        return False
    
    # Check for "All tests passed!"
    if "All tests passed!" in stdout:
        print("✅ Rust fallback tests passed")
        return True
    else:
        print("❌ Rust fallback tests didn't pass")
        print(f"Output:\n{stdout}")
        return False

def verify_readme_tags():
    """Verify README.md has correct tags"""
    print("\n=== Verifying README Tags ===")
    
    if not os.path.exists("README.md"):
        print("❌ README.md not found")
        return False
    
    with open("README.md", 'r') as f:
        content = f.read()
    
    required_tags = [
        "algorithm=wheel",
        "faithful=yes", 
        "bits=1",
        "parallel=no"
    ]
    
    missing_tags = []
    for tag in required_tags:
        if tag not in content:
            missing_tags.append(tag)
    
    if missing_tags:
        print(f"❌ Missing tags in README: {missing_tags}")
        return False
    
    print("✅ All required tags present in README")
    return True

def main():
    """Run all tests"""
    print("PrimeZeta Competition Submission Test Suite")
    print("=" * 50)
    
    tests = [
        ("Build", test_build),
        ("Single Iteration", test_single_iteration),
        ("Benchmark Format", test_benchmark_format),
        ("Infinite Loop", test_infinite_loop),
        ("Rust Fallback", test_rust_fallback),
        ("README Tags", verify_readme_tags),
    ]
    
    results = []
    all_passed = True
    
    for test_name, test_func in tests:
        try:
            passed = test_func()
            results.append((test_name, passed))
            if not passed:
                all_passed = False
        except Exception as e:
            print(f"❌ Exception in {test_name}: {e}")
            results.append((test_name, False))
            all_passed = False
    
    # Summary
    print("\n" + "=" * 50)
    print("TEST SUMMARY")
    print("=" * 50)
    
    for test_name, passed in results:
        status = "✅ PASS" if passed else "❌ FAIL"
        print(f"{test_name:20} {status}")
    
    print("\n" + "=" * 50)
    if all_passed:
        print("🎉 ALL TESTS PASSED - READY FOR COMPETITION!")
        return 0
    else:
        print("⚠️  SOME TESTS FAILED - NEEDS FIXING")
        return 1

if __name__ == "__main__":
    sys.exit(main())