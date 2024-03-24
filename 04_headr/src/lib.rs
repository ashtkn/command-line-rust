use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of `head`
pub struct Args {
    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long,
        default_value = "10",
        value_name = "LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

pub fn get_args() -> Result<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(config: Args) -> Result<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                if let Some(num_bytes) = config.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]),)
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
