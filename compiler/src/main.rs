// main.rs — U v0.6 CLI
// MIT License — Copyright (c) 2025 Webcien and U contributors

use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

mod lexer;
mod parser;
mod type_checker;
mod codegen;
mod actor_runtime;
mod traits;
mod diagnostics;
mod formatter;
mod linter;

#[derive(Parser)]
#[command(name = "u", version = "0.6.0", about = "U compiler")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compiles a .ul file to binary (with Zig) or to C
    Build {
        /// Input file (.ul)
        input: String,

        /// Zig target (e.g: x86_64-linux-musl, wasm32-wasi)
        #[arg(long, default_value = "x86_64-linux-musl")]
        target: String,

        /// Only generate C code (do not compile with Zig)
        #[arg(long)]
        no_link: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {
            input,
            target,
            no_link,
        } => {
            // 1. Read source file
            let source = fs::read_to_string(input)?;
            let input_path = std::path::Path::new(input);
            let output_stem = input_path.file_stem().unwrap().to_str().unwrap();

            // 2. Lexing
            let mut lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize();

            // 3. Parsing
            let mut parser = parser::Parser::new(tokens);
            let declarations = parser.parse()?;

            // 4. Type & Ownership Checking
            let mut checker = type_checker::TypeChecker::new();
            checker.check_program(declarations.clone())?;

            // 5. C code generation
            let mut codegen = codegen::c::CGenerator::new();
            let c_code = codegen.generate_program(declarations);

            let c_output = format!("{}.c", output_stem);
            fs::write(&c_output, c_code)?;
            eprintln!("✓ Generated: {}", c_output);

            if *no_link {
                return Ok(());
            }

            // 6. Compilation with Zig (requires `zig` in PATH)
            let bin_output = if target.starts_with("wasm32") {
                format!("{}.wasm", output_stem)
            } else {
                output_stem.to_string()
            };

            eprintln!("⚙️ Compiling with Zig for target: {}", target);
            let status = Command::new("zig")
                .args([
                    "cc",
                    "-target",
                    target,
                    "-O2",
                    "-static",
                    "-fno-sanitize=undefined", // to avoid UB checks in C
                    &c_output,
                    "-o",
                    &bin_output,
                ])
                .status()?;

            if !status.success() {
                eprintln!("❌ Error compiling with Zig");
                std::process::exit(1);
            }

            eprintln!("✅ Binary generated: {}", bin_output);
        }
    }

    Ok(())
}
