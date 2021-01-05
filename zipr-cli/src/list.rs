use anyhow::Result;
use nom::Finish;
use zipr::parsers::central_directory::try_parse_entries;

pub fn list_files(path: &str) -> Result<()> {
    let bytes = std::fs::read(path)?;
    let entries = try_parse_entries(&bytes)
        .finish()
        .map(|(_, entries)| entries)
        .map_err(|e| nom::error::Error::new("Unable to parse", e.code))?;
    for e in entries.iter() {
        println!("{:?}", e.file_name);
    }
    Ok(())
}
