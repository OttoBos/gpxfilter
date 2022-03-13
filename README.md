# gpxfilter

Loads and filters waypoints based on a regex (on the waypoint description)

Written to filter GPX files with electric vehicle chargers.

Use: `gpxfilter.exe inputfile outputfile [-cl value] [-cu value] [-f filter] [-s symbol]`

Where:

- `inputfile`: path to file to read from
- `outputfile`: path to file to write to (will be overwritten if exists)
- `--ccs-lower` or `-l`: minimum amount of ccs points to be present (in total for waypoint), leave empty to use `filter` argument
- `--ccs-upper` or `-u`: maximum amount of ccs points. Not used if ccs-lower is not present. If not provided, no upper limit is applied
- `--filter` or `-f`: regex filter to apply. Defaults to all waypoints with multiple CCS chargers
- `--symbol`or `-s`: optional symbol to set filtered waypoints to

Note:

- when using `--ccs-lower`, the names of the waypoints will also be updated to start with the total number of CCS chargers

Examples:

- `gpxfilter.exe ccs.gpx result.gpx -s "Convenience Store"` (applies standard filter & sets symbol)
- `gpxfilter.exe ccs.gpx result.pcs -l 1 -u 1` (returns waypoints with exactly one CCS charger)
