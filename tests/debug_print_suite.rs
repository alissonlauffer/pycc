//! Comprehensive debug print test suite for pycc
//!
//! This module provides a complete testing framework to evaluate the behavior
//! of pycc-compiled code against CPython reference implementation, focusing
//! on debug print statements and output verification.

#[path = "debug_print_test_cases.rs"]
mod debug_print_test_cases;
#[path = "debug_print_tests.rs"]
mod debug_print_tests;

use debug_print_tests::{ComparisonResult, DebugPrintTester};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Comprehensive test suite runner for debug print tests
pub struct DebugPrintSuite {
    tester: DebugPrintTester,
    results: Vec<TestSuiteResult>,
}

impl DebugPrintSuite {
    /// Create a new debug print test suite
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let tester = DebugPrintTester::new()?;
        Ok(DebugPrintSuite {
            tester,
            results: Vec::new(),
        })
    }

    /// Run all debug print tests and return comprehensive results
    pub fn run_all_tests(&mut self) -> Result<TestSuiteSummary, String> {
        println!("🚀 Starting Debug Print Test Suite...\n");

        let mut summary = TestSuiteSummary::new();

        // Test categories
        self.run_basic_print_tests(&mut summary)?;
        self.run_variable_tests(&mut summary)?;
        self.run_arithmetic_tests(&mut summary)?;
        self.run_function_tests(&mut summary)?;
        self.run_string_tests(&mut summary)?;
        self.run_fstring_tests(&mut summary)?;
        self.run_edge_case_tests(&mut summary)?;
        self.run_existing_file_tests(&mut summary)?;
        self.run_known_limitation_tests(&mut summary)?;

        self.print_summary(&summary);
        Ok(summary)
    }

    /// Run basic print statement tests
    fn run_basic_print_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("📝 Running basic print tests...");

        let tests = vec![
            ("print_integer", "print(42)"),
            ("print_negative_integer", "print(-42)"),
            ("print_zero", "print(0)"),
            ("print_float", "print(3.14)"),
            ("print_negative_float", "print(-2.71)"),
            ("print_zero_float", "print(0.0)"),
            ("print_string", "print(\"Hello, World!\")"),
            ("print_empty_string", "print(\"\")"),
            ("print_true", "print(True)"),
            ("print_false", "print(False)"),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Basic Print", summary)?;
        }

        Ok(())
    }

    /// Run variable assignment and printing tests
    fn run_variable_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("📦 Running variable tests...");

        let tests = vec![
            (
                "variable_assignment_integer",
                r#"
x = 42
print(x)
"#,
            ),
            (
                "variable_assignment_float",
                r#"
y = 3.14159
print(y)
"#,
            ),
            (
                "variable_assignment_string",
                r#"
name = "Python"
print(name)
"#,
            ),
            (
                "multiple_variables",
                r#"
a = 10
b = 20
c = 30
print(a)
print(b)
print(c)
"#,
            ),
            (
                "boolean_variables",
                r#"
is_true = True
is_false = False
print(is_true)
print(is_false)
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Variables", summary)?;
        }

        Ok(())
    }

    /// Run arithmetic operation tests
    fn run_arithmetic_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("🔢 Running arithmetic tests...");

        let tests = vec![
            (
                "arithmetic_addition",
                r#"
a = 10
b = 5
result = a + b
print(result)
"#,
            ),
            (
                "arithmetic_subtraction",
                r#"
a = 10
b = 5
result = a - b
print(result)
"#,
            ),
            (
                "arithmetic_multiplication",
                r#"
a = 6
b = 7
result = a * b
print(result)
"#,
            ),
            (
                "arithmetic_division",
                r#"
a = 20
b = 4
result = a / b
print(result)
"#,
            ),
            (
                "complex_arithmetic",
                r#"
x = 10
y = 20
z = (x + y) * 2 - 5
print(z)
"#,
            ),
            (
                "operator_precedence",
                r#"
result = 2 + 3 * 4
print(result)
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Arithmetic", summary)?;
        }

        Ok(())
    }

    /// Run function definition and call tests
    fn run_function_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("🔧 Running function tests...");

        let tests = vec![
            (
                "simple_function",
                r#"
def add(x, y):
    return x + y

result = add(3, 4)
print(result)
"#,
            ),
            (
                "function_with_multiple_calls",
                r#"
def multiply(a, b):
    return a * b

print(multiply(2, 3))
print(multiply(4, 5))
print(multiply(6, 7))
"#,
            ),
            (
                "nested_function_calls",
                r#"
def add_one(x):
    return x + 1

def add_two(x):
    return add_one(add_one(x))

result = add_two(5)
print(result)
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Functions", summary)?;
        }

        Ok(())
    }

    /// Run string operation tests
    fn run_string_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("📄 Running string tests...");

        let tests = vec![
            (
                "string_with_numbers",
                r#"
age = 25
name = "Alice"
print(name)
print(age)
"#,
            ),
            (
                "mixed_assignments",
                r#"
integer_var = 42
float_var = 3.14
string_var = "test"
bool_var = True
print(integer_var)
print(float_var)
print(string_var)
print(bool_var)
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Strings", summary)?;
        }

        Ok(())
    }

    /// Run f-string tests
    fn run_fstring_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("🎯 Running f-string tests...");

        let tests = vec![
            (
                "simple_fstring",
                r#"
name = "Alice"
print(f"Hello, {name}!")
"#,
            ),
            (
                "fstring_with_number",
                r#"
age = 25
print(f"Age: {age}")
"#,
            ),
            (
                "fstring_multiple_variables",
                r#"
name = "Bob"
age = 30
city = "New York"
print(f"{name} is {age} years old and lives in {city}")
"#,
            ),
            (
                "fstring_with_expressions",
                r#"
x = 10
y = 20
print(f"Sum: {x + y}")
print(f"Product: {x * y}")
"#,
            ),
            (
                "fstring_mixed_content",
                r#"
name = "Charlie"
score = 95.5
print(f"Student {name} scored {score} points")
"#,
            ),
            (
                "function_with_fstring",
                r#"
def greet(name):
    return f"Hello, {name}!"

message = greet("World")
print(message)
"#,
            ),
            (
                "arithmetic_with_fstring_output",
                r#"
a = 15
b = 3
print(f"{a} + {b} = {a + b}")
print(f"{a} * {b} = {a * b}")
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "F-Strings", summary)?;
        }

        Ok(())
    }

    /// Run edge case tests
    fn run_edge_case_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("⚡ Running edge case tests...");

        let tests = vec![
            (
                "large_numbers",
                r#"
big_int = 9223372036854775807
print(big_int)
"#,
            ),
            (
                "negative_arithmetic",
                r#"
x = -10
y = -5
result = x + y
print(result)
"#,
            ),
            (
                "zero_operations",
                r#"
x = 0
y = 10
print(x + y)
print(y - x)
print(x * y)
"#,
            ),
            (
                "multiple_prints_different_types",
                r#"
print("Starting program...")
x = 42
print(f"The answer is {x}")
print("Program finished.")
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test(name, source, "Edge Cases", summary)?;
        }

        Ok(())
    }

    /// Run tests with existing Python files
    fn run_existing_file_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("📁 Running existing file tests...");

        let files = vec![
            "tests/python_files/simple.py",
            "tests/python_files/arithmetic.py",
            "tests/python_files/function.py",
            "tests/python_files/string_comparison_test.py",
        ];

        for file_path in files {
            if Path::new(file_path).exists() {
                let source = fs::read_to_string(file_path)
                    .map_err(|e| format!("Failed to read {}: {}", file_path, e))?;
                let test_name = format!(
                    "existing_{}",
                    Path::new(file_path).file_stem().unwrap().to_str().unwrap()
                );

                self.run_single_test(&test_name, &source, "Existing Files", summary)?;
            } else {
                println!("⚠️  File not found: {}", file_path);
            }
        }

        Ok(())
    }

    /// Run tests that are known to fail due to current limitations
    fn run_known_limitation_tests(&mut self, summary: &mut TestSuiteSummary) -> Result<(), String> {
        println!("🔍 Running known limitation tests...");

        let tests = vec![
            (
                "fstring_complex_expression",
                r#"
x = 10
y = 20
print(f"Complex: {(x + y) * 2 - 5}")
"#,
            ),
            (
                "nested_fstring",
                r#"
name = "Alice"
age = 25
print(f"{name} says: f'I am {age} years old'")
"#,
            ),
        ];

        for (name, source) in tests {
            self.run_single_test_expected_failure(name, source, "Known Limitations", summary)?;
        }

        Ok(())
    }

    /// Run a single test and record the result
    fn run_single_test(
        &mut self,
        name: &str,
        source: &str,
        category: &str,
        summary: &mut TestSuiteSummary,
    ) -> Result<(), String> {
        let result = self.tester.compare_outputs(source, name)?;
        let passed = result.outputs_match();

        let test_result = TestSuiteResult {
            name: name.to_string(),
            category: category.to_string(),
            passed,
            result: Some(result),
            expected_failure: false,
        };

        self.results.push(test_result.clone());
        summary.add_test(test_result.clone());

        if passed {
            println!("  ✅ {}", name);
        } else {
            println!("  ❌ {}", name);
            if let Some(comp_result) = &test_result.result {
                println!("     PyCC: {}", comp_result.pycc_output.trim());
                println!("     CPython: {}", comp_result.cpython_output.trim());
            }
        }

        Ok(())
    }

    /// Run a single test that is expected to fail
    fn run_single_test_expected_failure(
        &mut self,
        name: &str,
        source: &str,
        category: &str,
        summary: &mut TestSuiteSummary,
    ) -> Result<(), String> {
        let result = self.tester.compare_outputs(source, name)?;
        let passed = !result.outputs_match(); // Expected to fail, so pass when outputs don't match

        let test_result = TestSuiteResult {
            name: name.to_string(),
            category: category.to_string(),
            passed,
            result: Some(result),
            expected_failure: true,
        };

        self.results.push(test_result.clone());
        summary.add_test(test_result.clone());

        if passed {
            println!("  ✅ {} (expected failure)", name);
        } else {
            println!("  ❌ {} (unexpectedly passed)", name);
        }

        Ok(())
    }

    /// Print comprehensive test suite summary
    fn print_summary(&self, summary: &TestSuiteSummary) {
        println!("\n📊 Test Suite Summary");
        println!("====================");
        println!("Total tests: {}", summary.total_tests);
        println!("Passed: {}", summary.passed_tests);
        println!("Failed: {}", summary.failed_tests.len());
        println!("Success rate: {:.1}%", summary.success_rate());

        if !summary.failed_tests.is_empty() {
            println!("\n❌ Failed Tests:");
            for test in &summary.failed_tests {
                println!("  - {} ({})", test.name, test.category);
            }
        }

        if !summary.category_results.is_empty() {
            println!("\n📈 Results by Category:");
            for (category, result) in &summary.category_results {
                println!(
                    "  {}: {}/{} ({:.1}%)",
                    category,
                    result.passed,
                    result.total,
                    result.success_rate()
                );
            }
        }

        println!("\n🎯 Debug Print Test Suite Complete!");
    }

    /// Get detailed results for all tests
    pub fn get_results(&self) -> &[TestSuiteResult] {
        &self.results
    }

    /// Export results to text format
    pub fn export_results_text(&self, file_path: &str) -> Result<(), String> {
        let mut content = String::new();
        content.push_str("# Debug Print Test Results\n\n");

        for result in &self.results {
            content.push_str(&format!("## {}\n", result.name));
            content.push_str(&format!("Category: {}\n", result.category));
            content.push_str(&format!("Passed: {}\n", result.passed));
            content.push_str(&format!("Expected Failure: {}\n", result.expected_failure));

            if let Some(ref comp_result) = result.result {
                content.push_str(&format!(
                    "PyCC Output: {}\n",
                    comp_result.pycc_output.trim()
                ));
                content.push_str(&format!(
                    "CPython Output: {}\n",
                    comp_result.cpython_output.trim()
                ));
                content.push_str(&format!("Outputs Match: {}\n", comp_result.outputs_match()));
            }

            content.push_str("\n");
        }

        fs::write(file_path, content)
            .map_err(|e| format!("Failed to write results to {}: {}", file_path, e))?;

        println!("📄 Results exported to {}", file_path);
        Ok(())
    }
}

/// Summary of test suite results
#[derive(Debug)]
pub struct TestSuiteSummary {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: Vec<TestSuiteResult>,
    pub category_results: HashMap<String, CategoryResult>,
}

impl TestSuiteSummary {
    pub fn new() -> Self {
        TestSuiteSummary {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: Vec::new(),
            category_results: HashMap::new(),
        }
    }

    pub fn add_test(&mut self, test: TestSuiteResult) {
        self.total_tests += 1;

        if test.passed {
            self.passed_tests += 1;
        } else {
            self.failed_tests.push(test.clone());
        }

        // Update category results
        let category_result = self
            .category_results
            .entry(test.category.clone())
            .or_insert_with(|| CategoryResult::new(&test.category));

        category_result.add_test(test.passed);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0
        }
    }
}

/// Result for a specific test category
#[derive(Debug)]
pub struct CategoryResult {
    pub category: String,
    pub total: usize,
    pub passed: usize,
}

impl CategoryResult {
    pub fn new(category: &str) -> Self {
        CategoryResult {
            category: category.to_string(),
            total: 0,
            passed: 0,
        }
    }

    pub fn add_test(&mut self, passed: bool) {
        self.total += 1;
        if passed {
            self.passed += 1;
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }
}

/// Individual test result
#[derive(Debug, Clone)]
pub struct TestSuiteResult {
    pub name: String,
    pub category: String,
    pub passed: bool,
    pub result: Option<ComparisonResult>,
    pub expected_failure: bool,
}

/// Run the complete debug print test suite
pub fn run_debug_print_suite() -> Result<TestSuiteSummary, String> {
    let mut suite =
        DebugPrintSuite::new().map_err(|e| format!("Failed to create test suite: {}", e))?;

    suite.run_all_tests()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_print_suite_creation() {
        let suite = DebugPrintSuite::new();
        assert!(suite.is_ok());
    }

    #[test]
    fn test_run_basic_suite() {
        let mut suite = DebugPrintSuite::new().expect("Failed to create suite");
        let result = suite.run_all_tests();
        assert!(result.is_ok());

        let summary = result.unwrap();
        assert!(summary.total_tests > 0);
    }
}
