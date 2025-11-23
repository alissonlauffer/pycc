pub mod ast;
pub mod cli;
pub mod codegen;
pub mod lexer;
pub mod parser;

// Re-export commonly used items
pub use ast::*;
pub use codegen::CodeGenerator;
pub use lexer::Lexer;
pub use parser::Parser;
