use std::path::Path;

use anyhow::Result;
use comfy_table::Table;
use nom::Finish;
use zipr::parsers::central_directory::try_parse_entries;

pub fn list_files(path: &Path) -> Result<()> {
    let bytes = std::fs::read(path)?;
    let entries = try_parse_entries(&bytes)
        .finish()
        .map(|(_, entries)| entries)
        .map_err(|e| nom::error::Error::new("Unable to parse", e.code))?;

    let mut table = Table::new();
    let mut total = 0;
    table.set_header(vec!["Length", "Date", "Time", "Name"]);

    for e in entries.iter() {
        let row = vec![
            format!("{}", e.uncompressed_size),
            format!("{}", e.file_modification_date),
            format!("{}", e.file_modification_time),
            format!("{}",e.file_name.to_string_lossy())
        ];
        total += e.uncompressed_size;
        table.add_row(row);
    }

    table.add_row(vec![
        format!("{}", total),
        String::new(),
        String::new(),
        format!("{}",entries.len())
    ]);
    println!("{}",table);
    Ok(())
}
