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
            let mut filtered_gpx = filter_gpx(&args.filter, gpx);
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
