
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "Encode/decode messages into PNG files", long_about = None)]

pub struct Cli{
    #[command(subcommand)]
    pub commands : Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands{
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove{
        file_path: String,
        chunk_type: String,
    },
    Print{
        file_path: String,
    }

}