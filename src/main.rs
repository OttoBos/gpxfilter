use regex::Regex;
use structopt::StructOpt;

use gpxfilter::Cli;

/// Load a GPX file and filter the POIs based on some parameters provided

fn main() {
    let args = Cli::from_args();
    println!("Loading file {:?}.", args.path);

    // Open GPX
    let gpx = gpxfilter::load_gpx(args);

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
