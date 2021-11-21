use gpx::errors::GpxError;
use gpx::read;
use gpx::Gpx;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;
use unicode_bom::Bom;

/// Load a GPX file and filter the POIs based on some parameters provided

fn main() {
    let args = Cli::from_args();
    println!("Loading file {:?}.", args.path);

    // Open GPX
    let gpx: Result<Gpx, GpxError> = load_gpx(args);

    match gpx {
        Err(e) => {
            println!("{}", e)
        }
        Ok(gpx) => {
            println!("Found {} waypoints.", gpx.waypoints.len());
            println!("Waypoints with > 1x CCS:");
            // filter with regex on description
            let re = Regex::new(r"\b([2-9]|1[0-9]|2[0-9]|3[0-9])\b x Combo Typ 2 \(CCS\)").unwrap();
            for wp in gpx.waypoints {
                if re.is_match(&wp.description.unwrap_or_default()) {
                    println!("- {}", wp.name.unwrap_or("Unnamed waypoint".to_string()));
                }
            }
        }
    }
}

#[derive(StructOpt)]
pub struct Cli {
    /// The path to the GPX file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub fn load_gpx(args: Cli) -> Result<Gpx, GpxError> {
    // detect BOM
    let bom = getbom(&args.path.to_string_lossy());

    let file = File::open(args.path).unwrap();
    let mut reader = BufReader::new(file);

    // skip BOM (assume always 3 bytes)
    if bom != Bom::Null {
        let mut x = [0; 3];
        let _y = reader.read_exact(&mut x);
    }

    // Open GPX
    let gpx: Result<Gpx, GpxError> = read(reader);

    gpx
}

fn getbom(path: &str) -> Bom {
    let mut file = File::open(path).unwrap();
    Bom::from(&mut file)
}
