use std::env;

use anyhow::Result;
mod zipr;
mod unzip;
mod list;

const UNZIP : &str = "unzip";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let head = args.first().map(String::as_str);
    match head {
        Some(UNZIP) => unzip::main(),    
        _ => zipr::main()
    }
}
