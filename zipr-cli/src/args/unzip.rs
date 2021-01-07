//! https://linux.die.net/man/1/unzip
use super::zipr::Opt as ZiprOpt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "list, test and extract compressed files in a ZIP archive")]
struct Opt {

    /// Input file
    #[structopt(parse(from_os_str), help = "Path of the ZIP archive(s)")]
    zipfile: PathBuf,

    /// Files to extract
    #[structopt(parse(from_os_str), help = "An optional list of archive members to be processed, separated by spaces")]
    files: Vec<PathBuf>,

    #[structopt(short, help = "list archive files (short format)")]
    list: bool,

    #[structopt(short = "z", help = "display archive comment")]
    comment: bool,
}

/// Parses the unzip style args into the zipr opt.
/// This allows us to pretend to be 'unzip' while
/// still using the zipr logic
pub fn parse_args() -> ZiprOpt {
    let opt = Opt::from_args();
    if opt.list {
        ZiprOpt::List { file: opt.zipfile }
    } else if opt.comment {
        ZiprOpt::ShowComment { file: opt.zipfile } 
    } else {
        ZiprOpt::List { file: opt.zipfile }
    }
}
