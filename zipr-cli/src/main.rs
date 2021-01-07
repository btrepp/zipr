mod args;
mod procedural;
use anyhow::Result;
use args::zipr::Opt;
use std::env;

const UNZIP: &str = "unzip";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let head = args.first().map(String::as_str);

    // if arg0 matches a different name, parse those arguments
    // occurs when symlinked as a different name
    let opt = match head {
        Some(UNZIP) => args::unzip::parse_args(),
        _ => args::zipr::parse_args(),
    };

    // Run logic;
    match opt {
        Opt::List { files } => procedural::list_files(files),
    }
}
