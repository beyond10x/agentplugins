# TODO

## Recording
- [ ] `birdlog add <species> --count N --at <place>` records a sighting with the current time
- [ ] species names are checked against a bundled checklist; unknown names are refused with the closest matches
- [ ] edit and delete a sighting by its number
- [x] store sightings in a single JSON Lines file under the user's data directory

## Looking back
- [ ] list sightings, newest first, filterable by species, place and date range
- [ ] a year list: every species seen this year, with the first date it was seen
- [ ] totals per place

## Sharing
- [ ] export a date range as CSV for the regional survey
- [ ] import the survey's CSV format, skipping rows already present

## Chores
- [ ] the checklist is two years old; find out how often the survey republishes it
- [ ] tests for the storage format before anything else touches it
