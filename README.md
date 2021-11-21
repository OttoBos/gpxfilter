# gpxfilter

Loads and filters waypoints based on a regex (on the waypoint description)

Written to filter GPX files with electric vehicle chargers.

Use: `gpxfilter.exe inputfile outputfile -f filter`

Where:

- `inputfile`: path to file to read from
- `outputfile`: path to file to write to (will be overwritten if exists)
- `filter` regex filter to apply. Defaults to all waypoints with multiiple CCS chargers
