//! https://linux.die.net/man/1/unzip
use super::zipr::Opt as ZiprOpt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "list, test and extract compressed files in a ZIP archive")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), help = "Path of the ZIP archive(s)")]
    files: Vec<PathBuf>,

    #[structopt(short, help = "list archive files (short format)")]
    list: bool,
}

/// Parses the unzip style args into the zipr opt.
/// This allows us to pretend to be 'unzip' while
/// still using the zipr logic
pub fn parse_args() -> ZiprOpt {
    let opt = Opt::from_args();
    let files = opt.files;
    if opt.list {
        ZiprOpt::List { files }
    } else {
        ZiprOpt::List { files }
    }
}
