use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "dcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show CSV, or Convert CSV to other format")]
    Csv(CsvOptions),
}
#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        // Err(format!("File '{}' does not exist", filename))
        Err("File does not exist")
    }
}
