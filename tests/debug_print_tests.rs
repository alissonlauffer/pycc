use inkwell::context::Context;
use pycc::codegen::CodeGenerator;
use pycc::lexer::Lexer;
use pycc::parser::Parser;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Test utility for comparing pycc output with CPython output
pub struct DebugPrintTester {
    temp_dir: TempDir,
}

impl DebugPrintTester {
    /// Create a new debug print tester with a temporary directory
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        Ok(DebugPrintTester { temp_dir })
    }

    /// Compile Python source code with pycc and return the path to the executable
    pub fn compile_with_pycc(&self, source: &str, executable_name: &str) -> Result<String, String> {
        // Write source to temporary file
        let source_path = self.temp_dir.path().join(format!("{}.py", executable_name));
        fs::write(&source_path, source)
            .map_err(|e| format!("Failed to write source file: {}", e))?;

        // Parse the program
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        // Generate LLVM IR
        let context = Context::create();
        let mut codegen = CodeGenerator::new(&context, "pycc_module");

        codegen
            .compile(&program)
            .map_err(|e| format!("Failed to compile to LLVM IR: {}", e))?;

        // Generate object file
        let object_file_name = format!("{}.o", executable_name);
        let object_path = self.temp_dir.path().join(&object_file_name);

        codegen
            .write_object_to_file(object_path.to_str().unwrap())
            .map_err(|e| format!("Failed to generate object file: {}", e))?;

        // Link object file to create executable
        let executable_path = self.temp_dir.path().join(executable_name);
        let output = Command::new("cc")
            .args([
                object_path.to_str().unwrap(),
                "-o",
                executable_path.to_str().unwrap(),
                "-no-pie",
            ])
            .output()
            .map_err(|e| format!("Failed to execute linker: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Linking failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(executable_path.to_str().unwrap().to_string())
    }

    /// Execute Python code with CPython and return the output
    pub fn execute_with_cpython(&self, source: &str) -> Result<String, String> {
        // Write source to temporary file
        let source_path = self.temp_dir.path().join("test_cpython.py");
        fs::write(&source_path, source)
            .map_err(|e| format!("Failed to write source file: {}", e))?;

        // Execute with CPython
        let output = Command::new("python3")
            .arg(source_path)
            .output()
            .map_err(|e| format!("Failed to execute CPython: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "CPython execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Execute compiled code and return the output
    pub fn execute_compiled(&self, executable_path: &str) -> Result<String, String> {
        let output = Command::new(executable_path)
            .output()
            .map_err(|e| format!("Failed to execute compiled program: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Compiled program execution failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Compare outputs from pycc and CPython for given source code
    pub fn compare_outputs(
        &self,
        source: &str,
        test_name: &str,
    ) -> Result<ComparisonResult, String> {
        // Compile with pycc
        let executable_path = self.compile_with_pycc(source, test_name)?;

        // Execute with pycc
        let pycc_output = self.execute_compiled(&executable_path)?;

        // Execute with CPython
        let cpython_output = self.execute_with_cpython(source)?;

        Ok(ComparisonResult {
            test_name: test_name.to_string(),
            pycc_output,
            cpython_output,
            source: source.to_string(),
        })
    }

    /// Assert that pycc and CPython outputs match
    pub fn assert_outputs_match(&self, source: &str, test_name: &str) -> Result<(), String> {
        let result = self.compare_outputs(source, test_name)?;

        if result.pycc_output.trim() == result.cpython_output.trim() {
            Ok(())
        } else {
            Err(format!(
                "Output mismatch for test '{}':\n\
                 PyCC output:\n{}\n\
                 CPython output:\n{}\n\
                 Source code:\n{}",
                result.test_name, result.pycc_output, result.cpython_output, result.source
            ))
        }
    }
}

/// Result of comparing pycc and CPython outputs
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    pub test_name: String,
    pub pycc_output: String,
    pub cpython_output: String,
    pub source: String,
}

impl ComparisonResult {
    /// Check if outputs match (ignoring trailing whitespace)
    pub fn outputs_match(&self) -> bool {
        self.pycc_output.trim() == self.cpython_output.trim()
    }

    /// Print detailed comparison information
    pub fn print_comparison(&self) {
        println!("Test: {}", self.test_name);
        println!("Source code:\n{}\n", self.source);
        println!("PyCC output:\n{}", self.pycc_output);
        println!("CPython output:\n{}", self.cpython_output);
        println!("Match: {}\n", self.outputs_match());
    }
}
