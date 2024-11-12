mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use core::panic;
use std::path::PathBuf;
use std::fs::File;

use args::Mode;
use chunk_type::ChunkType;
use clap::Parser;
use crate::args::Args;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn get_image(path: &str) -> File {
    let image_path: PathBuf = PathBuf::from(path);
    let path_display = image_path.display();

    let image = match File::open(&image_path) {
        Err(why) => panic!("couldn't open {}: {}", path_display, why),
        Ok(file) => file
    };

    image
}

fn encode(image: File, chunk_type: ChunkType, message: &str) {
    todo!()
}

fn decode(image: File, chunk_type: ChunkType) {
    todo!()
}

fn print_message(image: File) {
    todo!()
}

fn remove_message(image: File, chunk_type: ChunkType) {
    todo!()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let image = get_image(&args.input_file.unwrap());

    match args.mode {
        Mode::Encode    => todo!(),
        Mode::Decode    => todo!(),
        Mode::Print     => todo!(),
        Mode::Remove    => todo!(),
        _               => Err("not a valid mode")?
    }

    Ok(())
}
