use std::path::Path;

use crate::{
    display::display_entries,
    error::{AppError, AppResult},
    sequence::Sequence,
};

/// List all the files to console
pub fn list_files<P>(path: P) -> AppResult<()>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(path)?;
    let entries = zipr::nom::iter::zip_entry_iter(&bytes)
        .sequence()
        .map_err(Into::<AppError>::into)?;

    let table = display_entries(entries);
    println!("{}", table);
    Ok(())
}
