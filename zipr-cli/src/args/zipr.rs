use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Manipulate zip files")]
pub enum Opt {
    #[structopt(about = "List files in a zip file")]
    List {
        #[structopt(help = "The file to open")]
        file: PathBuf,
    },
    #[structopt(about = "Show zip file comment")]
    ShowComment { file: PathBuf },
    Extract {
        #[structopt(help = "The file to open")]
        file: PathBuf,
        #[structopt(help = "Files to extract (optional)")]
        files: Vec<PathBuf>,
        #[structopt(short, long, help = "The output folder", default_value = ".")]
        output: PathBuf,
    },
}

/// Parses the zipr arguments from the command line
pub fn parse_args() -> Opt {
    let opt = Opt::from_args();
    opt
}
