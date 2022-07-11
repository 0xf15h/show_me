use clap::{Parser, Subcommand};
use std::error::{Error};
use std::{u64};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Output value in decimal representation (i.e. base-10)
    #[clap(short, long, action)]
    decimal: bool,

    /// Output value in octal representation (i.e. base-8)
    #[clap(short, long, action)]
    octal: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Pretty print the bits of the input value.
    Bits {
        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,

        /// Number of bits to chunk the input by: 1, 2, or 4.
        #[clap(default_value_t = 4, value_parser = clap::value_parser!(u8))]
        chunk: u8,
    },

    /// Show the signed representation of the input value.
    Signed {
        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },
}

enum OutputFormat {
    Hexidecimal,
    Octal,
    Decimal,
}

fn value_from_string(input: String) -> Result<u64, Box<dyn Error>> {
    let prefix: String = input.chars().take(2).collect();
    let mut formatted_input = input.clone();

    // If octal or hex is used, remove the prefix for parsing
    let mut radix = 10;
    if prefix == "0x" {
        radix = 16;
        formatted_input = input.chars().skip(2).collect();
    } else if prefix == "0o" {
        radix = 8;
        formatted_input = input.chars().skip(2).collect();
    }

    let input_val = u64::from_str_radix(&formatted_input, radix)?;

    Ok(input_val)
}

fn show_me_bits(input: &String, chunk_size: u8) {
    let input_val = value_from_string(input.to_string()).unwrap();
    println!("Input: 0x{:x}", input_val);
}

fn show_me_signed(input: &String, output_format: OutputFormat) {}

fn main() {
    let cli = Cli::parse();

    if cli.decimal && cli.octal {
        println!("Cannot specify both decimal and octal output");
        return;
    }

    let mut output_format = OutputFormat::Hexidecimal;
    if cli.decimal {
        output_format = OutputFormat::Decimal;
    } else if cli.octal {
        output_format = OutputFormat::Octal;
    }

    match &cli.command {
        Commands::Bits { input, chunk } => {
            if !vec![1, 2, 4].contains(chunk) {
                println!("Can only chunk bits by 1, 2, or 4.");
                return;
            }

            show_me_bits(input, *chunk);
        }
        Commands::Signed { input } => {
            show_me_signed(input, output_format);
        }
    }
}
