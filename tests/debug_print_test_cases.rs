#[path = "debug_print_tests.rs"]
mod debug_print_tests;

use debug_print_tests::DebugPrintTester;

// Basic print tests
#[test]
fn test_print_integer() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(42)", "test_print_integer")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_negative_integer() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(-42)", "test_print_negative_integer")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_zero() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(0)", "test_print_zero")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_float() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(3.14)", "test_print_float")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_negative_float() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(-2.71)", "test_print_negative_float")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_zero_float() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(0.0)", "test_print_zero_float")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_string() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(\"Hello, World!\")", "test_print_string")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_empty_string() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(\"\")", "test_print_empty_string")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_string_with_spaces() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(\"  spaced out  \")", "test_print_string_with_spaces")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_true() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(True)", "test_print_true")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_print_false() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    tester
        .assert_outputs_match("print(False)", "test_print_false")
        .expect("Output mismatch between PyCC and CPython");
}

// Variable assignment and printing
#[test]
fn test_variable_assignment_integer() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = 42
print(x)
"#;
    tester
        .assert_outputs_match(source, "test_variable_assignment_integer")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_variable_assignment_float() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
y = 3.14159
print(y)
"#;
    tester
        .assert_outputs_match(source, "test_variable_assignment_float")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_variable_assignment_string() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
name = "Python"
print(name)
"#;
    tester
        .assert_outputs_match(source, "test_variable_assignment_string")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_multiple_variables() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 10
b = 20
c = 30
print(a)
print(b)
print(c)
"#;
    tester
        .assert_outputs_match(source, "test_multiple_variables")
        .expect("Output mismatch between PyCC and CPython");
}

// Arithmetic operations
#[test]
fn test_arithmetic_addition() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 10
b = 5
result = a + b
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_arithmetic_addition")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_arithmetic_subtraction() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 10
b = 5
result = a - b
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_arithmetic_subtraction")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_arithmetic_multiplication() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 6
b = 7
result = a * b
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_arithmetic_multiplication")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_arithmetic_division() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 20
b = 4
result = a / b
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_arithmetic_division")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_complex_arithmetic() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = 10
y = 20
z = (x + y) * 2 - 5
print(z)
"#;
    tester
        .assert_outputs_match(source, "test_complex_arithmetic")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_operator_precedence() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
result = 2 + 3 * 4
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_operator_precedence")
        .expect("Output mismatch between PyCC and CPython");
}

// Function definitions and calls
#[test]
fn test_simple_function() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
def add(x, y):
    return x + y

result = add(3, 4)
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_simple_function")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_function_with_multiple_calls() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
def multiply(a, b):
    return a * b

print(multiply(2, 3))
print(multiply(4, 5))
print(multiply(6, 7))
"#;
    tester
        .assert_outputs_match(source, "test_function_with_multiple_calls")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_nested_function_calls() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
def add_one(x):
    return x + 1

def add_two(x):
    return add_one(add_one(x))

result = add_two(5)
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_nested_function_calls")
        .expect("Output mismatch between PyCC and CPython");
}

// String operations
#[test]
fn test_string_concatenation_variables() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
first = "Hello"
second = "World"
print(first)
print(second)
"#;
    tester
        .assert_outputs_match(source, "test_string_concatenation_variables")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_string_with_numbers() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
age = 25
name = "Alice"
print(name)
print(age)
"#;
    tester
        .assert_outputs_match(source, "test_string_with_numbers")
        .expect("Output mismatch between PyCC and CPython");
}

// Boolean operations
#[test]
fn test_boolean_variables() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
is_true = True
is_false = False
print(is_true)
print(is_false)
"#;
    tester
        .assert_outputs_match(source, "test_boolean_variables")
        .expect("Output mismatch between PyCC and CPython");
}

// Mixed type operations
#[test]
fn test_mixed_assignments() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
integer_var = 42
float_var = 3.14
string_var = "test"
bool_var = True
print(integer_var)
print(float_var)
print(string_var)
print(bool_var)
"#;
    tester
        .assert_outputs_match(source, "test_mixed_assignments")
        .expect("Output mismatch between PyCC and CPython");
}

// F-string tests
#[test]
fn test_simple_fstring() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
name = "Alice"
print(f"Hello, {name}!")
"#;
    tester
        .assert_outputs_match(source, "test_simple_fstring")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_fstring_with_number() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
age = 25
print(f"Age: {age}")
"#;
    tester
        .assert_outputs_match(source, "test_fstring_with_number")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_fstring_multiple_variables() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
name = "Bob"
age = 30
city = "New York"
print(f"{name} is {age} years old and lives in {city}")
"#;
    tester
        .assert_outputs_match(source, "test_fstring_multiple_variables")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_fstring_with_expressions() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = 10
y = 20
print(f"Sum: {x + y}")
print(f"Product: {x * y}")
"#;
    tester
        .assert_outputs_match(source, "test_fstring_with_expressions")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_fstring_mixed_content() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
name = "Charlie"
score = 95.5
print(f"Student {name} scored {score} points")
"#;
    tester
        .assert_outputs_match(source, "test_fstring_mixed_content")
        .expect("Output mismatch between PyCC and CPython");
}

// Complex scenarios
#[test]
fn test_function_with_fstring() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
def greet(name):
    return f"Hello, {name}!"

message = greet("World")
print(message)
"#;
    tester
        .assert_outputs_match(source, "test_function_with_fstring")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_arithmetic_with_fstring_output() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
a = 15
b = 3
print(f"{a} + {b} = {a + b}")
print(f"{a} * {b} = {a * b}")
"#;
    tester
        .assert_outputs_match(source, "test_arithmetic_with_fstring_output")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_multiple_prints_different_types() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
print("Starting program...")
x = 42
print(f"The answer is {x}")
print("Program finished.")
"#;
    tester
        .assert_outputs_match(source, "test_multiple_prints_different_types")
        .expect("Output mismatch between PyCC and CPython");
}

// Edge cases
#[test]
fn test_large_numbers() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
big_int = 9223372036854775807
print(big_int)
"#;
    tester
        .assert_outputs_match(source, "test_large_numbers")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_negative_arithmetic() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = -10
y = -5
result = x + y
print(result)
"#;
    tester
        .assert_outputs_match(source, "test_negative_arithmetic")
        .expect("Output mismatch between PyCC and CPython");
}

#[test]
fn test_zero_operations() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = 0
y = 10
print(x + y)
print(y - x)
print(x * y)
"#;
    tester
        .assert_outputs_match(source, "test_zero_operations")
        .expect("Output mismatch between PyCC and CPython");
}

// Test with existing Python files
#[test]
fn test_existing_simple_file() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source =
        std::fs::read_to_string("tests/python_files/simple.py").expect("Failed to read simple.py");
    tester
        .assert_outputs_match(&source, "existing_simple_file")
        .expect("Output mismatch for simple.py");
}

#[test]
fn test_existing_arithmetic_file() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = std::fs::read_to_string("tests/python_files/arithmetic.py")
        .expect("Failed to read arithmetic.py");
    tester
        .assert_outputs_match(&source, "existing_arithmetic_file")
        .expect("Output mismatch for arithmetic.py");
}

#[test]
fn test_existing_function_file() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = std::fs::read_to_string("tests/python_files/function.py")
        .expect("Failed to read function.py");
    tester
        .assert_outputs_match(&source, "existing_function_file")
        .expect("Output mismatch for function.py");
}

#[test]
fn test_existing_string_comparison_file() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = std::fs::read_to_string("tests/python_files/string_comparison_test.py")
        .expect("Failed to read string_comparison_test.py");
    tester
        .assert_outputs_match(&source, "existing_string_comparison_file")
        .expect("Output mismatch for string_comparison_test.py");
}

// Test that demonstrates current limitations (expected to fail)
#[test]
fn test_fstring_complex_expression() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
x = 10
y = 20
print(f"Complex: {(x + y) * 2 - 5}")
"#;

    let result = tester
        .compare_outputs(source, "test_fstring_complex_expression")
        .expect("Failed to compare outputs");

    // We expect this to fail, so we assert that outputs don't match
    assert!(
        !result.outputs_match(),
        "Expected outputs to differ, but they matched:\nPyCC: {}\nCPython: {}",
        result.pycc_output,
        result.cpython_output
    );

    // Print the comparison for debugging
    result.print_comparison();
}

#[test]
fn test_nested_fstring() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");
    let source = r#"
name = "Alice"
age = 25
print(f"{name} says: f'I am {age} years old'")
"#;
    tester
        .assert_outputs_match(source, "test_nested_fstring")
        .expect("Output mismatch between PyCC and CPython");
}

// Performance and stress tests
#[test]
fn test_many_prints() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");

    let mut source = String::new();
    for i in 0..10 {
        source.push_str(&format!("print({})\n", i));
    }

    tester
        .assert_outputs_match(&source, "many_prints")
        .expect("Output mismatch for many prints test");
}

#[test]
fn test_large_program() {
    let tester = DebugPrintTester::new().expect("Failed to create debug print tester");

    let source = r#"
# A larger program with multiple constructs
def calculate_area(length, width):
    return length * width

def calculate_perimeter(length, width):
    return 2 * (length + width)

# Test data
length = 10
width = 5

# Calculations
area = calculate_area(length, width)
perimeter = calculate_perimeter(length, width)

# Output results
print(f"Length: {length}")
print(f"Width: {width}")
print(f"Area: {area}")
print(f"Perimeter: {perimeter}")
"#;

    tester
        .assert_outputs_match(source, "large_program")
        .expect("Output mismatch for large program test");
}
