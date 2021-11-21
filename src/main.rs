use structopt::StructOpt;

use gpxfilter::{filter_gpx, load_gpx, write_gpx, Cli};

fn main() {
    let args = Cli::from_args();
    println!("Loading file {:?}.", args.input);

    // Open GPX
    match load_gpx(args.input) {
        Err(e) => {
            println!("{}", e)
        }
        Ok(gpx) => {
            println!("Input file has {} waypoints.", gpx.waypoints.len());

            // Filter waypoints
            let filtered_gpx = filter_gpx(&args.filter, gpx);
            println!("Found {} waypoints.", filtered_gpx.waypoints.len());

            // write result to output file
            match write_gpx(&filtered_gpx, args.output) {
                Ok(_) => {
                    println!("Done!")
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    }
}
