use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
#[clap(propagate_version = true)]
struct Cli{
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, action)]
    decimal: bool,
}

#[derive(Subcommand)]
enum Commands {
    Bits {
        /// String of input value
        #[clap(value_parser)]
        input: String,

        /// Output value in decimal representation
        #[clap(short, long, action)]
        decimal: Option<bool>,
    },
}

fn show_me_bits(input: &String, decimal: bool) {
    println!("input: {} decimal: {}", input, decimal);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Bits{ input, decimal } => {
            println!("Bits!");
            
            show_me_bits(input, !decimal.is_none());
        }
    }
    println!("Hello, world!");
}
