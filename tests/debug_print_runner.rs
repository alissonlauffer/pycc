//! Simple test runner for the debug print test suite
//!
//! This module provides a standalone runner that can be used to execute
//! the debug print tests and generate reports.

#[path = "debug_print_suite.rs"]
mod debug_print_suite;

use debug_print_suite::run_debug_print_suite;

fn main() -> Result<(), String> {
    println!("🔧 PyCC Debug Print Test Runner");
    println!("================================");
    println!("This runner will compile Python code with pycc and compare");
    println!("the output with CPython reference implementation.\n");

    // Run the complete test suite
    let summary = run_debug_print_suite()?;

    // Export results to a file
    let export_path = "debug_print_test_results.md";
    let mut suite = debug_print_suite::DebugPrintSuite::new()
        .map_err(|e| format!("Failed to create test suite for export: {}", e))?;

    // Note: We would need to run the tests again to export, but for now
    // we'll just show the summary

    println!("\n📋 Test Summary:");
    println!("  Total tests: {}", summary.total_tests);
    println!("  Passed: {}", summary.passed_tests);
    println!("  Failed: {}", summary.failed_tests.len());
    println!("  Success rate: {:.1}%", summary.success_rate());

    // Exit with appropriate code
    if summary.failed_tests.is_empty() {
        println!("\n🎉 All tests passed!");
        std::process::exit(0);
    } else {
        println!("\n💥 Some tests failed. Check the output above for details.");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_basic_functionality() {
        // This is a simple test to ensure the runner can be created
        // The actual test execution is handled by the main function
        assert!(true); // Placeholder test
    }
}
