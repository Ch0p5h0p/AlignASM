use clap::Parser;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

mod parser;
mod format;

#[derive(Parser, Debug)]
#[command(
    name = "alignasm",
    version = "1.0",
    about = "A column-organized formatter for NASM"
)]
struct ArgSet {
    /// Overwrite the file in-place with formatted code
    #[arg(long, short = 'i', conflicts_with = "check", value_name = "FILE")]
    in_place: Option<PathBuf>,

    /// Check if formatting changes are needed. Exits with non-zero if changes are needed
    #[arg(long, short = 'c', conflicts_with = "in_place", value_name = "FILE")]
    check: Option<PathBuf>,

    /// Read from a file and print to stdout
    /// Ignored if --in-place or --check are used
    #[arg(value_name = "FILE", conflicts_with_all = ["in_place", "check"])]
    file: Option<PathBuf>,

    //TODO: replace clap-generated --help with a custom guide
}

fn main() -> io::Result<()> {
    let args = ArgSet::parse();

    let (raw_content, target_file, check, in_place) = if let Some(path) = args.in_place {
        let content = std::fs::read_to_string(&path)?;
        (content, Some(path), false, true)
    } else if let Some(path) = args.check {
        let content = std::fs::read_to_string(&path)?;
        (content, Some(path), true, false)
    } else if let Some(path) = args.file {
        let content = std::fs::read_to_string(&path)?;
        (content, None, false, false)
    } else {
        // Read from STDIN
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        (content, None, false, false)
    };

    let formatted = format::format_lines(parser::parse_asm(raw_content.clone()));

    if check {
        if formatted == raw_content {
            println!("Properly formatted.");
            process::exit(0)
        } else {
            println!("Improperly formatted.");
            process::exit(1)
        }
    } else if in_place {
        std::fs::write(target_file.unwrap(), formatted)?;
    } else {
        print!("{formatted}");
    }
    Ok(())
}
