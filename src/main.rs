mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::io::Read;
use std::path::PathBuf;
use std::fs::File;
use std::str::FromStr;
use std::io::Write;

use args::Mode;
use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;
use clap::Parser;
use crate::args::Args;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn get_path(path: &str) -> PathBuf {
    let image_path: PathBuf = PathBuf::from(path);
    image_path
}

fn encode(path: PathBuf, chunk_type: &str, message: &str, image_option: &bool) -> Result<()> {
    let mut image = File::open(path)?;
    let mut image_bytes: Vec<u8> = Vec::new();
    image.read_to_end(&mut image_bytes)?;

    let mut png = Png::try_from(image_bytes.as_ref())?;

    if !image_option {
        let chunk = Chunk::new(ChunkType::from_str(chunk_type)?, message.as_bytes().to_vec());
        png.append_chunk(chunk);
    } else {
        let image_to_message_path = PathBuf::from(message);
        let mut image_to_message = File::open(image_to_message_path)?;
        let mut image_to_message_bytes: Vec<u8> = Vec::new();
        image_to_message.read_to_end(&mut image_to_message_bytes)?;

        let chunk = Chunk::new(ChunkType::from_str(chunk_type)?, image_to_message_bytes);
        png.append_chunk(chunk);
    };

    let mut new_png = File::create_new("secret.png")?;
    new_png.write_all(png.as_bytes().as_ref())?;

    Ok(())
}

fn decode(path: PathBuf, chunk_type: &str, image_option: &bool) -> Result<()> {
    let mut image = File::open(path)?;
    let mut image_bytes: Vec<u8> = Vec::new();
    image.read_to_end(&mut image_bytes)?;

    let mut png = Png::try_from(image_bytes.as_ref())?;

    if !image_option {
        let chunk = png.remove_first_chunk(chunk_type)?;
        println!("MESSAGE: {}", chunk.data_as_string().unwrap());
    } else {
        let chunk = png.remove_first_chunk(chunk_type)?;
        let image_appended = Png::try_from(chunk.chunk_data.as_ref())?;
        let mut image_appended_writed = File::create("decoded.png")?;
        image_appended_writed.write_all(&image_appended.as_bytes())?;
    };

    Ok(())
}

fn print_message(path: PathBuf) {
    todo!()
}

fn remove_message(path: PathBuf, chunk_type: ChunkType) {
    todo!()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let path = get_path(&args.input_file.unwrap());

    match args.mode {
        Mode::Encode    => encode(path, &args.chunk_type.unwrap(), &args.message.unwrap(), &args.image)?,
        Mode::Decode    => decode(path, &args.chunk_type.unwrap(), &args.image)?,
        Mode::Print     => todo!(),
        Mode::Remove    => todo!(),
        _               => Err("not a valid mode")?
    }

    Ok(())
}
