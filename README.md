# gtfs-rt-validator

This tool takes a sequence of Protobuf files that claim to conform to the [GTFS-RT specification](https://developers.google.com/transit/gtfs-realtime/), performs a validation pass on it, and reports violations.

Errors are divided into within-file violations and between-file violations. Within-file violations occur wherever the file does not conform to the spec. Examples include file corruption, missing fields, and missing structs. Between-file violations occur wherever there are unexpected changes between files. Examples include trains going backwards or disappearing from the feed.

## Ontology

Coming soon!
