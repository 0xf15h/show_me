use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
struct Cli{
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

fn show_me_bits(input: &String, chunk_size: u8) {
}

fn show_me_signed(input: &String, output_format: OutputFormat) {
}

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
        Commands::Bits{ input, chunk } => {
            if !vec![1, 2, 4].contains(chunk) {
                println!("Can only chunk bits by 1, 2, or 4.");
                return;
            }

            show_me_bits(input, *chunk);
        }
        Commands::Signed{ input } => {
            show_me_signed(input, output_format);
        }
    }
}
