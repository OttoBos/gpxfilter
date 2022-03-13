use gpx::read;
use gpx::Gpx;
use gpx::Waypoint;
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
    // apply a new symbol to the resulting waypoints
    #[structopt(short = "s", long = "symbol", default_value = "")]
    pub symbol: String,
    // filter for waypoints with at least X CCS chargers
    #[structopt(short = "l", long = "ccs-lower")]
    pub ccs_lower: Option<i32>,
    // filter for waypoints with at most X CCS chargers
    #[structopt(short = "u", long = "ccs-upper")]
    pub ccs_upper: Option<i32>,
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

// filter the waypoints based on a regex on the description
pub fn filter_wpt_by_description(filter: &str, input_gpx: Gpx) -> Gpx {
    let re = Regex::new(filter).unwrap();
    let empty_string = String::new();
    let found_waypoints = input_gpx
        .waypoints
        .into_iter()
        .filter(|wp| re.is_match(&wp.description.as_ref().unwrap_or(&empty_string)))
        .collect();

    Gpx {
        waypoints: found_waypoints,
        ..input_gpx
    }
}

// filter waypoints specifically for a number of CCS chargers, min and max values both inclusive
pub fn filter_wpt_by_ccs_count(min: i32, max: i32, input_gpx: Gpx) -> Gpx {
    let ccs_regex = r"\b(\d+)\b x Combo Typ 2 \(CCS\)";
    let re = Regex::new(ccs_regex).unwrap();
    let found_waypoints = input_gpx
        .waypoints
        .into_iter()
        .filter(|wp| {
            let x = sum_ccs(&wp, &re);
            x >= min && x <= max
        })
        .collect();

    Gpx {
        waypoints: found_waypoints,
        ..input_gpx
    }
}

// helper function to sum all captures
fn sum_ccs(wpt: &Waypoint, re: &Regex) -> i32 {
    let empty_string = String::new();
    re.captures_iter(&wpt.description.as_ref().unwrap_or(&empty_string))
        .map(|m| m.get(1).unwrap().as_str().parse::<i32>().unwrap_or(0))
        .sum()
}

// update name based on CCS count
pub fn set_name_ccs_count(input_gpx: Gpx) -> Gpx {
    let ccs_str = r"\b(\d+)\b x Combo Typ 2 \(CCS\)";
    let ccs_regex = Regex::new(ccs_str).unwrap();
    let name_str = r"^(\d+)kW";
    let name_regex = Regex::new(name_str).unwrap();
    let updated_waypoints = input_gpx
        .waypoints
        .into_iter()
        .map(|wp| {
            let x = sum_ccs(&wp, &ccs_regex);
            let x = format!("{}x", x);
            let mut new_wp = wp.clone();
            new_wp.name = Some(name_regex.replace_all(&new_wp.name.unwrap(), x).to_string());
            new_wp
        })
        .collect();

    Gpx {
        waypoints: updated_waypoints,
        ..input_gpx
    }
}

// update the GPX symbol of the waypoints
pub fn set_symbol(symbol: &str, input_gpx: Gpx) -> Gpx {
    let updated_waypoints = input_gpx
        .waypoints
        .into_iter()
        .map(|wp| {
            let mut new_wp = wp.clone();
            new_wp.symbol = Some(symbol.to_string());
            new_wp
        })
        .collect();

    Gpx {
        waypoints: updated_waypoints,
        ..input_gpx
    }
}

// write GPX to file
pub fn write_gpx(gpx: &Gpx, path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    Ok(gpx::write(&gpx, file)?)
}

// open a file and test for the Byte Order Marker
fn getbom(path: &str) -> Result<Bom, std::io::Error> {
    let mut file = File::open(path)?;
    Ok(Bom::from(&mut file))
}
