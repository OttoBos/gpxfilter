use gpx::read;
use gpx::Gpx;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;
use unicode_bom::Bom;

#[derive(StructOpt)]
pub struct Cli {
    /// The path to the GPX file to read
    #[structopt(parse(from_os_str))]
    pub input: std::path::PathBuf,
    /// The path to the resulting GPX file to write
    #[structopt(parse(from_os_str))]
    pub output: std::path::PathBuf,
    // the regex filter to apply on the waypoint's descriptions
    #[structopt(
        short = "f",
        long = "filter",
        default_value = r"\b([2-9]|1[0-9]|2[0-9]|3[0-9])\b x Combo Typ 2 \(CCS\)"
    )]
    pub filter: String,
}

/// Load a GPX file from the path specified
pub fn load_gpx(path: std::path::PathBuf) -> Result<Gpx, Box<dyn std::error::Error>> {
    // detect BOM
    let bom = getbom(&path.to_string_lossy())?;

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // skip BOM (assume always 3 bytes)
    if bom != Bom::Null {
        let mut x = [0; 3];
        let _y = reader.read_exact(&mut x)?;
    }

    // Open GPX
    Ok(read(reader)?)
}

// filter the waypoints of a gpx
pub fn filter_gpx(filter: &str, input_gpx: Gpx) -> Gpx {
    let my_name = Some(env!("CARGO_PKG_NAME").to_string());

    let re = Regex::new(filter).unwrap();
    let empty_string = String::new();
    let found_waypoints = input_gpx
        .waypoints
        .into_iter()
        .filter(|wp| re.is_match(&wp.description.as_ref().unwrap_or(&empty_string)))
        .collect();
    // let no_metadata: Option<gpx::Metadata> = Some(Default::default());
    // let no_tracks: Vec<gpx::Track> = Vec::new();
    // let no_routes: Vec<gpx::Route> = Vec::new();

    Gpx {
        version: input_gpx.version,
        creator: my_name,
        metadata: input_gpx.metadata,
        waypoints: found_waypoints,
        tracks: input_gpx.tracks,
        routes: input_gpx.routes,
    }
}

pub fn write_gpx(gpx: &Gpx, path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    Ok(gpx::write(&gpx, file)?)
}

fn getbom(path: &str) -> Result<Bom, std::io::Error> {
    let mut file = File::open(path)?;
    Ok(Bom::from(&mut file))
}
