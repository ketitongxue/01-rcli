# RCLI

`rcli` is a Rust command line tool for:

- converting CSV files to `json`, `yaml`, or `toml`
- converting XLSX files to `json`, `yaml`, or `toml`
- encoding and decoding Base64 text or files

## Build

```bash
cargo build
```

Run the CLI locally with:

```bash
cargo run -- --help
```

## Top-Level Commands

```bash
rcli <COMMAND>
```

Available commands:

- `csv`: convert CSV files
- `xlsx`: convert XLSX files
- `base64`: encode or decode Base64 data

## CSV Conversion

Convert a CSV file into `json`, `yaml`, or `toml`.

### Usage

```bash
rcli csv [OPTIONS] --input <INPUT>
```

### Options

- `-i, --input <INPUT>`: input CSV file
- `-o, --output <OUTPUT>`: output file, default is `output/output.json`
- `-f, --format <FORMAT>`: output format, supports `json`, `yaml`, `toml`
- `-d, --delimiter <DELIMITER>`: CSV delimiter, default is `,`
- `--header <HEADER>`: whether the CSV has a header row, supports `true` or `false`

### Examples

Convert CSV to JSON:

```bash
cargo run -- csv -i assets/juventus.csv -o output/output.json
```

Convert CSV to YAML:

```bash
cargo run -- csv -i assets/juventus.csv -o output/output.yaml
```

Convert CSV to TOML:

```bash
cargo run -- csv -i assets/juventus.csv -o output/output.toml
```

Use a custom delimiter:

```bash
cargo run -- csv -i data/users.txt -o output/users.json -d ';'
```

Specify the format explicitly:

```bash
cargo run -- csv -i assets/juventus.csv -o output/result.data -f yaml
```

When `--header false` is used, output keys are generated automatically as `column1`, `column2`, and so on.

## XLSX Conversion

Convert an Excel `.xlsx` file into `json`, `yaml`, or `toml`.

### Usage

```bash
rcli xlsx [OPTIONS] --input <INPUT>
```

### Options

- `-i, --input <INPUT>`: input XLSX file
- `-o, --output <OUTPUT>`: output file, default is `output/output.json`
- `-f, --format <FORMAT>`: output format, supports `json`, `yaml`, `toml`
- `--sheet <SHEET>`: worksheet name, defaults to the first sheet
- `--header <HEADER>`: whether the first row is treated as a header row, supports `true` or `false`

### Examples

Convert XLSX to JSON:

```bash
cargo run -- xlsx -i assets/juventus.xlsx -o output/output.json
```

Convert XLSX to YAML:

```bash
cargo run -- xlsx -i assets/juventus.xlsx -o output/output.yaml
```

Convert XLSX to TOML:

```bash
cargo run -- xlsx -i assets/juventus.xlsx -o output/output.toml
```

Read from a specific worksheet:

```bash
cargo run -- xlsx -i assets/juventus.xlsx -o output/output.json --sheet Sheet1
```

Disable header handling:

```bash
cargo run -- xlsx -i assets/juventus.xlsx -o output/output.json --header false
```

When `--header false` is used, output keys are generated automatically as `column1`, `column2`, and so on.

## Base64 Encode and Decode

Use the `base64` command to encode plain text or files, and decode Base64 content back to text or files.

### Usage

```bash
rcli base64 <COMMAND>
```

Subcommands:

- `encode`: encode plain text or file content as Base64
- `decode`: decode Base64 text or file content

### Encode

```bash
rcli base64 encode [OPTIONS] <--text <TEXT>|--input <INPUT>>
```

Options:

- `-t, --text <TEXT>`: raw text input
- `-i, --input <INPUT>`: input file
- `-o, --output <OUTPUT>`: output file

Examples:

```bash
cargo run -- base64 encode --text hello
```

```bash
cargo run -- base64 encode --input README.md --output output/readme.b64
```

### Decode

```bash
rcli base64 decode [OPTIONS] <--text <TEXT>|--input <INPUT>>
```

Options:

- `-t, --text <TEXT>`: Base64 text input
- `-i, --input <INPUT>`: input file containing Base64 data
- `-o, --output <OUTPUT>`: output file

Examples:

```bash
cargo run -- base64 decode --text aGVsbG8=
```

```bash
cargo run -- base64 decode --input output/readme.b64 --output output/readme.txt
```

If `--output` is omitted:

- `encode` prints the Base64 result to stdout
- `decode` prints decoded text to stdout when the result is valid UTF-8

## Output Formats

Both `csv` and `xlsx` support these output formats:

- `json`
- `yaml`
- `toml`

If `--format` is not provided, `rcli` tries to infer the format from the output file extension:

- `.json` -> `json`
- `.yaml` or `.yml` -> `yaml`
- `.toml` -> `toml`

If no extension is present, JSON is used by default.

## Test

Run tests with:

```bash
cargo test
```
