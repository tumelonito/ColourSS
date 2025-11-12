use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colourss::parse_color;
use std::fs;
use std::path::PathBuf;

/// A simple CSS color parser CLI
#[derive(Parser, Debug)]
#[command(author, version, about = "ColourSS - A simple CSS color parser CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Defines the subcommands for the CLI
#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a file line by line
    Parse {
        /// The file to parse
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
    /// Shows author and license info
    Credits,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { file } => {
            parse_file(file)?;
        }
        Commands::Credits => {
            show_credits();
        }
    }

    Ok(())
}

// this function handles reading the file and parsing each line
fn parse_file(file_path: PathBuf) -> Result<()> {
    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Could not read file `{:?}`", file_path))?;

    println!("Parsing file: {:?}...", file_path);

    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, line) in content.lines().enumerate() {
        let line_num = i + 1;
        if line.trim().is_empty() {
            continue; // skip empty lines
        }

        match parse_color(line) {
            Ok(color) => {
                println!(
                    "  [Line {}] OK: '{}' -> Color(r: {}, g: {}, b: {})",
                    line_num, line, color.r, color.g, color.b
                );
                success_count += 1;
            }
            Err(e) => {
                println!("  [Line {}] FAIL: '{}' -> Error: {}", line_num, line, e);
                fail_count += 1;
            }
        }
    }

    println!(
        "\nParsing complete. {} successful, {} failed.",
        success_count, fail_count
    );
    Ok(())
}

fn show_credits() {
    println!("--- ColourSS v0.1.0 ---");
    println!("Written by: Maister Danylo");
    println!("License: MIT");
}