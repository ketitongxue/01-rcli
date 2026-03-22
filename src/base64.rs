use crate::{Base64Opts, Base64SubCommand};
use anyhow::{bail, Context};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::path::Path;

pub fn process_base64(cmd: &Base64SubCommand) -> anyhow::Result<()> {
    match cmd {
        Base64SubCommand::Encode(opts) => encode(opts),
        Base64SubCommand::Decode(opts) => decode(opts),
    }
}

fn encode(opts: &Base64Opts) -> anyhow::Result<()> {
    let input = read_source(opts)?;
    let encoded = STANDARD.encode(input);

    if let Some(output) = &opts.output {
        write_output(output, encoded.as_bytes())?;
    } else {
        println!("{encoded}");
    }

    Ok(())
}

fn decode(opts: &Base64Opts) -> anyhow::Result<()> {
    let input = read_source(opts)?;
    let input = trim_ascii_whitespace(&input);
    let decoded = STANDARD
        .decode(input)
        .context("failed to decode base64 input")?;

    if let Some(output) = &opts.output {
        write_output(output, &decoded)?;
    } else {
        let text = String::from_utf8(decoded)
            .map_err(|_| anyhow::anyhow!("decoded bytes are not valid UTF-8; use --output"))?;
        println!("{text}");
    }

    Ok(())
}

fn read_source(opts: &Base64Opts) -> anyhow::Result<Vec<u8>> {
    if let Some(text) = &opts.text {
        return Ok(text.as_bytes().to_vec());
    }

    if let Some(input) = &opts.input {
        return fs::read(input).with_context(|| format!("failed to read input file: {input}"));
    }

    bail!("either --text or --input is required")
}

fn trim_ascii_whitespace(input: &[u8]) -> &[u8] {
    let start = input
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(input.len());
    let end = input
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map(|index| index + 1)
        .unwrap_or(start);
    &input[start..end]
}

fn write_output(output: &str, content: &[u8]) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(output).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create output directory for {output}"))?;
        }
    }

    fs::write(output, content).with_context(|| format!("failed to write output file: {output}"))
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, trim_ascii_whitespace};
    use crate::Base64Opts;
    use std::fs;

    #[test]
    fn encodes_text_to_file() {
        let output = format!("{}/rcli-base64-encode.txt", std::env::temp_dir().display());
        let opts = Base64Opts {
            text: Some("hello".into()),
            input: None,
            output: Some(output.clone()),
        };

        encode(&opts).unwrap();

        assert_eq!(fs::read_to_string(output).unwrap(), "aGVsbG8=");
    }

    #[test]
    fn decodes_text_to_file() {
        let output = format!("{}/rcli-base64-decode.txt", std::env::temp_dir().display());
        let opts = Base64Opts {
            text: Some("aGVsbG8=".into()),
            input: None,
            output: Some(output.clone()),
        };

        decode(&opts).unwrap();

        assert_eq!(fs::read_to_string(output).unwrap(), "hello");
    }

    #[test]
    fn trims_wrapping_whitespace_before_decode() {
        assert_eq!(trim_ascii_whitespace(b"  hello \n"), b"hello");
    }
}
