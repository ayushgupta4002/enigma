use crate::args::{Cli, Commands};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod command;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main(){

    let cli = Cli::parse();
    match cli.commands {
        Commands::Encode { file_path, chunk_type, message, output_file } => {
            println!("Encode into: {}", file_path);
            println!("Chunk type: {}", chunk_type);
            println!("Message: {}", message);
            command::encode(&file_path, &chunk_type, &message, output_file.as_deref()).unwrap_or_else(|e| {
                eprintln!("Error encoding: {}", e);
                std::process::exit(1);
            });

            // Call your encode logic here
        }

        Commands::Decode { file_path, chunk_type } => {
            println!("Decode from: {}", file_path);
            println!("Chunk type: {}", chunk_type);
            command::decode(&file_path, &chunk_type).unwrap_or_else(|e| {
                eprintln!("Error decoding: {}", e);
                std::process::exit(1);
            });
            // Call your decode logic here
        }

        Commands::Remove { file_path, chunk_type } => {
            println!("Removing from: {}", file_path);
            println!("Chunk type: {}", chunk_type);
            command::remove(&file_path, &chunk_type).unwrap_or_else(|e| {
                eprintln!("Error removing: {}", e);
                std::process::exit(1);
            });
            // Call your remove logic here
        }

        Commands::Print { file_path } => {
            println!("Printing chunks from: {}", file_path);
            // Call your print logic here
            command::print(&file_path).unwrap_or_else(|e| {
                eprintln!("Error printing: {}", e);
                std::process::exit(1);
            });
        }
    }

}