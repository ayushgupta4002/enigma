use crate::{chunk_type::ChunkType, png::Png, Result};
use std::{fs, str::FromStr};




pub fn encode(file_path: &str, chunk_type: &str, message: &str, output_path: Option<&str>) -> Result<()> {
    let data = message.as_bytes().to_vec();
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let chunk = crate::chunk::Chunk::new(chunk_type, data);
    let input_bytes: Vec<u8> = std::fs::read(file_path)?;
    let mut png = Png::try_from(input_bytes.as_slice())?;
    png.append_chunk(chunk);

    let  output = output_path.unwrap_or(file_path);

    fs::write(output, png.as_bytes())?;
    println!("Encoded message into PNG file: {}", output);
    Ok(())
}

pub fn decode(path: &str, chunk_type: &str) -> Result<()> {
    let input_bytes = fs::read(path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
    
    if let Some(chunk) = png.chunk_by_type(chunk_type) {
        let message = chunk.data_as_string()?;
        println!("Decoded message: {}", message);
        Ok(())
    } else {
        Err("Chunk not found".into())
    }
}

pub fn remove(path: &str, chunk_type: &str) -> Result<()> {
    let input_bytes = fs::read(path)?;
    let mut png = Png::try_from(input_bytes.as_slice())?;
    
    let removed = png.remove_first_chunk(chunk_type)?;
    fs::write(path, png.as_bytes())?;
    println!("Removed chunk: {}", removed.chunk_type());
    Ok(())
}

pub fn print(path: &str) -> Result<()> {
    let input_bytes = fs::read(path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
    
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}