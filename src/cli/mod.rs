use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pycc")]
#[command(about = "A Python compiler", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile a Python file to LLVM IR or executable
    Compile {
        /// Input file to compile
        #[arg(value_name = "FILE")]
        input_file: PathBuf,

        /// Output file name
        #[arg(short, long, value_name = "OUTPUT")]
        output: Option<PathBuf>,

        /// Emit LLVM IR instead of executable
        #[arg(long)]
        emit_llvm: bool,

        /// Optimization level (0-3)
        #[arg(short = 'O', long, value_name = "LEVEL", default_value = "0")]
        optimization: u8,
    },
}
