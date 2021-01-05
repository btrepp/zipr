use anyhow::Result;
use structopt::StructOpt;

mod list;

#[derive(StructOpt)]
#[structopt(about = "Manipulate zip files")]
enum Zipr {
    #[structopt(about = "List files in a zip file")]
    List {
        #[structopt(help = "The file to open")]
        filename: String,
    },
}

fn main() -> Result<()> {
    let opt = Zipr::from_args();
    match opt {
        Zipr::List { filename } => list::list_files(&filename),
    }
}
