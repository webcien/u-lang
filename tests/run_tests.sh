#!/bin/bash
# run_tests.sh — U v1.0 Test Runner
# MIT License — Copyright (c) 2025 Webcien and U contributors

set -e

COMPILER="../compiler/target/release/ul"
TOTAL=0
PASSED=0
FAILED=0

echo "=========================================="
echo "U Language v1.0 Test Suite"
echo "=========================================="
echo ""

# Function to run a test
run_test() {
    local test_file=$1
    local expect_error=$2
    local test_name=$(basename "$test_file" .ul)
    
    TOTAL=$((TOTAL + 1))
    
    echo -n "Testing $test_name... "
    
    if $COMPILER build "$test_file" --no-link > /dev/null 2>&1; then
        if [ "$expect_error" = "true" ]; then
            echo "❌ FAILED (expected error, got success)"
            FAILED=$((FAILED + 1))
        else
            echo "✅ PASSED"
            PASSED=$((PASSED + 1))
        fi
    else
        if [ "$expect_error" = "true" ]; then
            echo "✅ PASSED (error as expected)"
            PASSED=$((PASSED + 1))
        else
            echo "❌ FAILED (unexpected error)"
            FAILED=$((FAILED + 1))
        fi
    fi
}

# Ownership tests
echo "=== Ownership Tests ==="
run_test "ownership/test_basic_ownership.ul" false
run_test "ownership/test_use_after_move.ul" true
run_test "ownership/test_mutability.ul" true
echo ""

# Concurrency tests
echo "=== Concurrency Tests ==="
run_test "concurrency/test_actor_basic.ul" false
echo ""

# GUI tests
echo "=== GUI Tests ==="
run_test "gui/test_simple_ui.ul" false
echo ""

# Summary
echo "=========================================="
echo "Test Summary"
echo "=========================================="
echo "Total:  $TOTAL"
echo "Passed: $PASSED"
echo "Failed: $FAILED"
echo ""

if [ $FAILED -eq 0 ]; then
    echo "✅ All tests passed!"
    exit 0
else
    echo "❌ Some tests failed"
    exit 1
fi
