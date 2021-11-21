use gpx::read;
use gpx::Gpx;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;
use unicode_bom::Bom;

#[derive(StructOpt)]
pub struct Cli {
    /// The path to the GPX file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

/// Load a GPX file from the path specified in the arguments
pub fn load_gpx(args: Cli) -> Result<Gpx, Box<dyn std::error::Error>> {
    // detect BOM
    let bom = getbom(&args.path.to_string_lossy())?;

    let file = File::open(args.path)?;
    let mut reader = BufReader::new(file);

    // skip BOM (assume always 3 bytes)
    if bom != Bom::Null {
        let mut x = [0; 3];
        let _y = reader.read_exact(&mut x)?;
    }

    // Open GPX
    Ok(read(reader)?)
}

fn getbom(path: &str) -> Result<Bom, std::io::Error> {
    let mut file = File::open(path)?;
    Ok(Bom::from(&mut file))
}
