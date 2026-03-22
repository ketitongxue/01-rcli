use crate::{CsvOpts, OutputFormat};
use anyhow::{bail, Context};
use csv::{ReaderBuilder, StringRecord};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn process_csv(opts: &CsvOpts) -> anyhow::Result<()> {
    let delimiter = parse_delimiter(&opts.delimiter)?;
    let format = resolve_output_format(opts.format, &opts.output)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(opts.header)
        .from_path(&opts.input)?;
    let mut records = Vec::with_capacity(128);
    let headers = if opts.header {
        Some(rdr.headers()?.clone())
    } else {
        None
    };

    for result in rdr.records() {
        let record = result?;
        let mapped = record_to_map(headers.as_ref(), &record);
        records.push(mapped);
    }

    let content = serialize_records(&records, format)?;
    fs::write(&opts.output, content)?;
    Ok(())
}

pub(crate) fn parse_delimiter(delimiter: &str) -> anyhow::Result<u8> {
    let mut chars = delimiter.chars();
    let Some(ch) = chars.next() else {
        bail!("delimiter cannot be empty");
    };
    if chars.next().is_some() {
        bail!("delimiter must be a single character");
    }
    if !ch.is_ascii() {
        bail!("delimiter must be an ASCII character");
    }
    Ok(ch as u8)
}

pub(crate) fn resolve_output_format(
    format: Option<OutputFormat>,
    output: &str,
) -> anyhow::Result<OutputFormat> {
    if let Some(format) = format {
        return Ok(format);
    }

    match Path::new(output)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("json") => Ok(OutputFormat::Json),
        Some("yaml" | "yml") => Ok(OutputFormat::Yaml),
        Some("toml") => Ok(OutputFormat::Toml),
        Some(ext) => bail!(
            "unsupported output extension '.{}'; use --format json|yaml|toml",
            ext
        ),
        None => Ok(OutputFormat::Json),
    }
}

pub(crate) fn record_to_map(
    headers: Option<&StringRecord>,
    record: &StringRecord,
) -> BTreeMap<String, String> {
    match headers {
        Some(headers) => headers
            .iter()
            .zip(record.iter())
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),
        None => record
            .iter()
            .enumerate()
            .map(|(index, value)| (format!("column{}", index + 1), value.to_string()))
            .collect(),
    }
}

pub(crate) fn serialize_records(
    records: &[BTreeMap<String, String>],
    format: OutputFormat,
) -> anyhow::Result<String> {
    match format {
        OutputFormat::Json => {
            serde_json::to_string_pretty(records).context("failed to serialize records as JSON")
        }
        OutputFormat::Yaml => {
            serde_yaml::to_string(records).context("failed to serialize records as YAML")
        }
        OutputFormat::Toml => toml::to_string_pretty(&TomlRecords { records })
            .context("failed to serialize records as TOML"),
    }
}

#[derive(Serialize)]
struct TomlRecords<'a> {
    records: &'a [BTreeMap<String, String>],
}

#[cfg(test)]
mod tests {
    use super::{parse_delimiter, record_to_map, resolve_output_format, serialize_records};
    use crate::OutputFormat;
    use csv::StringRecord;

    #[test]
    fn infers_format_from_extension() {
        assert_eq!(
            resolve_output_format(None, "output.yaml").unwrap(),
            OutputFormat::Yaml
        );
        assert_eq!(
            resolve_output_format(None, "output.toml").unwrap(),
            OutputFormat::Toml
        );
        assert_eq!(
            resolve_output_format(None, "output").unwrap(),
            OutputFormat::Json
        );
    }

    #[test]
    fn uses_generated_columns_without_headers() {
        let record = StringRecord::from(vec!["Alice", "7"]);
        let mapped = record_to_map(None, &record);

        assert_eq!(mapped.get("column1").unwrap(), "Alice");
        assert_eq!(mapped.get("column2").unwrap(), "7");
    }

    #[test]
    fn serializes_toml_with_records_root_key() {
        let record = StringRecord::from(vec!["Alice", "7"]);
        let records = vec![record_to_map(None, &record)];
        let content = serialize_records(&records, OutputFormat::Toml).unwrap();

        assert!(content.contains("[[records]]"));
        assert!(content.contains("column1 = \"Alice\""));
    }

    #[test]
    fn rejects_invalid_delimiter() {
        assert!(parse_delimiter("::").is_err());
    }
}
