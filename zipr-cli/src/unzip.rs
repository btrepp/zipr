//! https://linux.die.net/man/1/unzip
use std::path::PathBuf;

// Magic mode where we pretend to be the normal unzip command
use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[structopt(about = "list, test and extract compressed files in a ZIP archive")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), help="Path of the ZIP archive(s)")]
    files: Vec<PathBuf>,

    #[structopt(short, help="list archive files (short format)")]
    list: bool,
}


pub fn main() -> Result<()>{
    let opt = Opt::from_args();

    if opt.list {
        for f in opt.files {
            super::list::list_files(&f)?;   
        }    
    }
    Ok(())
}