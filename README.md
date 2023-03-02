# Help flight planning
This is a simple tool made to get the coordinates of some point thanks to
geocoding. This coordinates will be directly formatted to be used in an
aeronautical flightplan.

## Using it
Install (rust)[rust-lang.org] for your system.

Then compile using:
```bash
cargo build --release
```
And run:
```bash
cargo run --release
```
Don't forget to specify the path to a file in which you have entered some
addresses, one per line.
