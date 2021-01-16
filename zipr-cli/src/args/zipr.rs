use std::{fmt::Display, num::ParseIntError, path::PathBuf, str::FromStr};
use structopt::StructOpt;
use zipr::core::data::CompressionMethod;

#[derive(Debug)]
pub enum InspectKind {
    Eocd,
    Dir,
    Local,
}

pub struct Hex(pub usize);
impl From<Hex> for usize {
    fn from(x: Hex) -> Self {
        x.0
    }
}

impl FromStr for Hex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = usize::from_str_radix(s, 16)?;
        Ok(Hex(u))
    }
}

#[derive(Debug)]
pub struct InspectKindParseError(String);

impl Display for InspectKindParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("unrecognize inspect kind: {}", self.0).as_str())
    }
}
impl FromStr for InspectKind {
    type Err = InspectKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eocd" => Ok(InspectKind::Eocd),
            "dir" => Ok(InspectKind::Dir),
            "local" => Ok(InspectKind::Local),
            _ => Err(InspectKindParseError(s.to_owned())),
        }
    }
}

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

    #[structopt(
        about = "Inspect data structures in the zip file. Useful for investigating corrupt files"
    )]
    Inspect {
        #[structopt(help = "The file to open")]
        file: PathBuf,
        #[structopt(long, short, help = "offset to start inspecting")]
        offset: Hex,

        #[structopt(long, short, help = "take x bytes")]
        take: Option<Hex>,

        #[structopt(long, short, help = "Kind to inspect as [eocd,dir,local]")]
        kind: InspectKind,
    },
}

/// Parses the zipr arguments from the command line
pub fn parse_args() -> Opt {
    Opt::from_args()
}
