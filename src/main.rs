mod ast;
mod cli;
mod codegen;
mod interpreter;
mod lexer;
mod parser;

use clap::Parser as ClapParser;
use cli::{Cli, Commands};
use codegen::CodeGenerator;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser as PyParser;
use std::fs;
use std::process;
use std::process::Command;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { input_file } => {
            let input = match fs::read_to_string(&input_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {input_file:?}: {e}");
                    process::exit(1);
                }
            };

            let lexer = Lexer::new(&input);
            let mut py_parser = PyParser::new(lexer);
            let ast = py_parser.parse_program();

            // Interpret the AST
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(&ast) {
                Ok(_) => {
                    // Print the interpreter's output
                    let output = interpreter.get_output();
                    if !output.is_empty() {
                        println!("{output}");
                    }
                }
                Err(e) => println!("Error: {e}"),
            }
        }
        Commands::Compile {
            input_file,
            output,
            emit_llvm,
            optimization,
        } => {
            let input = match fs::read_to_string(&input_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {input_file:?}: {e}");
                    process::exit(1);
                }
            };

            let lexer = Lexer::new(&input);
            let mut py_parser = PyParser::new(lexer);
            let ast = py_parser.parse_program();

            // Generate LLVM IR
            let context = inkwell::context::Context::create();
            let mut codegen = CodeGenerator::new(&context, "pycc_module");

            match codegen.compile(&ast) {
                Ok(_) => {
                    if emit_llvm {
                        // Print IR to stdout or write to file
                        if let Some(output_file) = output {
                            match codegen
                                .write_ir_to_file(output_file.to_str().unwrap_or("output.ll"))
                            {
                                Ok(_) => println!("IR written to {output_file:?}"),
                                Err(e) => eprintln!("Error writing IR to file: {e}"),
                            }
                        } else {
                            codegen.print_ir();
                        }
                    } else {
                        // Compile to executable
                        let output_file_name = if let Some(output_file) = output {
                            output_file.to_str().unwrap_or("a.out").to_string()
                        } else {
                            "a.out".to_string()
                        };

                        // Generate object file
                        let object_file_name = format!("{output_file_name}.o");
                        match codegen.write_object_to_file(&object_file_name) {
                            Ok(_) => {
                                // Link object file to create executable
                                match Command::new("cc")
                                    .args([&object_file_name, "-o", &output_file_name, "-no-pie"])
                                    .status()
                                {
                                    Ok(status) => {
                                        if status.success() {
                                            println!(
                                                "Successfully compiled to executable: {output_file_name}"
                                            );

                                            // Clean up object file
                                            if std::fs::remove_file(&object_file_name).is_err() {
                                                eprintln!(
                                                    "Warning: Failed to remove temporary object file: {object_file_name}"
                                                );
                                            }
                                        } else {
                                            eprintln!("Error: Linking failed");
                                            process::exit(1);
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to execute linker: {e}");
                                        process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error generating object file: {e}");
                                process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error compiling to LLVM IR: {e}");
                    process::exit(1);
                }
            }
        }
    }
}
