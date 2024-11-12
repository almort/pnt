use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long, value_enum, required = true)]
    pub mode: Mode,

    #[arg(
        short,
        long,
        required = true,
    )]
    pub input_file: Option<String>,

    #[arg(
        short,
        long,
        required_if_eq("mode", "Mode::Encode"),
        required_if_eq("mode", "Mode::Decode"),
        required_if_eq("mode", "Mode::Remove")
    )]
    pub chunk_type: Option<String>,

    #[arg(
        short,
        long,
        required_if_eq("mode", "Mode::Encode")
    )]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Encode,
    Decode,
    Print,
    Remove
}
