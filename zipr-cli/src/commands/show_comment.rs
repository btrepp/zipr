use std::path::Path;

use nom::Finish;
use zipr::{data::borrowed::OEM437Symbols, nom::find_end_of_central_directory};

use crate::error::{AppError, AppResult};

/// Shows the comment of the zip archive
pub fn show_comment<P>(path: P) -> AppResult<()>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(path)?;
    let (_, file) = find_end_of_central_directory(&bytes)
        .finish()
        .map_err(Into::<AppError>::into)?;
    let comment = OEM437Symbols::from(file.comment);
    println!("{}", comment);
    Ok(())
}
