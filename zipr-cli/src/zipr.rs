use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Manipulate zip files")]
enum Zipr {
    #[structopt(about = "List files in a zip file")]
    List {
        #[structopt(help = "The file to open")]
        filename: PathBuf,
    },
}

pub fn main() -> Result<()> {
    let opt = Zipr::from_args();
    match opt {
        Zipr::List { filename } => crate::list::list_files(&filename),
    }
}
