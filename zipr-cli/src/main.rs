mod args;
mod commands;
mod display;
mod error;
mod sequence;
use args::zipr::Opt;
use error::AppResult;
use std::env;

const UNZIP: &str = "unzip";

fn main() -> AppResult<()> {
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
        Opt::List { file } => commands::list_files(file),
        Opt::ShowComment { file } => commands::show_comment(file),
        Opt::Extract {
            file,
            files,
            output,
        } => commands::extract_files(file, files, output),
        Opt::Add {
            file,
            files,
            compress,
        } => commands::add_files(file, files, compress),
        Opt::Inspect {
            file,
            offset,
            kind,
            take,
        } => commands::inspect(file, kind, offset.0, take.map(|x| x.into())),
    }
}
