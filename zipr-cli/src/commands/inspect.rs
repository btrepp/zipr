use std::path::Path;

use nom::Finish;
use zipr::nom::data::{parse_directory_header, parse_end_of_central_directory, parse_local_file};

use crate::{
    args::zipr::InspectKind,
    error::{AppError, AppResult},
};

pub fn inspect<P: AsRef<Path> + PartialEq>(
    file: P,
    kind: InspectKind,
    offset: usize,
    upto: Option<usize>,
) -> AppResult<()> {
    let bytes = std::fs::read(file)?;
    let slice = match upto {
        None => &bytes[offset..],
        Some(x) => &bytes[offset..offset + x],
    };
    match kind {
        InspectKind::Local => {
            let (_, local) = parse_local_file(slice)
                .finish()
                .map_err(Into::<AppError>::into)?;
            println!("{:x?}", local);
        }
        InspectKind::Dir => {
            let (_, local) = parse_directory_header(slice)
                .finish()
                .map_err(Into::<AppError>::into)?;
            println!("{:x?}", local);
        }
        InspectKind::Eocd => {
            let (_, local) = parse_end_of_central_directory(slice)
                .finish()
                .map_err(Into::<AppError>::into)?;
            println!("{:x?}", local);
        }
    }
    Ok(())
}
