use clap::{Parser, Subcommand};
use std::error::Error;
use std::u64;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pretty print the bits of the input value.
    Bits {
        /// Number of bits to chunk the input by: 1, 2, or 4.
        #[clap(value_parser = clap::value_parser!(u8))]
        chunk: u8,

        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },

    /// Show the signed decimal representation of the input value.
    Signed {
        /// Number of bits in the signed value: 8, 16, 32, or 64.
        #[clap(value_parser = clap::value_parser!(u8))]
        bits: u8,

        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },

    /// Show the hexidecimal representation of the input value (base-16).
    Hex {
        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },

    /// Show the decimal representation of the input value (base-10).
    Decimal {
        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },

    /// Show the octal representation of the input value (base-8).
    Octal {
        /// The base is determined by the value's prefix. E.g. 0oXX for octal, 0xXX for hexidecimal, and no prefix for
        /// decimal.
        #[clap(value_parser)]
        input: String,
    },
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

    // Deduce the number of bits need to represent the input value
    let u32_max = (1u64 << 32) - 1;
    let num_bits: u64 = if input_val > u32_max { 64u64 } else { 32u64 };

    // Table with chunk size 1
    //
    // 32  31  30
    //   +---+---+
    //   | 0 | 0 |
    //   +---+---+
    //
    // Table with chunk size 2
    //
    // 32   30   28
    //   +----+----+
    //   | 00 | 00 |
    //   +----+----+
    //
    // Table with chunk size 4
    //
    // 32     28     24
    //   +------+------+
    //   | 0000 | 0000 |
    //   +------+------+

    // Write the table header index values
    let mut table = String::new();
    for header_idx in (0..num_bits + 1).rev().step_by(chunk_size as usize) {
        if chunk_size == 1u8 {
            table.push_str(&format!("{: >3}|", header_idx).to_string());
        } else if chunk_size == 2u8 {
            table.push_str(&format!("{: >4}|", header_idx).to_string());
        } else {
            table.push_str(&format!("{: >6}|", header_idx).to_string());
        };
    }
    table.push_str("\n");

    let table_line = if chunk_size == 1u8 {
        let raw_line = "+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---";
        if num_bits == 64 {
            format!("   {}{}+\n   ", raw_line, raw_line)
        } else {
            format!("   {}+\n   ", raw_line)
        }
    } else if chunk_size == 2u8 {
        let raw_line =
            "+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----+----";
        if num_bits == 64 {
            format!("    {}{}+\n    ", raw_line, raw_line)
        } else {
            format!("    {}+\n    ", raw_line)
        }
    } else {
        let raw_line = "+------+------+------+------+------+------+------+------";
        if num_bits == 64 {
            format!("      {}{}+\n      ", raw_line, raw_line)
        } else {
            format!("      {}+\n      ", raw_line)
        }
    };
    table.push_str(&table_line);

    // Write each value
    for bit_idx in (0..(num_bits - chunk_size as u64) + 1)
        .rev()
        .step_by(chunk_size as usize)
    {
        if chunk_size == 1u8 {
            table.push_str(&format!("| {:b} ", ((input_val >> bit_idx) & 0b1)).to_string());
        } else if chunk_size == 2u8 {
            table.push_str(&format!("| {:0>2b} ", ((input_val >> bit_idx) & 0b11)).to_string());
        } else {
            table.push_str(&format!("| {:0>4b} ", ((input_val >> bit_idx) & 0b1111)).to_string());
        }
    }
    table.push_str("|\n");

    table.push_str(&table_line);

    println!("{}", table);
}

fn show_me_signed(input: &String, bits: &u8) {
    let input_val = value_from_string(input.to_string()).unwrap();

    match bits {
        8 => {
            println!("{}", input_val as i8);
        }
        16 => {
            println!("{}", input_val as i16);
        }
        32 => {
            println!("{}", input_val as i32);
        }
        64 => {
            println!("{}", input_val as i64);
        }
        _ => {
            panic!("Bits value must be 8, 16, 32, or 64");
        }
    }
}

fn show_me_hex(input: &String) {
    let input_val = value_from_string(input.to_string()).unwrap();

    let u32_max = (1u64 << 32) - 1;
    if input_val > u32_max {
        println!("0x{:x}", input_val);
    } else {
        println!("0x{:x}", input_val as u32);
    }
}

fn show_me_decimal(input: &String) {
    let input_val = value_from_string(input.to_string()).unwrap();
    println!("{}", input_val);
}

fn show_me_octal(input: &String) {
    let input_val = value_from_string(input.to_string()).unwrap();
    println!("{:0}", input_val);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bits { input, chunk } => {
            if !vec![1, 2, 4].contains(chunk) {
                eprintln!("Can only chunk bits by 1, 2, or 4.");
                return;
            }

            show_me_bits(input, *chunk);
        }
        Commands::Signed { input, bits } => {
            if !vec![8, 16, 32, 64].contains(bits) {
                eprintln!("Bits value must be 8, 16, 32, or 64 but found {}.", bits);
                return;
            }

            show_me_signed(input, &bits);
        }
        Commands::Hex { input } => {
            show_me_hex(input);
        }
        Commands::Decimal { input } => {
            show_me_decimal(input);
        }
        Commands::Octal { input } => {
            show_me_octal(input);
        }
    }
}
