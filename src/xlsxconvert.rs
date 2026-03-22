use crate::csvconvert::resolve_output_format;
use crate::csvconvert::serialize_records;
use crate::XlsxOpts;
use anyhow::Context;
use calamine::{open_workbook_auto, Reader};
use std::collections::BTreeMap;
use std::fs;

pub fn process_xlsx(opts: &XlsxOpts) -> anyhow::Result<()> {
    let format = resolve_output_format(opts.format, &opts.output)?;
    let records = read_xlsx_records(opts)?;
    let content = serialize_records(&records, format)?;
    fs::write(&opts.output, content)?;
    Ok(())
}

fn read_xlsx_records(opts: &XlsxOpts) -> anyhow::Result<Vec<BTreeMap<String, String>>> {
    let mut workbook = open_workbook_auto(&opts.input)
        .with_context(|| format!("failed to open workbook: {}", opts.input))?;

    let sheet_name = match &opts.sheet {
        Some(sheet) => sheet.clone(),
        None => workbook
            .sheet_names()
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("workbook contains no sheets"))?,
    };

    let range = workbook
        .worksheet_range(&sheet_name)
        .with_context(|| format!("failed to read sheet: {sheet_name}"))?;

    let mut rows = range.rows();
    let headers = if opts.header {
        rows.next().map(to_row_strings)
    } else {
        None
    };

    let mut records = Vec::with_capacity(range.height());
    for row in rows {
        records.push(row_to_map(headers.as_deref(), &to_row_strings(row)));
    }

    Ok(records)
}

fn to_row_strings<T: ToString>(row: &[T]) -> Vec<String> {
    row.iter().map(ToString::to_string).collect()
}

fn row_to_map(headers: Option<&[String]>, row: &[String]) -> BTreeMap<String, String> {
    match headers {
        Some(headers) => headers
            .iter()
            .zip(row.iter())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
        None => row
            .iter()
            .enumerate()
            .map(|(index, value)| (format!("column{}", index + 1), value.clone()))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::row_to_map;

    #[test]
    fn uses_sheet_headers_as_keys() {
        let headers = vec!["name".to_string(), "age".to_string()];
        let row = vec!["Alice".to_string(), "7".to_string()];
        let mapped = row_to_map(Some(&headers), &row);

        assert_eq!(mapped.get("name").unwrap(), "Alice");
        assert_eq!(mapped.get("age").unwrap(), "7");
    }

    #[test]
    fn generates_keys_without_headers() {
        let row = vec!["Alice".to_string(), "7".to_string()];
        let mapped = row_to_map(None, &row);

        assert_eq!(mapped.get("column1").unwrap(), "Alice");
        assert_eq!(mapped.get("column2").unwrap(), "7");
    }
}
