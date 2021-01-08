use std::path::PathBuf;
use structopt::StructOpt;
use zipr::core::data::CompressionMethod;

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
    #[structopt(about = "Extract from a zip file")]
    Extract {
        #[structopt(help = "The file to open")]
        file: PathBuf,
        #[structopt(help = "Files to extract (optional)")]
        files: Vec<PathBuf>,
        #[structopt(short, long, help = "The output folder", default_value = ".")]
        output: PathBuf,
    },
    #[structopt(about = "Add files to a zip file")]
    Add {
        #[structopt(help = "The file to open")]
        file: PathBuf,
        #[structopt(help = "Files to add to the zip")]
        files: Vec<PathBuf>,

        #[structopt(long, help = "Compression Method", default_value = "Deflate")]
        compress: CompressionMethod,
    },
}

/// Parses the zipr arguments from the command line
pub fn parse_args() -> Opt {
    Opt::from_args()
}
