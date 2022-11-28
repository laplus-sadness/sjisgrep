mod encoding;
mod search;

use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::Path;

use encoding::Encoding;

#[derive(Parser)]
#[command(author, long_about = None)]
#[command(version = "0.1.0")]
#[command(about = "Binary grep with native support for searching in different Japanese encodings")]
struct Cli {
    /// Return the address related to the first character of the string found, not the pattern
    #[arg(short, long)]
    beginning: bool,

    /// Pattern that will be searched in the binary
    pattern: String,

    /// Path of the binary
    file: String,

    /// Encoding of the pattern
    #[arg(value_enum, value_name = "ENCODING", default_value_t = Encoding::ShiftJis)]
    enc: Encoding,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let bytes = fs::read(Path::new(&cli.file))?;
    let pattern = cli.enc.encode(&cli.pattern)?;

    if let Some(matches) = search::grep(&bytes, &pattern) {
        for val in matches {
            println!("{}", cli.enc.display(val, &bytes, cli.beginning)?);
        }
    }

    Ok(())
}
