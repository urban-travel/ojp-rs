# OJP-RS

The `ojp-rs` crate is a Rust library for interacting with Open Journey Planner (OJP) services, a European 
standard for distributed, multimodal journey planning (see <https://opentransportdata.swiss/en/cookbook/open-journey-planner-ojp/>).

## Disclaimer

This crate is very early stage and used for testing purposes. It may or may not be extended in the future.


## Features

* Build and send OJP-compliant requests:
    * TripRequest: Plan journeys across multiple modes.
    * LocationInformationRequest: Search for stops and places.
* Parse XML responses into Rust types.
* Support for OJP v2.0 schema.
* Extensible design for additional OJP services (e.g., FareRequest, TripInfoRequest, StopEventRequest).
* Async support for HTTP requests.

## Use Cases

* Public transport apps
* Backend services for trip planning
* Research and analytics on multimodal transport

## Installation

Add to your `Cargo.toml`:

```console
cargo add ojp-rs
```
## Configuration

* Requires an API key from [opentransportdata.swiss](https://api-manager.opentransportdata.swiss/portal/catalogue-products/tedp_ojp20-1).
* By default the environment variable that must be set is `TOKEN` with the API key.

## Example

An example can be run with:

```console
cargo run --example find_journeys
```

