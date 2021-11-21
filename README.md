# gpxfilter

Loads and filters waypoints based on a regex (on the waypoint description)

Written to filter GPX files with electric vehicle chargers.

Use: `gpxfilter.exe inputfile outputfile -f filter -s symbol`

Where:

- `inputfile`: path to file to read from
- `outputfile`: path to file to write to (will be overwritten if exists)
- `filter`: regex filter to apply. Defaults to all waypoints with multiple CCS chargers
- `symbol`: optional symbol to set filtered waypoints to

Example:

`gpxfilter.exe ccs.gpx result.gpx -s "Convenience Store"`
