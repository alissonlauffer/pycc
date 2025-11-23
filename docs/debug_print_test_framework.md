# Debug Print Test Framework

## Overview

The Debug Print Test Framework is a comprehensive testing system designed to evaluate the behavior of pycc-compiled code against the CPython reference implementation. The framework focuses specifically on debug print statements and output verification, ensuring that pycc produces identical output to CPython for a wide range of Python constructs.

## Architecture

The framework consists of several key components:

### Core Components

1. **DebugPrintTester** (`tests/debug_print_tests.rs`)
   - Core testing utility that handles compilation and execution
   - Compiles Python code with pycc to create executables
   - Executes the same code with CPython for reference
   - Compares outputs and provides detailed results

2. **DebugPrintSuite** (`tests/debug_print_suite.rs`)
   - Comprehensive test suite runner
   - Organizes tests into categories (Basic Print, Variables, Arithmetic, Functions, etc.)
   - Provides detailed reporting and statistics
   - Supports exporting results to text format

3. **Test Cases** (`tests/debug_print_test_cases.rs`)
   - Individual test implementations covering various Python constructs
   - Includes both expected success and expected failure tests
   - Tests basic types, operations, functions, f-strings, and edge cases

4. **Test Runner** (`tests/debug_print_runner.rs`)
   - Standalone executable for running the complete test suite
   - Provides command-line interface for test execution
   - Returns appropriate exit codes based on test results

## Features

### Test Categories

1. **Basic Print Tests**
   - Integer, float, string, and boolean literals
   - Empty strings and special values
   - Positive and negative numbers

2. **Variable Tests**
   - Variable assignment and printing
   - Multiple variables
   - Different data types

3. **Arithmetic Tests**
   - Basic operations (+, -, *, /)
   - Complex expressions
   - Operator precedence

4. **Function Tests**
   - Function definitions and calls
   - Multiple function calls
   - Nested function calls

5. **String Tests**
   - String operations
   - Mixed type assignments

6. **F-String Tests**
   - Simple f-strings
   - Multiple variables
   - Expressions in f-strings
   - Mixed content

7. **Edge Case Tests**
   - Large numbers
   - Negative arithmetic
   - Zero operations
   - Multiple prints

8. **Known Limitation Tests**
   - Complex f-string expressions
   - Nested f-strings
   - Tests that are expected to fail due to current limitations

### Key Capabilities

- **Automatic Compilation**: Compiles Python source code with pycc
- **Reference Execution**: Runs the same code with CPython
- **Output Comparison**: Compares outputs character by character
- **Detailed Reporting**: Provides comprehensive test results and statistics
- **Export Support**: Exports results to text format for analysis
- **Categorized Testing**: Organizes tests by functionality
- **Expected Failure Support**: Handles tests that are known to fail

## Usage

### Running Individual Tests

```bash
# Run a specific test
cargo test test_print_integer --test debug_print_test_cases

# Run all tests in a category
cargo test test_print --test debug_print_test_cases

# Run f-string tests
cargo test test_fstring --test debug_print_test_cases
```

### Running the Complete Test Suite

```bash
# Run all debug print tests
cargo test --test debug_print_test_cases

# Run the test suite programmatically
cargo test test_run_basic_suite --test debug_print_suite
```

### Using the Test Framework in Code

```rust
use pycc::tests::debug_print_tests::DebugPrintTester;

// Create a tester
let tester = DebugPrintTester::new()?;

// Test a simple case
tester.assert_outputs_match("print(42)", "test_print")?;

// Compare outputs manually
let result = tester.compare_outputs("print('Hello')", "test_hello")?;
println!("PyCC: {}", result.pycc_output);
println!("CPython: {}", result.cpython_output);
println!("Match: {}", result.outputs_match());
```

### Running the Standalone Runner

```bash
# Compile and run the test runner
cargo run --bin debug_print_runner --test debug_print_runner
```

## Test Results

### Success Criteria

A test is considered successful when:
1. The pycc compilation completes without errors
2. Both pycc and CPython execution complete successfully
3. The outputs match exactly (ignoring trailing whitespace)

### Expected Failures

Some tests are marked as "expected failures" because they test features that are not yet fully implemented in pycc. These tests:
- Are expected to produce different outputs between pycc and CPython
- Still help track progress and identify areas needing improvement
- Are clearly marked in the test results

### Output Format

```
🚀 Starting Debug Print Test Suite...

📝 Running basic print tests...
  ✅ print_integer
  ✅ print_negative_integer
  ❌ print_complex_fstring
     PyCC: Complex: 30
     CPython: Complex: 30

📦 Running variable tests...
  ✅ variable_assignment_integer
  ...

📊 Test Suite Summary
====================
Total tests: 45
Passed: 42
Failed: 3
Success rate: 93.3%

❌ Failed Tests:
  - print_complex_fstring (F-Strings)
  - nested_fstring (Known Limitations)
  - ...

📈 Results by Category:
  Basic Print: 10/10 (100.0%)
  Variables: 5/5 (100.0%)
  F-Strings: 6/8 (75.0%)
  ...

🎯 Debug Print Test Suite Complete!
```

## Adding New Tests

### Creating Individual Tests

```rust
#[test]
fn test_new_feature() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
# Your Python code here
result = some_operation()
print(result)
"#;
    tester.assert_outputs_match(source, "test_new_feature")
        .expect("Output mismatch between PyCC and CPython");
}
```

### Adding Expected Failure Tests

```rust
#[test]
fn test_unsupported_feature() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
# Code that uses unsupported features
print(f"Complex: {some_unsupported_expression}")
"#;
    
    let result = tester.compare_outputs(source, "test_unsupported_feature")
        .expect("Failed to compare outputs");
    
    // We expect this to fail
    assert!(!result.outputs_match(), 
           "Expected outputs to differ, but they matched");
    
    // Print comparison for debugging
    result.print_comparison();
}
```

### Adding Tests to the Suite

To add tests to the comprehensive suite, modify the appropriate category method in `DebugPrintSuite`:

```rust
fn run_new_category_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
    println!("🔬 Running new category tests...");
    
    let tests = vec![
        ("new_test_1", "source_code_1"),
        ("new_test_2", "source_code_2"),
    ];

    for (name, source) in tests {
        self.run_single_test(name, source, "New Category", summary)?;
    }

    Ok(())
}
```

## Troubleshooting

### Common Issues

1. **Compilation Failures**
   - Ensure LLVM and clang are properly installed
   - Check that the pycc compilation pipeline is working
   - Verify that object files are generated correctly

2. **Execution Failures**
   - Ensure CPython (python3) is available in PATH
   - Check that compiled executables have proper permissions
   - Verify that temporary directories are accessible

3. **Output Mismatches**
   - Check for differences in whitespace handling
   - Verify that floating-point formatting is consistent
   - Ensure that string escaping is handled correctly

### Debugging Tips

1. **Enable Verbose Output**: Use the `print_comparison()` method to see detailed differences
2. **Test Manually**: Run the same code manually with both pycc and CPython
3. **Check Intermediate Files**: Examine generated LLVM IR and object files
4. **Isolate Issues**: Create minimal test cases to reproduce specific problems

## Future Enhancements

### Planned Features

1. **Performance Testing**: Add timing comparisons between pycc and CPython
2. **Memory Usage Testing**: Compare memory consumption
3. **Error Handling Tests**: Test error conditions and exception handling
4. **Standard Library Tests**: Expand to cover more standard library functions
5. **Regression Testing**: Automatic detection of performance regressions

### Integration Opportunities

1. **CI/CD Integration**: Integrate with continuous integration pipelines
2. **Benchmark Suite**: Create performance benchmarking capabilities
3. **Visual Reports**: Generate HTML reports with charts and graphs
4. **Test Database**: Store historical test results for trend analysis

## Contributing

When contributing to the debug print test framework:

1. **Follow the existing patterns** for test organization and naming
2. **Add comprehensive documentation** for new test categories
3. **Update this documentation** when adding new features
4. **Test both success and failure cases** where appropriate
5. **Consider edge cases** and boundary conditions

## License

This test framework is part of the pycc project and follows the same licensing terms.