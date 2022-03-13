use structopt::StructOpt;

use gpxfilter::*;

fn main() {
    let args = Cli::from_args();
    println!("Loading file {:?}.", args.input);

    // Open GPX
    match load_gpx(args.input) {
        Err(e) => {
            println!("{}", e)
        }
        Ok(gpx) => {
            println!("-> Input file has {} waypoints.", gpx.waypoints.len());

            // Filter waypoints
            let mut filtered_gpx = match args.ccs_lower {
                Some(_) => {
                    // if ccs-min parameter is present, filter by ccs count
                    let min = args.ccs_lower.unwrap_or(0);
                    let max = args.ccs_upper.unwrap_or(i32::MAX);
                    filter_wpt_by_ccs_count(min, max, gpx)
                }
                // otherwise filter by provided (or default) regex
                None => filter_wpt_by_description(&args.filter, gpx),
            };

            println!("-> Found {} waypoints.", filtered_gpx.waypoints.len());

            // update symbol, if required
            if !(String::is_empty(&args.symbol)) {
                filtered_gpx = set_symbol(&args.symbol, filtered_gpx);
                println!("-> Updated symbols to {}.", args.symbol);
            }

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
