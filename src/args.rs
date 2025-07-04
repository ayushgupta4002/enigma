use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "enigma",
    author,
    version,
    about = "Hide secret messages inside PNG images by manipulating ancillary chunks.",
    long_about = "Enigma is a small command-line program written in Rust that allows you to encode, decode, list and remove arbitrary text messages embedded in the ancillary chunks of valid PNG files.\n\nAll chunks created by Enigma are spec-compliant – we calculate the CRC for you so your images stay perfectly valid."
)]
pub struct Cli{
    #[command(subcommand)]
    pub commands : Commands,

}

#[derive(Subcommand, Debug)]
pub enum Commands{
    Encode {
        #[arg(value_name = "FILE_PATH")]
        file_path: String,

        #[arg(value_name = "CHUNK_TYPE")]
        chunk_type: String,

        #[arg(value_name = "MESSAGE")]
        message: String,

        #[arg(value_name = "OUTPUT_FILE")]
        output_file: Option<String>,
    },

    Decode {
        #[arg(value_name = "FILE_PATH")]
        file_path: String,

        #[arg(value_name = "CHUNK_TYPE")]
        chunk_type: String,
    },

    Remove{
        #[arg(value_name = "FILE_PATH")]
        file_path: String,

        #[arg(value_name = "CHUNK_TYPE")]
        chunk_type: String,
    },

    Print{
        #[arg(value_name = "FILE_PATH")]
        file_path: String,
    }
}