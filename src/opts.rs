use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, help = "Input file", value_parser = verify_input_file)]
    pub input: String,
    #[arg(
        short,
        long,
        help = "Output file",
        default_value = "output/output.json"
    )]
    pub output: String,
    #[arg(short, long, help = "Output format: json, yaml, or toml")]
    pub format: Option<OutputFormat>,
    #[arg(short, long, help = "Delimiter", default_value_t = ','.to_string())]
    pub delimiter: String,
    #[arg(long, help = "Csv has header or not", default_value_t = true)]
    pub header: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if std::path::Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err(format!("{} not found", file))
    }
}
