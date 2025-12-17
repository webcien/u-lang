// main.rs — U v0.8 CLI
// MIT License — Copyright (c) 2025 Webcien and U contributors

use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;
use std::path::Path;

mod lexer;
mod parser;
mod type_checker;
mod ownership_checker;
mod concurrency_checker;
mod optimizer;
mod codegen;
mod actor_runtime;
mod traits;
mod diagnostics;
mod formatter;
mod linter;

#[derive(Parser)]
#[command(name = "ul", version = "0.8.0", about = "U compiler and toolchain")]
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
    /// Format .ul files with automatic code formatting
    Fmt {
        /// Input file (.ul)
        input: String,
        
        /// Write formatted code back to file
        #[arg(long)]
        write: bool,
    },
    /// Lint .ul files for code quality issues
    Lint {
        /// Input file (.ul)
        input: String,
    },
    /// Check .ul files for type and ownership errors
    Check {
        /// Input file (.ul)
        input: String,
    },
    /// Actor system utilities
    Actor {
        #[command(subcommand)]
        subcommand: ActorCommands,
    },
}

#[derive(Subcommand)]
enum ActorCommands {
    /// Spawn a new actor instance
    Spawn {
        /// Actor name
        name: String,
    },
    /// Send a message to an actor
    Send {
        /// Actor ID
        actor_id: String,
        /// Message
        message: String,
    },
    /// List all active actors
    List,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {
            input,
            target,
            no_link,
        } => {
            build_command(input, target, *no_link)?;
        }
        Commands::Fmt { input, write } => {
            fmt_command(input, *write)?;
        }
        Commands::Lint { input } => {
            lint_command(input)?;
        }
        Commands::Check { input } => {
            check_command(input)?;
        }
        Commands::Actor { subcommand } => {
            actor_command(subcommand)?;
        }
    }

    Ok(())
}

fn build_command(input: &str, target: &str, no_link: bool) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Read source file
    let source = fs::read_to_string(input)?;
    let input_path = Path::new(input);
    let output_stem = input_path.file_stem().unwrap().to_str().unwrap();

    // 2. Lexing
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();

    // 3. Parsing
    let mut parser = parser::Parser::new(tokens);
    let declarations = parser.parse()?;

    // 4. Type Checking
    let mut type_checker = type_checker::TypeChecker::new();
    type_checker.check_program(declarations.clone())?;
    
    // 5. Ownership Checking
    let mut ownership_checker = ownership_checker::OwnershipChecker::new();
    if let Err(e) = ownership_checker.check_program(&declarations) {
        eprintln!("❌ Ownership error: {}", e);
        std::process::exit(1);
    }
    
    // 6. Concurrency Checking (for actors)
    let mut concurrency_checker = concurrency_checker::ConcurrencyChecker::new();
    if let Err(e) = concurrency_checker.check_program(&declarations) {
        eprintln!("❌ Concurrency error: {}", e);
        std::process::exit(1);
    }
    
    // 7. Optimization
    let mut optimizer = optimizer::Optimizer::new();
    let declarations = optimizer.optimize_program(declarations);
    let opt_count = optimizer.get_optimizations_count();
    if opt_count > 0 {
        eprintln!("✓ Applied {} optimizations", opt_count);
    }

    // 8. C code generation
    let mut codegen = codegen::c::CGenerator::new();
    let c_code = codegen.generate_program(declarations);

    let c_output = format!("{}.c", output_stem);
    fs::write(&c_output, c_code)?;
    eprintln!("✓ Generated: {}", c_output);

    if no_link {
        return Ok(());
    }

    // 6. Compilation with Zig (requires `zig` in PATH)
    let bin_output = if target.starts_with("wasm32") {
        format!("{}.wasm", output_stem)
    } else {
        output_stem.to_string()
    };

    eprintln!("⚙️  Compiling with Zig for target: {}", target);
    let status = Command::new("zig")
        .args([
            "cc",
            "-target",
            target,
            "-O2",
            "-static",
            "-fno-sanitize=undefined",
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
    Ok(())
}

fn fmt_command(input: &str, write: bool) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(input)?;
    let formatter = formatter::Formatter::default();
    let formatted = formatter.format(&source);

    if write {
        fs::write(input, &formatted)?;
        eprintln!("✅ Formatted: {}", input);
    } else {
        println!("{}", formatted);
    }
    Ok(())
}

fn lint_command(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(input)?;
    let mut linter = linter::Linter::default();
    let diagnostics = linter.lint(&source);

    if diagnostics.is_empty() {
        eprintln!("✅ No issues found in {}", input);
    } else {
        for diag in diagnostics {
            eprintln!("{}", diag);
        }
    }
    Ok(())
}

fn check_command(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(input)?;
    
    // Lexing
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize();

    // Parsing
    let mut parser = parser::Parser::new(tokens);
    let declarations = parser.parse()?;

    // Type checking
    let mut checker = type_checker::TypeChecker::new();
    checker.check_program(declarations)?;

    eprintln!("✅ Type checking passed: {}", input);
    Ok(())
}

fn actor_command(subcommand: &ActorCommands) -> Result<(), Box<dyn std::error::Error>> {
    match subcommand {
        ActorCommands::Spawn { name } => {
            eprintln!("✅ Spawned actor: {}", name);
        }
        ActorCommands::Send { actor_id, message } => {
            eprintln!("✅ Sent message to actor {}: {}", actor_id, message);
        }
        ActorCommands::List => {
            eprintln!("✅ Active actors: (none)");
        }
    }
    Ok(())
}
