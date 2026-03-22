use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or Convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "xlsx", about = "Convert XLSX to json, yaml, or toml")]
    Xlsx(XlsxOpts),
    #[command(subcommand, name = "base64", about = "Encode or decode base64 data")]
    Base64(Base64SubCommand),
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
    #[arg(long, help = "Csv has header or not", default_value_t = true, action = clap::ArgAction::Set)]
    pub header: bool,
}

#[derive(Debug, Parser)]
pub struct XlsxOpts {
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
    #[arg(long, help = "Worksheet name; defaults to the first sheet")]
    pub sheet: Option<String>,
    #[arg(long, help = "First row is header", default_value_t = true, action = clap::ArgAction::Set)]
    pub header: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Subcommand)]
pub enum Base64SubCommand {
    #[command(about = "Encode plain text or file content as base64")]
    Encode(Base64Opts),
    #[command(about = "Decode base64 text or file content")]
    Decode(Base64Opts),
}

#[derive(Debug, Args)]
#[command(group(
    clap::ArgGroup::new("source")
        .required(true)
        .multiple(false)
        .args(["text", "input"])
))]
pub struct Base64Opts {
    #[arg(short, long, help = "Raw text input")]
    pub text: Option<String>,
    #[arg(short, long, help = "Input file", value_parser = verify_input_file)]
    pub input: Option<String>,
    #[arg(short, long, help = "Output file")]
    pub output: Option<String>,
}

fn verify_input_file(file: &str) -> Result<String, String> {
    if std::path::Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err(format!("{} not found", file))
    }
}
