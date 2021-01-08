#[derive(StructOpt, Debug)]
#[structopt(about = "package and compress (archive) files")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str), help = "Path of the ZIP archive(s)")]
    zipfile: PathBuf,

    /// Files to extract
    #[structopt(
        parse(from_os_str),
        help = "Files to append to the zip archive"
    )]
    files: Vec<PathBuf>,

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
        ZiprOpt::Extract {
            file: opt.zipfile,
            files: opt.files,
            output: opt.exdir,
        }
    }
}
